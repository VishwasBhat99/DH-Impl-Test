use crate::configuration_parameters::ConfigurationParameters;
use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use rbdate::{incr_dt_by_mon_presrv_eom_checked, timestamp};
use slog::Logger;
use statics::DEFAULT_FLOAT;

pub fn create_account_with_cashflows(
    input_account: InputAccount,
    config_params: &ConfigurationParameters,
    _log: &Logger,
) -> AccountWithCashflows {
    let mut cfs: Vec<Cashflow> = Vec::new();
    let mut op = AccountWithCashflows::new();
    op.loan_id = input_account.loan_id.to_string();
    op.state = input_account.state;
    op.region = input_account.region;
    op.branch_name = input_account.branch_name;
    op.village = input_account.village;
    op.group = input_account.group;
    op.group_id = input_account.group_id;
    op.branch_model = input_account.branch_model;
    op.member_name = input_account.member_name;
    op.member_id = input_account.member_id;
    op.prod_name = input_account.prod_name;
    op.purpose_of_loan = input_account.purpose_of_loan;
    op.rate_of_interest = input_account.rate_of_int;
    op.disb_date = timestamp(
        input_account
            .disb_date
            .unwrap_or(*config_params.as_on_date()),
    );
    op.amt_disbursed = input_account.amt_disbursed;
    op.first_od_date = timestamp(
        input_account
            .first_od_date
            .unwrap_or(*config_params.as_on_date()),
    );
    op.npa_date = timestamp(
        input_account
            .npa_date
            .unwrap_or(*config_params.as_on_date()),
    );
    op.prin_due_on_npa = input_account.prin_due_npa;
    op.int_due_on_npa = input_account.int_due_npa;
    op.prin_os_on_npa = input_account.prin_os_npa;
    op.prin_collected = input_account.prin_collected;
    op.int_collected = input_account.int_collected;
    op.dpd_day = input_account.dpd_day;
    op.standard = input_account.standard;
    op.prin_due_on_reporting = input_account.prin_due_reporting;
    op.int_due_on_reporting = input_account.int_due_reporting;
    op.prin_os_on_reporting = input_account.prin_os_reporting;
    op.npa_int_accr = input_account.npa_int_accr;
    op.npa_int_during_period = input_account.npa_int_during_period;
    op.funder_name = input_account.funder_name;
    op.maturity_dt = timestamp(
        input_account
            .maturity_dt
            .unwrap_or(*config_params.as_on_date()),
    );
    op.currency = "INR".to_string();
    let cashflow = new_cashflow(
        DEFAULT_FLOAT,
        input_account.prin_os_reporting,
        timestamp(incr_dt_by_mon_presrv_eom_checked(*config_params.as_on_date(), 36).unwrap()),
    );
    cfs.push(cashflow);
    op.cashflows = protobuf::RepeatedField::from_vec(cfs);

    op
}

pub fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.principal_amount = p_a;
    cf.interest_amount = i_a;
    cf.cfdate = d;
    cf
}
