use rbdate::timestamp;
use rbdate::NaiveDate;
use rbdate::NaiveDateTime;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use stamp_ftp::cfinput::AccFieldNames;
use stamp_ftp::read_adjustments::Adj_key;
use stamp_ftp::CFout::AccountWithCashflows;
use std::collections::HashMap;
use std::time::Instant;
mod ftp_calc_acc_level;
use math::round::half_away_from_zero;
use stamp_ftp::bm_reader::{BmKey, IntermediateBmPoints};
use stamp_ftp::one_acc_view::One_acc_view;

pub fn calc_ftp(
    mut acc_data_in: &mut AccountWithCFs,
    mut cf_data_out: AccountWithCashflows,
    inputfieldnames: &AccFieldNames,
    method: i32,
    basecurve: i32,
    lst_adjustments: Vec<i32>,
    basecurve_file_path: String,
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
    mut saved_bm_rates: &mut HashMap<BmKey, Vec<IntermediateBmPoints>>,
    rate_precision: i8,
    bal_precision: i8,
) -> (AccountWithCashflows, One_acc_view) {
    let mut one_acc_op_new = One_acc_view::new();
    match method {
        1023 => {
            //Assign Rate2 Method
            let (cf_data_out1, one_acc_op) = ftp_calc_acc_level::calc_ftp_cflevel(
                &mut acc_data_in,
                cf_data_out,
                inputfieldnames,
                basecurve,
                lst_adjustments,
                log,
                ftprunid,
                from_date,
                to_date,
                *to_date,
                adj_rates,
                basecurve_file_path,
                &mut saved_bm_rates,
                rate_precision,
                bal_precision,
            );
            cf_data_out = cf_data_out1;
            one_acc_op_new = one_acc_op;
            one_acc_op_new.method = "Assign Rate 2".to_string();
            one_acc_op_new.bc_as_on_rule = timestamp(*to_date);
        }
        _ => {
            //TODO: Handle undefined method here
        }
    }

    (cf_data_out, one_acc_op_new)
}
