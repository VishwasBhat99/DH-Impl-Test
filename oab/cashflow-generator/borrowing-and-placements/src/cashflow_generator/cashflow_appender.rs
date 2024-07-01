use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use protobuf;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut tot_int_amt = 0.0;
    let mut tot_prin_amt = 0.0;

    out_acc.account_id = account.account_id.clone();
    out_acc.outstanding_bal = account.outstanding_bal;
    out_acc.currency = if account.currency == "" {
        "NA".to_string()
    } else {
        account.currency
    };
    out_acc.start_date = rbdate::timestamp(account.start_date);
    out_acc.maturity_date = rbdate::timestamp(account.maturity_date);
    out_acc.int_rate = account.int_rate;
    out_acc.int_rate_classification = account.int_rate_classification;
    out_acc.benchmark = account.benchmark;
    out_acc.repricing_frequency = account.repricing_frequency;
    out_acc.last_repr_date = account.last_repr_date;
    out_acc.next_repr_date = account.next_repr_date;
    out_acc.coupon_payment_start_date = account.coupon_payment_start_date;
    out_acc.coupon_payment_frequency = account.coupon_payment_frequency;
    out_acc.cust_constitution_code = account.cust_constitution_code;
    out_acc.instrument = account.instrument;
    out_acc.counter_party_id = account.counter_party_id;
    out_acc.counter_party_name = account.counter_party_name;
    out_acc.counter_party_type = account.counter_party_type;
    out_acc.customer_id = account.customer_id;
    out_acc.customer_name = account.customer_name;
    out_acc.product_code = account.product_code;
    out_acc.account_type = account.account_type;
    out_acc.gl = account.gl;
    out_acc.rate_flag = account.rate_flag;
    out_acc.branch = account.branch;
    out_acc.rm = account.rm;
    out_acc.group_code = account.group_code;
    out_acc.monthly_avg_bal = account.monthly_avg_bal;
    out_acc.tenor = account.tenor;
    for cf in &cashflows {
        tot_int_amt += cf.int_amt;
        tot_prin_amt += cf.prin_amt;
    }

    out_acc.total_interest_amount = tot_int_amt;
    out_acc.total_principal_amount = tot_prin_amt;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows.clone());

    out_acc
}
