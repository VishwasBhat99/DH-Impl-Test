use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::Cashflow;
use chrono::{Datelike, Duration};
use configuration_parameters::ConfigurationParameters;
use macros;
use math::round::half_away_from_zero;
use npa_cfdate_adjusment::npa_cfdate_adjusment;
use rbdate::*;
use rbdate::{
    datevalue_to_naive_date, get_month_end_date, incr_dt_by_mon_presrv_eom,
    increment_date_by_months, num_days_start_to_end, timestamp, DateParser,
};
use sdb_cf_gen::*;
use slog::Logger;
use std::collections::HashMap;

pub fn generate_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
    res_file_date: HashMap<String, Vec<Vec<String>>>,
) -> Result<Vec<Cashflow>, String> {
    let mut p_repaymemt_freq = 13;
    let mut i_repaymemt_freq = 13;
    let mut closing_prin = account.os_loan_bal_lcy;
    let date_parser = DateParser::new("%d/%m/%Y".to_string(), false);
    let mut cf_vec: Vec<Cashflow> = Vec::new();

    //Get Repayment Frequency
    for res_acc_data in res_file_date.get(&account.acc_no) {
        for val in res_acc_data {
            if val[4] == "Q" {
                if val[6] == "P" {
                    p_repaymemt_freq = 3;
                } else {
                    i_repaymemt_freq = 3;
                }
            } else if val[4] == "M" {
                if val[6] == "P" {
                    p_repaymemt_freq = 1;
                } else {
                    i_repaymemt_freq = 1;
                }
            } else if val[4] == "Y" {
                if val[6] == "P" {
                    p_repaymemt_freq = 12;
                } else {
                    i_repaymemt_freq = 12;
                }
            } else {
                if val[6] == "P" {
                    p_repaymemt_freq = 6;
                } else {
                    i_repaymemt_freq = 6;
                }
            }
        }
    }
    //Get P and I Staring CF Dates
    let p_first_cf_date = rbdate::increment_date_by_months(
        account.ei_start_date_crnt.expect("Could Not Find Date"),
        p_repaymemt_freq - 1,
    );
    let i_first_cf_date = rbdate::increment_date_by_months(
        account.ei_start_date_crnt.expect("Could Not Find Date"),
        i_repaymemt_freq - 1,
    );

    let mut p_succ_cf_date = p_first_cf_date;
    let mut i_succ_cf_date = i_first_cf_date;
    let mut prin_comp = 0.0;
    let mut intr_comp = 0.0;

    while i_succ_cf_date <= account.maturity_date.expect("Could Not Find Maturity Date")
        && closing_prin > 0.0
    {
        prin_comp = 0.0;
        intr_comp = 0.0;
        //Prin Cf Calculation
        while i_succ_cf_date > p_succ_cf_date && closing_prin >= 0.0 {
            for res_acc_data in res_file_date.get(&account.acc_no) {
                for val in res_acc_data {
                    if p_succ_cf_date >= date_parser.parse(&val[7])
                        && p_succ_cf_date <= date_parser.parse(&val[8])
                        && val[6] == "P"
                    {
                        prin_comp = val[5].parse::<f64>().unwrap_or(0.0);
                        if prin_comp > closing_prin {
                            cf_vec.push(new_cashflow(0.0, closing_prin, timestamp(p_succ_cf_date)));
                        } else {
                            cf_vec.push(new_cashflow(0.0, prin_comp, timestamp(p_succ_cf_date)));
                        }
                        closing_prin = closing_prin - prin_comp;
                    }
                }
            }
            p_succ_cf_date = rbdate::increment_date_by_months(p_succ_cf_date, p_repaymemt_freq);
        }
        //Int Cf Calculation
        for res_acc_data in res_file_date.get(&account.acc_no) {
            for val in res_acc_data {
                if i_succ_cf_date > date_parser.parse(&val[7])
                    && i_succ_cf_date <= date_parser.parse(&val[8])
                    && val[6] == "I"
                {
                    intr_comp = closing_prin * val[3].parse::<f64>().unwrap_or(0.0)
                        / ((12.0 / i_repaymemt_freq as f64) * 100.0);
                    cf_vec.push(new_cashflow(intr_comp, 0.0, timestamp(i_succ_cf_date)));
                }
            }
        }
        i_succ_cf_date = rbdate::increment_date_by_months(i_succ_cf_date, i_repaymemt_freq);
    }
    if i_repaymemt_freq == p_repaymemt_freq {
        cf_vec.push(new_cashflow(0.0, prin_comp, timestamp(p_succ_cf_date)));
        closing_prin = closing_prin - prin_comp;
    }
    if closing_prin > 0.0 {
        cf_vec.push(new_cashflow(
            0.0,
            closing_prin,
            timestamp(account.maturity_date.expect("Could Not Find Maturity Date")),
        ));
    }
    Ok(cf_vec)
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = d;

    cf
}
