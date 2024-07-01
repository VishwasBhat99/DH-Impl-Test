use super::tenor_calculations::get_months;
use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use rbdate::{timestamp, NaiveDate};
use statics::*;

pub fn create_account_with_cashflows(
    acc: InputAccount,
    nxt_rep_dt: Option<NaiveDate>,
    lst_rep_dt: Option<NaiveDate>,
    as_on_date: NaiveDate,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut tot_int_amt = DEFAULT_FLOAT;
    let mut tot_prin_amt = DEFAULT_FLOAT;

    out_acc.deal_no = acc.deal_no;
    out_acc.short_name = acc.short_name;
    out_acc.nxt_rep_dt = if let Some(dt) = nxt_rep_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.call_dt = if let Some(dt) = acc.call_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.put_dt = if let Some(dt) = acc.put_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.deal_dt = if let Some(dt) = acc.deal_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.portfolio = acc.portfolio;
    out_acc.deal_rt = acc.deal_rt;
    out_acc.org_face_val = acc.org_face_val;
    out_acc.os_face_val = acc.os_face_val;
    out_acc.org_cst_val = acc.org_cst_val;
    out_acc.acrd_int = acc.acrd_int;
    out_acc.book_yield = acc.book_yield;
    out_acc.int_basis = acc.int_basis;
    out_acc.avg_os_vd = acc.avg_os_vd;
    out_acc.avg_os_dd = acc.avg_os_dd;
    out_acc.prin_amt = acc.prin_amt;
    out_acc.org_bal = acc.org_bal;
    out_acc.coup_rt = acc.coup_rt;
    out_acc.nxt_coup_dt = if let Some(dt) = acc.nxt_coup_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.gl = acc.gl;
    out_acc.cf_dt = if let Some(dt) = acc.cf_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.secu_desc = acc.secu_desc;
    out_acc.prod_desc = acc.prod_desc;
    out_acc.prod_cd = acc.prod_cd;
    out_acc.lst_coup_dt = if let Some(dt) = acc.lst_coup_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.call_dt1 = if let Some(dt) = acc.call_dt1 {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.coup_freq = acc.coup_freq;
    out_acc.val_dt = if let Some(dt) = acc.val_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.acrl_freq = acc.acrl_freq;
    out_acc.lst_rep_dt = if let Some(dt) = lst_rep_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.lst_put_dt = if let Some(dt) = acc.lst_put_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.inst = acc.inst;
    out_acc.org_term = acc.org_term;
    out_acc.acrl_basis = acc.acrl_basis;
    out_acc.div = acc.div;
    out_acc.alm_line = acc.alm_line;
    out_acc.ia_line = acc.ia_line;
    out_acc.cmpnd_freq = acc.cmpnd_freq;
    out_acc.nxt_cmpnd_dt = if let Some(dt) = acc.nxt_cmpnd_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.rt_chng_freq = acc.rt_chng_freq;
    out_acc.rt_flg = acc.rt_flg;
    out_acc.rep_idx = acc.rep_idx;
    out_acc.nxt_pay_dt = if let Some(dt) = acc.nxt_pay_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.prev_rep_dt = if let Some(dt) = acc.prev_rep_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.int_pay_freq = acc.int_pay_freq;
    out_acc.int_rt = acc.int_rt;
    out_acc.port_typ = acc.port_typ;
    out_acc.sec_grp = acc.sec_grp;
    out_acc.sec_type = acc.sec_type;
    out_acc.sec_issuer = acc.sec_issuer;
    out_acc.sec_guaranteed = acc.sec_guaranteed;
    out_acc.mrkt = acc.mrkt;
    out_acc.idx_label = acc.idx_label;
    out_acc.bd_categ = acc.bd_categ;
    out_acc.bd_type = acc.bd_type;
    out_acc.listed = acc.listed;
    out_acc.npa_class = acc.npa_class;
    out_acc.entity = acc.entity;
    out_acc.desk = acc.desk;
    out_acc.acc_sec_igaap = acc.acc_sec_igaap;
    out_acc.os_cv_before_amort = acc.os_cv_before_amort;
    out_acc.os_cv_after_amort = acc.os_cv_after_amort;
    out_acc.mat_dt = if let Some(dt) = acc.mat_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.int_amt = acc.int_amt;
    out_acc.flow_type = acc.flow_type;

    for cf in &cashflows {
        tot_int_amt += cf.int_amt;
        tot_prin_amt += cf.prin_amt;
    }
    out_acc.tot_int_amt = tot_int_amt;
    out_acc.tot_prin_amt = tot_prin_amt;
    out_acc.isin = acc.isin;
    out_acc.wap_igaap = acc.wap_igaap;
    out_acc.ost_bal = acc.ost_bal;
    out_acc.contract_no = acc.contract_no;
    out_acc.instr_id = acc.instr_id;
    out_acc.parent_code = acc.parent_code;
    out_acc.issuer_name = acc.issuer_name;
    out_acc.rating = acc.rating;
    out_acc.tax_status = acc.tax_status;
    out_acc.slr_nslr = acc.slr_nslr;
    out_acc.deal_ytm = acc.deal_ytm;
    out_acc.intr_app_freq = acc.intr_app_freq;
    out_acc.comp_freq = acc.comp_freq;
    out_acc.intr_prac = acc.intr_prac;
    out_acc.rate_spread = acc.rt_spread;
    out_acc.asset_class = acc.asset_class;
    out_acc.intr_typ = acc.intr_typ;
    out_acc.sec_issuance_date = if let Some(dt) = acc.sec_issuance_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.coupon = acc.coupon;
    out_acc.last_intr_date = if let Some(dt) = acc.last_intr_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.next_intr_date = if let Some(dt) = acc.next_intr_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.amort_till_date = acc.amort_till_date;

    if acc.put_dt.is_some()
        && acc.mat_dt.is_some()
        && acc.put_dt < acc.mat_dt
        && acc.put_dt >= Some(as_on_date)
    {
        out_acc.org_tenor = get_months(acc.val_dt, acc.put_dt);
        out_acc.ftp_mat_dt = out_acc.put_dt;
    } else {
        out_acc.org_tenor = get_months(acc.val_dt, acc.mat_dt);
        out_acc.ftp_mat_dt = out_acc.mat_dt;
    }

    out_acc.ftp_lst_repr_dt = if let Some(dt) = acc.ftp_lst_repr_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.ftp_nxt_repr_dt = if let Some(dt) = acc.ftp_nxt_repr_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.org_tenor = get_months(acc.val_dt, acc.cf_dt);
    out_acc.rep_tenor = get_months(acc.nxt_rep_dt, acc.lst_rep_dt);
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
