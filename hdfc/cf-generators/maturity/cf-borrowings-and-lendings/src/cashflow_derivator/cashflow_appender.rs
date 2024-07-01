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
    let mut total_interest_amount = DEFAULT_FLOAT;
    let mut total_principal_amount = DEFAULT_FLOAT;

    out_acc.deal_no = account.deal_no;
    out_acc.int_basis = account.int_basis;
    out_acc.couprt = if let Some(rt) = account.couprt {
        rt
    } else {
        DEFAULT_FLOAT
    };
    out_acc.val_dt = if let Some(dt) = account.val_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.mat_dt = if let Some(dt) = account.mat_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.currcd = account.currcd;
    out_acc.orgballcy = if let Some(amt) = account.orgballcy {
        amt
    } else {
        DEFAULT_FLOAT
    };
    out_acc.int_amt = if let Some(amt) = account.int_amt {
        amt
    } else {
        DEFAULT_FLOAT
    };
    out_acc.counterpartyname = account.counterpartyname;
    out_acc.as_of_date = timestamp(account.as_of_date);
    out_acc.paymenttype = account.paymenttype;
    out_acc.rt_flag = account.rt_flag;
    out_acc.reprice_index = account.reprice_index;
    out_acc.reprice_spread = account.reprice_spread;
    out_acc.alm_line = account.alm_line;
    out_acc.nxt_rep_dt = DEFAULT_INT;
    if out_acc.rt_flag == "T" {
        out_acc.is_float = "Y".to_string();
    } else {
        out_acc.is_float = "N".to_string()
    }
    for cf in &cashflows {
        total_interest_amount += cf.int_amt;
        total_principal_amount += cf.prin_amt;
    }
    out_acc.total_interest_amount = total_interest_amount;
    out_acc.total_principal_amount = total_principal_amount;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
