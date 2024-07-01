use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use cashflow_derivator::field_struct::Fields;
use cashflow_derivator::{str_to_flt, str_to_int};
use macros;
use rbdate::timestamp;
use slog::Logger;
use statics;
use statics::*;
use std::collections::HashMap;
pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut tot_int_amt = DEFAULT_FLOAT;
    let mut tot_prin_amt = DEFAULT_FLOAT;

    out_acc.flow_id = account.flow_id;
    out_acc.grp_id = account.grp_id;
    out_acc.llg_id = account.llg_id;
    out_acc.amount = account.amount;
    out_acc.ccy_id = account.ccy_id;
    out_acc.intr_rate = account.intr_rate;
    out_acc.reprice_freq = account.reprice_freq;
    out_acc.reprice_dt = if let Some(dt) = account.reprice_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.mat_dt = if let Some(dt) = account.mat_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.acc_num = account.acc_num;
    out_acc.strt_dt = if let Some(dt) = account.strt_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.intr_cal_freq = account.intr_cal_freq;
    out_acc.is_float_rate = account.is_float_rate;
    out_acc.float_rate_bm = account.float_rate_bm;
    out_acc.bu_id = account.bu_id;
    out_acc.cust_id = account.cust_id;
    out_acc.cust_name = account.cust_name;
    out_acc.sprd = account.sprd;
    out_acc.schm_code = account.schm_code;
    out_acc.min_ir = account.min_ir;
    out_acc.max_ir = account.max_ir;
    out_acc.dep_amount = account.dep_amount;
    out_acc.mat_amt = account.mat_amt;
    out_acc.exch_rate = account.exch_rate;
    out_acc.cust_ctry_code = account.cust_ctry_code;
    out_acc.cust_crdt_rtng = account.cust_crdt_rtng;
    out_acc.cust_sect_code = account.cust_sect_code;
    out_acc.cust_indt_code = account.cust_indt_code;
    out_acc.custom1 = account.custom1;
    out_acc.custom2 = account.custom2;
    for cf in &cashflows {
        tot_int_amt += cf.interest_amount;
        tot_prin_amt += cf.principal_amount;
    }
    out_acc.total_interest_amount = tot_int_amt;
    out_acc.total_principal_amount = tot_prin_amt;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}
