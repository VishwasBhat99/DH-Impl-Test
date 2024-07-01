use chrono::NaiveDate;
use chrono::{prelude::*, TimeZone};
use std::{
    collections::{BTreeSet, HashSet},
    default,
};

use super::{
    extract_lines, input_account::ExchangeRateData, macros, read_file, AggregateData,
    ConfigurationParameters, GetDates, HashMap, InputAccount, InputParsedAccount, Logger,
};
use rbdate::{get_days_from_month, incr_dt_by_days, num_days_start_to_end};

pub fn get_data(
    dates: &GetDates,
    config_params: &ConfigurationParameters,
    bal_org: &mut HashMap<String, AggregateData>,
    log: &Logger,
    date_hash_set: &BTreeSet<NaiveDate>,
    exchange_rate_map: &HashMap<String, ExchangeRateData>,
    default_exchange_value: &ExchangeRateData,
    is_exchange_rate_applied: &bool,
    rate_code: &String,
    home_ccy: &String,
) {
    let as_on_date = config_params.as_on_date();
    let inp_file_path = config_params.input_file_path();
    let empty_file_path = inp_file_path.to_string();
    let mut reader = read_file(inp_file_path, &empty_file_path, log);
    let close_date: NaiveDate = // default date
        NaiveDate::parse_from_str("31-12-1999", "%d-%m-%Y").expect("Unable to get Close date.");
    let mut acc_data: HashMap<String, f64> = HashMap::new();

    for (line_num, lines) in reader.deserialize().enumerate() {
        let input_account: InputAccount = extract_lines(line_num, lines, inp_file_path, log);
        let mut account: InputParsedAccount = input_account.parse();
        if acc_data.contains_key(&account.acc_no.to_string()) {
            // account has previous balance
            let no_of_days;
            if account.end_eod_date == account.eod_date {
                //same day transaction
                no_of_days = 0;
            } else if account.end_eod_date != close_date {
                no_of_days = num_days_start_to_end(account.eod_date, account.end_eod_date);
            } else {
                no_of_days = num_days_start_to_end(account.eod_date, as_on_date);
            }
            if is_exchange_rate_applied == &true {
                let mut start_dt = account.eod_date;
                if start_dt.month() < as_on_date.month() || as_on_date.year() > start_dt.year() {
                    start_dt = NaiveDate::from_ymd(as_on_date.year(), as_on_date.month(), 1);
                }
                let mut end_date = account.end_eod_date;
                if end_date == close_date {
                    end_date = as_on_date;
                }
                let mut no_of_days = num_days_start_to_end(start_dt, end_date);
                let base_amt = account.amt;
                let mut new_amt = 0.0;
                while no_of_days >= 0 {
                    let nearest_date = get_nearest_date(date_hash_set, start_dt);
                    let fxd_ccy = &account.acct_crncy_code;
                    let home_ccy = home_ccy;
                    let rate_code = rate_code;
                    let concat_key =
                        format!("{}{}{}{}", nearest_date, fxd_ccy, home_ccy, rate_code);
                    let exc_data = exchange_rate_map
                        .get(&concat_key.clone())
                        .unwrap_or(default_exchange_value);
                    let exc_rate_val = exc_data.fxd_crncy_unit;
                    start_dt = incr_dt_by_days(start_dt, 1);
                    no_of_days -= 1;
                    new_amt += base_amt * exc_rate_val;
                }
                account.amt = new_amt;
            } else {
                account.amt += (account.amt) * (no_of_days as f64);
            }
            if account.int_rt != 0.0 && account.amt == 0.0 {
                account.int_rt = 0.0;
            }
        } else if account.end_eod_date >= dates.start_date
            && account.end_eod_date <= as_on_date
            && account.eod_date < dates.end_date
        {
            let no_of_days = if account.eod_date < dates.start_date {
                num_days_start_to_end(dates.start_date, account.end_eod_date)
            } else {
                num_days_start_to_end(account.eod_date, account.end_eod_date)
            };
            if is_exchange_rate_applied == &true {
                let mut start_dt = account.eod_date;
                if start_dt.month() < as_on_date.month() || as_on_date.year() > start_dt.year() {
                    start_dt = NaiveDate::from_ymd(as_on_date.year(), as_on_date.month(), 1);
                }
                let mut end_date = account.end_eod_date;
                if end_date == close_date {
                    end_date = as_on_date;
                }
                let mut no_of_days = num_days_start_to_end(start_dt, end_date);
                let base_amt = account.amt;
                let mut new_amt = 0.0;
                while no_of_days >= 0 {
                    let nearest_date = get_nearest_date(date_hash_set, start_dt);
                    let fxd_ccy = &account.acct_crncy_code;
                    let home_ccy = home_ccy;
                    let rate_code = rate_code;
                    let concat_key =
                        format!("{}{}{}{}", nearest_date, fxd_ccy, home_ccy, rate_code);
                    let exc_data = exchange_rate_map
                        .get(&concat_key.clone())
                        .unwrap_or(default_exchange_value);
                    let exc_rate_val = exc_data.fxd_crncy_unit;
                    start_dt = incr_dt_by_days(start_dt, 1);
                    no_of_days -= 1;
                    new_amt += base_amt * exc_rate_val;
                }
                account.amt = new_amt;
            } else {
                account.amt += (account.amt) * (no_of_days as f64);
            }
            if account.int_rt != 0.0 && account.amt == 0.0 {
                account.int_rt = 0.0;
            }
        } else if account.end_eod_date >= dates.start_date
            && account.end_eod_date <= as_on_date
            && account.end_eod_date != close_date
            && account.eod_date < as_on_date
        {
            // account created and maintained constant all over the month
            let no_of_days = num_days_start_to_end(account.eod_date, account.end_eod_date);

            if is_exchange_rate_applied == &true {
                let mut start_dt: NaiveDate = account.eod_date;
                if start_dt.month() < as_on_date.month() || as_on_date.year() > start_dt.year() {
                    start_dt = NaiveDate::from_ymd(as_on_date.year(), as_on_date.month(), 1);
                }
                let mut end_date = account.end_eod_date;
                if end_date == close_date {
                    end_date = as_on_date;
                }
                let base_amt = account.amt;
                let mut new_amt = 0.0;
                let mut no_of_days = num_days_start_to_end(start_dt, end_date);
                while no_of_days >= 0 {
                    let nearest_date = get_nearest_date(date_hash_set, start_dt);
                    let fxd_ccy = &account.acct_crncy_code;
                    let home_ccy = home_ccy;
                    let rate_code = rate_code;
                    let concat_key =
                        format!("{}{}{}{}", nearest_date, fxd_ccy, home_ccy, rate_code);
                    let exc_data = exchange_rate_map
                        .get(&concat_key.clone())
                        .unwrap_or(default_exchange_value);
                    let exc_rate_val = exc_data.fxd_crncy_unit;
                    start_dt = incr_dt_by_days(start_dt, 1);
                    no_of_days -= 1;
                    new_amt += base_amt * exc_rate_val;
                }
                account.amt = new_amt;
            } else {
                account.amt *= no_of_days as f64;
            }
            if account.int_rt != 0.0 && account.amt == 0.0 {
                account.int_rt = 0.0;
            }
        } else if account.end_eod_date == close_date {
            // account end eod date is '31-DEC-1999'
            let mut no_of_days = num_days_start_to_end(account.eod_date, as_on_date) + 1;
            if as_on_date != dates.end_date {
                no_of_days -= 1; // exclude that day if asondate is not end of month date
            }
            // account end date is in any month before present month
            if account.eod_date < dates.start_date {
                no_of_days = num_days_start_to_end(dates.start_date, as_on_date) + 1;
            }
            if is_exchange_rate_applied == &true {
                let mut start_dt = account.eod_date;
                if start_dt.month() < as_on_date.month() || as_on_date.year() > start_dt.year() {
                    start_dt = NaiveDate::from_ymd(as_on_date.year(), as_on_date.month(), 1);
                }
                let mut end_date = account.end_eod_date;
                if end_date == close_date {
                    end_date = as_on_date;
                }
                let base_amt = account.amt;
                let mut new_amt = 0.0;
                let mut no_of_days = num_days_start_to_end(start_dt, end_date);
                while no_of_days >= 0 {
                    let nearest_date = get_nearest_date(date_hash_set, start_dt);
                    let fxd_ccy = &account.acct_crncy_code;
                    let home_ccy = home_ccy;
                    let rate_code = rate_code;
                    let concat_key =
                        format!("{}{}{}{}", nearest_date, fxd_ccy, home_ccy, rate_code);
                    let exc_data = exchange_rate_map
                        .get(&concat_key.clone())
                        .unwrap_or(default_exchange_value);
                    let exc_rate_val = exc_data.fxd_crncy_unit;
                    start_dt = incr_dt_by_days(start_dt, 1);
                    no_of_days -= 1;
                    new_amt += base_amt * exc_rate_val;
                }
                account.amt = new_amt;
            } else {
                account.amt *= no_of_days as f64;
            }
        }
        if account.amt != 0.0 {
            account.int_rt *= account.amt;
        }
        acc_data.insert(account.acc_no.to_string(), account.amt);
        let mut builder = AggregateData::new();
        builder.add(&account);
        bal_org
            .entry(account.acc_no.to_string())
            .and_modify(|m| m.add(&account))
            .or_insert(builder);
    }
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
