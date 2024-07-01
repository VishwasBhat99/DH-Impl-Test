use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use protobuf;
use rbdate::timestamp;
use statics::DEFAULT_INT;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut tot_int_amt = 0.0;
    let mut tot_prin_amt = 0.0;

    // Standard Fields
    out_acc.account_id = account.account_id;
    out_acc.currency = if account.currency == "" {
        "NA".to_string()
    } else {
        account.currency
    };
    out_acc.int_rate = account.int_rate;
    out_acc.outstanding_bal = account.outstanding_bal;
    out_acc.gl = account.gl;
    out_acc.start_date = rbdate::timestamp(account.start_date);
    out_acc.maturity_date = rbdate::timestamp(account.maturity_date);
    out_acc.rate_flag = account.rate_flag;
    out_acc.branch = account.branch;
    out_acc.customer_id = account.customer_id;
    out_acc.customer_type = account.customer_type;
    out_acc.product_code = account.product_code;

    // Standard Passthrough
    out_acc.group = account.group;
    out_acc.acc_branch = account.acc_branch;
    out_acc.acc_number = account.acc_number;
    out_acc.acc_suffix = account.acc_suffix;
    out_acc.acc_type = account.acc_type;
    out_acc.deal_type = account.deal_type;
    out_acc.repricing_frequency = account.repricing_frequency;
    out_acc.last_repr_date = if let Some(dt) = account.last_repr_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.next_repr_date = if let Some(dt) = account.next_repr_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.int_compounding_frequency = account.int_compounding_frequency;
    out_acc.int_repayment_frequency = account.int_repayment_frequency;
    out_acc.margin_rate = account.margin_rate;
    out_acc.cpas = account.cpas;
    out_acc.cust_constitution_code = account.cust_constitution_code;
    out_acc.customer_rating = account.customer_rating;
    out_acc.p2 = account.p2;
    out_acc.analysis_code = account.analysis_code;
    out_acc.sundry_analysis_code = account.sundry_analysis_code;
    out_acc.numeric_analysis_code = account.numeric_analysis_code;
    out_acc.base_rate_code = account.base_rate_code;
    out_acc.differential_rate_code = account.differential_rate_code;
    out_acc.accrued_int_amt = account.accrued_int_amt;
    out_acc.next_rollover_date = if let Some(dt) = account.next_rollover_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    for cf in &cashflows {
        tot_int_amt += cf.interest_amount;
        tot_prin_amt += cf.principal_amount;
    }
    out_acc.interest_computation_type = account.interest_computation_type;
    out_acc.rm = account.rm;
    out_acc.customer_name = account.customer_name;
    out_acc.monthly_avg_bal = account.monthly_avg_bal;
    out_acc.pension_acc_flag = account.pension_acc_flag;
    out_acc.waiver_flag = account.waiver_flag;
    out_acc.A1 = account.A1;
    out_acc.A2 = account.A2;
    out_acc.A3 = account.A3;
    out_acc.A4 = account.A4;
    out_acc.A5 = account.A5;
    out_acc.A6 = if let Some(dt) = account.A6 {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.A7 = if let Some(dt) = account.A7 {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };;
    out_acc.A8 = account.A8;
    out_acc.A9 = account.A9;
    out_acc.A10 = account.A10;
    out_acc.A11 = account.A11;
    out_acc.A12 = account.A12;
    out_acc.A13 = account.A13;
    out_acc.A14 = account.A14;
    out_acc.A15 = account.A15;
    out_acc.A16 = account.A16;
    out_acc.A17 = account.A17;
    out_acc.A18 = account.A18;
    out_acc.A19 = account.A19;
    out_acc.A20 = account.A20;
    out_acc.total_interest_amount = tot_int_amt;
    out_acc.total_principal_amount = tot_prin_amt;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows.clone());

    out_acc
}
