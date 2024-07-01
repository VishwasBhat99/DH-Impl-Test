use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::Cashflow;
use cashflow_generator::get_holiday_data::*;
use chrono::Datelike;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::{
    incr_dt_by_days, increment_date_by_months_unchecked, num_days_start_to_end, timestamp,
    DateParser, NaiveDate,
};
use slog::Logger;
use std::collections::HashMap;

pub fn generate_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
    res_file_date: &HashMap<String, Vec<Vec<String>>>,
    holiday_map: &mut HashMap<NaiveDate, String>,
    as_on_date: NaiveDate,
) -> Result<Vec<Cashflow>, String> {
    let mut cf_vec: Vec<Cashflow> = Vec::new();
    let mut prdem_cf_vec: Vec<Cashflow> = Vec::new();
    let mut indem_cf_vec: Vec<Cashflow> = Vec::new();
    let mut res_data: HashMap<String, Vec<Vec<String>>> = HashMap::new();
    let date_parser = DateParser::new("%d-%m-%Y".to_string(), true);

    let mut final_prdem_cf_dates: Vec<NaiveDate> = Vec::new();
    let mut final_indem_cf_dates: Vec<NaiveDate> = Vec::new();

    let acc_start_date = account.acct_opn_date.unwrap_or(*config_params.as_on_date());
    let mut closing_out_bal = account.dis_amt;
    let clr_bal = account.clr_bal_amt;
    let int_rate = account.int_rate;
    let mut prev_date = acc_start_date;
    let default_res_data = vec![vec![
        "NA".to_string(),
        "0".to_string(),
        "0".to_string(),
        as_on_date.format("%d-%m-%Y").to_string(),
        account.clr_bal_amt.to_string(),
        "NA".to_string(),
        "0".to_string(),
        "0".to_string(),
        "PRDEM".to_string(),
        "".to_string(),
        0.0.to_string(),
        0.to_string(),
        "".to_string(),
    ]];
    let acc_res_data = res_file_date
        .get(&account.acid)
        .unwrap_or(&default_res_data);
    for acc in acc_res_data {
        let freq = get_freq(&acc[5]);
        if res_data.contains_key(&acc[8]) {
            res_data.get_mut(&acc[8]).as_mut().unwrap().push(vec![
                acc[3].to_string(),
                acc[4].to_string(),
                freq,
                acc[10].to_string(),
                acc[11].to_string(),
                acc[12].to_string(),
            ]);
        } else {
            res_data.insert(
                acc[8].to_string(),
                vec![vec![
                    acc[3].to_string(),
                    acc[4].to_string(),
                    freq,
                    acc[10].to_string(),
                    acc[11].to_string(),
                    acc[12].to_string(),
                ]],
            );
        }
    }

    let mut prdem_prin_amt: HashMap<NaiveDate, f64> = HashMap::new();
    let mut prdem_start_dates: Vec<NaiveDate> = Vec::new();
    let mut default_vec = vec![];

    //prdem
    let prdem_res = res_data
        .get_mut(&"PRDEM".to_string())
        .unwrap_or(&mut default_vec);
    for (idnx, val) in prdem_res.iter().enumerate() {
        let mut end_date;
        let freq = val[2].parse::<u16>().unwrap_or(1);
        let mut start_date = date_parser.parse(&val[0]);
        prdem_prin_amt.insert(start_date, val[1].parse::<f64>().unwrap_or(0.0));
        prdem_start_dates.push(start_date);
        let mut prdem_cf_dates: Vec<NaiveDate>;
        if idnx + 1 < prdem_res.len() {
            // end_date = date_parser.parse(&prdem_res[idnx + 1][0]);
            end_date = account.ei_perd_end_date.expect("Could Not Get Date");
            prdem_cf_dates = get_dates(
                account,
                log,
                &mut start_date,
                &mut end_date,
                freq,
                prdem_res[idnx][3]
                    .parse::<f64>()
                    .expect("Error getting SHDL_NUM from Res-File"),
                prdem_res[idnx][4]
                    .parse::<u32>()
                    .expect("Error getting LR_FREQ_START_DATE from Res-File"),
                prdem_res[idnx][5].to_string(),
                &holiday_map,
            );
        } else {
            // end_date = account.ei_perd_end_date.expect("Could Not Get Date");
            end_date = account.ei_perd_end_date.expect("Could Not Get Date");
            prdem_cf_dates = get_dates(
                account,
                log,
                &mut start_date,
                &mut end_date,
                freq,
                prdem_res[idnx][3]
                    .parse::<f64>()
                    .expect("Error getting SHDL_NUM from Res-File"),
                prdem_res[idnx][4]
                    .parse::<u32>()
                    .expect("Error getting LR_FREQ_START_DATE from Res-File"),
                prdem_res[idnx][5].to_string(),
                &holiday_map,
            );
            if freq == 7 {
                end_date = incr_dt_by_days(
                    account.ei_perd_end_date.expect("Could Not Get Date"),
                    freq.into(),
                );
            } else {
                end_date = increment_date_by_months_unchecked(
                    account.ei_perd_end_date.expect("Could Not Get Date"),
                    freq,
                );
            }
        }
        for cf_date in prdem_cf_dates {
            final_prdem_cf_dates.push(cf_date)
        }
    }
    //write a cf when clr bal is greater than closing out bal
    if clr_bal > closing_out_bal {
        if final_prdem_cf_dates.is_empty() {
            final_prdem_cf_dates.push(as_on_date);
        }
        cf_vec.push(new_cashflow(
            0.0,
            clr_bal - closing_out_bal,
            timestamp(final_prdem_cf_dates[0]),
        ))
    }
    //get indem dates
    let indem_res = res_data
        .get_mut(&"INDEM".to_string())
        .unwrap_or(&mut default_vec);
    for (idnx, val) in indem_res.iter().enumerate() {
        let mut end_date;
        let freq = val[2].parse::<u16>().unwrap_or(1);
        let mut start_date = date_parser.parse(&val[0]);

        let mut indem_cf_dates: Vec<NaiveDate>;
        if idnx + 1 < indem_res.len() {
            // end_date = date_parser.parse(&indem_res[idnx + 1][0]);
            end_date = account.ei_perd_end_date.expect("Could Not Get Date");
            indem_cf_dates = get_dates_in(
                account,
                log,
                &mut start_date,
                &mut end_date,
                freq,
                indem_res[idnx][3]
                    .parse::<f64>()
                    .expect("Error getting SHDL_NUM from Res-File"),
                indem_res[idnx][4]
                    .parse::<u32>()
                    .expect("Error getting LR_FREQ_START_DATE from Res-File"),
                indem_res[idnx][5].to_string(),
                &holiday_map,
            );
        } else {
            end_date = account.ei_perd_end_date.expect("Could Not Get Date");
            indem_cf_dates = get_dates_in(
                account,
                log,
                &mut start_date,
                &mut end_date,
                freq,
                indem_res[idnx][3]
                    .parse::<f64>()
                    .expect("Error getting SHDL_NUM from Res-File"),
                indem_res[idnx][4]
                    .parse::<u32>()
                    .expect("Error getting LR_FREQ_START_DATE from Res-File"),
                indem_res[idnx][5].to_string(),
                &holiday_map,
            );
            if freq == 7 {
                end_date = incr_dt_by_days(
                    account.ei_perd_end_date.expect("Could Not Get Date"),
                    freq.into(),
                );
            } else {
                end_date = increment_date_by_months_unchecked(
                    account.ei_perd_end_date.expect("Could Not Get Date"),
                    freq,
                );
            }
        }
        for cf_date in indem_cf_dates {
            final_indem_cf_dates.push(cf_date)
        }
    }

    let mut prdem_counter = 0;
    let mut indem_counter = 0;
    if prdem_start_dates.is_empty() {
        prdem_start_dates.push(as_on_date);
    }
    while prdem_counter < final_prdem_cf_dates.len() && indem_counter < final_indem_cf_dates.len() {
        if closing_out_bal <= 0.0 {
            break;
        }
        if final_prdem_cf_dates[prdem_counter] < final_indem_cf_dates[indem_counter] {
            let cf_date = final_prdem_cf_dates[prdem_counter];
            let no_of_days = num_days_start_to_end(prev_date, cf_date);
            let prdem_int_amt = closing_out_bal * int_rate * no_of_days as f64 / 36500.0;
            let mut last_date = prdem_start_dates[0];
            let mut prin_amt = 0.0;
            let mut is_enc = false;
            for (_val, date) in prdem_start_dates.iter().enumerate() {
                if cf_date < *date && cf_date >= last_date {
                    is_enc = true;
                    prin_amt = *prdem_prin_amt
                        .get(&last_date)
                        .expect("Could not get Last-Date");
                }
                last_date = *date;
            }
            if !is_enc {
                prin_amt = *prdem_prin_amt
                    .get(&prdem_start_dates.last().expect("Could Not Get Date"))
                    .unwrap_or(&0.0);
            }

            prdem_cf_vec.push(new_cashflow(prdem_int_amt, prin_amt, timestamp(cf_date)));
            prev_date = cf_date;
            closing_out_bal = closing_out_bal - prin_amt;
            prdem_counter += 1;
        } else {
            let cf_date = final_indem_cf_dates[indem_counter];
            let no_of_days = num_days_start_to_end(prev_date, cf_date);
            let indem_int_amt = closing_out_bal * int_rate * no_of_days as f64 / 36500.0;
            indem_cf_vec.push(new_cashflow(indem_int_amt, 0.0, timestamp(cf_date)));
            prev_date = cf_date;
            indem_counter += 1;
        }
    }
    while indem_counter < final_indem_cf_dates.len() && prdem_counter < final_prdem_cf_dates.len() {
        if closing_out_bal <= 0.0 {
            break;
        }
        let cf_date = final_indem_cf_dates[indem_counter];
        let no_of_days = num_days_start_to_end(prev_date, cf_date);
        let indem_int_amt = closing_out_bal * int_rate * no_of_days as f64 / 36500.0;
        indem_cf_vec.push(new_cashflow(indem_int_amt, 0.0, timestamp(cf_date)));
        prev_date = cf_date;
        indem_counter += 1;
        indem_counter += 1;
    }
    if prdem_start_dates.is_empty() {
        prdem_start_dates.push(as_on_date);
    }
    while prdem_counter < final_prdem_cf_dates.len() {
        if closing_out_bal <= 0.0 {
            break;
        }
        let cf_date = final_prdem_cf_dates[prdem_counter];
        let no_of_days = num_days_start_to_end(prev_date, cf_date);
        let prdem_int_amt = closing_out_bal * int_rate * no_of_days as f64 / 36500.0;
        let mut last_date = prdem_start_dates[0];
        let mut prin_amt = 0.0;
        let mut is_enc = false;
        for (_val, date) in prdem_start_dates.iter().enumerate() {
            if cf_date < *date {
                is_enc = true;
                prin_amt = *prdem_prin_amt.get(&last_date).expect("asd");
            }
            last_date = *date;
        }
        if !is_enc {
            prin_amt = *prdem_prin_amt
                .get(&prdem_start_dates.last().expect("Could Not Get Date"))
                .unwrap_or(&0.0);
        }
        prdem_cf_vec.push(new_cashflow(prdem_int_amt, prin_amt, timestamp(cf_date)));
        prev_date = cf_date;
        closing_out_bal = closing_out_bal - prin_amt;
        prdem_counter += 1;
    }

    let mut prdem_counter: i32 = (prdem_cf_vec.len() - 1) as i32;
    let mut indem_counter: i32 = (indem_cf_vec.len() - 1) as i32;
    let mut final_bal = closing_out_bal;
    let mut is_adv_paid = false;
    if closing_out_bal >= clr_bal {
        cf_vec.push(new_cashflow(
            0.0,
            clr_bal,
            if holiday_map.contains_key(&account.ei_perd_end_date.expect("Could Not Get Date")) {
                timestamp(
                    *final_prdem_cf_dates
                        .iter()
                        .max()
                        .expect("Could Not Get Date"),
                )
            } else {
                timestamp(account.ei_perd_end_date.expect("Could Not Get Date"))
            },
        ));
        is_adv_paid = true;
    } else {
        cf_vec.push(new_cashflow(
            0.0,
            closing_out_bal,
            if holiday_map.contains_key(&account.ei_perd_end_date.expect("Could Not Get Date")) {
                timestamp(
                    *final_prdem_cf_dates
                        .iter()
                        .max()
                        .expect("Could Not Get Date"),
                )
            } else {
                timestamp(account.ei_perd_end_date.expect("Could Not Get Date"))
            },
        ));
    }
    while prdem_counter >= 0 && indem_counter >= 0 && !is_adv_paid {
        if prdem_cf_vec[prdem_counter as usize].date == indem_cf_vec[indem_counter as usize].date {
            final_bal = final_bal + prdem_cf_vec[prdem_counter as usize].principal_amount;
            if final_bal <= clr_bal {
                cf_vec.push(prdem_cf_vec[prdem_counter as usize].to_owned());
                cf_vec.push(indem_cf_vec[indem_counter as usize].to_owned());
                prdem_counter -= 1;
                indem_counter -= 1;
                continue;
            }
            if final_bal > clr_bal {
                cf_vec.push(new_cashflow(
                    prdem_cf_vec[prdem_counter as usize].interest_amount,
                    clr_bal - (final_bal - prdem_cf_vec[prdem_counter as usize].principal_amount),
                    prdem_cf_vec[prdem_counter as usize].date,
                ))
            }
            break;
        } else if prdem_cf_vec[prdem_counter as usize].date
            > indem_cf_vec[indem_counter as usize].date
        {
            final_bal = final_bal + prdem_cf_vec[prdem_counter as usize].principal_amount;
            if final_bal <= clr_bal {
                cf_vec.push(prdem_cf_vec[prdem_counter as usize].to_owned());
                prdem_counter -= 1;
                continue;
            }
            if final_bal > clr_bal {
                cf_vec.push(new_cashflow(
                    prdem_cf_vec[prdem_counter as usize].interest_amount,
                    clr_bal - (final_bal - prdem_cf_vec[prdem_counter as usize].principal_amount),
                    prdem_cf_vec[prdem_counter as usize].date,
                ))
            }
            break;
        } else {
            cf_vec.push(indem_cf_vec[indem_counter as usize].to_owned());
            indem_counter -= 1;
        }
        if indem_counter == 0 || prdem_counter == 0 {
            break;
        }
    }

    while prdem_counter >= 0 && !is_adv_paid && final_bal < clr_bal {
        final_bal = final_bal + prdem_cf_vec[prdem_counter as usize].principal_amount;
        if final_bal <= clr_bal {
            cf_vec.push(prdem_cf_vec[prdem_counter as usize].to_owned());
            prdem_counter -= 1;
            continue;
        }
        if final_bal > clr_bal {
            cf_vec.push(new_cashflow(
                prdem_cf_vec[prdem_counter as usize].interest_amount,
                clr_bal - (final_bal - prdem_cf_vec[prdem_counter as usize].principal_amount),
                prdem_cf_vec[prdem_counter as usize].date,
            ))
        }
        break;
    }
    //removing -ve prin-amt cf and adding it to prev cf
    if cf_vec[0].principal_amount < 0.0 && cf_vec.len() > 1 {
        cf_vec.reverse();
        let neg_cf = cf_vec.pop();
        let len = cf_vec.len() - 1;
        cf_vec[len].principal_amount =
            cf_vec[len].principal_amount + neg_cf.unwrap().principal_amount;
    }
    Ok(cf_vec)
}

pub fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = d;

    cf
}

fn get_freq(freq: &str) -> String {
    match freq {
        "W" => 7.to_string(),
        "M" => 1.to_string(),
        "Q" => 3.to_string(),
        "H" => 6.to_string(),
        "Y" => 12.to_string(),
        _ => 15.to_string(),
    }
}

fn get_dates_in(
    account: &mut InputAccount,
    log: &Logger,
    start_date: &mut NaiveDate,
    end_date: &mut NaiveDate,
    freq: u16,
    co: f64,
    start_dd: u32,
    date_flag: String,
    holiday_map: &HashMap<NaiveDate, String>,
) -> Vec<NaiveDate> {
    let mut dates: Vec<NaiveDate> = Vec::new();
    dates.push(*start_date);
    if freq == 15 {
        return dates;
    }

    while start_date <= end_date {
        if freq == 7 {
            *start_date = incr_dt_by_days(*start_date, freq.into());
        } else {
            *start_date = increment_date_by_months_unchecked(*start_date, freq);
        }

        let mut cf_date = if get_valid_flag(*start_date, start_dd) == "VALID" {
            NaiveDate::from_ymd(start_date.year(), start_date.month(), start_dd)
        } else if get_valid_flag(*start_date, start_dd) == "ZERO" {
            warn!(
                log,
                "Day `{}` lesser than 1 in res-file for acc-no `{}`", start_dd, account.acid
            );
            NaiveDate::from_ymd(start_date.year(), start_date.month(), start_date.day())
        } else {
            warn!(
                log,
                "Day `{}` exceeded in res-file for acc-no `{}`", start_dd, account.acid
            );
            NaiveDate::from_ymd(
                start_date.year(),
                start_date.month(),
                rbdate::get_days_from_month(*start_date) as u32,
            )
        };
        if cf_date > *end_date {
            break;
        }
        while holiday_map.get(&cf_date).unwrap_or(&"W".to_string()) != "W" {
            match date_flag.trim() {
                "P" => {
                    cf_date = cf_date.pred();
                }
                "N" => {
                    cf_date = cf_date.succ();
                }
                "S" => {
                    break;
                }
                _ => {
                    panic!("Invalid flag");
                }
            }
        }
        dates.push(cf_date);
    }
    dates
}

fn get_dates(
    account: &mut InputAccount,
    log: &Logger,
    start_date: &mut NaiveDate,
    end_date: &mut NaiveDate,
    freq: u16,
    co: f64,
    start_dd: u32,
    date_flag: String,
    holiday_map: &HashMap<NaiveDate, String>,
) -> Vec<NaiveDate> {
    let mut count = 1.0;
    let mut dates: Vec<NaiveDate> = Vec::new();
    if co == 0.0 {
        return dates;
    }
    dates.push(*start_date);
    if freq == 15 || co == 1.0 {
        return dates;
    }

    while count < co && start_date <= end_date {
        if freq == 7 {
            *start_date = incr_dt_by_days(*start_date, freq.into());
        } else {
            *start_date = increment_date_by_months_unchecked(*start_date, freq);
        }

        let mut cf_date = if get_valid_flag(*start_date, start_dd) == "VALID" {
            NaiveDate::from_ymd(start_date.year(), start_date.month(), start_dd)
        } else if get_valid_flag(*start_date, start_dd) == "ZERO" {
            warn!(
                log,
                "Day `{}` lesser than 1 in res-file for acc-no `{}`", start_dd, account.acid
            );
            NaiveDate::from_ymd(start_date.year(), start_date.month(), start_date.day())
        } else {
            warn!(
                log,
                "Day `{}` exceeded in res-file for acc-no `{}`", start_dd, account.acid
            );
            NaiveDate::from_ymd(
                start_date.year(),
                start_date.month(),
                rbdate::get_days_from_month(*start_date) as u32,
            )
        };
        if cf_date > *end_date {
            break;
        }
        while holiday_map.get(&cf_date).unwrap_or(&"W".to_string()) != "W" {
            match date_flag.trim() {
                "P" => {
                    cf_date = cf_date.pred();
                }
                "N" => {
                    cf_date = cf_date.succ();
                }
                "S" => {
                    break;
                }
                _ => {
                    panic!("Invalid flag");
                }
            }
        }
        dates.push(cf_date);
        count += 1.0;
    }
    dates
}

pub fn get_valid_flag(date: rbdate::NaiveDate, day: u32) -> String {
    if day < 1 {
        return "ZERO".to_string();
    }
    match date.month() {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => {
            if day > 31 {
                return "EXCEEDED".to_string();
            }
        }
        4 | 6 | 9 | 11 => {
            if day > 30 {
                return "EXCEEDED".to_string();
            }
        }
        2 => {
            if is_leap_year(date.year()) {
                if day > 29 {
                    return "EXCEEDED".to_string();
                }
            } else {
                if day > 28 {
                    return "EXCEEDED".to_string();
                }
            }
        }
        _ => {
            return "EXCEEDED".to_string();
        }
    }
    return "VALID".to_string();
}
