use crate::configuration_parameters::ConfigurationParameters;
use crate::process::config::{BALMFCField, BillsFields};
use crate::process::input_account::*;
use crate::process::output_account::{format_output, get_writer, OutputField};
use chrono::prelude::*;
use health_report::HealthReport;
use rbdate::get_days_from_month;
use slog::Logger;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::{fs, io::Write};
mod config;
mod input_account;
mod output_account;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let mut acc_enc = 0;
    let mut acc_proc = 0;
    let mut ip_amt = 0.0;
    let mut op_amt = 0.0;
    let files_config = config::get_files(config_params.config_file_path());
    let mut bills_file_details: BillsFields = Default::default();
    let mut balm_eit_file_details: BALMFCField = Default::default();

    let mut local__ccy = "".to_string();
    let mut is_exchange_rate_applied = false;
    let mut rate_code = "".to_string();
    let def_multiplier = 1.0;
    for config_fields in files_config.files {
        bills_file_details = config_fields.bills_fields;
        balm_eit_file_details = config_fields.balm_fc_eit_fields;
        local__ccy = config_fields.local_ccy;
        is_exchange_rate_applied = config_fields.is_exchange_rate_applied;
        rate_code = config_fields.rate_code;
    }
    //Assigning default value
    let default_value_ex: ExchangeRateData = ExchangeRateData {
        rtlist_num: String::default(),
        rtlist_date: NaiveDate::from_ymd(1999, 1, 1),
        fxd_crncy_code: String::default(),
        var_crncy_code: String::default(),
        rate_code: String::default(),
        fxd_crncy_unit: 1.0,
    };
    //Reading balm eit file
    let mut balm_eit_map: HashMap<String, f64> = HashMap::new();
    let balm_eit_file_reader = fs::read_to_string(&balm_eit_file_details.balm_fc_eit_file_path)
        .expect("Could not read `Balm eit file path`");
    let eit_id_index = balm_eit_file_details.entity_id_index;
    let eit_int_rate_index = balm_eit_file_details.int_rate_index;
    for (line_no, line) in balm_eit_file_reader.lines().enumerate().skip(0) {
        let balm_eit_vec: Vec<&str> = line.split('|').collect::<Vec<&str>>();
        let entity_id = get_str(
            &balm_eit_file_details.balm_fc_eit_file_path.to_string(),
            &balm_eit_vec,
            (eit_id_index - 1) as usize,
            line_no,
        );
        let int_rate_str = get_str(
            &balm_eit_file_details.balm_fc_eit_file_path.to_string(),
            &balm_eit_vec,
            (eit_int_rate_index - 1) as usize,
            line_no,
        );
        let int_rate = int_rate_str.parse::<f64>().unwrap_or(0.0);
        balm_eit_map.insert(entity_id, int_rate);
    }

    // Reading exchange rate file

    let default_value_ex: ExchangeRateData = ExchangeRateData {
        rtlist_num: String::default(),
        rtlist_date: NaiveDate::from_ymd(1999, 1, 1),
        fxd_crncy_code: String::default(),
        var_crncy_code: String::default(),
        rate_code: String::default(),
        fxd_crncy_unit: def_multiplier,
    };
    let mut exchange_rate_map: HashMap<String, ExchangeRateData> = HashMap::new();
    let mut date_hash_set: BTreeSet<NaiveDate> = BTreeSet::new();

    let exchange_rate_reader = fs::read_to_string(config_params.balm_fc_rth_file_path())
        .expect("Could not read `Balm rth file path`");
    let first_day_of_curr_month = NaiveDate::from_ymd(
        config_params.as_on_date().year(),
        config_params.as_on_date().month(),
        1,
    );
    let prev_month_end_date = first_day_of_curr_month - chrono::Duration::days(1);
    let as_on_date_no_hyphen = config_params.as_on_date().format("%d%m%Y").to_string();
    let prev_date_no_hyphen = prev_month_end_date.format("%d%m%Y").to_string();
    let prev_rth_path = config_params
        .balm_fc_rth_file_path()
        .replace(&as_on_date_no_hyphen, &prev_date_no_hyphen);
    let prev_exchange_rate_reader =
        fs::read_to_string(prev_rth_path).expect("Could not read `Prev rth file path`");
    if is_exchange_rate_applied == true {
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

        for (line_no, line) in exchange_rate_reader.lines().enumerate().skip(0) {
            let exchange_file_vec: Vec<&str> = line.split('|').collect::<Vec<&str>>();
            let exchange_rate_data: ExchangeRateData = ExchangeRateData::new(
                config_params,
                &exchange_rate_reader,
                &exchange_file_vec,
                line_no + 1,
            );
            let rtlist_num = exchange_rate_data.rtlist_num.to_string();
            let rtlist_date = exchange_rate_data.rtlist_date.clone();
            let fxd_crncy_code = exchange_rate_data.fxd_crncy_code.to_string();
            let var_crncy_code = exchange_rate_data.var_crncy_code.to_string();
            let rate_code = exchange_rate_data.rate_code.to_string();
            let fxd_crncy_unit = exchange_rate_data.fxd_crncy_unit.clone();
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
    }
    // Reading Bills daily file
    let as_on_date = *config_params.as_on_date();
    let days = get_days_from_month(as_on_date);
    let curr_year = as_on_date.year();
    let month = as_on_date.month();
    let days_in_month = get_days_from_month(as_on_date);
    let mut start_day = 1;
    let mut bills_map: HashMap<String, BillsFile> = HashMap::new();
    while start_day <= days_in_month {
        let curr_date = start_day as u32;
        let ref_start_date = NaiveDate::from_ymd(curr_year, month, curr_date);
        let formated_ref_date = ref_start_date.format("%d%m%Y").to_string();
        let bills_daily_file_path = format!(
            "{}_{}.txt",
            &bills_file_details.bills_daily_file_path, formated_ref_date
        );
        let bills_file_reader = fs::read_to_string(bills_daily_file_path).expect(&format!(
            "Could not read `Bills file path` for date : {}",
            formated_ref_date
        ));
        for (line_no, line) in bills_file_reader.lines().enumerate().skip(0) {
            acc_enc += 1;
            let bills_file_vec: Vec<&str> = line.split('|').collect::<Vec<&str>>();
            let bills_data: BillsFile = BillsFile::new(
                bills_file_details.to_owned(),
                &bills_file_reader,
                &bills_file_vec,
                line_no + 1,
            );
            let bills_id = bills_data.bills_id.to_string();
            let mut amt = bills_data.bills_amt_inr;
            let ccy_code = bills_data.acct_crncy_code.to_string();
            let mut temp_exc_flag = is_exchange_rate_applied;
            if ccy_code.trim().to_uppercase() == "INR" {
                temp_exc_flag = false;
            }
            if temp_exc_flag == true && is_exchange_rate_applied == true {
                let nearest_date = get_nearest_date(&date_hash_set, ref_start_date);
                let fxd_ccy = bills_data.acct_crncy_code;
                let var_ccy = local__ccy.to_string();
                let rate_code = rate_code.to_string();
                let concat_key = format!("{}{}{}{}", nearest_date, fxd_ccy, var_ccy, rate_code);
                let exc_data = exchange_rate_map
                    .get(&concat_key.clone())
                    .unwrap_or(&default_value_ex);
                let exc_rate_val = exc_data.fxd_crncy_unit;
                amt = amt * exc_rate_val;
            }
            let curr_bills_data: BillsFile = BillsFile {
                bills_id: bills_id.to_string(),
                bills_amt_inr: amt,
                acct_crncy_code: ccy_code,
            };
            bills_map
                .entry(bills_id)
                .and_modify(|bill_data| {
                    bill_data.bills_amt_inr += amt;
                })
                .or_insert(curr_bills_data);
        }
        start_day += 1;
    }
    let mut op_writer = get_writer(config_params.output_file());
    for (bills_id, bills_data) in bills_map {
        acc_proc += 1;
        let mut acct_bal = bills_data.bills_amt_inr;
        let wt_int_rate = balm_eit_map.get(&bills_id.clone()).unwrap_or(&0.0);
        let output_data: OutputField = OutputField {
            acid: bills_id.to_string(),
            avg_bal: bills_data.bills_amt_inr / days_in_month as f64,
            wt_int_rate: *wt_int_rate,
            int_amt: "".to_string(),
        };
        writeln!(op_writer, "{}", format_output(output_data)).expect("Error in Writing Output");
    }

    let health_report = HealthReport::new(acc_enc, acc_proc, 0, 0.0, 0.0, 0);
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file());
}
pub fn get_nearest_date(set: &BTreeSet<NaiveDate>, target_date: NaiveDate) -> NaiveDate {
    if set.contains(&target_date) {
        return target_date;
    }
    let mut iter = set.iter().rev();
    while let Some(value) = iter.next() {
        if value < &target_date {
            return *value;
        }
    }
    target_date
}
