use macros;
use rbdate::*;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_agg_rules_adj::agg_rules::AggRules_adj;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use stamp_ftp::append_output::append_out;
use stamp_ftp::bm_reader::{BmKey, IntermediateBmPoints};
use stamp_ftp::calc_ftp;
use stamp_ftp::cfinput::AccFieldNames;
use stamp_ftp::one_acc_view::One_acc_view;
use stamp_ftp::read_adjustments::Adj_key;
use stamp_ftp::rule_stamper;
use stamp_ftp::CFout::AccountWithCashflows;
use std::collections::HashMap;

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
    ftp_rates_file_path: &str,
    default_method: i32,
    default_basecurve: i32,
    mut saved_bm_rates: &mut HashMap<BmKey, Vec<IntermediateBmPoints>>,
    rate_precision: i8,
    bal_precision: i8,
) -> (String) {
    let method = rule_stamper::get_method(&acc_data, &m_rules, default_method, diag_log);

    let basecurve = rule_stamper::get_bc(&acc_data, &bc_rules, default_basecurve, diag_log);

    let lst_adjustments = rule_stamper::get_adj(&acc_data, &adj_rules, diag_log);

    let (cf_data1, mut one_acc_op) = calc_ftp::calc_ftp(
        acc_data,
        cf_data,
        inputfields,
        method,
        basecurve,
        lst_adjustments,
        bc_file_path,
        log,
        diag_log,
        ftprunid,
        from_date,
        to_date,
        out_path,
        ftp_rates,
        lock_adjs,
        adj_rates,
        ftp_rates_file_path,
        &mut saved_bm_rates,
        rate_precision,
        bal_precision,
    );
    cf_data = cf_data1;
    cf_data.Method = method;
    cf_data.BaseCurve = basecurve;

    let log_str_method = format!(
        "Method Assigned for Account {} is : {}",
        &cf_data.account_no, method
    );
    log_debug!(diag_log, "{}", log_str_method);

    let log_str_bc = format!(
        "Base curve Assigned for Account {} is : {}",
        &cf_data.account_no, basecurve
    );
    log_debug!(diag_log, "{}", log_str_bc);

    one_acc_op.account_number = cf_data.account_no;
    one_acc_op.cust_name = cf_data.concat;
    one_acc_op.ccy = cf_data.seg_8;
    one_acc_op.alm_line = cf_data.alm_line;
    one_acc_op.rate_flag = cf_data.cf_type;
    one_acc_op.mis1 = cf_data.seg_3;
    one_acc_op.source_file_name = String::from("CASA");
    let as_on_date = timestamp(*to_date);
    one_acc_op.value_date = as_on_date;
    one_acc_op.maturity_date = as_on_date;
    one_acc_op.lst_rep_dt = as_on_date;
    one_acc_op.nxt_rep_dt = as_on_date;
    one_acc_op.outstanding_bal = cf_data.bal_total;
    one_acc_op.gl = cf_data.seg_1;

    let mut final_out = append_out(&one_acc_op);

    final_out.push('\n');

    (final_out)
}
