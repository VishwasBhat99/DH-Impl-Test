use crate::configuration_parameters::ConfigurationParameters;
use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use rbdate::timestamp;

pub fn create_account_with_cashflows(
    input_account: InputAccount,
    config_params: &ConfigurationParameters,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut op = AccountWithCashflows::new();
    op.customer_id = input_account.customer_id;
    op.account_id = input_account
        .account_id
        .to_string()
        .replace("'", "")
        .replace("\"", "")
        .replace("`", "");
    op.prod_type = input_account.prod_type;
    op.scheme_type = input_account.scheme_type;
    op.prod_code = input_account.prod_code;
    op.currency = input_account.currency;
    op.customer_type = input_account.customer_type;
    op.gl_account_principal = input_account.gl_account_principal;
    op.open_date = if input_account.open_date.is_some() {
        timestamp(
            input_account
                .open_date
                .unwrap_or(*config_params.as_on_date()),
        )
    } else {
        0
    };

    op.value_date = if input_account.value_date.is_some() {
        timestamp(
            input_account
                .value_date
                .unwrap_or(*config_params.as_on_date()),
        )
    } else {
        0
    };

    op.maturity_date = if input_account.maturity_date.is_some() {
        timestamp(
            input_account
                .maturity_date
                .unwrap_or(*config_params.as_on_date()),
        )
    } else {
        0
    };
    op.limit_amt = input_account.limit_amt;
    op.curr_bal_amount = input_account.curr_bal_amount;
    op.asset_type = input_account.asset_type;
    op.loan_limit_amount = input_account.loan_limit_amount;
    op.index_code = input_account.index_code;
    op.effective_roi = input_account.effective_roi;
    op.reset_frequency = input_account.reset_frequency;
    op.next_reset_date = if input_account.next_reset_date.is_some() {
        timestamp(
            input_account
                .next_reset_date
                .unwrap_or(*config_params.as_on_date()),
        )
    } else {
        0
    };
    op.classification = input_account.classification;

    if input_account.maturity_date == input_account.final_reset_date {
        let derived_final_reset_date = cashflows[cashflows.len() - 1].date;
        op.final_reset_date = derived_final_reset_date
    } else {
        op.final_reset_date = if input_account.final_reset_date.is_some() {
            timestamp(
                input_account
                    .final_reset_date
                    .unwrap_or(*config_params.as_on_date()),
            )
        } else {
            0
        };
    }
    op.npa_final_status = input_account.npa_final_status;
    op.add_string_1 = input_account.add_string_1;
    op.add_string_2 = input_account.add_string_2;
    op.add_int_1 = input_account.add_int_1;
    op.add_int_2 = input_account.add_int_2;
    op.add_float_1 = input_account.add_float_1;
    op.add_float_2 = input_account.add_float_2;
    op.add_date_1 = if input_account.add_date_1.is_some() {
        timestamp(
            input_account
                .add_date_1
                .unwrap_or(*config_params.as_on_date()),
        )
    } else {
        0
    };
    op.add_date_2 = if input_account.add_date_2.is_some() {
        timestamp(
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
