use macros;
use rbdate::NaiveDate;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use sdb_dyn_proto_rdr::reader::Reader;
use slog::Logger;
use stamp_ftp::bm_reader::{BmKey, IntermediateBmPoints};
use stamp_ftp::cfinput::AccFieldNames;
use stamp_ftp::read_adjustments::AdjKey;
use std::collections::HashMap;

use super::restructured_op::additional_struct::AmbData;

mod ftp_calc_acc_level;

pub fn calc_ftp(
    input_reader: &Reader,
    acc_data_in: &mut AccountWithCFs,
    inputfieldnames: &AccFieldNames,
    log: &Logger,
    _diag_log: &Logger,
    from_date: &NaiveDate,
    to_date: &NaiveDate,
    method: i32,
    basecurve: i32,
    fix_lst_adjustments: Vec<i32>,
    var_lst_adjustments: Vec<i32>,
    basecurve_file_path: String,
    fixed_adj_count: i32,
    _var_adj_count: i32,
    mut saved_bm_rates: &mut HashMap<BmKey, Vec<IntermediateBmPoints>>,
    rate_precision: i8,
    bal_precision: i8,
    adj_rates: &HashMap<AdjKey, f64>,
    is_closed: bool,
    amb_map: &HashMap<String, AmbData>,
) -> (String, String) {
    match method {
        1023 => {
            //Assign Rate 3 Method
            //cpd : AsOnDate
            ftp_calc_acc_level::calc_ftp(
                input_reader,
                &acc_data_in,
                inputfieldnames,
                basecurve,
                fix_lst_adjustments,
                var_lst_adjustments,
                basecurve_file_path,
                log,
                from_date,
                to_date,
                *to_date,
                *to_date,
                adj_rates,
                fixed_adj_count,
                &mut saved_bm_rates,
                rate_precision,
                bal_precision,
                method,
                "Assign Rate 3",
                is_closed,
                amb_map,
            )
        }
        _ => {
            //TODO: Handle undefined method here
            log_debug!(
                log,
                "Method: `{}` not found for account: `{}`.",
                method,
                acc_data_in
                    .get_string_for_key(&inputfieldnames.cod_acc_no)
                    .unwrap_or(&String::default())
                    .to_string()
            );
            (String::default(), String::default())
        }
    }
}
