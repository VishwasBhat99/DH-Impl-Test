use account_reader_writer::account_reader::input_account::InputAccount;
use account_reader_writer::account_with_cashflows::Cashflow;
use macros;
use rbdate::{incr_dt_by_days, DateParser, NaiveDate};
use serde::__private::de::InternallyTaggedUnitVisitor;
use slog::Logger;
use statics::DEFAULT_FLOAT;

pub fn generate_cashflows(
    account: &mut InputAccount,
    log: &Logger,
    rollover: i32,
    as_on_date: &NaiveDate
) -> Result<Vec<Cashflow>, String> {
    let date_parser = rbdate::DateParser::new("%d-%m-%Y".to_string(), true);

    let mut cfs = Vec::new();
    let tenor = rbdate::num_days_start_to_end(
        account.acc_open_date.expect("Could not get acc open date."),
        account.mat_date.expect("Could not get mat date."),
    );

    if account.volatility == "N" && (tenor <=90 || tenor > 1825) {
        cfs.push(new_cashflow(
            0.0,
            account.curr_outstanding_bal_lcy,
            &account.mat_date.expect("Could not read maturity date"),
        ));
        return Ok(cfs)
    }
    if account.volatility == "Y" && (tenor <=90 || tenor > 1825) {
        cfs.push(new_cashflow(
            0.0,
            account.curr_outstanding_bal * account.premature_ratio,
            &account.effective_mat_date.expect("Could not read effective maturity date"),
        ));
        cfs.push(new_cashflow(
            0.0,
            account.curr_outstanding_bal - account.curr_outstanding_bal * account.premature_ratio,
            &account.mat_date.expect("Could not read maturity date"),
        ));
        return Ok(cfs)
    } 


    if account.td_overdue_flag == "Y" {
        cfs.push(new_cashflow(
            0.0,
            account.curr_outstanding_bal_lcy,
            &account.mat_date.expect("Could not read maturity date"),
        ));
    } else {
        let mut roll_over_prin: Vec<f64> = Vec::new();
        if account.volatility == "N" {
            roll_over_prin.push(account.curr_outstanding_bal);
            for cf_no in 2..rollover {
                roll_over_prin
                    .push(roll_over_prin[cf_no as usize - 2] * account.rollover_ratio_non_volatile);
            }
        } else {
            roll_over_prin.push(
                account.curr_outstanding_bal
                    - (account.curr_outstanding_bal * account.premature_ratio),
            );
            for cf_no in 2..rollover {
                roll_over_prin
                    .push(roll_over_prin[cf_no as usize - 2] * account.overall_rollover_ratio);
            }
        }

        log_debug!(
            log,
            "Rollover amounts for account number: {} is {:?}",
            account.account_number,
            roll_over_prin
        );
        if account.volatility == "N" {
            let mut prev_date = account.mat_date.expect("Could not read mat date");
            for idx in 1..rollover - 1 {
                if idx == 1 {
                    prev_date = account.mat_date.expect("Could not read mat date");
                    cfs.push(new_cashflow(
                        0.0,
                        roll_over_prin[(idx - 1) as usize]
                            * account.non_rollover_ratio_non_volatile,
                        &prev_date,
                    ));
                } else {
                    prev_date = incr_dt_by_days(prev_date, account.period_of_deposits);
                    cfs.push(new_cashflow(
                        0.0,
                        roll_over_prin[(idx - 1) as usize]
                            * account.non_rollover_ratio_non_volatile,
                        &prev_date,
                    ));
                }
            }
            if tenor >= 91 && tenor < 365 {
                cfs.push(new_cashflow(
                    0.0,
                    roll_over_prin[roll_over_prin.len() - 1],
                    &rbdate::incr_dt_by_mon_presrv_eom_checked(*as_on_date, 48).expect("Could not increase as on date."),
                ));
            } else if tenor >= 365 && tenor <= 1825 {
                cfs.push(new_cashflow(
                    0.0,
                    roll_over_prin[roll_over_prin.len() - 1],
                    &rbdate::incr_dt_by_mon_presrv_eom_checked(*as_on_date, 96).expect("Could not increase as on date."),
                ));
            }
            
        } else {
            let mut prev_date = account.mat_date.expect("Could not read mat date");
            for idx in 0..rollover - 1 {
                if idx == 0 {
                    prev_date = account.effective_mat_date.expect("Could not read mat date");
                    cfs.push(new_cashflow(
                        0.0,
                        account.curr_outstanding_bal * account.premature_ratio,
                        &prev_date,
                    ));
                } else if idx == 1 {
                    prev_date = account.mat_date.expect("Could not read mat date");
                    cfs.push(new_cashflow(
                        0.0,
                        roll_over_prin[(idx - 1) as usize] * account.non_rollover_ratio_volatile,
                        &prev_date,
                    ));
                } else {
                    prev_date = incr_dt_by_days(prev_date, account.period_of_deposits);
                    cfs.push(new_cashflow(
                        0.0,
                        roll_over_prin[(idx - 1) as usize] * account.non_rollover_ratio_volatile,
                        &prev_date,
                    ));
                }
            }
            if tenor >= 91 && tenor < 365 {
                cfs.push(new_cashflow(
                    0.0,
                    roll_over_prin[roll_over_prin.len() - 1],
                    &rbdate::incr_dt_by_mon_presrv_eom_checked(*as_on_date, 48).expect("Could not increase as on date."),
                ));
            } else if tenor >= 365 && tenor <= 1825 {
                cfs.push(new_cashflow(
                    0.0,
                    roll_over_prin[roll_over_prin.len() - 1],
                    &rbdate::incr_dt_by_mon_presrv_eom_checked(*as_on_date, 96).expect("Could not increase as on date."),
                ));
            }
        }
    }
    return Ok(cfs);
}

fn new_cashflow(interest_amount: f64, principal_amount: f64, date: &NaiveDate) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = interest_amount;
    cf.principal_amount = principal_amount;
    cf.date = rbdate::timestamp(*date);
    cf
}
