use macros;
use rbdate::NaiveDate;
use rbdate::NaiveDateTime;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_agg_rules_adj::agg_rules::AggRules_adj;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use stamp_ftp::amb_file_reader::AmbVal;
use stamp_ftp::append_output::append_out;
use stamp_ftp::bm_reader::{BmKey, IntermediateBmPoints};
use stamp_ftp::calc_ftp;
use stamp_ftp::cfinput::AccFieldNames;
use stamp_ftp::one_acc_view::One_acc_view;
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
    avg_bal: &HashMap<String, AmbVal>,
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
        &cf_data.acc_no, method
    );
    log_debug!(diag_log, "{}", log_str_method);

    let log_str_bc = format!(
        "Base curve Assigned for Account {} is : {}",
        &cf_data.acc_no, basecurve
    );
    log_debug!(diag_log, "{}", log_str_bc);

    one_acc_op.account_number = cf_data.acc_no;
    one_acc_op.ccy = cf_data.ccy;
    one_acc_op.value_date = cf_data.st_dt;
    one_acc_op.maturity_date = cf_data.mat_dt;
    one_acc_op.prod_type = cf_data.div;
    one_acc_op.alm_line = cf_data.alm_line;
    one_acc_op.cust_name = cf_data.counter_party;
    one_acc_op.gl = cf_data.gl_no.to_string();

    one_acc_op.mis1 = match acc_data.get_i64_for_key(&inputfields.compmis1) {
        Ok(result) => result,
        Err(e) => 0,
    }
    .to_string();
    one_acc_op.mis2 = match acc_data.get_i64_for_key(&inputfields.compmis2) {
        Ok(result) => result,
        Err(e) => 0,
    }
    .to_string();

    if is_closed {
        one_acc_op.outstanding_bal = 0.0;
    } else {
        one_acc_op.outstanding_bal = match acc_data.get_f64_for_key(&inputfields.amt) {
            Ok(result) => result,
            Err(e) => 0.0,
        };
    }

    one_acc_op.source_file_name = String::from("BullionLoans");

    let mut final_out = append_out(&one_acc_op, bal_precision, to_date, from_date);
    final_out.push('\n');

    (final_out, cf_out)
}
