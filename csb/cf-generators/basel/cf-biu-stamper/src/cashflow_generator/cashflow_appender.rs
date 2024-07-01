use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::Account;
use rbdate::timestamp;
use statics::*;

pub fn append_data<'a>(
    account: InputAccount,
    t1: String,
    t2: String,
    t3: String,
    t4: String,
    total_deposits: f64,
) -> Account {
    let mut out_acc = Account::new();
    out_acc.acc_no = account.acc_no;
    out_acc.cust_id = account.cust_id;
    out_acc.prod_code = account.prod_code;
    out_acc.currency = account.currency;
    out_acc.mis = account.mis;
    out_acc.mat_date = if let Some(dt) = account.mat_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.amount = account.amount;
    out_acc.lcy_amount = account.lcy_amount;
    out_acc.cust_type = account.cust_type;
    out_acc.res_days = account.res_days;
    out_acc.is_nwd = account.is_nwd;
    out_acc.is_nwd_final = account.is_nwd_final;
    out_acc.bkt_id = account.bkt_id;
    out_acc.t1 = t1;
    out_acc.t2 = t2;
    out_acc.t3 = t3;
    out_acc.t4 = t4;
    out_acc.total_deposits = total_deposits;
    out_acc
}
