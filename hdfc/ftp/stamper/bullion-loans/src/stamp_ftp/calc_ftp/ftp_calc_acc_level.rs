use macros;
use math::round::half_away_from_zero;
use rbdate::timestamp;
use rbdate::NaiveDate;
use rbdate::NaiveDateTime;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use stamp_ftp::amb_file_reader::AmbVal;
use stamp_ftp::bm_reader;
use stamp_ftp::bm_reader::yieldrate_calc;
use stamp_ftp::bm_reader::{BmKey, IntermediateBmPoints};
use stamp_ftp::cfinput::AccFieldNames;
use stamp_ftp::one_acc_view::One_acc_view;
use stamp_ftp::read_adjustments::Adj_key;
use stamp_ftp::CFout::AccountWithCashflows;
use stamp_ftp::CFout::Cashflow;
use statics::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

pub fn calc_ftp_cflevel(
    acc_data_in: &mut AccountWithCFs,
    mut cf_data_out: AccountWithCashflows,
    inputfieldnames: &AccFieldNames,
    basecurve: i32,
    lst_adjustments: Vec<i32>,
    basecurve_file_path: String,
    log: &Logger,
    from_date: &NaiveDate,
    to_date: &NaiveDate,
    cpd: NaiveDate,
    tsd: NaiveDate,
    ted: NaiveDate,
    is_lock: bool,
    adj_rates: &HashMap<Adj_key, f64>,
    avg_bal: &HashMap<String, AmbVal>,
    out_path: &str,
    saved_bm_rates: &mut HashMap<BmKey, Vec<IntermediateBmPoints>>,
    mut spread_writer: &mut BufWriter<File>,
    rate_precision: i8,
    bal_precision: i8,
) -> (AccountWithCashflows, One_acc_view, String) {
    let mut cf_ftp = Vec::new();
    let mut lst_bm: Vec<IntermediateBmPoints> = Vec::new();
    let mut lst_out: Vec<String> = Vec::new();
    let mut total_balance = 0.0;
    let mut total_interest_ftp = 0.0;
    let mut total_ftp = 0.0;
    let mut ftp_rate = 0.0;
    let mut adj_rates_lock = vec![DEFAULT_FLOAT; 6];
    let mut adj_codes = vec![DEFAULT_INT; 6];
    let mut one_acc_op = One_acc_view::new();
    let mut yield_rate = DEFAULT_FLOAT;

    one_acc_op.bc_as_on_applied = timestamp(cpd);
    one_acc_op.tenor_start_date_applied = timestamp(tsd);
    one_acc_op.tenor_end_date_applied = timestamp(ted);

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
    let mut adj_str: String = String::new();
    let mut lock_str: String = String::new();
    let mut lock_str_adj: String = String::new();
    let mut out_str: String = String::new();
    let mut cf_str: String = String::new();
    let mut total_base_rate_prod = 0.0;
    let mut total_end_rate_prod = 0.0;
    let mut total_org_tenor_bal = 0.0;
    let mut residual_days = 0;

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

        if ted > cpd {
            residual_days = rbdate::num_days_start_to_end(cpd, ted);
        }

        if residual_days <= 0 {
            residual_days = 0;
        }

        intr_calc_days = run_duration;

        yield_rate = half_away_from_zero(
            yieldrate_calc::calc_yieldrate(&mut lst_bm, residual_days, cpd, log),
            rate_precision,
        );

        if yield_rate < 0.0 {
            yield_rate = 0.0
        }

        for cf in acc_data_in
            .remove_cfs_for_key(&inputfieldnames.cashflows)
            .expect("fail")
            .iter_mut()
        {
            let mut cf_obj = Cashflow::new();
            cf_obj.interest_amount = cf.interest_amount;
            cf_obj.principal_amount = cf.principal_amount;
            cf_obj.date = cf.date;
            cf_obj.base_rate = yield_rate;
            cf_obj.base_rate_amount = (cf.principal_amount * yield_rate * intr_calc_days as f64)
                / (max_days_in_year as f64 * 100.0);

            total_balance += cf.principal_amount;
            total_interest_ftp += cf.interest_amount;
            total_ftp += cf_obj.base_rate_amount;

            ftp_rate = ftp_rate + (yield_rate * cf.principal_amount);

            cf_ftp.push(cf_obj);
        }
    } else {
        for cf in acc_data_in
            .remove_cfs_for_key(&inputfieldnames.cashflows)
            .expect("fail")
            .iter_mut()
        {
            let mut cf_obj = Cashflow::new();
            cf_obj.interest_amount = cf.interest_amount;
            cf_obj.principal_amount = cf.principal_amount;
            cf_obj.date = cf.date;
            cf_obj.base_rate = 0.0;
            cf_obj.base_rate_amount = 0.0;

            total_balance += cf.principal_amount;
            total_interest_ftp += cf.interest_amount;
            total_ftp += cf_obj.base_rate_amount;

            ftp_rate = 0.0;

            cf_ftp.push(cf_obj);
        }

        log_debug!(
            log,
            "File does not exist's in the path : {}. Hence Base rate will be zero for the account :{}",
            full_file_path, cf_data_out.acc_no
        );
    }

    total_balance = half_away_from_zero(total_balance, bal_precision);
    total_interest_ftp = half_away_from_zero(total_interest_ftp, bal_precision);

    cf_data_out.total_balance = total_balance;
    cf_data_out.total_interest_ftp = total_interest_ftp;
    if total_balance != 0.0 {
        cf_data_out.FTP_Rate = ftp_rate / total_balance;
    }
    cf_data_out.total_ftp = half_away_from_zero(
        (total_balance * cf_data_out.FTP_Rate * intr_calc_days as f64)
            / (max_days_in_year as f64 * 100.0),
        bal_precision,
    );

    let mut base_rate = cf_data_out.FTP_Rate;

    //Stamping adjustment rates.
    for i in 0..lst_adjustments.len() {
        let adj_key = Adj_key::new(cf_data_out.st_dt, lst_adjustments[i]);
        let adj_rate = half_away_from_zero(
            match adj_rates.get(&adj_key) {
                Some(x) => *x,
                None => {
                    let st_dt = NaiveDateTime::from_timestamp(cf_data_out.st_dt, 0)
                        .date()
                        .format("%d-%m-%Y");

                    log_debug!(
            log,
            "Adjustments does not exists for adjustment id :{}, date : {}, account id :{}", 
            lst_adjustments[i], st_dt, cf_data_out.acc_no
        );
                    0.0
                }
            },
            rate_precision,
        );

        adj_rates_lock[i] = adj_rate;
        adj_codes[i] = lst_adjustments[i] as i64;

        adj_str.push_str(&format!("{}|{}|", lst_adjustments[i], adj_rate));
        lock_str.push_str(&format!("{}|", adj_rate));
        lock_str_adj.push_str(&format!("{}|", lst_adjustments[i]));
        cf_data_out.FTP_Rate += adj_rate;
    }

    //Load zero rate for remaining adjustments if mapped adjustments are less than 6
    let len_remain = 6 - lst_adjustments.len() as i32;
    if len_remain != 0 && len_remain > 0 {
        for i in 1..=len_remain {
            adj_str.push_str("|0.0|");
            lock_str.push_str("0.0|");
            lock_str_adj.push_str("|");
        }
    }

    lock_str.truncate(lock_str.len() - 1);
    lock_str_adj.truncate(lock_str.len() - 1);
    cf_data_out.cashflows = protobuf::RepeatedField::from_vec(cf_ftp);

    let default_avg = AmbVal::new(DEFAULT_FLOAT, DEFAULT_FLOAT);
    let average_balance = match avg_bal.get(&cf_data_out.acc_no) {
        Some(x) => x,
        None => {
            log_debug!(
        log,
        "Average balance is not availale for account id :{} . Hence considering zero balance for the same.", 
         cf_data_out.acc_no
    );
            &default_avg
        }
    };

    cf_data_out.FTP_Rate = half_away_from_zero(cf_data_out.FTP_Rate, rate_precision);
    base_rate = half_away_from_zero(base_rate, rate_precision);
    let avg_bal = half_away_from_zero(average_balance.avg_bal, bal_precision);
    let int_amt = half_away_from_zero(average_balance.int_amt, bal_precision);

    one_acc_op.average_balance = avg_bal;
    one_acc_op.int_rate = cf_data_out.int_rt;
    one_acc_op.accr_int = int_amt;
    one_acc_op.final_ftp_rate = cf_data_out.FTP_Rate;
    one_acc_op.final_ftp_amt = half_away_from_zero(
        (avg_bal * cf_data_out.FTP_Rate * intr_calc_days as f64)
            / (max_days_in_year as f64 * 100.0),
        bal_precision,
    );
    one_acc_op.rate_curve = basecurve.to_string();
    one_acc_op.base_rate = base_rate;
    one_acc_op.adj1 = adj_rates_lock[0];
    one_acc_op.adj2 = adj_rates_lock[1];
    one_acc_op.adj3 = adj_rates_lock[2];
    one_acc_op.adj4 = adj_rates_lock[3];
    one_acc_op.adj5 = adj_rates_lock[4];
    one_acc_op.adj6 = adj_rates_lock[5];

    out_str = format!(
        "{}|{}|{}|{}|{}|{}|{}|",
        avg_bal,
        cf_data_out.int_rt,
        int_amt,
        cf_data_out.FTP_Rate,
        one_acc_op.final_ftp_amt,
        basecurve,
        base_rate
    );
    out_str.push_str(&adj_str);

    let mut adj_lock_str: String = String::new();
    for i in 0..adj_codes.len() {
        let str_lock = format!("{}|{}|", adj_codes[i], adj_rates_lock[i]);
        adj_lock_str.push_str(&str_lock);
    }

    if is_lock {
        let spread = half_away_from_zero(cf_data_out.int_rt - cf_data_out.FTP_Rate, rate_precision);
        let ftp_rates_out = format!(
            "{}|{}|{}|{}|{}{}|{}",
            cf_data_out.acc_no,
            cf_data_out.FTP_Rate,
            basecurve,
            base_rate,
            adj_lock_str,
            cf_data_out.int_rt,
            spread
        );
        write!(spread_writer, "{}", ftp_rates_out).expect("Error while write spread file.");
        one_acc_op.fx_spread = spread;

        out_str = format!("{}{}|", out_str, spread);
    } else {
        out_str = format!("{}{}|", out_str, 0.0);
    }

    (cf_data_out, one_acc_op, cf_str)
}
