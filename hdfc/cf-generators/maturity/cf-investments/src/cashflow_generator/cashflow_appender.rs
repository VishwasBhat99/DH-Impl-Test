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
    let mut tot_int_amt = DEFAULT_FLOAT;
    let mut tot_prin_amt = DEFAULT_FLOAT;

    out_acc.deal_no = account.deal_no;
    out_acc.short_name = account.short_name;
    out_acc.nxt_rep_dt = if let Some(dt) = account.nxt_rep_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.deal_dt = if let Some(dt) = account.deal_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.portfolio = account.portfolio;
    out_acc.deal_rt = account.deal_rt;
    out_acc.org_face_val = account.org_face_val;
    out_acc.os_face_val = account.os_face_val;
    out_acc.org_cst_val = account.org_cst_val;
    out_acc.acrd_int = account.acrd_int;
    out_acc.book_yield = account.book_yield;
    out_acc.int_basis = account.int_basis;
    out_acc.avg_os_vd = account.avg_os_vd;
    out_acc.avg_os_dd = account.avg_os_dd;
    out_acc.os_cost_val = if let Some(amt) = account.os_cost_val {
        amt
    } else {
        DEFAULT_FLOAT
    };
    out_acc.org_bal = account.org_bal;
    out_acc.coup_rt = account.coup_rt;
    out_acc.nxt_coup_dt = if let Some(dt) = account.nxt_coup_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.gl = account.gl;
    out_acc.mat_dt = if let Some(dt) = account.mat_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.secu_desc = account.secu_desc;
    out_acc.prod_desc = account.prod_desc;
    out_acc.prod_cd = account.prod_cd;
    out_acc.lst_coup_dt = if let Some(dt) = account.lst_coup_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.call_dt1 = if let Some(dt) = account.call_dt1 {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };

    out_acc.coup_freq = account.coup_freq;
    out_acc.val_dt = if let Some(dt) = account.val_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.acrl_freq = account.acrl_freq;
    out_acc.lst_rep_dt = if let Some(dt) = account.lst_rep_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.lst_put_dt = if let Some(dt) = account.lst_put_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.inst = account.inst;
    out_acc.org_term = account.org_term;
    out_acc.acrl_basis = account.acrl_basis;
    out_acc.div = account.div;
    out_acc.alm_line = account.alm_line;
    out_acc.cmpnd_freq = account.cmpnd_freq;
    out_acc.nxt_cmpnd_dt = if let Some(dt) = account.nxt_cmpnd_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.rt_chng_freq = account.rt_chng_freq;
    out_acc.rt_flg = account.rt_flg;
    out_acc.nxt_pay_dt = if let Some(dt) = account.nxt_pay_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.prev_rep_dt = if let Some(dt) = account.prev_rep_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.int_pay_freq = account.int_pay_freq;
    out_acc.int_rt = if let Some(rt) = account.int_rt {
        rt
    } else {
        DEFAULT_FLOAT
    };
    out_acc.call_dt = if let Some(dt) = account.call_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.put_dt = if let Some(dt) = account.put_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.port_typ = account.port_typ;
    for cf in &cashflows {
        tot_int_amt += cf.int_amt;
        tot_prin_amt += cf.prin_amt;
    }
    out_acc.total_interest_amount = tot_int_amt;
    out_acc.total_principal_amount = tot_prin_amt;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}
