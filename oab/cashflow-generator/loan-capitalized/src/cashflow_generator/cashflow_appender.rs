use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use rbdate::{timestamp, NaiveDate};
use statics::*;

pub fn create_account_with_cashflows(
    account: InputAccount,
    nxt_rep_dt: Option<NaiveDate>,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut tot_int_amt = DEFAULT_FLOAT;
    let mut tot_prin_amt = DEFAULT_FLOAT;

    // Standard Fields
    out_acc.account_id = account.account_id;
    out_acc.currency = if account.currency == "" {
        "NA".to_string()
    } else {
        account.currency
    };
    out_acc.int_rate = account.int_rate;
    out_acc.outstanding_bal = if let Some(val) = account.outstanding_bal {
        val
    } else {
        DEFAULT_FLOAT
    };
    out_acc.field_type = account.field_type.clone();
    out_acc.cf_principal_amount = account.cf_principal_amount;
    out_acc.cf_date = if let Some(dt) = account.cf_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.gl = account.gl;
    out_acc.start_date = if let Some(dt) = account.start_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.maturity_date = if let Some(dt) = account.maturity_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
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
    out_acc.next_repr_date = if let Some(dt) = nxt_rep_dt {
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
    out_acc.npa_flag = account.npa_flag;
    out_acc.npa_type = account.npa_type;
    for cf in &cashflows {
        tot_int_amt += cf.int_amt;
        tot_prin_amt += cf.prin_amt;
    }

    out_acc.rm = account.rm;
    out_acc.customer_name = account.customer_name;
    out_acc.monthly_avg_bal = account.monthly_avg_bal;
    out_acc.pension_account_flag = account.pension_account_flag;
    out_acc.waiver_flag = account.waiver_flag;
    out_acc.total_interest_amount = tot_int_amt;
    out_acc.total_principal_amount = tot_prin_amt;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows.clone());

    out_acc
}
