use crate::stamp_ftp::append_output::additional_append_out;

use super::AverageBalance;
use macros;
use rbdate::NaiveDate;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_agg_rules_adj::agg_rules::AggRules_adj;
use sdb_dyn_proto_rdr::reader::account_with_cfs::{get_field_value, AccountWithCFs};
use sdb_dyn_proto_rdr::reader::Reader;
use slog::Logger;
use stamp_ftp::append_output::append_out;
use stamp_ftp::bm_reader::{BmKey, IntermediateBmPoints};
use stamp_ftp::calc_ftp;
use stamp_ftp::cfinput::AccFieldNames;
use stamp_ftp::one_acc_view::One_acc_view;
use stamp_ftp::read_adjustments::Adj_key;
use stamp_ftp::rule_stamper;
use stamp_ftp::CFout::AccountWithCashflows;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufWriter;
use std::time::Instant;

pub fn calculate_ftp(
    input_reader: &Reader,
    acc_data: &mut AccountWithCFs,
    mut cf_data: AccountWithCashflows,
    m_rules: &AggRules,
    bc_rules: &AggRules,
    fix_adj_rules: &AggRules_adj,
    var_adj_rules: &AggRules_adj,
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
    avg_bal: &HashMap<String, AverageBalance>,
    ftp_rates_file_path: &str,
    default_method: i32,
    default_basecurve: i32,
    fix_adj_count: i32,
    var_adj_count: i32,
    mut saved_bm_rates: &mut HashMap<BmKey, Vec<IntermediateBmPoints>>,
    is_cf_req: bool,
    mut spread_writer: &mut BufWriter<File>,
    rate_precision: i8,
    bal_precision: i8,
    config_map: &HashMap<String, HashSet<String>>,
    asondate: &NaiveDate,
    spread_map: &HashMap<String,String>
) -> (String, One_acc_view, String, String) {
    let get_data_timer = Instant::now();
    let method = rule_stamper::get_method(&acc_data, &m_rules, default_method, diag_log);
    let basecurve = rule_stamper::get_bc(&acc_data, &bc_rules, default_basecurve, diag_log);
    let fix_lst_adjustments =
        rule_stamper::get_adj(&acc_data, &fix_adj_rules, fix_adj_count, diag_log);
    let var_lst_adjustments =
        rule_stamper::get_adj(&acc_data, &var_adj_rules, var_adj_count, diag_log);
    let get_data_time = get_data_timer.elapsed();

    let calc_ftp_timer = Instant::now();
    let (cf_data1, mut one_acc_op, cf_out, adj_string) = calc_ftp::calc_ftp(
        acc_data,
        cf_data,
        inputfields,
        method,
        basecurve,
        fix_lst_adjustments,
        var_lst_adjustments,
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
        avg_bal,
        ftp_rates_file_path,
        fix_adj_count,
        var_adj_count,
        &mut saved_bm_rates,
        is_cf_req,
        &mut spread_writer,
        rate_precision,
        bal_precision,
    );
    let calc_ftp_time = calc_ftp_timer.elapsed();

    let other_timer = Instant::now();
    cf_data = cf_data1.clone();
    cf_data.method = method;
    cf_data.base_curve = basecurve;

    let log_str_method = format!(
        "Method Assigned for Account {} is : {}",
        &cf_data.account_number, method
    );
    log_debug!(diag_log, "{}", log_str_method);

    let log_str_bc = format!(
        "Base curve Assigned for Account {} is : {}",
        &cf_data.account_number, basecurve
    );
    log_debug!(diag_log, "{}", log_str_bc);
    one_acc_op.account_number = cf_data.account_number;
    one_acc_op.ccy = cf_data.currency;
    one_acc_op.value_date = cf_data.value_date;
    one_acc_op.maturity_date = cf_data.maturity_date;
    one_acc_op.prod_type = cf_data.product_code;
    one_acc_op.cust_name = cf_data.cust_name;
    one_acc_op.alm_line = cf_data.alm_line;
    one_acc_op.rate_flag = cf_data.rate_flag;
    one_acc_op.outstanding_bal = cf_data.outstanding_bal;
    one_acc_op.org_tenor = cf_data.org_tenor;
    one_acc_op.rep_tenor = cf_data.rep_tenor;
    one_acc_op.lst_rep_dt = cf_data.lst_rep_date;
    one_acc_op.nxt_rep_dt = cf_data.nxt_rep_date;
    one_acc_op.int_rate = cf_data.int_rate;
    one_acc_op.gl = cf_data.gl;
    one_acc_op.source_file_name = String::from("FinnoneLoans");
    one_acc_op.psl_code = match acc_data.get_string_for_key(&inputfields.weaker) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();
    one_acc_op.mis2 = match acc_data.get_string_for_key(&inputfields.psl) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();
    one_acc_op.mis1 = match acc_data.get_string_for_key(&inputfields.scheme_id) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    one_acc_op.branch = cf_data.branch;
    one_acc_op.orig_bal = cf_data.orig_bal;
    one_acc_op.npa = match get_field_value(&acc_data, &input_reader, inputfields.npa.to_string()) {
        Ok(value) => value.to_string(),
        Err(_error) => "".to_string(),
    };
    one_acc_op.input_benchmark = match get_field_value(&acc_data, &input_reader, inputfields.repricing_index.to_string()) {
        Ok(value) => value.to_string(),
        Err(_error) => "".to_string(),
    };
    let mut final_out = append_out(&one_acc_op, &acc_data, inputfields, bal_precision, to_date, from_date);
    let mut add_final_out = additional_append_out(
        &one_acc_op,
        bal_precision,
        to_date,
        from_date,
        adj_string,
        &cf_data1,
        &acc_data,
        inputfields,
        config_map,
        asondate,
        spread_map
    );

    final_out.push('\n');
    add_final_out.push('\n');

    (final_out, one_acc_op, cf_out, add_final_out)
}
