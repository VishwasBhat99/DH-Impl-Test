use self::account::AccData;
use self::llg_key::LLGKey;
use self::util::{add_to_prev_data, get_days};
use aggregator::account_field_names::AccFieldNames;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_dyn_proto_rdr::reader;
use slog::Logger;
use std::collections::HashMap;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::time::SystemTime;

mod account;
mod account_field_names;
mod grp_key;
mod llg_key;
mod util;

pub fn aggregate_cashflows(
    config_params: ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) {
    // Prepare data we will require for processing.
    let start_time = SystemTime::now();
    let mut output_file = match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&config_params.output_file_path())
    {
        Ok(create) => create,
        Err(error) => {
            panic!(
                "Could not create file: `{}` on location `{}` : {:?}.",
                config_params.output_file_path(),
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    let keys = AccFieldNames::new_from_path(config_params.known_fields_file_path());
    let ex_rate =
        File::open(config_params.exchange_rate_file()).expect("Could Not Read Exrt Rate File");
    let exrt_reader = BufReader::new(ex_rate);
    let mut exrt_map: HashMap<String, f64> = HashMap::new();
    for (line_no, lines) in exrt_reader.lines().enumerate() {
        let acc_info: String = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    logger,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_params.exchange_rate_file(),
                    line_no + 1,
                    error
                );
                Default::default()
            }
        };
        let exrt_fields: Vec<&str> = acc_info.split('|').collect();
        if exrt_fields[1].to_string().trim().to_uppercase()
            == config_params.base_ccy().trim().to_uppercase()
        {
            exrt_map.insert(
                exrt_fields[0].to_string().trim().to_uppercase(),
                exrt_fields[2].to_string().parse::<f64>().unwrap_or(1.0),
            );
        }
    }
    let mut account_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );

    let mut aggr_data: HashMap<LLGKey, Vec<f64>> = HashMap::new();
    // TODO: Calc dates_bkt_map in init section
    let bkt_info = config_params.bkt_info();
    let bkts: Vec<&str> = bkt_info.split(',').collect();
    let mut num_day_bkts = Vec::new();
    for val in bkts {
        let num_days = get_days(val, config_params.as_on_date());
        num_day_bkts.push(num_days);
    }
    let mut acc_enc = 0;
    let mut acc_succ = 0;
    let mut ip_amt = 0.0;
    let mut op_amt = 0.0;
    for account in account_reader.iter() {
        acc_enc += 1;
        let acc_data: AccData = log_measurements!(
            diag_logger,
            [format!(
                "Type: GetLLG, Identifier: {:?}",
                account.get_string_for_key(&keys.scheme_id).expect("fail")
            )],
            grp_key::fetch_acc_data(
                account,
                &keys,
                &num_day_bkts,
                &config_params,
                logger,
                &mut exrt_map
            )
        );
        for val in &acc_data.acc_data {
            ip_amt += val;
        }
        if aggr_data.contains_key(&acc_data.grp_key) {
            let prev_data: Vec<f64> = aggr_data
                .remove_entry(&acc_data.grp_key)
                .expect("Unexpected unwrap error.")
                .1;
            let accnt_data: Vec<f64> = acc_data.acc_data;
            let new_data: Vec<f64> = add_to_prev_data(prev_data, &accnt_data);
            aggr_data.insert(acc_data.grp_key, new_data);
        } else {
            aggr_data.insert(acc_data.grp_key, acc_data.acc_data);
        }
        acc_succ += 1;
    }
    for (key, data) in aggr_data.drain() {
        write!(
            output_file,
            "{}|{}",
            config_params.as_on_date().format("%d-%m-%Y").to_string(),
            key
        )
        .expect("Unable to write key to summary file.");
        let mut data_op = String::new();
        for val in data {
            op_amt += val;
            data_op.push_str(&val.to_string());
            data_op.push_str("|");
        }
        data_op.pop();
        data_op.push_str("\n");
        write!(output_file, "{}", data_op).expect("Unable to write data to summary file.");
    }
    let health_report = HealthReport::new(acc_enc, acc_succ, acc_enc - acc_succ, ip_amt, op_amt, 0);
    health_report.gen_health_rpt(&config_params.output_file_path());
    let total_duration = print_return_time_since!(start_time);
    log_info!(logger, "Total time for aggregation: {:?}", total_duration);
}
