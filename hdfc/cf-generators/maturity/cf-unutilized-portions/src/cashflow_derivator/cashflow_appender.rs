use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use rbdate::timestamp;

use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();

    out_acc.account_number = account.account_number;
    out_acc.cust_ref_code = if let Some(val) = account.cust_ref_code {
        val
    } else {
        DEFAULT_INT
    };
    out_acc.pp_table = account.pp_table;
    out_acc.ccf_percent = if let Some(amt) = account.ccf_percent {
        amt
    } else {
        DEFAULT_FLOAT
    };
    out_acc.exp_start_date = if let Some(dt) = account.exp_start_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.exp_end_date = if let Some(dt) = account.exp_end_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.undrawn_amount = if let Some(amt) = account.undrawn_amount {
        amt
    } else {
        DEFAULT_FLOAT
    };
    out_acc.prod_type_desc = account.prod_type_desc;
    out_acc.party_type_desc = account.party_type_desc;
    out_acc.undrn_cov_amount = if let Some(amt) = account.undrn_cov_amount {
        amt
    } else {
        DEFAULT_FLOAT
    };
    out_acc.pre_mitigation_rw_ul = if let Some(amt) = account.pre_mitigation_rw_ul {
        amt
    } else {
        DEFAULT_FLOAT
    };
    out_acc.undrn_rwa = if let Some(amt) = account.undrn_rwa {
        amt
    } else {
        DEFAULT_FLOAT
    };
    out_acc.gl_code = if let Some(amt) = account.gl_code {
        amt
    } else {
        DEFAULT_INT
    };
    out_acc.ccy_code = account.ccy_code;
    out_acc.ret_corporate_ind = account.ret_corporate_ind;
    out_acc.nxt_rep_dt = DEFAULT_INT;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
