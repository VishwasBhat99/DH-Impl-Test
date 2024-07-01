use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use rbdate::timestamp;
use statics::*;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();

    out_acc.col_id = account.col_id;
    out_acc.acc_id = account.acc_id;
    out_acc.acc_type = account.acc_type;
    out_acc.cust_id = account.cust_id;
    out_acc.col_type_cd = account.col_type_cd;
    out_acc.col_type_desc = account.col_type_desc;
    out_acc.tot_val_of_col = account.tot_val_of_col;
    out_acc.ccy = account.ccy;
    out_acc.tot_mk_val_of_col = account.tot_mk_val_of_col;
    out_acc.mat_dt = timestamp(account.mat_dt);

    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}
