use account_reader_writer::account_reader::input_account::InputAccount;
use account_reader_writer::account_without_cashflows::OutputAccount;
use macros;
use rbdate;
use slog::Logger;
use statics::*;

pub fn create_account_without_cashflows(account: InputAccount, log: &Logger) -> OutputAccount {
    let mut out_acc = OutputAccount::new();
    out_acc.key_1 = account.key_1;
    out_acc.branch_no = account.branch_no;
    out_acc.curr_status = account.curr_status;
    out_acc.acct_type = account.acct_type;
    out_acc.int_cat = account.int_cat;
    out_acc.inv_type = account.inv_type;
    out_acc.currency = account.currency;
    out_acc.customer_no = account.customer_no;
    out_acc.cr_limit = account.cr_limit;
    out_acc.curr_bal = account.curr_bal;
    out_acc.wdl_flag=account.wdl_flag;
    out_acc.int_available = account.int_available;
    out_acc.acc_open_dt = if let Some(dt) = account.acc_open_dt {
        rbdate::timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.int_frm_dt = if let Some(dt) = account.int_frm_dt {
        rbdate::timestamp(dt)
    } else {
        DEFAULT_INT
    }; 
    out_acc.int_to_dt = if let Some(dt) = account.int_to_dt {
        rbdate::timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.no_dues = account.no_dues;
    out_acc.var_int_rate = account.var_int_rate;
    out_acc.rval_ind = account.rval_ind;
    out_acc.od_visa_area=account.od_visa_area;
    out_acc.lst_ovr_limit_date =if let Some(dt) = account.lst_ovr_limit_date {
        rbdate::timestamp(dt)
    } else {
        DEFAULT_INT
    }; 
    out_acc.cr_store_rate = account.cr_store_rate;
    out_acc.dr_store_rate = account.dr_store_rate;
    out_acc.gl_class_code = account.gl_class_code;
    out_acc.mop_type = account.mop_type;
    out_acc.instl_due_day = account.instl_due_day;
    out_acc.term_int_comp_freq = account.term_int_comp_freq;
    out_acc.term_int_cmp_eop_dt =if let Some(dt) = account.term_int_cmp_eop_dt {
        rbdate::timestamp(dt)
    } else {
        DEFAULT_INT
    }; 
    out_acc.term_int_cmp_sop_dt =if let Some(dt) = account.term_int_cmp_sop_dt {
        rbdate::timestamp(dt)
    } else {
        DEFAULT_INT
    }; 
    out_acc.term_int_comp_amt = account.term_int_comp_amt;
    out_acc.lending_status = account.lending_status;
    out_acc.name = account.name;
    out_acc.cust_acct_no = account.cust_acct_no;
    out_acc.prim_acct = account.prim_acct;
    out_acc.segment_code = account.segment_code;
    out_acc.industry_code = account.industry_code;
    out_acc.grup_code = account.grup_code;
    out_acc.bus_sector_code = account.bus_sector_code;
    out_acc.tier_cust_type = account.tier_cust_type;
    out_acc.a1 = account.a1;
    out_acc.a2 = account.a2;
    out_acc.a3 = account.a3;
    out_acc.a4 = account.a4;
    out_acc.a5 = account.a5;
    out_acc.a6 = account.a6;
    out_acc.a7 = account.a7;
    out_acc.a8 = account.a8;
    out_acc.a9 = if let Some(dt) = account.a9 {
        rbdate::timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.a10 = if let Some(dt) = account.a10 {
        rbdate::timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.gl_code = account.gl_code;
    out_acc.int_rate = account.int_rate;
    out_acc.curr_bal_lcy = account.curr_bal_lcy;
    out_acc.as_on_date = if let Some(dt) = account.as_on_date {
        rbdate::timestamp(dt)
    } else {
        DEFAULT_INT
    };

    out_acc
}
