use rbdate::NaiveDate;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_agg_rules_adj::agg_rules::AggRules_adj;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use sdb_dyn_proto_rdr::reader::Reader;
use slog::Logger;
use stamp_ftp::bm_reader::{BmKey, IntermediateBmPoints};
use stamp_ftp::calc_ftp;
use stamp_ftp::cfinput::AccFieldNames;
use stamp_ftp::read_adjustments::AdjKey;
use stamp_ftp::rule_stamper;
use std::collections::HashMap;

use crate::stamp_ftp::restructured_op::additional_struct::AmbData;

pub fn calculate_ftp(
    input_reader: &Reader,
    acc_data: &mut AccountWithCFs,
    inputfieldnames: &AccFieldNames,
    log: &Logger,
    diag_log: &Logger,
    from_date: &NaiveDate,
    to_date: &NaiveDate,
    m_rules: &AggRules,
    bc_rules: &AggRules,
    fix_adj_rules: &AggRules_adj,
    var_adj_rules: &AggRules_adj,
    bc_file_path: String,
    default_method: i32,
    default_basecurve: i32,
    fixed_adj_count: i32,
    var_adj_count: i32,
    mut saved_bm_rates: &mut HashMap<BmKey, Vec<IntermediateBmPoints>>,
    rate_precision: i8,
    bal_precision: i8,
    adj_rates: &HashMap<AdjKey, f64>,
    is_closed: bool,
    amb_map: &HashMap<String, AmbData>,
) -> (String, String) {
    let method = rule_stamper::get_method(&acc_data, &m_rules, default_method, diag_log);

    let basecurve = rule_stamper::get_bc(&acc_data, &bc_rules, default_basecurve, diag_log);

    let fix_lst_adjustments =
        rule_stamper::get_adj(&acc_data, &fix_adj_rules, fixed_adj_count, diag_log);
    let var_lst_adjustments =
        rule_stamper::get_adj(&acc_data, &var_adj_rules, var_adj_count, diag_log);

    calc_ftp::calc_ftp(
        input_reader,
        acc_data,
        inputfieldnames,
        log,
        diag_log,
        from_date,
        to_date,
        method,
        basecurve,
        fix_lst_adjustments,
        var_lst_adjustments,
        bc_file_path,
        fixed_adj_count,
        var_adj_count,
        &mut saved_bm_rates,
        rate_precision,
        bal_precision,
        adj_rates,
        is_closed,
        amb_map,
    )
}
