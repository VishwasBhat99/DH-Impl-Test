use macros;
use rbdate::NaiveDate;
use rbdate::NaiveDateTime;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_agg_rules_adj::agg_rules::AggRules_adj;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use stamp_ftp::calc_ftp;
use stamp_ftp::cfinput::AccFieldNames;
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
) -> (AccountWithCashflows, String, String) {
    let method = rule_stamper::get_method(&acc_data, &m_rules, default_method, diag_log);

    let basecurve = rule_stamper::get_bc(&acc_data, &bc_rules, default_basecurve, diag_log);

    let lst_adjustments = rule_stamper::get_adj(&acc_data, &adj_rules, diag_log);

    let (cf_data1, output, cf_out) = calc_ftp::calc_ftp(
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
    );
    cf_data = cf_data1;
    cf_data.Method = method;
    cf_data.BaseCurve = basecurve;

    let log_str_method = format!(
        "Method Assigned for Account {} is : {}",
        &cf_data.deal_id, method
    );
    log_debug!(diag_log, "{}", log_str_method);

    let log_str_bc = format!(
        "Base curve Assigned for Account {} is : {}",
        &cf_data.deal_id, basecurve
    );
    log_debug!(diag_log, "{}", log_str_bc);

    let start_date = NaiveDateTime::from_timestamp(cf_data.val_date, 0)
        .date()
        .format("%d-%m-%Y");
    let mat_date = NaiveDateTime::from_timestamp(cf_data.maturity_dt, 0)
        .date()
        .format("%d-%m-%Y");

    let pre_out = format!(
        "{}|{}|{}|{}|{}|{}|{}|",
        ftprunid,
        from_date.format("%d-%m-%Y"),
        to_date.format("%d-%m-%Y"),
        cf_data.deal_id,
        cf_data.currency,
        start_date,
        mat_date
    );

    let post_out = format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        cf_data.flow_type,
        cf_data.glcode,
        "",
        //empty_cust_id
        "",
        //empty branch
        "",
        //RM and Department
        "",
        "",
        cf_data.dealer_name,
        cf_data.alm_line,
        //RL2, RL3 . RL1 filled with alm line here
        cf_data.division,
        ""
    );

    let mut final_out = format!("{}{}{}", pre_out, output, post_out);

    final_out.push('\n');

    (cf_data, final_out, cf_out)
}
