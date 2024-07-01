use super::super::account::AccountData;
use super::super::op_gen::op_account::Account;
use super::super::op_gen::op_account::Cashflow;
use rbdate::timestamp;

pub fn create_account_with_cashflows(acc: AccountData, cashflows: Vec<Cashflow>) -> Account {
    let mut out_acc = Account::new();
    out_acc.coa = acc.coa;
    out_acc.acc_id = acc.acc_id;
    out_acc.acc_open_date = timestamp(acc.acc_open_date);
    out_acc.tenor = acc.tenor;
    out_acc.payout_freq = acc.payout_freq;
    out_acc.int_pay_freq = acc.int_pay_freq;
    out_acc.maturity_date = timestamp(acc.maturity_date);
    out_acc.os_amount = acc.os_amount;
    out_acc.currency = acc.currency;
    out_acc.cf_type = acc.cf_type;
    out_acc.int_basis = acc.int_basis;
    out_acc.int_rate = acc.int_rate;
    out_acc.bm = acc.bm;
    out_acc.bm_freq = acc.bm_freq;
    out_acc.bm_res_days = acc.bm_res_days;
    out_acc.next_rep_date = timestamp(acc.next_rep_date);
    out_acc.bm_rate = acc.bm_rate;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}
