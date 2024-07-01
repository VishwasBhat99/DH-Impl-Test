use self::structs::OutputData;
use self::structs::{AggrData, AggrKey};
use aggregator::account_field_names::AccFieldNames;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::NaiveDate;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::types::Type;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;
use std::time::SystemTime;

mod account_field_names;
mod currency;
pub mod structs;

pub fn aggregate_cashflows(
    config_params: ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) {
    let start_time = SystemTime::now();
    let mut op_path = String::new();
    op_path.push_str(config_params.output_file_path());
    op_path.push_str(".dat");
    let op_file_name = Path::new(&op_path)
        .file_name()
        .expect("Cannot read output file name.")
        .to_str()
        .expect("Cannot convert file name to &str.");
    let mut writer = match buf_file_wrtr(&op_path, None) {
        Ok(wrtr) => wrtr,
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
    let mut ex_rt: f64 = 1.0;
    let currency_map =
        currency::get_exchange_rate(config_params.base_ccy(), config_params.exrt_file_path());
    let keys = AccFieldNames::new_from_path(config_params.req_fields_file_path());
    let mut account_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );
    let field_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );
    let data_origin = config_params.source();
    let as_on_date = config_params.as_on_date();
    let scenario_num = "1";
    let n_acc_id = "1";
    let org_unit_code = "NA";
    let identity_code = "";
    let cf_type = "I";
    let mut header = String::new();
    header.push_str("HDR~");
    header.push_str(&config_params.as_on_date());
    header.push_str("~");
    header.push_str(op_file_name);
    header.push_str("\n");
    write_data(&mut writer, header);
    let all_bucket_years = gen_all_bucket_years(&as_on_date.as_str(), config_params.bucket_years());
    let mut count = 0;
    let mut total_account_enc = 0;
    let mut total_cf_read = 0;
    let mut tot_prin_amt = 0.0;
    let mut tot_int_amt = 0.0;
    let mut tot_prin_amt_in_op = 0.0;
    let mut tot_int_amt_in_op = 0.0;
    for mut account in account_reader.iter() {
        let currency_code = account
            .get_string_for_key(&keys.currency)
            .unwrap_or(&"INR".to_string())
            .to_string();
        total_account_enc += 1;
        let gl_code = match field_reader.get_field_type(&keys.gl_code) {
            Some(typ) => match typ {
                Type::I64 => account
                    .get_i64_for_key(&keys.gl_code)
                    .unwrap_or(0)
                    .to_string(),
                Type::F64 => account
                    .get_f64_for_key(&keys.gl_code)
                    .unwrap_or(0.0)
                    .to_string(),
                Type::String => account
                    .get_string_for_key(&keys.gl_code)
                    .unwrap_or(&"NA".to_string())
                    .to_string(),
                _ => "NA".to_string(),
            },
            None => "NA".to_string(),
        };
        let account_num = match field_reader.get_field_type(&keys.account_num) {
            Some(typ) => match typ {
                Type::I64 => account
                    .get_i64_for_key(&keys.account_num)
                    .unwrap_or(0)
                    .to_string(),
                Type::F64 => account
                    .get_f64_for_key(&keys.account_num)
                    .unwrap_or(0.0)
                    .to_string(),
                Type::String => account
                    .get_string_for_key(&keys.account_num)
                    .unwrap_or(&"".to_string())
                    .to_string(),
                _ => "1".to_string(),
            },
            None => "1".to_string(),
        };
        let currency_type_code = "1";
        let mut cf_seq = 1;
        let mut aggr: HashMap<AggrKey, AggrData> = HashMap::new();
        let mut acc_cf_count = 0;
        let mut cashflows = account
            .remove_cfs_for_key(&keys.cashflows)
            .expect("Error while removing cashflow from the pool of cashflows.");
        if cashflows.len() == 1 {
            log_debug!(
                logger,
                "Account no: {} : {:#?} cashflows.",
                account_num,
                cashflows
            );
        }
        if config_params.is_conversion_required() {
            ex_rt = *currency_map.get(&currency_code).expect(&format!(
                "Could not fetch exchange rate for {}",
                currency_code
            ));
        }
        for cf in cashflows.iter_mut() {
            acc_cf_count += 1;
            total_cf_read += 1;
            let principal_amount = cf.principal_amount * ex_rt;
            let interest_amount = cf.interest_amount;
            tot_prin_amt += principal_amount;
            tot_int_amt += interest_amount;
            let cf_date = naivedate_from_timestamp(cf.get_date());
            let mut fin_ele_code;
            if principal_amount != 0.0 {
                count += 1;
                fin_ele_code = 210;
                let (aggr_key, op) = append_cf(
                    cf_seq,
                    fin_ele_code,
                    &gl_code,
                    &account_num,
                    &currency_type_code,
                    principal_amount,
                    cf_date,
                    data_origin,
                    &as_on_date,
                    &currency_code,
                    scenario_num,
                    org_unit_code,
                    &all_bucket_years,
                    &config_params.source_sys_code(),
                    cf_type,
                    n_acc_id,
                );
                cf_seq += 1;
                let aggr_data = AggrData {
                    data: op,
                    amount: principal_amount,
                };
                if let Some(prev_data) = aggr.get(&aggr_key) {
                    let upt_data = AggrData {
                        data: prev_data.data.clone(),
                        amount: prev_data.amount + principal_amount,
                    };
                    aggr.insert(aggr_key, upt_data);
                } else {
                    aggr.insert(aggr_key, aggr_data);
                }
            }
            if interest_amount != 0.0 {
                count += 1;
                fin_ele_code = 430;
                let (aggr_key, op) = append_cf(
                    cf_seq,
                    fin_ele_code,
                    &gl_code,
                    &account_num,
                    &currency_type_code,
                    interest_amount,
                    cf_date,
                    data_origin,
                    &as_on_date,
                    &currency_code,
                    scenario_num,
                    org_unit_code,
                    &all_bucket_years,
                    &config_params.source_sys_code(),
                    cf_type,
                    n_acc_id,
                );
                cf_seq += 1;
                let aggr_data = AggrData {
                    data: op,
                    amount: interest_amount,
                };
                if let Some(prev_data) = aggr.get(&aggr_key) {
                    let upt_data = AggrData {
                        data: prev_data.data.clone(),
                        amount: prev_data.amount + interest_amount,
                    };
                    aggr.insert(aggr_key, upt_data);
                } else {
                    aggr.insert(aggr_key, aggr_data);
                }
            }
        }
        log_debug!(
            diag_logger,
            "Account no: {} has {} cashflows and {} aggregated data.",
            account_num,
            acc_cf_count,
            aggr.len()
        );
        let (prin_amt_op, int_amt_op) = write_aggr_data(&mut writer, aggr, &all_bucket_years);
        tot_prin_amt_in_op += prin_amt_op;
        tot_int_amt_in_op += int_amt_op;
    }
    let mut footer = String::new();
    footer.push_str("TRL~");
    footer.push_str(&config_params.as_on_date());
    footer.push_str("~");
    footer.push_str(&count.to_string());
    footer.push_str("\n");
    write_data(&mut writer, footer);

    // Info Logs
    log_info!(logger, "Total Account Input: {}", total_account_enc);
    log_info!(logger, "Total Cashflows Input: {}", total_cf_read);
    log_info!(logger, "Total Principal Input: {}", tot_prin_amt);
    log_info!(logger, "Total Interest Input: {}", tot_int_amt);
    log_info!(logger, "Total Principal Output: {}", tot_prin_amt_in_op);
    log_info!(logger, "Total Interest Output: {}", tot_int_amt_in_op);

    let health_stat = HealthReport::new(
        total_account_enc,
        total_account_enc,
        0,
        tot_prin_amt,
        tot_int_amt,
        total_cf_read,
    );
    health_stat.gen_health_rpt(config_params.output_file_path());

    let total_duration = print_return_time_since!(start_time);
    log_info!(logger, "Total time for aggregation: {:?}", total_duration);
}

pub fn write_data(writer: &mut BufWriter<File>, op: String) {
    let output_as_bytes = op.as_bytes();
    match writer.write(output_as_bytes) {
        Ok(_val) => {}
        Err(err) => println!("Error writing to output file. Error: {}", err),
    }
}

pub fn write_aggr_data(
    writer: &mut BufWriter<File>,
    mut op: HashMap<AggrKey, AggrData>,
    all_bucket_years: &Vec<NaiveDate>,
) -> (f64, f64) {
    let mut prin_amt = 0.0;
    let mut int_amt = 0.0;
    for (key, value) in op.drain() {
        if key.cf_type == "Principal" {
            prin_amt += value.amount;
        } else {
            int_amt += value.amount;
        }
        let bucket_date: NaiveDate = all_bucket_years[(key.bucket_id - 1) as usize];
        let final_op = format!(
            "{}~{}~{}~{}~{}~{}~{}~{}~{}~{}~{}~{}~{}~{}~{}\n",
            value.data.account_num,
            value.data.as_on_date,
            value.data.cf_count,
            value.data.scenario_num,
            bucket_date.format("%d/%m/%Y").to_string(),
            value.data.data_origin,
            value.data.cf_type,
            value.amount,
            value.data.fin_ele_type,
            value.data.currency_code,
            value.data.org_unit_code,
            bucket_date.format("%Y%m%d").to_string(),
            value.data.n_acc_id,
            value.data.currency_type_code,
            value.data.gl_code
        );
        let output_as_bytes = final_op.as_bytes();
        match writer.write(output_as_bytes) {
            Ok(_val) => {}
            Err(err) => println!("Error writing to output file. Error: {}", err),
        }
    }
    (prin_amt, int_amt)
}

fn naivedate_from_timestamp(t: i64) -> NaiveDate {
    let naive_date_time = rbdate::NaiveDateTime::from_timestamp(t, 0);
    naive_date_time.date()
}

fn append_cf(
    cf_no: i64,
    fin_ele_code: i64,
    gl_code: &str,
    account_num: &str,
    currency_type_code: &str,
    amount: f64,
    date: NaiveDate,
    data_origin: &str,
    as_on_date: &str,
    currency_code: &str,
    scenario_num: &str,
    org_unit_code: &str,
    all_bucket_years: &Vec<NaiveDate>,
    source_sys_code: &str,
    cf_type: &str,
    n_acc_id: &str,
) -> (AggrKey, OutputData) {
    let modified_acc_num: String = source_sys_code.to_string() + &account_num.to_string();
    let ason: NaiveDate =
        NaiveDate::parse_from_str(as_on_date, "%Y%m%d").expect("Could not parse as on date");
    let fin_ele_type = if fin_ele_code == 210 {
        "P".to_string()
    } else {
        "I".to_string()
    };
    let op = OutputData {
        account_num: modified_acc_num,
        as_on_date: ason.format("%d/%m/%Y").to_string(),
        cf_count: cf_no.to_string(),
        scenario_num: scenario_num.to_string(),
        cf_date: date.format("%d/%m/%Y").to_string(),
        data_origin: data_origin.to_string(),
        cf_type: cf_type.to_string(),
        amount: amount.to_string(),
        fin_ele_type: fin_ele_type,
        currency_code: currency_code.to_string(),
        org_unit_code: org_unit_code.to_string(),
        date: date.format("%Y%m%d").to_string(),
        n_acc_id: n_acc_id.to_string(),
        currency_type_code: currency_type_code.to_string(),
        gl_code: gl_code.to_string(),
    };
    let bucket_year = get_bucket_year_for_date(&date, all_bucket_years);
    let cf_type;
    if fin_ele_code == 210 {
        cf_type = "Principal";
    } else {
        cf_type = "Interest";
    }
    let aggr_key = AggrKey {
        bucket_id: bucket_year,
        cf_type: cf_type.to_string(),
    };
    (aggr_key, op)
}

fn gen_all_bucket_years(as_on_date: &str, years: i32) -> Vec<NaiveDate> {
    let date_parse = rbdate::DateParser::new("%Y%m%d".to_string(), false);
    let as_on_dt = date_parse.parse(as_on_date);
    let mut all_bucket_years: Vec<NaiveDate> = Vec::new();
    for i in 1..=years {
        let next_year = match rbdate::incr_dt_by_mon_presrv_eom_checked(as_on_dt, (i * 12) as usize)
        {
            None => continue,
            Some(data) => data,
        };
        all_bucket_years.push(next_year);
    }
    all_bucket_years
}

fn get_bucket_year_for_date(cf_date: &NaiveDate, all_bucket_years: &Vec<NaiveDate>) -> i32 {
    for (i, curr_date) in all_bucket_years.iter().enumerate() {
        if cf_date <= curr_date {
            return (i as i32) + 1;
        }
    }
    -1
}

#[test]
fn test_bucketing_years() {
    let all_bucket_years = gen_all_bucket_years("20200531", 100);
    print!("{}", all_bucket_years[0]);
    let cf_date = NaiveDate::parse_from_str("20200724", "%Y%m%d").unwrap();
    assert_eq!(get_bucket_year_for_date(&cf_date, &all_bucket_years), 1);
}
