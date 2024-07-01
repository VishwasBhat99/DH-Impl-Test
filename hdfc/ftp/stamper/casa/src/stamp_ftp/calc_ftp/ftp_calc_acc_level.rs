use macros;
use math::round::half_away_from_zero;
use rbdate::*;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use stamp_ftp::bm_reader;
use stamp_ftp::bm_reader::yieldrate_calc;
use stamp_ftp::bm_reader::{BmKey, IntermediateBmPoints};
use stamp_ftp::cfinput::AccFieldNames;
use stamp_ftp::one_acc_view::One_acc_view;
use stamp_ftp::read_adjustments::Adj_key;
use stamp_ftp::CFout::AccountWithCashflows;
use statics::*;
use std::collections::HashMap;
use std::path::Path;

pub fn calc_ftp_cflevel(
    acc_data_in: &mut AccountWithCFs,
    mut cf_data_out: AccountWithCashflows,
    inputfieldnames: &AccFieldNames,
    basecurve: i32,
    lst_adjustments: Vec<i32>,
    log: &Logger,
    _ftprunid: i64,
    from_date: &NaiveDate,
    to_date: &NaiveDate,
    cpd: NaiveDate,
    adj_rates: &HashMap<Adj_key, f64>,
    basecurve_file_path: String,
    saved_bm_rates: &mut HashMap<BmKey, Vec<IntermediateBmPoints>>,
    rate_precision: i8,
    bal_precision: i8,
) -> (AccountWithCashflows, One_acc_view) {
    let mut lst_bm: Vec<IntermediateBmPoints> = Vec::new();
    let mut total_balance = 0.0;
    let mut total_interest_ftp = 0.0;
    let mut one_acc_op = One_acc_view::new();

    one_acc_op.bc_as_on_applied = timestamp(cpd);

    let full_file_path = format!(
        "{}{}_{}.txt",
        basecurve_file_path,
        cpd.format("%d-%m-%Y"),
        basecurve
    );
    let bm_key = BmKey {
        date: cpd.format("%d-%m-%Y").to_string(),
        base_curve_id: basecurve,
    };
    let max_days_in_year = rbdate::num_days_start_to_end(
        *to_date,
        rbdate::increment_date_by_months(*to_date, (12) as u16),
    );
    let mut yield_rate = DEFAULT_FLOAT;
    let mut adj_str: String = String::new();
    let mut out_str: String = String::new();

    //Added +1 -- It includes both from date and to dates.
    let run_duration = rbdate::num_days_start_to_end(*from_date, *to_date) + 1;

    let mut intr_calc_days = 0;
    if Path::new(&full_file_path).exists() {
        if saved_bm_rates.contains_key(&bm_key) {
            lst_bm = bm_reader::get_new_bm_points(saved_bm_rates, bm_key).to_vec();
        } else {
            bm_reader::get_bm_points(&full_file_path, cpd, log, &mut lst_bm);
            saved_bm_rates.insert(bm_key, lst_bm.clone());
        }

        intr_calc_days = run_duration;

        yield_rate = half_away_from_zero(
            yieldrate_calc::calc_yieldrate(&mut lst_bm, run_duration, cpd, log),
            rate_precision,
        );

        if yield_rate < 0.0 {
            yield_rate = 0.0
        }

        cf_data_out.FTP_Rate = yield_rate;
    } else {
        log_debug!(
            log,
            "File does not exist's in the path : {}. Hence Base rate will be zero for the account :{}",
            full_file_path, cf_data_out.account_no
        );
    }

    total_balance = cf_data_out.bal_total;
    total_interest_ftp = (cf_data_out.bal_total * cf_data_out.int_rate * intr_calc_days as f64)
        / (max_days_in_year as f64 * 100.0);

    cf_data_out.total_balance = total_balance;
    cf_data_out.total_interest_ftp = total_interest_ftp;

    let date = to_date.format("%d-%m-%Y");
    let ason = timestamp(*to_date);
    //Stamping adjustment rates.
    for i in 0..lst_adjustments.len() {
        let adj_key = Adj_key::new(ason, lst_adjustments[i]);
        let adj_rate = half_away_from_zero(
            match adj_rates.get(&adj_key) {
                Some(x) => *x,
                None => {
                    let st_dt = NaiveDateTime::from_timestamp(ason, 0)
                        .date()
                        .format("%d-%m-%Y");

                    log_debug!(
            log,
            "Adjustments does not exists for adjustment id :{}, date : {}, account id :{}", 
            lst_adjustments[i], st_dt, cf_data_out.account_no
        );
                    0.0
                }
            },
            rate_precision,
        );

        adj_str.push_str(&format!("{}|{}|", lst_adjustments[i], adj_rate));
        cf_data_out.FTP_Rate += adj_rate;
    }

    //Load zero rate for remaining adjustments if mapped adjustments are less than 6
    let len_remain = 6 - lst_adjustments.len() as i32;
    if len_remain != 0 && len_remain > 0 {
        for i in 1..=len_remain {
            adj_str.push_str("|0.0|");
        }
    }

    total_balance = half_away_from_zero(total_balance, bal_precision);
    total_interest_ftp = half_away_from_zero(total_interest_ftp, bal_precision);
    cf_data_out.FTP_Rate = half_away_from_zero(cf_data_out.FTP_Rate, rate_precision);

    one_acc_op.average_balance = total_balance;
    one_acc_op.int_rate = cf_data_out.int_rate;
    one_acc_op.accr_int = total_interest_ftp;
    one_acc_op.final_ftp_rate = cf_data_out.FTP_Rate;
    one_acc_op.final_ftp_amt = half_away_from_zero(
        (total_balance * cf_data_out.FTP_Rate * intr_calc_days as f64)
            / (max_days_in_year as f64 * 100.0),
        bal_precision,
    );
    one_acc_op.rate_curve = basecurve.to_string();
    one_acc_op.base_rate = cf_data_out.FTP_Rate;

    out_str.push_str(&adj_str);

    (cf_data_out, one_acc_op)
}
