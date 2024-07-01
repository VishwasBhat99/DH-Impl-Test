use crate::aggregator::input_account::ExchangeRateData;

use self::get_dates::GetDates;
use self::input_account::{InputAccount, InputParsedAccount};
use self::io::*;
use self::reader::get_data;
use self::structs::AggregateData;
use self::writer::get_op_line;
use chrono::{Datelike, Duration, NaiveDate};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use slog::Logger;
use statics::*;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::default::Default;
use std::fs;
use std::time::SystemTime;
mod config;
mod get_dates;
mod input_account;
mod io;
mod reader;
mod structs;
mod writer;

pub fn process(config_params: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_read_timer = SystemTime::now();
    let dates = GetDates::new(config_params.as_on_date());
    let mut writer = get_writer(config_params.output_file_path());
    let files_config = config::get_files(config_params.config_file_path());
    let mut ttl_amt = DEFAULT_FLOAT;
    let mut bal_org: HashMap<String, AggregateData> = HashMap::new();
    let mut curr_rth_path = "".to_string();
    let mut is_exchange_rate_applied = false;
    let mut rate_code = "".to_string();
    let mut home_crcy = "".to_string();
    let mut def_multiplier = 1.0;
    for config_fields in files_config.files {
        curr_rth_path = config_fields.curr_rth_file_path;
        is_exchange_rate_applied = config_fields.is_exchange_rate_applied;
        rate_code = config_fields.rate_code;
        home_crcy = config_fields.home_crcy;
        def_multiplier = config_fields.def_multiplier;
    }
    let default_value_ex: ExchangeRateData = ExchangeRateData {
        rtlist_num: String::default(),
        rtlist_date: NaiveDate::from_ymd(1999, 1, 1),
        fxd_crncy_code: String::default(),
        var_crncy_code: String::default(),
        rate_code: String::default(),
        fxd_crncy_unit: def_multiplier,
    };
    let first_day_of_curr_month = NaiveDate::from_ymd(
        config_params.as_on_date().year(),
        config_params.as_on_date().month(),
        1,
    );
    let prev_month_end_date = first_day_of_curr_month - chrono::Duration::days(1);
    let as_on_date_no_hyphen = config_params.as_on_date().format("%d%m%Y").to_string();
    let prev_date_no_hyphen = prev_month_end_date.format("%d%m%Y").to_string();
    let prev_rth_path = curr_rth_path.replace(&as_on_date_no_hyphen, &prev_date_no_hyphen);
    let mut exchange_rate_map: HashMap<String, ExchangeRateData> = HashMap::new();
    let mut date_hash_set: BTreeSet<NaiveDate> = BTreeSet::new();
    let prev_exchange_rate_reader =
        fs::read_to_string(prev_rth_path).expect("Could not read `Prev rth file path`");
    let curr_exchange_rate_reader =
        fs::read_to_string(curr_rth_path).expect("Could not read `Curr rth file path`");

    for (line_no, line) in prev_exchange_rate_reader.lines().enumerate().skip(0) {
        let prev_exchange_vec: Vec<&str> = line.split('|').collect::<Vec<&str>>();
        let exchange_rate_data: ExchangeRateData = ExchangeRateData::new(
            &config_params,
            &prev_exchange_rate_reader,
            &prev_exchange_vec,
            line_no + 1,
        );
        let rtlist_num = exchange_rate_data.rtlist_num.to_string();
        let rtlist_date = exchange_rate_data.rtlist_date;
        let fxd_crncy_code = exchange_rate_data.fxd_crncy_code.to_string();
        let var_crncy_code = exchange_rate_data.var_crncy_code.to_string();
        let rate_code = exchange_rate_data.rate_code.to_string();
        let fxd_crncy_unit = exchange_rate_data.fxd_crncy_unit;
        let concat = format!(
            "{}{}{}{}",
            rtlist_date, fxd_crncy_code, var_crncy_code, rate_code
        );
        exchange_rate_map
            .entry(concat.clone())
            .and_modify(|prev_exchange_rate_data| {
                if prev_exchange_rate_data.rtlist_num <= rtlist_num {
                    prev_exchange_rate_data.rtlist_num = rtlist_num;
                    prev_exchange_rate_data.fxd_crncy_unit = fxd_crncy_unit
                }
            })
            .or_insert(exchange_rate_data);
        date_hash_set.insert(rtlist_date);
    }

    for (line_no, line) in curr_exchange_rate_reader.lines().enumerate().skip(0) {
        let curr_exchange_vec: Vec<&str> = line.split('|').collect::<Vec<&str>>();
        let exchange_rate_data: ExchangeRateData = ExchangeRateData::new(
            &config_params,
            &curr_exchange_rate_reader,
            &curr_exchange_vec,
            line_no + 1,
        );
        let rtlist_num = exchange_rate_data.rtlist_num.to_string();
        let rtlist_date = exchange_rate_data.rtlist_date;
        let fxd_crncy_code = exchange_rate_data.fxd_crncy_code.to_string();
        let var_crncy_code = exchange_rate_data.var_crncy_code.to_string();
        let rate_code = exchange_rate_data.rate_code.to_string();
        let fxd_crncy_unit = exchange_rate_data.fxd_crncy_unit;
        let concat = format!(
            "{}{}{}{}",
            rtlist_date, fxd_crncy_code, var_crncy_code, rate_code
        );
        exchange_rate_map
            .entry(concat.clone())
            .and_modify(|prev_exchange_rate_data| {
                if prev_exchange_rate_data.rtlist_num <= rtlist_num {
                    prev_exchange_rate_data.rtlist_num = rtlist_num;
                    prev_exchange_rate_data.fxd_crncy_unit = fxd_crncy_unit
                }
            })
            .or_insert(exchange_rate_data);
        date_hash_set.insert(rtlist_date);
    }
    get_data(
        &dates,
        &config_params,
        &mut bal_org,
        log,
        &date_hash_set,
        &exchange_rate_map,
        &default_value_ex,
        &is_exchange_rate_applied,
        &rate_code,
        &home_crcy,
    );
    let end_read_timer = SystemTime::now();
    let duration = end_read_timer
        .duration_since(start_read_timer)
        .expect("Could not calculate total duration read timer.");
    debug!(
        diag_log,
        "Reading Reference Files, Total Duration: {:?}.", duration
    );

    get_op_line(
        dates.no_of_days as f64,
        &mut bal_org,
        &mut ttl_amt,
        &mut writer,
    );
    let health_report = HealthReport::new(
        bal_org.len() as i64,
        bal_org.len() as i64,
        DEFAULT_INT,
        ttl_amt,
        ttl_amt,
        DEFAULT_INT,
    );
    health_report.gen_health_rpt(config_params.output_file_path());

    let report_string = format!(
        "Accounts encountered: {}\n\
         Accounts proccessed suceessfully: {}\n\
         Accounts failed to process: {}",
        bal_org.len(),
        bal_org.len(),
        DEFAULT_INT,
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);
}
