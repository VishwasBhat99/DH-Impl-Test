use chrono::NaiveDateTime;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use process::passthrough::AccFieldNames;
use rbdate::NaiveDate;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::env;
use std::io::prelude::*;

mod currency;
mod passthrough;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let keys = AccFieldNames::new_from_path(config_params.req_fields_file_path());
    let mut account_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );
    let mut writer = match buf_file_wrtr(&config_params.output_file_path(), None) {
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
    // Init currency converter
    let currency_converter = currency::create_currency_converter(
        config_params.home_currency(),
        config_params.exchange_rate_file_path(),
    );
    let mut acc_enc = 0;
    let mut acc_succ = 0;
    let mut ip_amt = 0.0;
    for account in account_reader.iter() {
        acc_enc += 1;
        let ccy = account
            .get_string_for_key(&keys.ccy)
            .expect("Cannot get ccy for account.");
        let initial_bal = account
            .get_f64_for_key(&keys.initial_bal)
            .expect("Cannot get initial bal for account.");
        ip_amt += initial_bal;
        let conv_bal =
            currency_converter.convert(&ccy, &initial_bal, config_params.is_consolidated(), logger);
        let bal;
        let bal_lcy;
        if config_params.is_consolidated() {
            bal = conv_bal;
            bal_lcy = initial_bal;
        } else {
            bal = initial_bal;
            bal_lcy = conv_bal;
        }
        let acc_open_timestamp = account
            .get_i64_for_key(&keys.acc_open_dt)
            .expect("Cannot get acc open date for account.");
        let acc_open_date = naivedate_from_timestamp(acc_open_timestamp);
        if acc_open_date == *config_params.incr_date() && bal_lcy >= *config_params.threshold_bal()
        {
            let op = get_op_line(&account, &keys, &config_params, bal, bal_lcy, &ccy);
            match writer.write(op.as_bytes()) {
                Ok(_val) => {}
                Err(err) => println!("Error writing to output file. Error: {}", err),
            }
        } else {
            continue;
        }
        acc_succ += 1;
    }
    let health_report = HealthReport::new(acc_enc, acc_succ, acc_enc - acc_succ, ip_amt, ip_amt, 0);
    health_report.gen_health_rpt(&config_params.output_file_path());
}

pub fn get_op_line(
    account: &AccountWithCFs,
    keys: &AccFieldNames,
    config_params: &ConfigurationParameters,
    bal: f64,
    bal_lcy: f64,
    ccy: &str,
) -> String {
    let mut op = String::new();
    let prod_code = match account.get_string_for_key(&keys.prod_code) {
        Ok(val) => val.to_string(),
        Err(_) => "NA".to_string(),
    };
    let acc_id = match account.get_string_for_key(&keys.acc_id) {
        Ok(val) => val.to_string(),
        Err(_) => "NA".to_string(),
    };
    let cust_id = account.get_i64_for_key(&keys.cust_id).unwrap_or(0);
    let cust_name = match account.get_string_for_key(&keys.cust_name) {
        Ok(val) => val.to_string(),
        Err(_) => "NA".to_string(),
    };
    let division = match account.get_string_for_key(&keys.division) {
        Ok(val) => val.to_string(),
        Err(_) => "NA".to_string(),
    };
    let benchmark = match account.get_string_for_key(&keys.benchmark) {
        Ok(val) => val.to_string(),
        Err(_) => "NA".to_string(),
    };
    let alm_line = match account.get_string_for_key(&keys.alm_line) {
        Ok(val) => val.to_string(),
        Err(_) => "NA".to_string(),
    };
    let value_date_i64 = account.get_i64_for_key(&keys.acc_open_dt).unwrap_or(0);
    let mat_date_i64 = account.get_i64_for_key(&keys.mat_date).unwrap_or(0);
    let roi = account.get_f64_for_key(&keys.roi).unwrap_or(0.0);
    let value_date = naivedate_from_timestamp(value_date_i64);
    let mat_date = naivedate_from_timestamp(mat_date_i64);

    op.push_str(config_params.rpt_id());
    op.push('|');
    op.push_str(&config_params.incr_date().format("%d-%m-%Y").to_string());
    op.push('|');
    op.push_str(&acc_id);
    op.push('|');
    op.push_str(&cust_id.to_string());
    op.push('|');
    op.push_str(&cust_name);
    op.push('|');
    op.push_str(&division);
    op.push('|');
    op.push_str(&alm_line);
    op.push('|');
    op.push_str(&prod_code);
    op.push('|');
    op.push_str(&benchmark);
    op.push('|');
    op.push_str(&value_date.format("%d-%m-%Y").to_string());
    op.push('|');
    op.push_str(&mat_date.format("%d-%m-%Y").to_string());
    op.push('|');
    op.push_str(&bal.to_string());
    op.push('|');
    op.push_str(&bal_lcy.to_string());
    op.push('|');
    op.push_str(ccy);
    op.push('|');
    op.push_str(&roi.to_string());
    op.push('\n');

    op
}
pub fn naivedate_from_timestamp(t: i64) -> NaiveDate {
    let naive_date_time = NaiveDateTime::from_timestamp(t, 0);
    naive_date_time.date()
}
