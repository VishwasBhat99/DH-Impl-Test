use account_reader_writer::account_reader::input_account::InputAccount;
use account_reader_writer::account_without_cashflows::Cashflow;
use account_reader_writer::account_without_cashflows::OutputAccount;
use chrono::Datelike;
use macros;
use rbdate::get_days_from_month;
use rbdate::get_month_end_date;
use rbdate::incr_dt_by_mon_presrv_eom_checked;
use rbdate::timestamp;
use rbdate::NaiveDate;
use sdb_day_convention::conventions::Conventions;
use sdb_day_convention::conventions::Days;
use sdb_day_convention::days_with_convn;
use slog::Logger;
use statics::*;

pub fn create_account_without_cashflows(
    account: InputAccount,
    log: &Logger,
    as_on_date: &NaiveDate,
    conventions: Conventions,
) -> OutputAccount {
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
    out_acc.dt_open_acc = account
        .dt_open_acc
        .map(|dt| timestamp(dt))
        .unwrap_or_else(|| DEFAULT_INT);
    out_acc.cod_int_accr_bas = account.cod_int_accr_bas;
    out_acc.freq_int_accr = account.freq_int_accr;
    out_acc.dt_acc_close = account
        .dt_acc_close
        .map(|dt| timestamp(dt))
        .unwrap_or_else(|| DEFAULT_INT);
    out_acc.cod_collat_id = account.cod_collat_id;
    out_acc.collat_desc = account.collat_desc;
    out_acc.as_of_dt = account
        .as_of_dt
        .map(|dt| timestamp(dt))
        .unwrap_or_else(|| DEFAULT_INT);
    out_acc.cost_cntr = account.cost_cntr;
    out_acc.gl_acc_no = account.gl_acc_no;
    out_acc.rt_flg = account.rt_flg;
    out_acc.inst = account.inst;
    out_acc.crnt_book_bal = account.crnt_book_bal.unwrap_or_else(|| {
        log_error!(
            log,
            "`current_book_balance` not well formatted for account : `{}`.",
            out_acc.cod_acc_no
        );
        DEFAULT_FLOAT
    });
    out_acc.acrl_basis = account.acrl_basis;
    out_acc.int_rt = account.int_rt.unwrap_or_else(|| {
        log_error!(
            log,
            "`interest_rate` is not well formatted for account : `{}`.",
            out_acc.cod_acc_no
        );
        DEFAULT_FLOAT
    });
    out_acc.div = account.div;
    out_acc.alm_line = account.alm_line;
    out_acc.ia_llg = account.ia_llg;
    out_acc.balm_llg = account.balm_llg;
    out_acc.mis1 = account.mis1;
    out_acc.npa_flg = account.npa_flg;
    out_acc.benchmark = account.benchmark;
    out_acc.rep_freq = account.rep_freq;
    out_acc.nxt_rep_dt = account
        .nxt_rep_dt
        .map(|dt| timestamp(dt))
        .unwrap_or_else(|| DEFAULT_INT);
    out_acc.lst_rep_dt = account
        .lst_rep_dt
        .map(|dt| timestamp(dt))
        .unwrap_or_else(|| DEFAULT_INT);
    out_acc.cust_typ = account.cust_typ;
    out_acc.country = account.country;
    out_acc.bm_id_lookup = account.bm_id_lookup;
    out_acc.alm_concat = account.alm_concat;
    out_acc.mis2_code = account.mis2_code;
    out_acc.der_int_rate = account.der_int_rate;
    out_acc.bnchmrk_rate = account.bnchmrk_rate;
    out_acc.spread = account.spread;
    out_acc.fully_floating_flg = account.fully_floating_flg;
    let amt = out_acc.crnt_book_bal.abs();
    if amt < 0.0 {
        log_warn!(
            log,
            "Amount is negative after taking absolute value for Account: {}",
            out_acc.cod_acc_no
        )
    }
    let first_cf_date = get_month_end_for_ason(*as_on_date);

    let end_cf_date = incr_dt_by_mon_presrv_eom_checked(*as_on_date, 12).unwrap_or_else(|| {
        panic!(
            "Failed to calculate the last cashflow date for account: {}",
            out_acc.cod_acc_no
        )
    });

    let mut cashflows = Vec::new();
    let days = days_with_convn(*as_on_date, first_cf_date, &conventions)
        .expect("Failed to calculate days with convention");
    let i_a = interest_amount(
        account.crnt_book_bal.unwrap_or(0.0).abs(),
        account.int_rt.unwrap_or(0.0),
        days,
    );
    let cf = new_cashflow(i_a, 0.0, &first_cf_date);
    cashflows.push(cf);
    let mut prev_cf_date = first_cf_date;
    while prev_cf_date <= end_cf_date {
        let mut cashflow_date =
            incr_dt_by_mon_presrv_eom_checked(prev_cf_date, 1).unwrap_or_else(|| {
                panic!(
                    "Failed to calculate the next cashflow date for account: {}",
                    out_acc.cod_acc_no
                )
            });
        cashflow_date = get_month_end_date(cashflow_date);
        if cashflow_date <= end_cf_date {
            let days = days_with_convn(prev_cf_date, cashflow_date, &conventions)
                .expect("Failed to calculate days with convention");
            let i_a = interest_amount(
                account.crnt_book_bal.unwrap_or(0.0).abs(),
                account.int_rt.unwrap_or(0.0),
                days,
            );
            let cf = new_cashflow(i_a, 0.0, &cashflow_date);
            cashflows.push(cf);

            prev_cf_date = cashflow_date;
        } else {
            break;
        }
    }
    if prev_cf_date < end_cf_date {
        let days = days_with_convn(prev_cf_date, end_cf_date, &conventions)
            .expect("Failed to calculate days with convention");
        let i_a = interest_amount(
            account.crnt_book_bal.unwrap_or(0.0).abs(),
            account.int_rt.unwrap_or(0.0),
            days,
        );
        let cf = new_cashflow(i_a, 0.0, &end_cf_date);
        cashflows.push(cf);
    }
    cashflows
        .last_mut()
        .expect("Cashflows matured without generating any cashflows.")
        .principal_amount = account.crnt_book_bal.unwrap_or(0.0).abs();
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    log_debug!(
        log,
        "account: `{}`, current book balance: `{}`, interest rate: `{}`.",
        out_acc.cod_acc_no,
        out_acc.crnt_book_bal,
        out_acc.int_rt
    );
    let mut total_principal = 0.0000;
    for cashflow in out_acc.cashflows.to_owned() {
        total_principal += cashflow.principal_amount;
    }
    out_acc.b1 = (total_principal * account.b1) / 100.0000;
    out_acc.b2 = (total_principal * account.b2) / 100.0000;
    out_acc.b3 = (total_principal * account.b3) / 100.0000;
    out_acc.flg_frequency = account.flg_frequency;
    out_acc.dat_start_frq = account
        .dat_start_frq
        .map(|dt| timestamp(dt))
        .unwrap_or(DEFAULT_INT);
    out_acc.dat_frq_last_reset = account
        .dat_frq_last_reset
        .map(|dt| timestamp(dt))
        .unwrap_or(DEFAULT_INT);
    out_acc.dat_frq_next_reset = account
        .dat_frq_next_reset
        .map(|dt| timestamp(dt))
        .unwrap_or(DEFAULT_INT);
    out_acc.rat_var_penalty = account.rat_var_penalty;
    out_acc.sma_flag = account.sma_flag;
    out_acc
}

fn get_month_end_for_ason(date: NaiveDate) -> NaiveDate {
    let days_in_month = get_days_from_month(date);

    if date.day() == days_in_month as u32 {
        // Given date is the end of the month, return the next month end
        incr_dt_by_mon_presrv_eom_checked(date, 1)
            .unwrap_or_else(|| panic!("Failed to calculate the next month end for date: {}", date))
    } else {
        // Given date is not the end of the month, return the current month end
        get_month_end_date(date)
    }
}

fn interest_amount(o_a: f64, i_r: f64, days: Days) -> f64 {
    let num_days = days.days_btw_dts;
    let days_in_yr = days.day_in_yr as f64;
    (o_a * i_r * num_days as f64) / (days_in_yr * 100.0)
}

fn new_cashflow(i_a: f64, p_a: f64, d: &NaiveDate) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = rbdate::timestamp(*d);
    cf
}
