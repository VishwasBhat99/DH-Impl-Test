use self::aggr_data::AggrData;
use self::aggr_key::AggrKey;
use self::dates::get_aggregation_date_limit;
use self::writer::write_aggr_smry;
use aggregator::bucket::{bucket_eneq_dist_op, bucket_eq_dist_op};
use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::{incr_dt_by_mon_presrv_eom, incr_dt_by_mon_presrv_eom_checked, num_days_start_to_end};
use sdb_agg_rules_txt::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use slog::Logger;
use std::{collections::HashMap, fs, time::SystemTime};
mod aggr_data;
mod aggr_key;
mod bucket;
mod dates;
mod implementation;
mod writer;

pub fn aggregate_cashflows(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) {
    let account_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );
    let (
        mut tot_accounts,
        mut acc_read_succ,
        mut acc_read_fail,
        mut tot_amt_ip,
        mut tot_amt_op,
        tot_no_cf,
    ) = (0 as i64, 0 as i64, 0 as i64, 0.0 as f64, 0.0 as f64, 0);
    let rules = AggRules::new_from_path(config_params.rules_file_path(), &account_reader);
    let mut aggr_map: HashMap<AggrKey, AggrData> = HashMap::new();
    //read the config_file
    let mut config_map: HashMap<String, Vec<i64>> = HashMap::new();
    if !config_params.is_equally_distributed() {
        let config_file = fs::read_to_string(&config_params.config_file_path()).expect(&format!(
            "Failed to read input file  for path : {}",
            config_params.config_file_path()
        ));
        let mut line_no = 0;
        for line in config_file.lines() {
            line_no += 1;
            let config_fields: Vec<&str> = line.split("|").collect();
            config_map.insert(
                config_fields
                    .get(0)
                    .expect(&format!("config_file row no {} doesn't have llg ", line_no))
                    .to_string(),
                config_fields[1..]
                    .iter()
                    .map(|val| val.parse::<i64>().unwrap_or(0))
                    .collect(),
            );
        }
    }
    let as_on_date = config_params.as_on_date();
    log_debug!(logger, "Input file reading started");
    let input_file = fs::read_to_string(&config_params.input_file_path()).expect(&format!(
        "Failed to read input file  for path : {}",
        config_params.input_file_path()
    ));

    for line in input_file.lines() {
        tot_accounts += 1;
        let llg = log_measurements!(
            diag_logger,
            ["Type: GetLLG, Identifier".to_string()],
            implementation::llg_for_txt_account(
                &line.to_string(),
                &rules,
                &account_reader,
                config_params,
            )
        );
        let input_fields = line.split('|').collect::<Vec<&str>>();
        if input_fields.len() < 14 {
            acc_read_fail += 1;
            continue;
        }
        acc_read_succ += 1;
        let mut daywise_amt_map: HashMap<NaiveDate, f64> = HashMap::new();
        let mut monthwise_amt_map: HashMap<NaiveDate, f64> = HashMap::new();
        tot_amt_ip += input_fields[13].parse::<f64>().unwrap_or(0.0);
        group_by_day(
            &as_on_date
                .succ_opt()
                .expect("can not get as_on_date preceding date"),
            &get_aggregation_date_limit(as_on_date, 1),
            input_fields[5].parse().unwrap_or(0.0),
            &mut daywise_amt_map,
        );
        group_by_day(
            &get_aggregation_date_limit(as_on_date, 1)
                .succ_opt()
                .expect("can not get as_on_date preceding date"),
            &get_aggregation_date_limit(as_on_date, 3),
            input_fields[6].parse().unwrap_or(0.0),
            &mut daywise_amt_map,
        );
        group_by_day(
            &get_aggregation_date_limit(as_on_date, 3)
                .succ_opt()
                .expect("can not get as_on_date preceding date"),
            &get_aggregation_date_limit(as_on_date, 6),
            input_fields[7].parse().unwrap_or(0.0),
            &mut daywise_amt_map,
        );
        group_by_month(
            &get_aggregation_date_limit(as_on_date, 6)
                .succ_opt()
                .expect("can not get as_on_date preceding date"),
            3,
            input_fields[8].parse().unwrap_or(0.0),
            *as_on_date,
            &mut monthwise_amt_map,
        );
        group_by_month(
            &get_aggregation_date_limit(as_on_date, 9)
                .succ_opt()
                .expect("can not get as_on_date preceding date"),
            3,
            input_fields[9].parse().unwrap_or(0.0),
            *as_on_date,
            &mut monthwise_amt_map,
        );
        group_by_month(
            &get_aggregation_date_limit(as_on_date, 12)
                .succ_opt()
                .expect("can not get as_on_date preceding date"),
            12,
            input_fields[10].parse().unwrap_or(0.0) / 2.0,
            *as_on_date,
            &mut monthwise_amt_map,
        );
        let above_two_year_amount = input_fields[10].parse().unwrap_or(0.0) / 2.0
            + input_fields[11].parse().unwrap_or(0.0)
            + input_fields[12].parse().unwrap_or(0.0);
        //Aggregation:
        //1. Write to output amounts of each day upto 184 days.
        //2. From 185th day onwards, sum values for each month upto 24 months and write to output.
        //3. Write values after 24 months to single bucket labeled as after 2 years.
        let aggr_key = AggrKey {
            as_on_date: as_on_date.format("%Y-%m-%d").to_string(),
            country: config_params.country().to_string(),
            currency: config_params.currency().to_string(),
            llg: llg.to_string(),
        };
        let dist_vec;
        if config_params.is_equally_distributed() || !config_map.contains_key(input_fields[1]) {
            dist_vec = bucket_eq_dist_op(
                daywise_amt_map,
                monthwise_amt_map,
                above_two_year_amount,
                *as_on_date,
                &mut tot_amt_op,
            );
        } else {
            let config_vec: &Vec<i64> = config_map
                .get(input_fields[1])
                .expect("can not get the llg id in config_map");
            dist_vec = bucket_eneq_dist_op(
                daywise_amt_map,
                monthwise_amt_map,
                &config_vec,
                above_two_year_amount,
                *as_on_date,
                &mut tot_amt_op,
            );
        }
        let aggr_data = AggrData { data: dist_vec };
        aggr_map
            .entry(aggr_key)
            .and_modify(|data| data.append_data(aggr_data.to_owned()))
            .or_insert(aggr_data);
    }
    write_aggr_smry(aggr_map, config_params);

    let health_report = HealthReport::new(
        tot_accounts,
        acc_read_succ,
        acc_read_fail,
        tot_amt_ip,
        tot_amt_op,
        tot_no_cf,
    );
    health_report.gen_health_rpt(config_params.output_file_path());
}
fn group_by_day(
    start_date: &NaiveDate,
    end_date: &NaiveDate,
    amount: f64,
    map: &mut HashMap<NaiveDate, f64>,
) {
    let no_of_days = num_days_start_to_end(*start_date, *end_date) + 1;
    let amount_per_day = if no_of_days > 0 {
        amount / no_of_days as f64
    } else {
        amount
    };
    let mut temp_date = *start_date;
    while temp_date <= *end_date {
        map.insert(temp_date, amount_per_day);
        temp_date = temp_date.succ_opt().expect(&format!(
            "date {}can not be increased by one day",
            temp_date
        ));
    }
}
fn group_by_month(
    start_date: &NaiveDate,
    no_of_months: usize,
    amount: f64,
    _as_on_date: NaiveDate,
    map: &mut HashMap<NaiveDate, f64>,
) {
    let end_dt = incr_dt_by_mon_presrv_eom(*start_date, no_of_months).expect(&format!(
        "can not get the end date from start date :{}",
        start_date
    ));
    let amount_per_months = if no_of_months > 0 {
        amount / no_of_months as f64
    } else {
        amount
    };
    let mut temp_date = *start_date;
    while temp_date < end_dt {
        map.insert(temp_date, amount_per_months);
        temp_date = incr_dt_by_mon_presrv_eom_checked(temp_date, 1).expect(&format!(
            "bucket_date :{} can not be increased by one month",
            temp_date
        ));
    }
}
