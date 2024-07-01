use crate::configuration_parameters::ConfigurationParameters;
use cashflow_gen::account_appender::create_account_with_cashflows;
use cashflow_gen::account_reader::input_account::InputAccount;
use cashflow_gen::account_with_cashflows::Cashflow;
use cashflow_gen::account_writer::AccountWithCashflowsWriter;
use macros;
use rbdate::*;
use rbdate::{timestamp, DateParser};
use sdb_day_convention::conventions::Days;
use sdb_day_convention::days_with_convn;
use slog::Logger;
use std::time::SystemTime;

pub fn derive_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
    writer: &mut AccountWithCashflowsWriter,
) -> (f64, f64, usize) {
    let default_date = DateParser::new("%d-%m-%Y".to_string(), false).parse("31-12-1899");
    let mut amount = account.amount;

    let mut cashflow: Vec<Cashflow> = Vec::new();
    let mut total_prin_amt = 0.0;
    let mut prev_cf_date = account
        .date_of_availment
        .expect("Error Reading Date-of-Availment");
    let freq_month = match &account.frequency.to_uppercase()[0..1] {
        "H" => 6,
        "M" => 1,
        "Q" => 3,
        "Y" => 12,
        "B" => 0,
        _ => 0,
    };

    if account.mat_date.unwrap_or(default_date) < *config_params.as_on_date() {
        log_info!(
            log,
            "Skipping Matured Acoount: {}, Mat-Date: {:?}.",
            account.source,
            account.mat_date.unwrap_or(default_date)
        );
        return (0.0, 0.0, 0);
    }

    if freq_month == 0 || account.no_of_installment <= 0 {
        log_info!(
            log,
            "For the Source : {} getting frequency {} Writing {} to Mat-Date: {}.",
            account.source,
            account.frequency,
            account.amount,
            account
                .mat_date
                .unwrap_or(default_date)
                .format("%d-%m-%Y")
                .to_string()
        );
        let days = days_with_convn(
            prev_cf_date,
            account.pmt_st_dt.expect("Error Reading PMT-Start-Date"),
            config_params.day_convention(),
        )
        .expect("Error getting days using Conventions");
        cashflow.push(new_cashflow(
            get_int_amt(account.amount, account, days),
            account.amount,
            timestamp(account.mat_date.unwrap_or(default_date)),
        ));
    } else {
        let mut cf_date = account.pmt_st_dt.unwrap_or(default_date);
        for _count in 0..account.no_of_installment - 1 {
            if cf_date > *config_params.as_on_date() {
                let days = days_with_convn(prev_cf_date, cf_date, config_params.day_convention())
                    .expect("Error getting days using Conventions");
                cashflow.push(new_cashflow(
                    get_int_amt(amount, account, days),
                    account.p_installment,
                    timestamp(cf_date),
                ));
            }
            amount -= account.p_installment;
            total_prin_amt += account.p_installment;
            prev_cf_date = cf_date;
            cf_date = incr_dt_by_mon_presrv_eom(cf_date, freq_month).unwrap_or(default_date);
        }

        total_prin_amt += account.p_installment;
        let days = days_with_convn(
            prev_cf_date,
            account.mat_date.unwrap_or(default_date),
            config_params.day_convention(),
        )
        .expect("Error getting days using Conventions");
        cashflow.push(new_cashflow(
            get_int_amt(account.p_installment, account, days),
            account.p_installment,
            timestamp(account.mat_date.unwrap_or(default_date)),
        ));

        let adjust_amt = account.amount - total_prin_amt;

        if adjust_amt != 0.0 && config_params.is_adj_cf_req() {
            cashflow.push(new_cashflow(
                0.0,
                adjust_amt,
                timestamp(account.mat_date.unwrap_or(default_date)),
            ));
            log_debug!(
                log,
                "Adjustment Prin-Amt: {:?} Written on Mat-Date: {:?}",
                adjust_amt,
                account.mat_date.unwrap_or(default_date)
            );
        }
    }

    let (account_with_cashflows, int_amt, prin_amt, num_of_cfs) = log_measurements!(
        log,
        [format!(
            "Type: CreateAccWithCFs, Identifier: {}",
            account.source
        )],
        create_account_with_cashflows(
            account.clone(),
            total_prin_amt,
            cashflow.to_owned(),
            config_params
        )
    );
    log_measurements!(
        log,
        [format!(
            "Type: WriteAccWithCFs, Identifier: {}",
            account_with_cashflows.source
        )],
        writer.write(account_with_cashflows)
    );
    (int_amt, prin_amt, num_of_cfs)
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = d;
    cf
}

fn get_int_amt(principal_amt: f64, account: &InputAccount, days: Days) -> f64 {
    let num_days = days.days_btw_dts;
    let days_in_yr = days.day_in_yr as f64;
    (principal_amt * account.roi * num_days as f64) / (days_in_yr * 100.0)
}
