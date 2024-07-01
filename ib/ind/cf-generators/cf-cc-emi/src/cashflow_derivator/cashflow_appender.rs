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
    let mut tot_int_amt = DEFAULT_FLOAT;
    let mut tot_prin_amt = DEFAULT_FLOAT;

    out_acc.card_no = account.card_no;
    out_acc.card_status = account.card_status;
    out_acc.prd_code = account.prd_code;
    out_acc.acc_no = account.acc_no;
    out_acc.outstanding_bal = account.outstanding_bal;
    out_acc.emi_amt = account.emi_amt;
    out_acc.in_date = if let Some(dt) = account.in_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.tenurs = account.tenurs;
    out_acc.del_cnt = account.del_cnt;
    out_acc.cif_no = account.cif_no;
    out_acc.pan_no = account.pan_no;
    out_acc.maturity_date = if let Some(dt) = account.maturity_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.intrate = account.intrate.parse().unwrap_or(0.0);
    out_acc.bgl = account.bgl;
    out_acc.cgl = account.cgl;
    out_acc.branchcode = account.branchcode;
    out_acc.intamount = account.intamount;
    out_acc.duedate = account.duedate;
    out_acc.group = account.group;
    out_acc.llg = account.llg;
    out_acc.currency = account.currency;
    for cf in &cashflows {
        tot_int_amt += cf.int_amt;
        tot_prin_amt += cf.prin_amt;
    }
    out_acc.tot_int_amt = tot_int_amt;
    out_acc.tot_prin_amt = tot_prin_amt;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
