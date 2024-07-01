use super::tenor_calculations::get_months;
use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use rbdate::timestamp;
use statics::*;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut tot_int_amt = 0.0;
    let mut tot_prin_amt = 0.0;
    out_acc.sl_no = account.sl_no;
    out_acc.deposit_number = account.deposit_number;
    out_acc.financial_year = account.financial_year;
    out_acc.demand_no = account.demand_no;
    out_acc.administering_inst = account.administering_inst;
    out_acc.gl_code = account.gl_code;
    out_acc.nature_of_dep = account.nature_of_dep;
    out_acc.dep_type = account.dep_type;
    out_acc.deposit_date = if let Some(dt) = account.deposit_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.mat_date = if let Some(dt) = account.mat_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.int_rate = account.int_rate;
    out_acc.tenor = account.tenor;
    out_acc.closure_date = if let Some(dt) = account.closure_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.tenor_unit = account.tenor_unit;
    out_acc.investment_amt = account.investment_amt;
    out_acc.remarks = account.remarks;
    out_acc.mat_amt = account.mat_amt;
    out_acc.currency = account.currency;
    out_acc.net_val = account.net_val;
    out_acc.gl_desc = account.gl_desc;
    out_acc.w4b_cd = account.w4b_cd;
    out_acc.w4b_desc = account.w4b_desc;
    out_acc.balm_llg = account.balm_llg;
    out_acc.care_llg = account.care_llg;
    out_acc.ba_llg = account.ba_llg;

    for cf in &cashflows {
        tot_int_amt += cf.int_amt;
        tot_prin_amt += cf.prin_amt;
    }
    out_acc.tot_prin_amt = tot_prin_amt;
    out_acc.tot_int_amt = tot_int_amt;

    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}
