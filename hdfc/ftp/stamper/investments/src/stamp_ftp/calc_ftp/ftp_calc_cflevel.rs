use macros;
use math::round;
use math::round::half_away_from_zero;
use rbdate::*;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use stamp_ftp::amb_file_reader::AvgBalances;
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
    is_lock: bool,
    ftp_rates_file_path: &str,
    adj_rates: &HashMap<Adj_key, f64>,
    avg_bal: &HashMap<String, AvgBalances>,
    saved_bm_rates: &mut HashMap<BmKey, Vec<IntermediateBmPoints>>,
    mut spread_writer: &mut BufWriter<File>,
    rate_precision: i8,
    bal_precision: i8,
) -> (AccountWithCashflows, One_acc_view, String) {
    let mut cf_ftp = Vec::new();
    let mut total_balance = 0.0;
    let mut total_interest_ftp = 0.0;
    let mut base_rate_prod;
    let mut end_rate_prod;
    let mut org_bal_tenor;
    let mut lst_bm: Vec<IntermediateBmPoints> = Vec::new();
    let mut one_acc_op = One_acc_view::new();

    one_acc_op.bc_as_on_applied = timestamp(cpd);
    one_acc_op.tenor_start_date_applied = timestamp(tsd);

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
    let mut out_str;
    let mut cf_str: String = String::new();
    let mut total_base_rate_prod = 0.0;
    let mut total_end_rate_prod = 0.0;
    let mut total_org_tenor_bal = 0.0;
    let mut base_rate = 0.0;
    let mut adj_rates_lock = vec![DEFAULT_FLOAT; 6];

    //Added +1 -- It includes both from date and to dates.
    let run_duration = rbdate::num_days_start_to_end(*from_date, *to_date) + 1;
    //println!("file path :{}", full_file_path );
    let mut intr_calc_days = 0;
    if Path::new(&full_file_path).exists() {
        if saved_bm_rates.contains_key(&bm_key) {
            lst_bm = bm_reader::get_new_bm_points(saved_bm_rates, bm_key).to_vec();
        } else {
            bm_reader::get_bm_points(&full_file_path, cpd, log, &mut lst_bm);
            saved_bm_rates.insert(bm_key, lst_bm.clone());
        }

        for cf in acc_data_in
            .remove_cfs_for_key(&inputfieldnames.cashflows)
            .expect("fail")
            .iter_mut()
        {
            one_acc_op.tenor_end_date_applied = cf.date;
            let mut adj_rate = 0.0;
            let cashflow_date = NaiveDateTime::from_timestamp(cf.date, 0).date();
            let mut residual_days = rbdate::num_days_start_to_end(cpd, cashflow_date);

            if residual_days <= 0 {
                residual_days = 1;
            }

            intr_calc_days = run_duration;

            let mut yield_rate = round::ceil(
                yieldrate_calc::calc_yieldrate(&mut lst_bm, residual_days, cpd, log),
                4,
            );

            if yield_rate < 0.0 {
                yield_rate = 0.0
            }

            for index in 0..lst_adjustments.len() {
                let adj_key = Adj_key::new(cf_data_out.val_dt, lst_adjustments[index]);
                let rate = match adj_rates.get(&adj_key) {
                    Some(x) => *x,
                    None => 0.0,
                };

                adj_rate += rate;
            }

            let mut cf_obj = Cashflow::new();
            cf_obj.interest_amount = cf.interest_amount;
            cf_obj.principal_amount = cf.principal_amount;
            cf_obj.date = cf.date;
            cf_obj.base_rate = yield_rate;
            cf_obj.base_rate_amount = (cf.principal_amount * yield_rate * intr_calc_days as f64)
                / (max_days_in_year as f64 * 100.0);

            base_rate_prod = yield_rate * residual_days as f64 * cf_obj.principal_amount;
            end_rate_prod =
                (yield_rate + adj_rate) * residual_days as f64 * cf_obj.principal_amount;
            org_bal_tenor = residual_days as f64 * cf_obj.principal_amount;

            total_base_rate_prod += base_rate_prod;
            total_end_rate_prod += end_rate_prod;
            total_org_tenor_bal += org_bal_tenor;

            total_balance += cf.principal_amount;
            total_interest_ftp += cf.interest_amount;

            cf_ftp.push(cf_obj);

            let str_cf = format!(
                "{}|{}|{}|{}|{}|{}|{}|{}|{}",
                cf_data_out.deal_no,
                cashflow_date.format("%d-%m-%Y"),
                residual_days,
                cf.principal_amount,
                yield_rate,
                adj_rate,
                org_bal_tenor,
                base_rate_prod,
                end_rate_prod
            );

            cf_str.push_str(&str_cf);
            cf_str.push('\n');
        }
    } else {
        for cf in acc_data_in
            .remove_cfs_for_key(&inputfieldnames.cashflows)
            .expect("fail")
            .iter_mut()
        {
            one_acc_op.tenor_end_date_applied = cf.date;
            let mut cf_obj = Cashflow::new();
            cf_obj.interest_amount = cf.interest_amount;
            cf_obj.principal_amount = cf.principal_amount;
            cf_obj.date = cf.date;
            cf_obj.base_rate = 0.0;
            cf_obj.base_rate_amount = 0.0;

            total_balance += cf.principal_amount;
            total_interest_ftp += cf.interest_amount;

            cf_ftp.push(cf_obj);
        }

        log_debug!(
            log,
            "File does not exist's in the path : {}. Hence Base rate will be zero for the account :{}",
            full_file_path, cf_data_out.gl
        );
    }

    total_balance = half_away_from_zero(total_balance, bal_precision);
    total_interest_ftp = half_away_from_zero(total_interest_ftp, bal_precision);
    total_org_tenor_bal = half_away_from_zero(total_org_tenor_bal, bal_precision);

    cf_data_out.total_balance = total_balance;
    cf_data_out.total_interest_ftp = total_interest_ftp;
    if total_org_tenor_bal != 0.0 {
        cf_data_out.FTP_Rate =
            half_away_from_zero(total_end_rate_prod / total_org_tenor_bal, rate_precision);
        base_rate = half_away_from_zero(total_base_rate_prod / total_org_tenor_bal, rate_precision);
    }
    cf_data_out.total_ftp = half_away_from_zero(
        (total_balance * cf_data_out.FTP_Rate * intr_calc_days as f64)
            / (max_days_in_year as f64 * 100.0),
        bal_precision,
    );

    //Stamping adjustment rates.
    for index in 0..lst_adjustments.len() {
        let adj_key = Adj_key::new(cf_data_out.val_dt, lst_adjustments[index]);
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
            lst_adjustments[index], st_dt, cf_data_out.deal_no
        );
                    0.0
                }
            },
            rate_precision,
        );

        adj_rates_lock[index] = adj_rate;
        adj_str.push_str(&format!("{}|{}|", lst_adjustments[index], adj_rate));
        lock_str.push_str(&format!("{}|", adj_rate));
        lock_str_adj.push_str(&format!("{}|", lst_adjustments[index]));
    }

    //Load zero rate for remaining adjustments if mapped adjustments are less than 6
    let len_remain = 6 - lst_adjustments.len() as i32;
    if len_remain != 0 && len_remain > 0 {
        for _index in 1..=len_remain {
            adj_str.push_str("|0.0|");
            lock_str.push_str("0.0|");
            lock_str_adj.push_str("|");
        }
    }

    lock_str.truncate(lock_str.len() - 1);
    lock_str_adj.truncate(lock_str.len() - 1);
    cf_data_out.cashflows = protobuf::RepeatedField::from_vec(cf_ftp);

    let def_avg_bals = AvgBalances {
        avg_balance: DEFAULT_FLOAT,
        accr_int: DEFAULT_FLOAT,
    };
    let average_balances = match avg_bal.get(&cf_data_out.deal_no) {
        Some(x) => x,
        None => {
            log_debug!(
                    log,
                    "Average balance is not availale for account id :{} . Hence considering zero balance for the same.", 
                    cf_data_out.deal_no
                );
            &def_avg_bals
        }
    };
    let average_balance = half_away_from_zero(average_balances.avg_balance, bal_precision);

    cf_data_out.book_yield = half_away_from_zero(cf_data_out.book_yield, rate_precision);
    cf_data_out.FTP_Rate = half_away_from_zero(cf_data_out.FTP_Rate, rate_precision);

    one_acc_op.average_balance = average_balance;
    one_acc_op.int_rate = cf_data_out.book_yield;
    one_acc_op.accr_int = average_balances.accr_int;
    one_acc_op.final_ftp_rate = cf_data_out.FTP_Rate;
    one_acc_op.final_ftp_amt = half_away_from_zero(
        (average_balance * cf_data_out.FTP_Rate * intr_calc_days as f64)
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
        average_balance,
        cf_data_out.book_yield,
        total_interest_ftp,
        cf_data_out.FTP_Rate,
        one_acc_op.final_ftp_amt,
        basecurve,
        base_rate
    );

    out_str.push_str(&adj_str);

    if is_lock {
        let spread = half_away_from_zero(
            cf_data_out.book_yield - cf_data_out.FTP_Rate,
            rate_precision,
        );
        let ftp_rates_out = format!(
            "{}|{}|{}|{}|{}|{}|{}|{}",
            cf_data_out.deal_no,
            cf_data_out.FTP_Rate,
            base_rate,
            lock_str,
            cf_data_out.book_yield,
            spread,
            basecurve,
            lock_str_adj
        );
        one_acc_op.fx_spread = spread;
        write!(spread_writer, "{}", ftp_rates_out).expect("Error while write spread file.");

        out_str = format!("{}{}|", out_str, spread);
    } else {
        out_str = format!("{}{}|", out_str, 0.0);
    }

    (cf_data_out, one_acc_op, cf_str)
}