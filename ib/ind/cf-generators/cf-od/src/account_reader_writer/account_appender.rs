use account_reader_writer::account_reader::input_account::InputAccount;
use account_reader_writer::account_without_cashflows::OutputAccount;
use rbdate::timestamp;
use statics::*;

pub fn create_account_without_cashflows(acc: InputAccount) -> OutputAccount {
    let mut out_acc = OutputAccount::new();
    out_acc.key_1 = acc.key_1;
    out_acc.branch_no = acc.branch_no;
    out_acc.curr_status = acc.curr_status;
    out_acc.acct_type = acc.acct_type;
    out_acc.int_cat = acc.int_cat;
    out_acc.inv_type = acc.inv_type;
    out_acc.currency = acc.currency;
    out_acc.customer_no = acc.customer_no;
    out_acc.cr_limit = acc.cr_limit;
    out_acc.curr_bal = acc.curr_bal;
    out_acc.int_available = acc.int_available;
    out_acc.acct_open_dt = if let Some(dt) = acc.acct_open_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.int_from_dt = if let Some(dt) = acc.int_from_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.int_to_dt = if let Some(dt) = acc.int_to_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.no_dues = acc.no_dues;
    out_acc.var_int_rate = acc.var_int_rate;
    out_acc.rval_ind = acc.rval_ind;
    out_acc.lst_ovr_limit_date = if let Some(dt) = acc.lst_ovr_limit_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.cr_store_rate = acc.cr_store_rate;
    out_acc.dr_store_rate = acc.dr_store_rate;
    out_acc.gl_class_code = acc.gl_class_code;
    out_acc.mop_type = acc.mop_type;
    out_acc.instl_due_day = acc.instl_due_day;
    out_acc.lending_status = acc.lending_status;
    out_acc.npa_clsfn = acc.npa_clsfn;
    out_acc.name = acc.name;
    out_acc.cust_acc_no = acc.cust_acc_no;
    out_acc.prim_accnt = acc.prim_accnt;
    out_acc.segment_code = acc.segment_code;
    out_acc.industry_code = acc.industry_code;
    out_acc.grup_code = acc.grup_code;
    out_acc.bus_sector_code = acc.bus_sector_code;
    out_acc.tier_cust_type = acc.tier_cust_type;
    out_acc.a1 = acc.a1;
    out_acc.a2 = acc.a2;
    out_acc.a3 = acc.a3;
    out_acc.a4 = if let Some(dt) = acc.a4 {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.a5 = if let Some(dt) = acc.a5 {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.a6 = if let Some(dt) = acc.a6 {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.a7 = acc.a7;
    out_acc.a8 = acc.a8;
    out_acc.a9 = acc.a9;
    out_acc.a10 = acc.a10;
    out_acc.gl_code = acc.gl_code;
    out_acc.int_rate = acc.int_rate;
    out_acc.curr_bal_lcy = acc.curr_bal_lcy;
    out_acc.as_on_date = if let Some(dt) = acc.as_on_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.account_status = acc.account_status;
    out_acc.a12 = acc.a12;
    out_acc.a13 = acc.a13;
    out_acc.a14 = acc.a14;
    out_acc.a15 = acc.a15;
    out_acc.a16 = acc.a16;

    out_acc
}
