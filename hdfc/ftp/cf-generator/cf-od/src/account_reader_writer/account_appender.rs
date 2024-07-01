use account_reader_writer::account_reader::input_account::InputAccount;
use account_reader_writer::account_without_cashflows::Cashflow;
use account_reader_writer::account_without_cashflows::OutputAccount;
use macros;
use rbdate::timestamp;
use slog::Logger;
use statics::*;

pub fn create_account_without_cashflows(account: InputAccount, log: &Logger) -> OutputAccount {
    let mut out_acc = OutputAccount::new();

    out_acc.cod_acc_no = account.cod_acc_no;
    out_acc.cod_cc_brn = account.cod_cc_brn;
    out_acc.cod_prod = account.cod_prod;
    out_acc.bal_book = account.bal_book;
    out_acc.bal_book_lcy = account.bal_book_lcy;
    out_acc.amt_od_lmt = account.amt_od_lmt;
    out_acc.amt_od_lmt_lcy = account.amt_od_lmt_lcy;
    out_acc.cod_cust = account.cod_cust;
    out_acc.cod_acc_title = account.cod_acc_title;
    out_acc.dt_open_acc = if let Some(dt) = account.dt_open_acc {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.cod_int_accr_bas = account.cod_int_accr_bas;
    out_acc.freq_int_accr = account.freq_int_accr;
    out_acc.dt_acc_close = if let Some(dt) = account.dt_acc_close {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.cod_collat_id = account.cod_collat_id;
    out_acc.collat_desc = account.collat_desc;
    out_acc.as_of_dt = timestamp(account.as_of_dt);
    out_acc.cost_cntr = account.cost_cntr;
    out_acc.gl_acc_no = account.gl_acc_no;
    out_acc.rt_flg = account.rt_flg;
    out_acc.inst = account.inst;
    out_acc.crnt_book_bal = if let Some(bal) = account.crnt_book_bal {
        bal
    } else {
        log_error!(
            log,
            "`current_book_balance` not well formatted for account : `{}`.",
            out_acc.cod_acc_no
        );
        DEFAULT_FLOAT
    };
    out_acc.acrl_basis = account.acrl_basis;
    out_acc.int_rt = if let Some(rt) = account.int_rt {
        rt
    } else {
        log_error!(
            log,
            "`interest_rate` is not well formatted for account : `{}`.",
            out_acc.cod_acc_no
        );
        DEFAULT_FLOAT
    };
    out_acc.div = account.div;
    out_acc.alm_line = account.alm_line;
    out_acc.ia_llg = account.ia_llg;
    out_acc.balm_llg = account.balm_llg;
    out_acc.mis1 = account.mis1;
    out_acc.npa_flg = account.npa_flg;
    out_acc.benchmark = account.benchmark;
    out_acc.rep_freq = account.rep_freq;
    out_acc.nxt_rep_dt = if let Some(dt) = account.nxt_rep_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.lst_rep_dt = if let Some(dt) = account.lst_rep_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.cust_typ = account.cust_typ;
    out_acc.country = account.country;
    out_acc.bm_id_lookup = account.bm_id_lookup;
    out_acc.mis2 = account.mis2;
    out_acc.avg_bal = account.avg_bal;
    out_acc.is_acc_weaker = account.is_acc_weaker;
    out_acc.ews_weaker_value = account.ews_weaker_value;
    out_acc.alm_concat = account.alm_concat;
    out_acc.two_point_concat = account.two_point_concat;
    out_acc.weaker_desc = account.weaker_desc;
    out_acc.bdp_division = account.bdp_division;
    out_acc.bdp_coa = account.bdp_coa;
    let mut cfs = Cashflow::new();
    let amt = out_acc.crnt_book_bal.abs();
    if amt < 0.0 {
        log_warn!(
            log,
            "Amount is negative after taking absolute value for Account: {}",
            out_acc.cod_acc_no
        )
    }
    cfs.set_principal_amount(amt);
    cfs.set_date(out_acc.nxt_rep_dt);
    out_acc.cashflows = protobuf::RepeatedField::from_vec(vec![cfs]);
    log_debug!(
        log,
        "account: `{}`, current book balance: `{}`, interest rate: `{}`.",
        out_acc.cod_acc_no,
        out_acc.crnt_book_bal,
        out_acc.int_rt
    );

    out_acc
}
