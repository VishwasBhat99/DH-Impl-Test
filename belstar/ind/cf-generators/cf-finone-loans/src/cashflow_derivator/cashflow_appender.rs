use crate::configuration_parameters::ConfigurationParameters;
use crate::statics::DEFAULT_INT;
use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use slog::Logger;

pub fn create_account_with_cashflows(
    input_account: InputAccount,
    config_params: &ConfigurationParameters,
    cashflows: Vec<Cashflow>,
    _log: &Logger,
    total_int: f64,
    total_prin: f64,
) -> AccountWithCashflows {
    let mut op = AccountWithCashflows::new();
    op.loan_account_no = input_account.loan_account_no;
    op.loan_id = input_account.loan_id;
    op.customer_id = input_account.customer_id;
    op.product_id = input_account.product_id;
    op.product_desc = input_account.product_desc;
    op.product_type = input_account.product_type;
    op.recovery_type = input_account.recovery_type;
    op.cust_name = input_account.cust_name;
    op.disbursal_date = {
        if input_account.disbursal_date.is_some() {
            rbdate::timestamp(
                input_account
                    .disbursal_date
                    .unwrap_or(*config_params.as_on_date()),
            )
        } else {
            DEFAULT_INT
        }
    };
    op.due_day = input_account.due_day;
    op.maturity_date = {
        if input_account.maturity_date.is_some() {
            rbdate::timestamp(
                input_account
                    .maturity_date
                    .unwrap_or(*config_params.as_on_date()),
            )
        } else {
            DEFAULT_INT
        }
    };
    op.original_tenure = input_account.original_tenure;
    op.current_tenure = input_account.current_tenure;
    op.principal_recry_freq = input_account.principal_recry_freq;

    op.days_past_due = input_account.days_past_due;
    op.int_recry_freq = input_account.int_recry_freq;
    op.asset_clsfn = input_account.asset_clsfn;
    op.int_type = input_account.int_type;
    op.cust_int_rate = input_account.cust_int_rate;
    op.rate_type = input_account.rate_type;

    op.overdue_prin_amount = input_account.overdue_prin_amount;
    op.overdue_interest_amount = input_account.overdue_interest_amount;
    op.os_prin = input_account.os_prin;
    op.emi_amount = input_account.emi_amount;
    op.accrued_not_recieved_int = input_account.accrued_not_recieved_int;

    op.last_payment_date = {
        if input_account.last_payment_date.is_some() {
            rbdate::timestamp(
                input_account
                    .last_payment_date
                    .unwrap_or(*config_params.as_on_date()),
            )
        } else {
            DEFAULT_INT
        }
    };
    op.next_instmt_due_date = {
        if input_account.next_instmt_due_date.is_some() {
            rbdate::timestamp(
                input_account
                    .next_instmt_due_date
                    .unwrap_or(*config_params.as_on_date()),
            )
        } else {
            DEFAULT_INT
        }
    };
    op.branch_id = input_account.branch_id;
    op.currency_code = input_account.currency_code;
    op.fraud = input_account.fraud;
    op.restructure = input_account.restructure;
    op.A1 = input_account.A1;
    op.A2 = input_account.A2;
    op.A3 = input_account.A3;
    op.A4 = input_account.A4;
    op.A5 = input_account.A5;
    op.A6 = input_account.A6;
    op.A7 = input_account.A7;
    op.A8 = {
        if input_account.A8.is_some() {
            rbdate::timestamp(input_account.A8.unwrap_or(*config_params.as_on_date()))
        } else {
            DEFAULT_INT
        }
    };
    op.A9 = input_account.A9;
    op.A10 = input_account.A10;
    op.A11 = input_account.A11;
    op.A12 = input_account.A12;
    op.A13 = input_account.A13;
    op.A14 = input_account.A14;
    op.A15 = input_account.A15;
    op.A16 = input_account.A16;
    op.A17 = input_account.A17;
    op.A18 = input_account.A18;
    op.A19 = input_account.A19;
    op.A20 = input_account.A20;
    op.A21 = input_account.A21;
    op.A22 = input_account.A22;
    op.A23 = input_account.A23;
    op.A24 = input_account.A24;
    op.A25 = input_account.A25;
    op.A26 = input_account.A26;
    op.A27 = input_account.A27;
    op.A28 = input_account.A28;
    op.A29 = input_account.A29;
    op.A30 = input_account.A30;
    op.A31 = input_account.A31;
    op.A32 = input_account.A32;
    op.A33 = input_account.A33;
    op.A34 = input_account.A34;
    op.A35 = input_account.A35;
    op.A36 = {
        if input_account.A36.is_some() {
            rbdate::timestamp(input_account.A36.unwrap_or(*config_params.as_on_date()))
        } else {
            DEFAULT_INT
        }
    };
    op.A37 = {
        if input_account.A37.is_some() {
            rbdate::timestamp(input_account.A37.unwrap_or(*config_params.as_on_date()))
        } else {
            DEFAULT_INT
        }
    };
    op.A38 = {
        if input_account.A38.is_some() {
            rbdate::timestamp(input_account.A38.unwrap_or(*config_params.as_on_date()))
        } else {
            DEFAULT_INT
        }
    };
    op.A39 = {
        if input_account.A39.is_some() {
            rbdate::timestamp(input_account.A39.unwrap_or(*config_params.as_on_date()))
        } else {
            DEFAULT_INT
        }
    };
    op.A40 = {
        if input_account.A40.is_some() {
            rbdate::timestamp(input_account.A40.unwrap_or(*config_params.as_on_date()))
        } else {
            DEFAULT_INT
        }
    };
    op.total_interest = total_int;
    op.total_principal = total_prin;
    op.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    op
}
