use chrono::Datelike;
use macros;
use math::round::half_away_from_zero;
use rbdate::*;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use stamp_ftp::amb_file_reader::AmbVal;
use stamp_ftp::bm_reader::IntermediateBmPoints;
use stamp_ftp::cfinput::AccFieldNames;
use stamp_ftp::one_acc_view::One_acc_view;
use stamp_ftp::read_adjustments::Adj_key;
use stamp_ftp::CFout::AccountWithCashflows;
use stamp_ftp::CFout::Cashflow;
use statics::{DEFAULT_FLOAT, DEFAULT_INT};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

pub fn calc_ftp(
    acc_data_in: &mut AccountWithCFs,
    mut cf_data_out: AccountWithCashflows,
    inputfieldnames: &AccFieldNames,
    ftp_rates: &Vec<f64>,
    lock_adjs: &HashMap<i32, String>,
    log: &Logger,
    ftprunid: i64,
    from_date: &NaiveDate,
    to_date: &NaiveDate,
    out_path: &str,
    tsd: NaiveDate,
    ted: NaiveDate,
    var_lst_adjustments: Vec<i32>,
    adj_rates: &HashMap<Adj_key, f64>,
    avg_bal: &HashMap<String, AmbVal>,
    fixed_adj_count: i32,
    var_adj_count: i32,
    rate_precision: i8,
    bal_precision: i8,
    mut spread_writer: &mut BufWriter<File>,
    method: i32,
) -> (AccountWithCashflows, One_acc_view, String) {
    let mut cf_ftp = Vec::new();
    let mut total_balance = 0.0;
    let mut total_interest_ftp = 0.0;
    let mut total_ftp = 0.0;
    let mut ftp_rate = 0.0;
    let mut adj_rates_lock = vec![DEFAULT_FLOAT; 6];
    let mut adj_codes = vec![DEFAULT_INT; 6];
    let mut total_adj = DEFAULT_FLOAT;
    let mut var_adj_rate = DEFAULT_FLOAT;
    let mut one_acc_op = One_acc_view::new();
    let mut fixed_adj_rate = DEFAULT_FLOAT;

    one_acc_op.tenor_start_date_applied = timestamp(tsd);
    one_acc_op.tenor_end_date_applied = timestamp(ted);

    //Added +1 -- It includes both from date and to dates.
    let run_duration = num_days_start_to_end(*from_date, *to_date) + 1;
    let mut intr_calc_days = 0;
    let mut adj_str: String = String::new();

    let mut residual_days = num_days_start_to_end(tsd, ted);
    let max_days_in_year =
        num_days_start_to_end(*to_date, increment_date_by_months(*to_date, (12) as u16));

    if residual_days <= 0 {
        residual_days = 0;
    }

    intr_calc_days = run_duration;

    for i in 0..fixed_adj_count as i64 {
        let rate = half_away_from_zero(ftp_rates[(i + 1) as usize], rate_precision);
        let adjid = ftp_rates[(i + 10) as usize] as i64;

        adj_rates_lock[i as usize] = rate;
        total_adj += rate;
        fixed_adj_rate += rate;
        adj_codes[i as usize] = adjid;
    }

    for i in 0..var_lst_adjustments.len() {
        let adj_key = Adj_key::new(cf_data_out.val_dt, var_lst_adjustments[i]);
        let adj_rate = half_away_from_zero(
            match adj_rates.get(&adj_key) {
                Some(x) => *x,
                None => {
                    let st_dt = NaiveDateTime::from_timestamp(cf_data_out.val_dt, 0)
                        .date()
                        .format("%d-%m-%Y");

                    log_debug!(
            log,
            "Adjustments does not exists for adjustment id :{}, date : {}, account id :{}", 
            var_lst_adjustments[i], st_dt, cf_data_out.reference
        );
                    0.0
                }
            },
            rate_precision,
        );

        adj_rates_lock[(fixed_adj_count as usize + i)] = adj_rate;
        total_adj += adj_rate;
        var_adj_rate += adj_rate;
        adj_codes[(fixed_adj_count as usize + i)] = var_lst_adjustments[i] as i64;
    }

    for i in 0..adj_codes.len() {
        let str_lock = format!("{}|{}|", adj_codes[i], adj_rates_lock[i]);
        adj_str.push_str(&str_lock);
    }

    let lrp_dt = date_from_timestamp(cf_data_out.lst_repricing_dt);
    let prev_int_rt = ftp_rates[7];
    let prev_spread = ftp_rates[8];
    let wt_avg_tpr = if lrp_dt.month() == from_date.month() && lrp_dt.year() == from_date.year() {
        let prev_days = num_days_start_to_end(*from_date, lrp_dt) as f64;
        let new_days = run_duration as f64 - prev_days;
        half_away_from_zero(
            ((prev_int_rt - prev_spread) * prev_days
                + (cf_data_out.norm_int_rt - prev_spread) * new_days)
                / run_duration as f64,
            rate_precision,
        )
    } else {
        cf_data_out.norm_int_rt - prev_spread
    };

    let total_tpr = half_away_from_zero(wt_avg_tpr + var_adj_rate, rate_precision);
    let baserate = half_away_from_zero(total_tpr - total_adj, rate_precision);

    for cf in acc_data_in
        .remove_cfs_for_key(&inputfieldnames.cashflows)
        .expect("fail")
        .iter_mut()
    {
        let mut cf_obj = Cashflow::new();
        cf_obj.interest_amount = cf.interest_amount;
        cf_obj.principal_amount = cf.principal_amount;
        cf_obj.date = cf.date;
        cf_obj.base_rate = baserate;
        cf_obj.base_rate_amount = (cf.principal_amount * baserate * intr_calc_days as f64)
            / (max_days_in_year as f64 * 100.0);

        total_balance += cf.principal_amount;
        total_interest_ftp += cf.interest_amount;
        total_ftp += cf_obj.base_rate_amount;

        ftp_rate = ftp_rate + (ftp_rates[0] * cf.principal_amount);

        cf_ftp.push(cf_obj);
    }

    let basecurve = ftp_rates[9];

    let base_str = format!("{}|{}", basecurve, baserate);

    let default_avg = AmbVal::new(DEFAULT_FLOAT, DEFAULT_FLOAT);
    let average_balance = match avg_bal.get(&cf_data_out.reference) {
        Some(x) => x,
        None => {
            log_debug!(
                log,
                "Average balance is not availale for account id :{} . \
        Hence considering zero balance for the same.",
                cf_data_out.reference
            );
            &default_avg
        }
    };

    total_balance = half_away_from_zero(total_balance, bal_precision);
    total_interest_ftp = half_away_from_zero(total_interest_ftp, bal_precision);
    total_ftp = half_away_from_zero(total_ftp, bal_precision);
    var_adj_rate = half_away_from_zero(var_adj_rate, rate_precision);
    let avg_bal = half_away_from_zero(average_balance.avg_bal, bal_precision);
    let int_amt = half_away_from_zero(average_balance.int_amt, bal_precision);

    cf_data_out.total_balance = total_balance;
    cf_data_out.total_interest_ftp = total_interest_ftp;
    cf_data_out.total_ftp = total_ftp;
    if total_balance != 0.0 {
        cf_data_out.FTP_Rate = half_away_from_zero(ftp_rate / total_balance, rate_precision);
    }

    one_acc_op.average_balance = avg_bal;
    one_acc_op.int_rate = cf_data_out.norm_int_rt;
    one_acc_op.accr_int = int_amt;
    one_acc_op.final_ftp_rate = total_tpr;
    one_acc_op.final_ftp_amt = half_away_from_zero(
        (avg_bal * total_tpr * intr_calc_days as f64) / (max_days_in_year as f64 * 100.0),
        bal_precision,
    );
    one_acc_op.rate_curve = basecurve.to_string();
    one_acc_op.adj1 = adj_rates_lock[0];
    one_acc_op.adj2 = adj_rates_lock[1];
    one_acc_op.adj3 = adj_rates_lock[2];
    one_acc_op.adj4 = adj_rates_lock[3];
    one_acc_op.adj5 = adj_rates_lock[4];
    one_acc_op.adj6 = adj_rates_lock[5];
    one_acc_op.fx_spread = half_away_from_zero(
        cf_data_out.norm_int_rt - (baserate + fixed_adj_rate),
        rate_precision,
    );
    one_acc_op.var_spread = var_adj_rate;
    one_acc_op.base_rate = baserate;

    let out_str = format!(
        "{}|{}|{}|{}|{}|{}|{}{}|{}|{}|",
        avg_bal,
        cf_data_out.norm_int_rt,
        int_amt,
        total_tpr,
        half_away_from_zero(
            (avg_bal * total_tpr * intr_calc_days as f64) / (max_days_in_year as f64 * 100.0),
            bal_precision
        ),
        base_str,
        adj_str,
        one_acc_op.fx_spread,
        var_adj_rate,
        half_away_from_zero((cf_data_out.norm_int_rt - total_tpr), rate_precision),
    );

    let mut rates_out = String::new();
    let mut id_index = 9;
    for rate_index in 0..7 {
        id_index = 9 + rate_index;
        rates_out.push_str(&format!(
            "{}|{}|",
            ftp_rates[id_index] as i64, ftp_rates[rate_index],
        ));
    }
    let ftp_rates_out = format!(
        "{}|{}|{}{}|{}|{}\n",
        cf_data_out.reference,
        ftp_rates[15],
        rates_out,
        cf_data_out.norm_int_rt,
        prev_spread,
        method,
    );
    write!(spread_writer, "{}", ftp_rates_out).expect("Error while write spread file.");

    (cf_data_out, one_acc_op, adj_str)
}
