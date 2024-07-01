use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::Cashflow;
use chrono::{Duration, Utc};
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::*;
use sdb_day_convention::{days_with_convn, Conventions};
use slog::Logger;
use statics::*;
use std::collections::*;
use std::process;

pub fn generate_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
    prod_code_records: &HashMap<String, String>,
) -> Result<Vec<Cashflow>, String> {
    let mut cf_vec: Vec<Cashflow> = Vec::new();
    let initial_depo_amt = account.initial_deposit_amount;
    let int_rate = account.interest_rate;
    let period_months = account.period_months;
    let acc_prd_code = &account.product;
    let prin_amt = account.initial_deposit_amount
        + if let Some(cum_interest) = account.cum_interest {
            cum_interest
        } else {
            DEFAULT_FLOAT
        };
    let initial_depo_date = if let Some(dt) = account.deposit_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    let maturity_date = if let Some(dt) = account.maturity_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    //Principal Cashflpws
    cf_vec.push(new_cashflow(0.0, prin_amt, maturity_date));
    //calc total interest amount to be paid
    let tot_int_amt = get_total_int_amt(initial_depo_amt, int_rate, period_months);
    let mut pay_tot_int = 0.0;
    //get payment freq from product code file based on product code
    let mut pay_freq = match prod_code_records
        .get(acc_prd_code)
        .expect("Could not find the product code") as &str
    {
        "ANNUAL" => 12,
        "HALF YEARLY" => 6,
        "QUARTERLY" => 3,
        "MONTHLY" => 1,
        "NON-CUMULATIVE" => account.interest_payment_freq,
        _ => account.interest_payment_freq,
    };
    if pay_freq == 0 {
        pay_freq = 1
    }
    if prod_code_records
        .get(acc_prd_code)
        .expect("Cound not find prod code.")
        == "PAYOUT ON MATURITY- SIMPLE INTEREST"
    {
        let no_of_days = num_days_start_to_end(
            rbdate::date_from_timestamp(initial_depo_date),
            rbdate::date_from_timestamp(maturity_date),
        );
        let cf_int = (initial_depo_amt * no_of_days as f64 * int_rate) / 36500.0;
        cf_vec.push(new_cashflow(cf_int, 0.0, maturity_date));
    } else if prod_code_records
        .get(acc_prd_code)
        .expect("Could not find prod code")
        == "CUMULATIVE (NO PAYOUT)"
    {
        let mut pay_freq = account.compounding_freq;
        if pay_freq == 0 {
            pay_freq = 1;
        }
        let tds_rate = account.tds_rate;
        let mut tot_cf_int = 0.0;
        let mut tot_actual_int = 0.0;
        let initial_depo_date_1 = if let Some(dt) = account.interest_accrued_upto {
            timestamp(dt)
        } else {
            DEFAULT_INT
        };
        let int_created_upto = if let Some(dt) = account.intrest_craeted_upto {
            timestamp(dt)
        } else {
            DEFAULT_INT
        };
        if maturity_date == int_created_upto {
            cf_vec.push(new_cashflow(
                account.cum_interest.unwrap_or(0.0),
                0.0,
                maturity_date,
            ));
        } else {
            let mut initial_depo_date =
                rbdate::timestamp(rbdate::date_from_timestamp(initial_depo_date_1).succ());
            //get total cumulative interest amount
            let mut depo_amt = account.initial_deposit_amount + account.cum_interest.unwrap_or(0.0);
            let tot_cum_int_amt =
                get_cumulative_tot_int(initial_depo_amt, int_rate, period_months, pay_freq);
            let ini_depo_date = rbdate::date_from_timestamp(initial_depo_date).to_string();
            let initial_depo_date_fields: Vec<&str> = ini_depo_date.split("-").collect();
            let start_month = initial_depo_date_fields[1].parse::<i64>().unwrap_or(0);
            let mut no_of_mon = 1;
            let mut to_date;
            //get first to date
            if pay_freq == 1 {
                to_date =
                    rbdate::get_month_end_date(rbdate::date_from_timestamp(initial_depo_date));
            } else {
                //get no. of months to increase initial deposit date
                if start_month <= 3 && (pay_freq == 3 || pay_freq == 6 || pay_freq == 12) {
                    no_of_mon = 3 - start_month;
                } else if start_month > 3 && start_month <= 6 && pay_freq == 3 {
                    no_of_mon = 6 - start_month;
                } else if start_month > 3 && start_month <= 9 && (pay_freq == 6 || pay_freq == 3) {
                    no_of_mon = 9 - start_month;
                } else {
                    no_of_mon = 15 - start_month;
                }
                to_date = rbdate::incr_dt_by_mon_presrv_eom(
                    rbdate::get_month_end_date(rbdate::date_from_timestamp(initial_depo_date)),
                    no_of_mon as usize,
                )
                .expect("Could not get date");
            }
            let no_of_days = rbdate::num_days_start_to_end(
                rbdate::date_from_timestamp(initial_depo_date),
                to_date,
            ) + 1;
            let actual_int = depo_amt * int_rate * no_of_days as f64 / 36500.0;
            tot_actual_int += actual_int;
            let tds_amt = tds_rate * actual_int / 100.0;
            tot_cf_int += actual_int - tds_amt;
            depo_amt += actual_int - tds_amt;

            initial_depo_date = rbdate::timestamp(to_date);
            to_date = rbdate::incr_dt_by_mon_presrv_eom(to_date, pay_freq as usize)
                .expect("Could not get date");
            while timestamp(
                rbdate::incr_dt_by_mon_presrv_eom(
                    date_from_timestamp(initial_depo_date),
                    pay_freq as usize,
                )
                .expect("Could not get date"),
            ) < maturity_date
            {
                let no_of_days = rbdate::num_days_start_to_end(
                    rbdate::date_from_timestamp(initial_depo_date),
                    to_date,
                );
                let actual_int = depo_amt * int_rate * no_of_days as f64 / 36500.0;
                tot_actual_int += actual_int;
                let tds_amt = tds_rate * actual_int / 100.0;
                tot_cf_int += actual_int - tds_amt;
                depo_amt += actual_int - tds_amt;
                initial_depo_date = rbdate::timestamp(to_date);
                to_date = rbdate::incr_dt_by_mon_presrv_eom(to_date, pay_freq as usize)
                    .expect("Could not get date");
            }
            if initial_depo_date < maturity_date {
                let no_of_days = rbdate::num_days_start_to_end(
                    rbdate::date_from_timestamp(initial_depo_date),
                    date_from_timestamp(maturity_date),
                ) - 1;
                let mut actual_int = depo_amt * int_rate * no_of_days as f64 / 36500.0;
                tot_actual_int += actual_int;
                if !(actual_int
                    < tot_cum_int_amt - tot_actual_int + account.cum_interest.unwrap_or(0.0))
                {
                    actual_int =
                        tot_cum_int_amt - tot_actual_int + account.cum_interest.unwrap_or(0.0);
                }
                let tds_amt = tds_rate * actual_int / 100.0;
                tot_cf_int += actual_int - tds_amt;
            }
            cf_vec.push(new_cashflow(
                account.cum_interest.unwrap_or(0.0) + tot_cf_int,
                0.0,
                maturity_date,
            ));
        }
    }
    //when 1, 3, 6, 12 and non cumulative
    //get 1st broken cf int
    else {
        let broken_cf_int = get_broken_cf_int(initial_depo_amt, initial_depo_date, int_rate);
        pay_tot_int += broken_cf_int;
        if get_month_end_date(rbdate::date_from_timestamp(initial_depo_date))
            > *config_params.as_on_date()
        {
            cf_vec.push(new_cashflow(
                broken_cf_int,
                0.0,
                timestamp(get_month_end_date(rbdate::date_from_timestamp(
                    initial_depo_date,
                ))),
            ));
        }
        if timestamp(get_month_end_date(rbdate::date_from_timestamp(
            initial_depo_date,
        ))) < maturity_date
        {
            let next_cf_start_date = rbdate::date_from_timestamp(initial_depo_date)
                .succ()
                .to_string();
            let mut no_of_mon = 0;
            let next_cf_start_fields: Vec<&str> = next_cf_start_date.split("-").collect();
            if next_cf_start_fields[1].parse::<i64>().unwrap_or(0) > 3 {
                no_of_mon = (15 - next_cf_start_fields[1].parse::<i64>().unwrap_or(0)) as u16;
            } else {
                no_of_mon = (3 - next_cf_start_fields[1].parse::<i64>().unwrap_or(0)) as u16;
            }
            let mut next_cf_end_date = incr_dt_by_mon_presrv_eom(
                get_month_end_date(rbdate::date_from_timestamp(initial_depo_date).succ()),
                no_of_mon as usize,
            )
            .expect("Could not get date");
            let mut next_cf_int_amt = (no_of_mon as f64 * int_rate * initial_depo_amt) / 1200.0;
            pay_tot_int += next_cf_int_amt;
            if next_cf_end_date > *config_params.as_on_date() {
                cf_vec.push(new_cashflow(
                    next_cf_int_amt,
                    0.0,
                    timestamp(next_cf_end_date),
                ));
            }
            while timestamp(
                incr_dt_by_mon_presrv_eom(next_cf_end_date, pay_freq as usize)
                    .expect("Could not get date"),
            ) < maturity_date
            {
                next_cf_end_date = incr_dt_by_mon_presrv_eom(next_cf_end_date, pay_freq as usize)
                    .expect("Could not get date");
                next_cf_int_amt = (initial_depo_amt * int_rate * pay_freq as f64) / 1200.0;
                pay_tot_int += next_cf_int_amt;
                if next_cf_end_date > *config_params.as_on_date() {
                    cf_vec.push(new_cashflow(
                        next_cf_int_amt,
                        0.0,
                        timestamp(next_cf_end_date),
                    ));
                }
            }
            cf_vec.push(new_cashflow(tot_int_amt - pay_tot_int, 0.0, maturity_date));
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

fn get_total_int_amt(initial_depo_amt: f64, int_rate: f64, period_months: i64) -> f64 {
    (initial_depo_amt * int_rate * period_months as f64) / 1200.0
}

fn get_cumulative_tot_int(
    initial_depo_amt: f64,
    int_rate: f64,
    period_months: i64,
    pay_freq: i64,
) -> f64 {
    let mut period_months = period_months;
    let mut cum_amt = initial_depo_amt;
    let mut cumulative_tot_int = 0.0;
    while period_months - pay_freq > 0 {
        let int_amt = cum_amt * int_rate * pay_freq as f64 / 1200.0;
        cumulative_tot_int += int_amt;
        cum_amt += int_amt;
        period_months -= pay_freq;
    }
    if period_months > 0 {
        let int_amt = cum_amt * int_rate * period_months as f64 / 1200.0;
        cumulative_tot_int += int_amt;
    }
    cumulative_tot_int
}

// initial_depo_amt, initial_depo_date,int_rate
fn get_broken_cf_int(initial_depo_amt: f64, initial_depo_date: i64, int_rate: f64) -> f64 {
    let no_of_days = num_days_start_to_end(
        rbdate::date_from_timestamp(initial_depo_date),
        get_month_end_date(rbdate::date_from_timestamp(initial_depo_date)),
    ) + 1;
    let broken_int_amt = (initial_depo_amt * int_rate * no_of_days as f64) / 36500.0;
    broken_int_amt
}
