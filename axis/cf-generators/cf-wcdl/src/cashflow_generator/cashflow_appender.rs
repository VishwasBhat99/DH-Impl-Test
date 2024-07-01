use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::{AccountWithCashflows, Cashflow};
use rbdate;
use statics::*;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    out_acc.branchcode = account.branchcode;
    out_acc.currency = account.currency;
    out_acc.currencyconvertionrate = account.currencyconvertionrate;
    out_acc.acct_num = account.acct_num;
    out_acc.product_id = account.product_id;
    out_acc.customer_id = account.customer_id;
    out_acc.customer_name = account.customer_name;
    out_acc.start_date = account.start_date;
    out_acc.sanctioned_amt = account.sanctioned_amt;
    out_acc.distributed_amt = account.distributed_amt;
    out_acc.int_rate = account.int_rate;
    out_acc.inst_prin_amt = account.inst_prin_amt;
    out_acc.inst_int_amt = account.inst_int_amt;
    out_acc.acc_end_date = if let Some(dt) = account.acc_end_date {
        rbdate::timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.int_cal_freq = account.int_cal_freq;
    out_acc.is_floating_rate = account.is_floating_rate;
    out_acc.benchmark_ass = account.benchmark_ass;
    out_acc.spread = account.spread;
    out_acc.min_int_rate = account.min_int_rate;
    out_acc.max_int_rate = account.max_int_rate;
    out_acc.early_date = if let Some(dt) = account.early_date {
        rbdate::timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.rep_freq = account.rep_freq;
    out_acc.cust_ctry_code = account.cust_ctry_code;
    out_acc.cust_crtd_rt = account.cust_crtd_rt;
    out_acc.cust_sect_code = account.cust_sect_code;
    out_acc.cust_indt_code = account.cust_indt_code;
    out_acc.custom1 = account.custom1;
    out_acc.custom2 = account.custom2;
    out_acc.npa_classification = account.npa_classification;
    out_acc.overdue_days = account.overdue_days;
    out_acc.wcdl_bucket_days = account.wcdl_bucket_days;
    out_acc.gl_sub_head_code = account.gl_sub_head_code;
    out_acc.schm_code = account.schm_code;
    out_acc.seg_code = account.seg_code;
    out_acc.final_seg_code = account.final_seg_code;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
