use cashflow_generator::account_reader::input_account::*;
use cashflow_generator::account_with_cashflows::Cashflow;
use chrono::Datelike;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::{get_month_end_date, timestamp, NaiveDate};
use sdb_day_convention::conventions::Days;
use sdb_day_convention::days_with_convn;
use slog::Logger;
use std::collections::HashMap;

pub fn generate_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
    repay_map: &HashMap<String, Vec<RepaymentData>>,
    ovd_map: &HashMap<String, Vec<(f64, NaiveDate)>>,
) -> Result<Vec<Cashflow>, String> {
    let mut cf_vec: Vec<Cashflow> = Vec::new();
    let mut all_cfs: Vec<Cashflow> = Vec::new();
    let mut tot_overdue_amt = 0.0;
    match ovd_map.get(&account.acid.to_string()) {
        Some(ovds) => {
            for ovd_data in ovds.iter() {
                cf_vec.push(new_cashflow(0.0, ovd_data.0, ovd_data.1));
                tot_overdue_amt += ovd_data.0;
            }
        }
        None => {
            log_error!(
                log,
                "Could not find Overdue Data for acid: {:?}",
                account.acid
            );
        }
    };
    let repay_data = match repay_map.get(&account.acid.to_string()) {
        Some(data) => data,
        None => {
            log_error!(
                log,
                "Could not find EIDEM Repayment Structure for acid: {:?}",
                account.acid
            );
            return Ok(vec![new_cashflow(
                0.0,
                account.clr_bal_amt - tot_overdue_amt,
                account.ei_perd_end_date.unwrap_or_else(|| {
                    panic!(
                        "Unable to read EI-Perd-End-Date for Account: {}",
                        account.acid
                    )
                }),
            )]);
        }
    };
    let ei_perd_end_date = account
        .ei_perd_end_date
        .expect("Error Reading EI-Perd-End-Date");
    let first_cf_freq = get_freq(&repay_data[0].lr_freq_type);
    let first_cf_date = repay_data[0].flow_start_date;
    let mut prev_cf_date = if first_cf_freq != 7 || first_cf_freq != 14 {
        rbdate::decr_dt_by_mon_presrv_eom(first_cf_date, first_cf_freq)
            .expect("Unable to derive first cf's prev-date")
    } else {
        rbdate::dcr_dt_by_days(first_cf_date, first_cf_freq as i64)
    };
    let mut out_bal = f64::min(account.rephasement_principal, account.dis_amt);
    for res_data in repay_data.iter() {
        let mut num_of_flows = res_data.num_of_flows;
        let freq = get_freq(&res_data.lr_freq_type);
        let mut cf_date = NaiveDate::from_ymd_opt(
            res_data.flow_start_date.year(),
            res_data.flow_start_date.month(),
            res_data.lr_freq_start_dd as u32,
        )
        .unwrap_or(res_data.flow_start_date);
        let emi_amt = res_data.flow_amt;
        while num_of_flows > 0 && cf_date <= ei_perd_end_date {
            let days = days_with_convn(prev_cf_date, cf_date, config_params.convention())
                .expect("Error deriving days using conventions");
            let int_amt = interest_amount(out_bal, account.int_rate, days);
            let mut prin_amt = emi_amt - int_amt;
            prin_amt = f64::min(prin_amt, out_bal);
            let cf = new_cashflow(int_amt, prin_amt, cf_date);
            all_cfs.push(cf);
            prev_cf_date = cf_date;
            cf_date = if freq != 7 || freq != 14 {
                rbdate::increment_date_by_months(cf_date, freq as u16)
            } else {
                rbdate::incr_dt_by_days(cf_date, freq as i64)
            };
            cf_date = if res_data.lr_freq_start_dd >= 29 && cf_date.month() == 2 {
                NaiveDate::from_ymd_opt(
                    cf_date.year(),
                    cf_date.month(),
                    get_month_end_date(cf_date).day(),
                )
                .expect("Unable to derive Flow-Start-Date")
            } else {
                NaiveDate::from_ymd_opt(
                    cf_date.year(),
                    cf_date.month(),
                    res_data.lr_freq_start_dd as u32,
                )
                .unwrap_or(
                    NaiveDate::from_ymd_opt(
                        cf_date.year(),
                        cf_date.month(),
                        get_month_end_date(cf_date).day(),
                    )
                    .expect("Unable to derive Flow-Start-Date"),
                )
            };
            out_bal -= prin_amt;
            num_of_flows -= 1;
        }
    }

    if out_bal > 0.0 {
        log_warn!(
            log,
            "Writing Pending Out-Bal: {:?} on EI-Perd-End-Date: {:?} (Temp CFs)",
            out_bal,
            ei_perd_end_date
        );
        all_cfs.push(new_cashflow(0.0, out_bal, ei_perd_end_date));
    }
    let mut clr_bal_amt = account.clr_bal_amt.abs() - tot_overdue_amt;
    if clr_bal_amt <= 0.0 {
        log_warn!(
            log,
            "Writing Negative Clr-Bal-Amt: {:?} on EI-Perd-End-Date: {:?}",
            clr_bal_amt,
            ei_perd_end_date
        );
        cf_vec.push(new_cashflow(0.0, clr_bal_amt, ei_perd_end_date));
        return Ok(cf_vec);
    }
    let mut prev_cf_date = *config_params.as_on_date();
    for cf in all_cfs.iter() {
        if rbdate::date_from_timestamp(cf.get_date()) > *config_params.as_on_date() {
            if clr_bal_amt < cf.get_principal_amount() {
                if config_params.adj_cf_type() == "LAST" {
                    cf_vec.push(new_cashflow(0.0, clr_bal_amt, prev_cf_date));
                } else {
                    cf_vec.push(new_cashflow(
                        0.0,
                        clr_bal_amt,
                        rbdate::date_from_timestamp(cf.get_date()),
                    ));
                }
                clr_bal_amt = 0.0;
            } else {
                cf_vec.push(cf.to_owned());
                clr_bal_amt -= cf.get_principal_amount();
            }
            prev_cf_date = rbdate::date_from_timestamp(cf.get_date());
        }
    }
    if clr_bal_amt > 0.0 {
        log_warn!(
            log,
            "Writing Pending Clr-Bal-Amt: {:?} on EI-Perd-End-Date: {:?}",
            clr_bal_amt,
            ei_perd_end_date
        );
        cf_vec.push(new_cashflow(0.0, clr_bal_amt, ei_perd_end_date));
    }
    Ok(cf_vec)
}

pub fn new_cashflow(i_a: f64, p_a: f64, d: NaiveDate) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = timestamp(d);

    cf
}

fn get_freq(freq: &str) -> usize {
    match freq {
        "W" => 7,
        "F" => 14,
        "M" => 1,
        "B" => 2,
        "Q" => 3,
        "H" => 6,
        "Y" => 12,
        _ => 15,
    }
}

fn interest_amount(o_a: f64, i_r: f64, days: Days) -> f64 {
    let num_days = days.days_btw_dts;
    let days_in_yr = days.day_in_yr as f64;
    (o_a * i_r * num_days as f64) / (days_in_yr * 100.0)
}
