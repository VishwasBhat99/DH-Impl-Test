use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use rbdate::{timestamp, NaiveDate};
pub static DEFAULT_INT: i64 = 0;

pub fn create_account_with_cashflows(
    account: InputAccount,
    as_on_date: NaiveDate,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();

    out_acc.flow_id = account.flow_id;
    out_acc.group_id = account.group_id;
    out_acc.sub_group_id = account.sub_group_id;
    out_acc.amount = account.amount;
    out_acc.currency = account.currency;
    out_acc.int_rate = account.int_rate;
    out_acc.repr_freq = account.repr_freq;
    out_acc.early_date = if let Some(dt) = account.early_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.maturity_date = if let Some(dt) = account.maturity_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.account_id = account.account_id;
    out_acc.start_date = if let Some(dt) = account.start_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.int_freq = account.int_freq;
    out_acc.is_floating_rate = account.is_floating_rate;
    out_acc.business_unit_id = account.business_unit_id;
    out_acc.floating_bnchmrk = account.floating_bnchmrk;
    out_acc.cust_id = account.cust_id;
    out_acc.cust_name = account.cust_name;
    out_acc.spread = account.spread;
    out_acc.scheme_code = account.scheme_code;
    out_acc.min_ir = account.min_ir;
    out_acc.max_ir = account.max_ir;
    out_acc.principal_amount = account.principal_amount;
    out_acc.maturity_value = account.maturity_value;
    out_acc.ccy_conv_rate = account.ccy_conv_rate;
    out_acc.cust_cnrty_code = account.cust_cnrty_code;
    out_acc.cust_crd_rating = account.cust_crd_rating;
    out_acc.cust_sec_code = account.cust_sec_code;
    out_acc.cust_indt_code = account.cust_indt_code;
    out_acc.custom1 = account.custom1;
    out_acc.custom2 = account.custom2;
    out_acc.waiver = account.waiver;
    out_acc.maturity_modify = account.maturity_modify;
    out_acc.as_on_date = timestamp(as_on_date);

    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}
