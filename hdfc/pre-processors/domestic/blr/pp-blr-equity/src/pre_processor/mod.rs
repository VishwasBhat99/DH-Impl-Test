use super::io::reader;
use super::*;
use chrono::{Datelike, NaiveDate};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use pre_processor::structs::account::{get_account_from_line, Account};
use pre_processor::structs::fields_number::{reader_json, FieldsConfig, StdDev};
use pre_processor::structs::output_data::OutputData;
use slog::Logger;
use std::collections::HashMap;
use std::io::BufRead;
use statics::*;

pub mod structs;
pub fn process(config_param: ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let fields_config = reader_json(config_param.fields_file_path(), log);

    let all_accounts = get_required_accounts(&config_param, log, &fields_config);

    let mut output_data = generate_output_data(&config_param, log, &fields_config, &all_accounts);
    let mut use_exchange_rate = false;
    if !config_param.exchange_rate_file_path().is_empty() {
        use_exchange_rate = true;
    }
    if use_exchange_rate {
        convert_accounts1(
            &mut output_data,
            config_param.exchange_rate_file_path(),
            log,
        );
    }
    super::io::write_output(&output_data, config_param.output_file_path());
}

//used for filtering accounts, see fn get_required_accounts
fn should_keep(
    symbol: &str,
    account_date: &NaiveDate,
    as_on_date: &NaiveDate,
    account_symbol: &str,
    fileter_on_symbol: bool,
) -> bool {
    if fileter_on_symbol {
        if account_symbol.to_lowercase().eq(&symbol.to_lowercase())
            && check_same_month(*account_date, *as_on_date)
        {
            true
        } else {
            false
        }
    } else {
        check_same_month(*account_date, *as_on_date)
    }
}

//to check if two month belongs to same month
fn check_same_month(acount_date: NaiveDate, as_on_date: NaiveDate) -> bool {
    if acount_date.year().eq(&as_on_date.year()) && acount_date.month().eq(&as_on_date.month()) {
        true
    } else {
        false
    }
}
/*
1-reads all line
2-filters accounts based on symbol and then on date( date should be
- of same month as as on date)
3- sorts the data based on dates
*/
fn get_required_accounts(
    config_param: &ConfigurationParameters,
    log: &Logger,
    fields_config: &FieldsConfig,
) -> Vec<Account> {
    let mut tot_acc_encntrd = DEFAULT_INT;
    let mut acc_pro_suc = DEFAULT_INT;
    let mut tot_amt = DEFAULT_FLOAT;
    let mut all_accounts: Vec<Account> = Vec::new();
    let input_file_reader = reader(config_param.input_file_path(), log);
    for line in input_file_reader.lines().skip(1) {
        tot_acc_encntrd += 1;
        let each_line = match line {
            Ok(t) => t,
            Err(t) => {
                log_error!(log, "error: {:?}", t);
                continue;
            }
        };
        if each_line.is_empty() {
            continue;
        }
        acc_pro_suc += 1;
        let account: Account = get_account_from_line(
            &each_line.as_str(),
            &fields_config.field,
            fields_config.delimiter.as_str(),
        );
        tot_amt += account.high_price;
        if should_keep(
            config_param.symbol(),
            &account.date,
            &config_param.as_on_date(),
            account.symbol.as_str(),
            fields_config.filter_on_symbol,
        ) {
            all_accounts.push(account);
        }
    }
    all_accounts.sort_by_key(|account| account.date);
    let health_report = HealthReport::new(
        tot_acc_encntrd,
        acc_pro_suc,
        tot_acc_encntrd - acc_pro_suc,
        tot_amt,
        tot_amt,
        DEFAULT_INT,
    );
    health_report.gen_health_rpt(&config_param.output_file_path());

    let report_string = format!(
        "Accounts encountered: {}\n\
         Accounts proccessed suceessfully: {}\n\
         Accounts failed to process: {}",
        tot_acc_encntrd,
        acc_pro_suc,
        tot_acc_encntrd - acc_pro_suc,
    );
    println!("{}",report_string);

    all_accounts
}

//generates required output data
fn generate_output_data(
    config_param: &ConfigurationParameters,
    _log: &Logger,
    fields_config: &FieldsConfig,
    all_accounts: &Vec<Account>,
) -> OutputData {
    let len = all_accounts.len();
    let mut output_data = OutputData::new(
        &config_param,
        &all_accounts[0],
        &all_accounts[len - 1],
        &fields_config.symbol_name.as_str(),
        &fields_config.series_name.as_str(),
    );
    let mut all_close_values: Vec<f64> = Vec::new();
    for i in 1..all_accounts.len() {
        let close_val =
            calc_close_val(all_accounts[i].close_price, all_accounts[i - 1].close_price);
        all_close_values.push(close_val);

        if all_accounts[i].high_price > output_data.high_bal {
            output_data.high_bal = all_accounts[i].high_price;
            output_data.high_date = all_accounts[i].date;
        }
        if all_accounts[i].low_price < output_data.low_bal {
            output_data.low_bal = all_accounts[i].low_price;
            output_data.low_date = all_accounts[i].date;
        }
    }
    output_data.std_dev = calc_std_dev(&all_close_values, &fields_config.std_dev) * 100f64;

    output_data
}
//used for calculating std dev
fn calc_close_val(curr: f64, prev: f64) -> f64 {
    (curr - prev) / prev
}

/*
function to calculate std dev
use_sample = true, will calculate std dev on sample
use_sample = false, will calculate std dev on population
*/
fn calc_std_dev(all_close_val: &Vec<f64>, calc_type: &StdDev) -> f64 {
    if calc_type.use_sample {
        math::stats::standard_deviation(all_close_val.iter(), 1)
    } else {
        math::stats::standard_deviation(all_close_val.iter(), 0)
    }
}

/*
Function to read exchange rate file
and get hashmap<date, rate>
*/
fn get_exchange_rates(exchange_rate_path: &str, log: &Logger) -> HashMap<NaiveDate, f64> {
    let mut exchanges_rates: HashMap<NaiveDate, f64> = HashMap::new();
    let exch_rate_reader = reader(exchange_rate_path, log);
    for line in exch_rate_reader.lines() {
        let each_line = match line {
            Ok(t) => t,
            Err(t) => {
                log_error!(log, "error: {:?}", t);
                continue;
            }
        };
        let fields: Vec<&str> = each_line.split('|').collect();
        if fields.len() == 2 {
            exchanges_rates.insert(
                NaiveDate::parse_from_str(fields[0], "%d-%m-%Y").expect("Invalid date format!"),
                fields[1].parse().unwrap_or(DEFAULT_FLOAT),
            );
        }
    }
    exchanges_rates
}

//converts the output account w.r.t exchange rates

fn convert_accounts1(output_account: &mut OutputData, exchange_rate_path: &str, log: &Logger) {
    let exchange_rate = get_exchange_rates(exchange_rate_path, log);
    output_account.close_bal *= get_ex_rate_for_dt(&exchange_rate, &output_account.close_date, log);
    output_account.low_bal *= get_ex_rate_for_dt(&exchange_rate, &output_account.low_date, log);
    output_account.op_bal *= get_ex_rate_for_dt(&exchange_rate, &output_account.op_date, log);
    output_account.high_bal *= get_ex_rate_for_dt(&exchange_rate, &output_account.high_date, log);
}
fn get_ex_rate_for_dt(
    exchange_rate: &HashMap<NaiveDate, f64>,
    date: &NaiveDate,
    log: &Logger,
) -> f64 {
    if !exchange_rate.contains_key(date) {
        log_error!(
            log,
            "exchange rate for `{}` not found, please check exchange rate file",
            date
        );
    }
    let rate = exchange_rate.get(date).unwrap_or(&1f64);
    *rate
}
