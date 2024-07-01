use macros;
use rbdate::NaiveDate;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_agg_rules_adj::agg_rules::AggRules_adj;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use stamp_ftp::amb_file_reader::AvgBalances;
use stamp_ftp::append_output::append_out;
use stamp_ftp::bm_reader::{BmKey, IntermediateBmPoints};
use stamp_ftp::calc_ftp;
use stamp_ftp::cfinput::AccFieldNames;
use stamp_ftp::read_adjustments::Adj_key;
use stamp_ftp::rule_stamper;
use stamp_ftp::CFout::AccountWithCashflows;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;

pub fn calculate_ftp(
    acc_data: &mut AccountWithCFs,
    mut cf_data: AccountWithCashflows,
    m_rules: &AggRules,
    bc_rules: &AggRules,
    adj_rules: &AggRules_adj,
    bc_file_path: String,
    inputfields: &AccFieldNames,
    log: &Logger,
    diag_log: &Logger,
    ftprunid: i64,
    from_date: &NaiveDate,
    to_date: &NaiveDate,
    out_path: &str,
    ftp_rates: &mut HashMap<String, Vec<f64>>,
    lock_adjs: &HashMap<i32, String>,
    adj_rates: &HashMap<Adj_key, f64>,
    avg_bal: &HashMap<String, AvgBalances>,
    ftp_rates_file_path: &str,
    default_method: i32,
    default_basecurve: i32,
    is_closed: bool,
    mut saved_bm_rates: &mut HashMap<BmKey, Vec<IntermediateBmPoints>>,
    mut spread_writer: &mut BufWriter<File>,
    rate_precision: i8,
    bal_precision: i8,
) -> (String, String) {
    let method = rule_stamper::get_method(&acc_data, &m_rules, default_method, diag_log);

    let basecurve = rule_stamper::get_bc(&acc_data, &bc_rules, default_basecurve, diag_log);

    let lst_adjustments = rule_stamper::get_adj(&acc_data, &adj_rules, diag_log);

    let (cf_data1, mut one_acc_op, cf_out) = calc_ftp::calc_ftp(
        acc_data,
        cf_data,
        inputfields,
        method,
        basecurve,
        lst_adjustments,
        bc_file_path,
        log,
        ftprunid,
        from_date,
        to_date,
        out_path,
        ftp_rates,
        lock_adjs,
        adj_rates,
        avg_bal,
        ftp_rates_file_path,
        &mut saved_bm_rates,
        &mut spread_writer,
        rate_precision,
        bal_precision,
    );
    cf_data = cf_data1;
    cf_data.Method = method;
    cf_data.BaseCurve = basecurve;

    let log_str_method = format!(
        "Method Assigned for Account {} is : {}",
        &cf_data.deal_no, method
    );
    log_debug!(diag_log, "{}", log_str_method);

    let log_str_bc = format!(
        "Base curve Assigned for Account {} is : {}",
        &cf_data.deal_no, basecurve
    );
    log_debug!(diag_log, "{}", log_str_bc);

    one_acc_op.entity = cf_data.entity;
    one_acc_op.deal_no = cf_data.deal_no;
    one_acc_op.contract_no = cf_data.contract_no;
    one_acc_op.isin = cf_data.isin;
    one_acc_op.instr_id = cf_data.instr_id;
    one_acc_op.parent_code = cf_data.parent_code;
    one_acc_op.short_name = cf_data.short_name;
    one_acc_op.issuer_name = cf_data.issuer_name;
    one_acc_op.intr_typ = cf_data.intr_typ;
    one_acc_op.sec_issuance_date = cf_data.sec_issuance_date;
    one_acc_op.coupon = cf_data.coupon;
    one_acc_op.last_intr_dt = cf_data.last_intr_dt;
    one_acc_op.next_intr_dt = cf_data.next_intr_dt;
    one_acc_op.nxt_repricing_dt = cf_data.nxt_rep_dt;
    one_acc_op.rating = cf_data.rating;
    one_acc_op.mat_dt = cf_data.mat_dt;
    one_acc_op.call_dt = cf_data.call_dt;
    one_acc_op.put_dt = cf_data.put_dt;
    one_acc_op.tax_status = cf_data.tax_status;
    one_acc_op.product = cf_data.prod_cd.clone();
    one_acc_op.prod_desc = cf_data.prod_desc;
    one_acc_op.slr_nslr = cf_data.slr_nslr;
    one_acc_op.deal_dt = cf_data.deal_dt;
    one_acc_op.portfolio = cf_data.portfolio;
    one_acc_op.desk = cf_data.desk;
    one_acc_op.acc_sec_igaap = cf_data.acc_sec_igaap;
    one_acc_op.port_typ = cf_data.port_typ;
    one_acc_op.deal_ytm = cf_data.deal_ytm;
    one_acc_op.deal_rt = cf_data.deal_rt;
    one_acc_op.currency = cf_data.inst;
    one_acc_op.os_face_val = cf_data.os_face_val;
    one_acc_op.os_cv_before_amort = cf_data.os_cv_before_amort;
    one_acc_op.os_cv_after_amort = cf_data.os_cv_after_amort;
    one_acc_op.value_date = cf_data.val_dt;
    one_acc_op.int_rate = cf_data.int_rt;
    one_acc_op.intr_app_freq = cf_data.intr_app_freq;
    one_acc_op.comp_freq = cf_data.comp_freq;
    one_acc_op.intr_prac = cf_data.intr_prac;
    one_acc_op.rate_spread = cf_data.rate_spread;
    one_acc_op.asset_class = cf_data.asset_class;
    one_acc_op.amort_till_dt = cf_data.amort_till_dt;
    one_acc_op.lst_rep_dt = cf_data.lst_rep_dt;
    one_acc_op.nxt_rep_dt = cf_data.nxt_rep_dt;
    one_acc_op.prod_type = cf_data.prod_cd;
    one_acc_op.alm_line = cf_data.alm_line;
    one_acc_op.rate_flag = cf_data.rt_flg;
    one_acc_op.gl = cf_data.gl.to_string();
    one_acc_op.source_file_name = String::from("MurexSecurityComposition");

    if is_closed {
        one_acc_op.outstanding_bal = 0.0;
    } else {
        one_acc_op.outstanding_bal = cf_data.os_cv_after_amort;
    }
    let mut final_out = append_out(&one_acc_op, bal_precision, to_date, from_date);

    final_out.push('\n');

    (final_out, cf_out)
}
