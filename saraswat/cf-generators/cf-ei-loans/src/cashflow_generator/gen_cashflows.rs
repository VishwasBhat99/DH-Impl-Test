use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::Cashflow;
use chrono::Duration;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::{
    date_from_timestamp, decr_dt_by_mon_presrv_eom, increment_date_by_months_unchecked,
    num_days_start_to_end, timestamp, DateParser, NaiveDate,
};
use sdb_day_convention::{days_with_convn, Conventions};
use slog::Logger;
use std::collections::HashMap;

pub fn generate_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
    res_file_date: &HashMap<String, Vec<Vec<String>>>,
) -> Result<Vec<Cashflow>, String> {
    let mut cf_vec: Vec<Cashflow> = Vec::new();
    let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
    let convention = &account.int_basis;
    let res = match res_file_date.get(&account.acid) {
        Some(val) => val,
        None => {
            log_error!(
                log,
                "Could not find restructure data for account id: {}",
                &account.acid
            );
            panic!(
                "Could not find restructure data for account id: {}",
                &account.acid
            )
        }
    };
    let mut int_end_date = {
        match res[0][5].as_str() {
            "F" => date_parser.parse(&res[0][3]) - Duration::days(14),
            "B" => decr_dt_by_mon_presrv_eom(date_parser.parse(&res[0][3]), 0)
                .unwrap_or(*config_params.as_on_date()),
            "M" => decr_dt_by_mon_presrv_eom(date_parser.parse(&res[0][3]), 1)
                .unwrap_or(*config_params.as_on_date()),
            "Q" => decr_dt_by_mon_presrv_eom(date_parser.parse(&res[0][3]), 3)
                .unwrap_or(*config_params.as_on_date()),
            "H" => decr_dt_by_mon_presrv_eom(date_parser.parse(&res[0][3]), 6)
                .unwrap_or(*config_params.as_on_date()),
            "Y" => decr_dt_by_mon_presrv_eom(date_parser.parse(&res[0][3]), 12)
                .unwrap_or(*config_params.as_on_date()),
            _ => *config_params.as_on_date(),
        }
    };
    let mut os_bal = account.dis_amt;
    let mut total_cf_amt = 0.0;
    let mut all_cfs: Vec<Cashflow> = Vec::new();
    let mut end_date = *config_params.as_on_date();
    let mut freq = 1;
    let mut eidem_present = false;
    let mut countr_1 = 0;
    let res_len = res.len() - 1;
    let mut res_count = 1;
    let mat_date = account
    .ei_perd_end_date
    .expect("Could not find Ei period end date");
    let mut flow_end_date = account
        .ei_perd_end_date
        .expect("Could not find Ei period end date");
    for res_acc in res {
        if res_count <= res_len {
            flow_end_date = decr_dt_by_mon_presrv_eom(date_parser.parse(&res[res_count][3]), 1)
                .expect("Could not get date");
            res_count += 1;
        } else {
            flow_end_date = account
                .ei_perd_end_date
                .expect("Could not find Ei period end date");
        }
        freq = match &res_acc[5][..] {
            "M" => 1,
            "Q" => 3,
            "H" => 6,
            "Y" => 12,
            _ => 0,
        };
        if res_acc[8] == "INDEM" {
            end_date = date_parser.parse(&res_acc[3]);

            let emi_bal = &res_acc[4];
            let mut start_date = int_end_date;
            let int_rate = account.int_rate;
            let mut count = 1;
            let mut freq_incr = freq;
            while increment_date_by_months_unchecked(start_date, freq)
                < date_parser.parse(&res[1][3])
            {
                let days = num_days_start_to_end(start_date, end_date);
                let int_amt = match convention as &str {
                    "Ac360" => ((os_bal * int_rate * days as f64) / 36500.0).ceil(),
                    _ => ((os_bal * int_rate) / 1200.0).ceil(),
                };
                cf_vec.push(new_cashflow(int_amt, 0.0, timestamp(end_date)));
                int_end_date = end_date;
                start_date = end_date;
                end_date =
                    increment_date_by_months_unchecked(date_parser.parse(&res_acc[3]), freq_incr);

                count += 1;
                freq_incr += freq;
            }
        }
        if res_acc[8] == "EIDEM" {
            eidem_present = true;
            end_date = date_parser.parse(&res_acc[3]);
            let mut start_date = int_end_date;
            countr_1 += 1;
            let mut countr_2 = 1;
            if countr_1 != 1 {
                start_date =
                    decr_dt_by_mon_presrv_eom(end_date, freq as usize).expect("Could not get date");
            }
            let mut freq_incr = freq;
            let emi_bal = &res_acc[4];
            let int_rate = account.int_rate;
            //bullet payment case
            if freq == 0 {
                cf_vec.push(new_cashflow(0.0, os_bal, timestamp(mat_date)));
                break;
            }
            while (increment_date_by_months_unchecked(start_date, freq) <= flow_end_date
                && os_bal > 0.0)
                || (increment_date_by_months_unchecked(start_date, freq) >= flow_end_date
                    && countr_2 == 1)
            {
                countr_2 += 1;
                let mut cf = Cashflow::new();
                let days = num_days_start_to_end(start_date, end_date);
                let int_amt = match convention as &str {
                    "Ac360" => ((os_bal * int_rate * days as f64) / 36500.0).ceil(),
                    _ => ((os_bal * int_rate * days as f64) / 36500.0).ceil(),
                };
                let prin_amt = if os_bal < (emi_bal.parse::<f64>().unwrap_or(0.0) - int_amt)
                    || increment_date_by_months_unchecked(start_date, freq)
                        >= account
                            .ei_perd_end_date
                            .expect("Could not find Ei period end date")
                {
                    os_bal
                } else {
                    (emi_bal.parse::<f64>().unwrap_or(0.0) - int_amt)
                };
                os_bal = os_bal - prin_amt;
                cf.interest_amount = int_amt;
                cf.principal_amount = prin_amt;
                cf.date = timestamp(end_date);
                all_cfs.push(cf);
                start_date = end_date;
                end_date =
                    increment_date_by_months_unchecked(date_parser.parse(&res_acc[3]), freq_incr);
                freq_incr += freq;
            }
        }
    }
    if eidem_present {
        if os_bal > 0.0 {
            let mut cf = Cashflow::new();
            cf.interest_amount = 0.0;
            cf.principal_amount = os_bal;
            cf.date = timestamp(
                decr_dt_by_mon_presrv_eom(end_date, freq as usize).expect("Could not find date"),
            );
            all_cfs.push(cf);
        }
        if all_cfs.len() != 0 {
            let all_cfs_len = (all_cfs.len() - 1);
            for indx in (0..=all_cfs_len).rev() {
                let cf = &all_cfs[indx];
                if total_cf_amt + cf.principal_amount > account.clr_bal_amt {
                    cf_vec.push(new_cashflow(
                        0.0,
                        account.clr_bal_amt - total_cf_amt,
                        cf.date,
                    ));
                    total_cf_amt += account.clr_bal_amt - total_cf_amt;
                    break;
                }
                total_cf_amt += cf.principal_amount;
                cf_vec.push(all_cfs[indx].to_owned());
            }
        }
        if total_cf_amt < account.clr_bal_amt {
            cf_vec.push(new_cashflow(
                0.0,
                account.clr_bal_amt - total_cf_amt,
                timestamp(
                    decr_dt_by_mon_presrv_eom(end_date, freq as usize)
                        .expect("Could not find date"),
                ),
            ));
        }
    }
    for cf in cf_vec.iter_mut() {
        if date_from_timestamp(cf.date) > mat_date {
            cf.date = timestamp(mat_date);
        }
    }
    Ok(cf_vec)
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = d;

    cf
}
