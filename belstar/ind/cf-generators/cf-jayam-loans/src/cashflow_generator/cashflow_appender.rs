use crate::configuration_parameters::ConfigurationParameters;
use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use rbdate::timestamp;
use slog::Logger;

pub fn create_account_with_cashflows(
    input_account: InputAccount,
    config_params: &ConfigurationParameters,
    _log: &Logger,
) -> AccountWithCashflows {
    let mut op = AccountWithCashflows::new();
    op.loan_acc_no = input_account.loan_acc_no.to_string();
    op.cust_id = input_account.cust_id;
    op.cust_name = input_account.cust_name;
    op.prod_name = input_account.prod_name;
    op.no_of_installments = input_account.no_of_installments;
    op.sanc_amt = input_account.sanc_amt;
    op.disbursed_amt = input_account.disbursed_amt;
    op.prin_os = input_account.prin_os;
    op.od_prin = input_account.od_prin;
    op.od_int = input_account.od_int;
    op.int_rate = input_account.int_rate;
    op.int_type = input_account.int_type;
    op.currency = input_account.currency;
    op.branch_cd = input_account.branch_cd;
    op.value_dt = timestamp(
        input_account
            .value_dt
            .unwrap_or(*config_params.as_on_date()),
    );
    op.maturity_dt = timestamp(
        input_account
            .maturity_dt
            .unwrap_or(*config_params.as_on_date()),
    );
    op.gl_code = input_account.gl_code;
    op.date = input_account.date;
    op.interest = input_account.interest;
    op.principal = input_account.principal;
    op.principal_os = input_account.principal_os;
    op.spread = input_account.spread;
    op.last_payment_dt = timestamp(
        input_account
            .last_payment_dt
            .unwrap_or(*config_params.as_on_date()),
    );
    op.next_reset_date = timestamp(
        input_account
            .next_reset_date
            .unwrap_or(*config_params.as_on_date()),
    );
    op.last_reset_dt = timestamp(
        input_account
            .last_reset_dt
            .unwrap_or(*config_params.as_on_date()),
    );
    op.division = input_account.division;
    op.alm_line = input_account.alm_line;
    op.ia_llg = input_account.ia_llg;
    op.balm_llg = input_account.balm_llg;
    op.tenure = input_account.tenure;
    op.remaining_tenure = input_account.remaining_tenure;
    op.as_on_date = timestamp(*config_params.as_on_date());
    op.cust_type = input_account.cust_type;
    op.npa_type = input_account.npa_type;

    op
}
