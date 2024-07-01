use crate::configuration_parameters::ConfigurationParameters;
use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use rbdate::NaiveDate;

pub fn create_account_with_cashflows(
    input_account: InputAccount,
    config_params: &ConfigurationParameters,
    last_repricing_date: String,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut op = AccountWithCashflows::new();
    op.customer_id = input_account.customer_id;
    op.cod_acct_no = input_account.cod_acct_no;
    op.prod_code = input_account.prod_code;
    op.ccy_code = input_account.ccy_code;
    op.customer_type = input_account.customer_type;
    op.gl_account_principal = input_account.gl_account_principal;
    op.gl_account_interest = input_account.gl_account_interest;
    op.gl_account_accrued = input_account.gl_account_accrued;
    op.acct_open_date = if input_account.acct_open_date.is_some() {
        rbdate::timestamp(
            input_account
                .acct_open_date
                .unwrap_or(*config_params.as_on_date()),
        )
    } else {
        0
    };

    op.first_disb_date = if input_account.first_disb_date.is_some() {
        rbdate::timestamp(
            input_account
                .first_disb_date
                .unwrap_or(*config_params.as_on_date()),
        )
    } else {
        0
    };

    op.maturity_date = if input_account.maturity_date.is_some() {
        rbdate::timestamp(
            input_account
                .maturity_date
                .unwrap_or(*config_params.as_on_date()),
        )
    } else {
        0
    };
    op.due_date = if input_account.maturity_date.is_some() {
        rbdate::timestamp(
            input_account
                .maturity_date
                .unwrap_or(*config_params.as_on_date()),
        )
    } else {
        0
    };
    op.eop_balance = input_account.eop_balance;
    op.as_on_date = rbdate::timestamp(*config_params.as_on_date());
    op.index_rate = input_account.index_rate;
    op.net_rate = input_account.net_rate;
    op.benchmark_name = input_account.benchmark_name;
    op.rate_type_1 = input_account.rate_type_1;
    op.npa_status = input_account.npa_status;
    op.npa_final_status = input_account.npa_final_status;
    op.ptc_flag = input_account.ptc_flag;
    op.last_repricing_date = if NaiveDate::parse_from_str(&last_repricing_date, "%d-%m-%y").is_err()
    {
        0
    } else {
        rbdate::timestamp(NaiveDate::parse_from_str(&last_repricing_date, "%d-%m-%y").unwrap())
    };
    op.index_code = input_account.index_code;
    op.rate_type_2 = input_account.rate_type_2;
    op.next_reset_date = if input_account.next_reset_date.is_some() {
        rbdate::timestamp(
            input_account
                .next_reset_date
                .unwrap_or(*config_params.as_on_date()),
        )
    } else {
        0
    };
    op.reset_frequency = input_account.reset_frequency;
    op.derived_reset_date = if input_account.derived_reset_date.is_some() {
        rbdate::timestamp(
            input_account
                .derived_reset_date
                .unwrap_or(*config_params.as_on_date()),
        )
    } else {
        0
    };

    op.derived_arrear_date = if input_account.derived_arrear_date.is_some() {
        rbdate::timestamp(
            input_account
                .derived_arrear_date
                .unwrap_or(*config_params.as_on_date()),
        )
    } else {
        0
    };
    op.arrear_prin = input_account.arrear_prin;
    op.add_string_1 = input_account.add_string_1;
    op.add_string_2 = input_account.add_string_2;
    op.add_int_1 = input_account.add_int_1;
    op.add_int_2 = input_account.add_int_2;
    op.add_float_1 = input_account.add_float_1;
    op.add_float_2 = input_account.add_float_2;
    op.add_date_1 = if input_account.add_date_1.is_some() {
        rbdate::timestamp(
            input_account
                .add_date_1
                .unwrap_or(*config_params.as_on_date()),
        )
    } else {
        0
    };
    op.add_date_2 = if input_account.add_date_2.is_some() {
        rbdate::timestamp(
            input_account
                .add_date_1
                .unwrap_or(*config_params.as_on_date()),
        )
    } else {
        0
    };
    op.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    op
}
