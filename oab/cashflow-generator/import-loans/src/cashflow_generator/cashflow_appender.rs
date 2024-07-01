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
    out_acc.customer_id = account.customer_id.to_string();
    out_acc.customer_type = account.customer_type;
    out_acc.lcy_amount = account.lcy_amount;
    out_acc.reference = account.reference;
    out_acc.npa_flag = account.npa_flag;
    out_acc.npa_type = account.npa_type;
    out_acc.interest_type = account.interest_type;
    out_acc.int_repayment_frequency = account.int_repayment_frequency;
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
    out_acc.cust_constitution_code = account.cust_constitution_code;
    for cf in &cashflows {
        tot_int_amt += cf.int_amt;
        tot_prin_amt += cf.prin_amt;
    }
    out_acc.rate_flag = account.rate_flag;
    out_acc.customer_name = account.customer_name;
    out_acc.product_code = account.product_code;
    out_acc.account_type = account.account_type;
    out_acc.branch = account.branch;
    out_acc.rm = account.rm;
    out_acc.group_code = account.group_code;
    out_acc.monthly_avg_bal = account.monthly_avg_bal;
    out_acc.customer_rating = account.customer_rating;
    out_acc.p2 = account.p2;
    out_acc.waiver_flag = account.waiver_flag;
    out_acc.accrued_int_amt = account.accrued_int_amt;
    out_acc.string1 = account.string1;
    out_acc.string2 = account.string2;
    out_acc.string3 = account.string3;
    out_acc.number1 = account.number1;
    out_acc.number2 = account.number2;
    out_acc.number3 = account.number3;

    out_acc.total_interest_amount = tot_int_amt;
    out_acc.total_principal_amount = tot_prin_amt;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows.clone());

    out_acc
}
