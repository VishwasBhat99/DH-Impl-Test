use math::round::half_away_from_zero;
use rbdate::{date_from_timestamp, num_days_start_to_end, NaiveDate, NaiveDateTime};
use stamp_ftp::one_acc_view::One_acc_view;

pub fn append_out(
    one_acc_op: &One_acc_view,
    bal_precision: i8,
    to_date: &NaiveDate,
    from_date: &NaiveDate,
) -> String {
    let mut op_line: String = String::new();
    let days_in_month = rbdate::num_days_start_to_end(*from_date, *to_date) + 1;
    let days_in_year = rbdate::num_days_start_to_end(
        *to_date,
        rbdate::increment_date_by_months(*to_date, (12) as u16),
    );
    let yld_to_call: f64 = (one_acc_op.accr_int / one_acc_op.average_balance)
        * 100.0 as f64
        * (days_in_year as f64 / days_in_month as f64);

    let val_date = date_from_timestamp(one_acc_op.value_date);
    let maturity_date = date_from_timestamp(one_acc_op.mat_dt);
    let org_tenor = num_days_start_to_end(val_date, maturity_date);
    let next_repr_date = date_from_timestamp(one_acc_op.nxt_rep_dt);
    let last_repr_date = date_from_timestamp(one_acc_op.lst_rep_dt);
    let rep_tenor = num_days_start_to_end(last_repr_date, next_repr_date);
    op_line = format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|\
    {}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        one_acc_op.entity,
        one_acc_op.deal_no,
        one_acc_op.contract_no,
        one_acc_op.isin,
        one_acc_op.instr_id,
        one_acc_op.parent_code,
        one_acc_op.short_name,
        one_acc_op.issuer_name,
        one_acc_op.intr_typ,
        get_date_str(one_acc_op.sec_issuance_date),
        one_acc_op.coupon,
        get_date_str(one_acc_op.last_intr_dt),
        get_date_str(one_acc_op.next_intr_dt),
        get_date_str(one_acc_op.nxt_repricing_dt),
        one_acc_op.rating,
        get_date_str(one_acc_op.mat_dt),
        get_date_str(one_acc_op.call_dt),
        get_date_str(one_acc_op.put_dt),
        one_acc_op.tax_status,
        one_acc_op.product,
        one_acc_op.prod_desc,
        one_acc_op.slr_nslr,
        get_date_str(one_acc_op.deal_dt),
        one_acc_op.portfolio,
        one_acc_op.desk,
        one_acc_op.acc_sec_igaap,
        one_acc_op.port_typ,
        one_acc_op.deal_ytm,
        one_acc_op.deal_rt,
        one_acc_op.currency,
        one_acc_op.os_face_val,
        one_acc_op.accr_int,
        one_acc_op.os_cv_before_amort,
        one_acc_op.amort_till_dt,
        one_acc_op.os_cv_after_amort,
        one_acc_op.intr_app_freq,
        one_acc_op.comp_freq,
        one_acc_op.intr_prac,
        one_acc_op.rate_spread,
        one_acc_op.asset_class,
        one_acc_op.average_balance,
        half_away_from_zero(yld_to_call, bal_precision),
        one_acc_op.int_rate,
        one_acc_op.base_rate,
        one_acc_op.final_ftp_rate,
        get_date_str(one_acc_op.value_date),
        get_date_str(one_acc_op.nxt_rep_dt),
        get_date_str(one_acc_op.lst_rep_dt),
        one_acc_op.mis1,
        one_acc_op.mis2,
        one_acc_op.psl_code,
        one_acc_op.prod_type,
        one_acc_op.rate_flag,
        one_acc_op.branch,
        one_acc_op.source_file_name,
        one_acc_op.gl,
        one_acc_op.cust_id,
        one_acc_op.final_ftp_amt,
        one_acc_op.alm_line,
        one_acc_op.trade_dt,
        one_acc_op.orig_bal,
        one_acc_op.outstanding_bal,
        one_acc_op.adj1,
        one_acc_op.adj2,
        one_acc_op.adj3,
        one_acc_op.adj4,
        one_acc_op.adj5,
        one_acc_op.adj6,
        one_acc_op.liquidity_premia,
        one_acc_op.psl_charge,
        one_acc_op.input_benchmark,
        one_acc_op.pdo,
        one_acc_op.npl,
        one_acc_op.specific_provision,
        one_acc_op.method,
        one_acc_op.rate_curve,
        org_tenor,
        rep_tenor,
        one_acc_op.fx_spread,
        one_acc_op.var_spread,
        one_acc_op.first_ftp,
        get_date_str(one_acc_op.bc_as_on_rule),
        get_date_str(one_acc_op.tenor_start_date_rule),
        get_date_str(one_acc_op.tenor_end_date_rule),
        get_date_str(one_acc_op.bc_as_on_applied),
        get_date_str(one_acc_op.tenor_start_date_applied),
        get_date_str(one_acc_op.tenor_end_date_applied),
    );

    op_line
}

pub fn get_date_str(date: i64) -> String {
    let start_date = NaiveDateTime::from_timestamp(date, 0)
        .date()
        .format("%d-%m-%Y");

    start_date.to_string()
}
