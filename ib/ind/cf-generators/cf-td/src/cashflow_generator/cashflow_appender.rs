use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use macros;
use protobuf;
use rbdate::num_days_start_to_end;
use rbdate::timestamp;
use rbdate::DateParser;
use slog::Logger;
use statics::{DEFAULT_FLOAT, DEFAULT_INT};

pub fn create_account_with_cashflows(
    acc: InputAccount,
    cashflows: Vec<Cashflow>,
    log: &Logger,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
    let min_date_value = date_parser
        .parse_opt("01-01-1970")
        .expect("Cannot parse `min_date_value` value as `DD-MM-YYYY` format");
    let mut tot_int_amt = 0.0;
    let mut tot_prin_amt = 0.0;

    out_acc.key_1 = acc.key_1;
    out_acc.branch_no = acc.branch_no;
    out_acc.curr_status = acc.curr_status;
    out_acc.acc_type = acc.acc_type;
    out_acc.int_cat = acc.int_cat;
    out_acc.inv_type = acc.inv_type;
    out_acc.currency = acc.currency;
    out_acc.customer_no = acc.customer_no;
    out_acc.cr_limit = acc.cr_limit;
    out_acc.curr_bal = acc.curr_bal;
    out_acc.wdl_flag = acc.wdl_flag;
    out_acc.int_available = acc.int_available;
    out_acc.int_proj = acc.int_proj;
    out_acc.acct_open_dt = if let Some(dt) = acc.acct_open_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.int_frm_dt = if let Some(dt) = acc.int_frm_dt {
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
    out_acc.mat_dt = if let Some(dt) = acc.mat_dt {
        if dt < min_date_value {
            DEFAULT_INT
        } else {
            timestamp(dt)
        }
    } else {
        DEFAULT_INT
    };
    out_acc.mat_amt = acc.mat_amt;
    out_acc.lst_rollovr_dt = if let Some(dt) = acc.lst_rollovr_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.lst_ovr_limit_dt = if let Some(dt) = acc.lst_ovr_limit_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.cr_store_rate = acc.cr_store_rate;
    out_acc.dr_store_rate = acc.dr_store_rate;
    out_acc.gl_class_code = acc.gl_class_code;
    out_acc.mop_type = acc.mop_type;
    out_acc.instl_due_day = acc.instl_due_day;
    out_acc.term_int_comp_freq = acc.term_int_comp_freq;
    out_acc.term_int_comp_sop_dt = if let Some(dt) = acc.term_int_comp_sop_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.term_int_comp_eop_dt = if let Some(dt) = acc.term_int_comp_eop_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.term_int_comp_amt = acc.term_int_comp_amt;
    out_acc.lending_status = acc.lending_status;
    out_acc.int_repay_freq = acc.int_repay_freq;
    out_acc.name = acc.name;
    out_acc.cust_acct_no = acc.cust_acct_no;
    out_acc.prim_acct = acc.prim_acct;
    out_acc.segment_code = acc.segment_code;
    out_acc.industry_code = acc.industry_code;
    out_acc.group_code = acc.group_code;
    out_acc.bus_sector_code = acc.bus_sector_code;
    out_acc.tier_cust_type = acc.tier_cust_type;
    out_acc.a1 = acc.a1;
    out_acc.a2 = acc.a2;
    out_acc.a3 = acc.a3;
    out_acc.a4 = acc.a4;
    out_acc.a5 = acc.a5;
    out_acc.a6 = acc.a6;
    out_acc.a7 = acc.a7;
    out_acc.a8 = acc.a8;
    out_acc.a9 = acc.a9;
    out_acc.a10 = acc.a10;
    out_acc.glcode = acc.glcode;
    out_acc.int_rate = acc.int_rate;
    out_acc.curr_bal_lcy = acc.curr_bal_lcy;
    out_acc.as_on_date = if let Some(dt) = acc.as_on_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.residual_days = num_days_start_to_end(
        acc.as_on_date.unwrap_or(min_date_value),
        acc.mat_dt.unwrap_or(min_date_value),
    );
    for cf in &cashflows {
        tot_int_amt += cf.int_amt;
        tot_prin_amt += cf.prin_amt;
    }
    out_acc.tot_int_amt = tot_int_amt;
    out_acc.tot_prin_amt = tot_prin_amt;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
