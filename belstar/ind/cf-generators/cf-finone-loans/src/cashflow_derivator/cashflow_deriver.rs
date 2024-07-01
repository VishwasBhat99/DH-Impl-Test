// use chrono::format::parse;
use chrono::Datelike;
// use chrono::Month;
use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::Cashflow;
use chrono::Duration;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::{incr_dt_by_mon_presrv_eom_checked, DateParser};
use sdb_day_convention::{get_int_amt, Conventions};
use slog::Logger;
use statics::*;
pub struct CfWithAmount {
    pub cfs: Vec<Cashflow>,
    pub total_interest: f64,
    pub total_principal: f64,
}
pub fn cashflow_deriver(
    input_account: InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
    tot_cfs: &mut usize,
    tot_acc_with_cfs: &mut i64,
    tot_prin_in_ip: &mut f64,
    tot_prin_in_op: &mut f64,
    tot_int_in_op: &mut f64,
    tot_int_in_ip: &mut f64,
) -> CfWithAmount {
    let mut cfs: Vec<Cashflow> = Vec::new();
    let mut cf: Cashflow = Cashflow::new();
    let mut total_interest = DEFAULT_FLOAT;
    let mut total_principal = DEFAULT_FLOAT;
    *tot_acc_with_cfs += 1;
    //check the frequency type
    let freq_type = if input_account.int_recry_freq.is_empty() {
        "Monthly".to_string()
    } else {
        input_account.int_recry_freq.clone()
    };
    //get the cashflow frequency on the basis of frequency type
    let cf_freq = match &freq_type[..] {
        "Weekly" => 7,
        "Monthly" => 1,
        "Quarterly" => 3,
        "Half Yearly" => 6,
        "Yearly" => 12,
        _ => {
            log_warn!(
                    log,
                    "Payment frequency '{}' is incorrect for account: {}. Using Default Pay Freq: Monthly",
                    freq_type, input_account.loan_account_no
                );
            1
        }
    };
    //Derivation of Maturity Date
    let maturity_date = if input_account.maturity_date.is_some() {
        input_account
            .maturity_date
            .unwrap_or(*config_params.as_on_date())
    } else {
        let day = input_account.due_day;
        let derive_disbursal_date = incr_dt_by_mon_presrv_eom_checked(
            input_account
                .disbursal_date
                .unwrap_or(*config_params.as_on_date()),
            input_account.original_tenure as usize,
        )
        .unwrap_or(*config_params.as_on_date());
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let formatted_maturity_date = format!(
            "{}-{}-{}",
            &day,
            derive_disbursal_date.month(),
            &derive_disbursal_date.year()
        );
        date_parser.parse(&formatted_maturity_date)
    };
    //starting CF DATE
    let mut cf_date = input_account
        .next_instmt_due_date
        .unwrap_or(*config_params.as_on_date());
    let convention: Conventions = match config_params.convention_type() {
        "ACTbyACT" => Conventions::ACTbyACT,
        "ACTby365" => Conventions::ACTby365,
        "ACTby360" => Conventions::ACTby360,
        "Thirtyby365" => Conventions::Thirtyby360,
        _default => Conventions::ACTby365,
    };
    // get the interest amount using the disbursal_Date and next_instmt_due_date
    let disbursal_date = if input_account.last_payment_date.is_some() {
        input_account.last_payment_date
    } else {
        if input_account.disbursal_date.is_some() {
            input_account.disbursal_date
        } else {
            panic!("No logic can be used for deriving at the exact Disbursal Date.Because disbursal date and last payment date bith has null values");
        }
    };
    let mut interest_amount = get_int_amt(
        disbursal_date.unwrap_or(*config_params.as_on_date()),
        input_account
            .next_instmt_due_date
            .unwrap_or(*config_params.as_on_date()),
        &convention,
        input_account.os_prin,
        input_account.cust_int_rate,
    )
    .unwrap_or(DEFAULT_FLOAT);
    *tot_int_in_op += interest_amount;
    total_interest += interest_amount;
    *tot_int_in_ip += interest_amount;
    let mut principal_amount = input_account.emi_amount - interest_amount;
    total_principal += principal_amount;
    *tot_prin_in_op += principal_amount;
    *tot_prin_in_ip += principal_amount;

    let mut curr_cf_date;
    let mut outstanding_amt = input_account.os_prin;
    //out
    while cf_date <= maturity_date && outstanding_amt >= principal_amount {
        *tot_cfs += 1;
        if input_account.emi_amount < interest_amount {
            break;
        }
        cf = new_cashflow(
            interest_amount,
            principal_amount,
            rbdate::timestamp(cf_date),
        );
        cfs.push(cf);
        if freq_type == "Weekly" {
            curr_cf_date = cf_date + Duration::days(cf_freq);
        } else {
            curr_cf_date = incr_dt_by_mon_presrv_eom_checked(cf_date, cf_freq as usize)
                .unwrap_or(*config_params.as_on_date());
        }
        outstanding_amt -= principal_amount;
        interest_amount = get_int_amt(
            cf_date,
            curr_cf_date,
            &convention,
            outstanding_amt,
            input_account.cust_int_rate,
        )
        .unwrap_or(0.0);
        principal_amount = input_account.emi_amount - interest_amount;
        total_principal += principal_amount;
        total_interest += interest_amount;
        cf_date = curr_cf_date;
        *tot_int_in_op += interest_amount;
        *tot_prin_in_op += principal_amount;
    }
    //derive the last cash flow While outstanding amt is not 0 but less than Emi amount
    if outstanding_amt > 0.0 {
        *tot_cfs += 1;
        curr_cf_date = maturity_date;
        interest_amount = get_int_amt(
            cf_date,
            curr_cf_date,
            &convention,
            outstanding_amt,
            input_account.cust_int_rate,
        )
        .unwrap_or(DEFAULT_FLOAT);
        cf = new_cashflow(
            interest_amount,
            outstanding_amt,
            rbdate::timestamp(curr_cf_date),
        );
        *tot_int_in_op += interest_amount;
        *tot_prin_in_op += outstanding_amt;
        total_interest += interest_amount;
        total_principal += outstanding_amt;
        cfs.push(cf);
    }
    CfWithAmount {
        cfs,
        total_interest,
        total_principal,
    }
}
pub fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = d;
    cf
}
