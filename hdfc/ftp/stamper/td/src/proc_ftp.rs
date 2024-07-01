use macros;
use rbdate::NaiveDate;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_agg_rules_adj::agg_rules::AggRules_adj;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use stamp_ftp::bm_reader::{BmKey, IntermediateBmPoints};
// use stamp_ftp::calc_ftp;
use stamp_ftp::aggr_key::Customer;
use stamp_ftp::calc_ftp::{assign_rate, margin_method};
use stamp_ftp::cfinput;
use stamp_ftp::cfinput::AccFieldNames;
use stamp_ftp::one_acc_view::One_acc_view;
use stamp_ftp::read_adjustments::Adj_key;
use stamp_ftp::rule_stamper;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::time::Instant;

pub fn calculate_ftp(
    acc_data: &mut AccountWithCFs,
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
    avg_bal: &HashMap<String, f64>,
    ftp_rates_file_path: &str,
    default_method: i32,
    default_basecurve: i32,
    fix_adj_count: i32,
    var_adj_count: i32,
    mut saved_bm_rates: &mut HashMap<BmKey, Vec<IntermediateBmPoints>>,
    mut spread_writer: &mut BufWriter<File>,
    rate_precision: i8,
    bal_precision: i8,
    input_field_names: &cfinput::AccFieldNames,
    aggr_bal: &HashMap<Customer, f64>,
) -> (String, String) {
    let get_data_timer = Instant::now();
    let method = rule_stamper::get_method(&acc_data, &m_rules, default_method, diag_log);
    let basecurve = rule_stamper::get_bc(&acc_data, &bc_rules, default_basecurve, diag_log);
    let fix_lst_adjustments =
        rule_stamper::get_adj(&acc_data, &fix_adj_rules, fix_adj_count, diag_log);
    let var_lst_adjustments =
        rule_stamper::get_adj(&acc_data, &var_adj_rules, var_adj_count, diag_log);
    let get_data_time = get_data_timer.elapsed();
    let one_acc_op1 = One_acc_view::new();
    let calc_ftp_timer = Instant::now();
    let mut final_out_str = String::new();
    let mut final_out_type = String::new();
    match method {
        1021 => {
            let (out_str, out_type) = assign_rate::assign_rate_1(
                acc_data,
                m_rules,
                bc_rules,
                fix_adj_rules,
                var_adj_rules,
                bc_file_path,
                inputfields,
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
                default_method,
                default_basecurve,
                fix_adj_count,
                var_adj_count,
                saved_bm_rates,
                spread_writer,
                rate_precision,
                bal_precision,
                basecurve,
                input_field_names,
                fix_lst_adjustments,
                var_lst_adjustments,
                aggr_bal,
            );
            final_out_str = out_str;
            final_out_type = out_type;
        }
        1011 => {
            let (out_str, out_type) = margin_method::margin_method(
                method,
                acc_data,
                m_rules,
                bc_rules,
                fix_adj_rules,
                var_adj_rules,
                bc_file_path,
                inputfields,
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
                default_method,
                default_basecurve,
                fix_adj_count,
                var_adj_count,
                saved_bm_rates,
                spread_writer,
                rate_precision,
                bal_precision,
                basecurve,
                input_field_names,
                fix_lst_adjustments,
                var_lst_adjustments,
                aggr_bal,
            );
            final_out_str = out_str;
            final_out_type = out_type;
        }
        1042 => {
            let (out_str, out_type) = margin_method::margin_method_2(
                method,
                acc_data,
                m_rules,
                bc_rules,
                fix_adj_rules,
                var_adj_rules,
                bc_file_path,
                inputfields,
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
                default_method,
                default_basecurve,
                fix_adj_count,
                var_adj_count,
                saved_bm_rates,
                spread_writer,
                rate_precision,
                bal_precision,
                basecurve,
                input_field_names,
                fix_lst_adjustments,
                var_lst_adjustments,
                aggr_bal,
            );
            final_out_str = out_str;
            final_out_type = out_type;
        }
        _ => {}
    }

    final_out_str.push('\n');

    (final_out_str, final_out_type)
}
