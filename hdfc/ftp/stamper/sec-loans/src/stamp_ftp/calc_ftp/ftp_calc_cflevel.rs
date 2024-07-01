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
use stamp_ftp::one_acc_view::OneAccView;
use stamp_ftp::read_adjustments::AdjKey;
use stamp_ftp::CFout::AccountWithCashflows;
use stamp_ftp::CFout::Cashflow;
use statics::{DEFAULT_FLOAT, DEFAULT_INT};
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
    fix_lst_adjustments: Vec<i32>,
    var_lst_adjustments: Vec<i32>,
    basecurve_file_path: String,
    log: &Logger,
    ftprunid: i64,
    from_date: &NaiveDate,
    to_date: &NaiveDate,
    cpd: NaiveDate,
    tsd: NaiveDate,
    is_lock: bool,
    out_path: &str,
    adj_rates: &HashMap<AdjKey, f64>,
    avg_bal: &HashMap<String, AmbVal>,
    fixed_adj_count: i32,
    var_adj_count: i32,
    saved_bm_rates: &mut HashMap<BmKey, Vec<IntermediateBmPoints>>,
    mut spread_writer: &mut BufWriter<File>,
    rate_precision: i8,
    bal_precision: i8,
    method: i32,
) -> (AccountWithCashflows, OneAccView, String) {
    let mut cf_ftp = Vec::new();
    let mut lst_bm: Vec<IntermediateBmPoints> = Vec::new();
    let mut total_balance = 0.0;
    let mut total_interest_ftp = 0.0;
    let mut total_ftp = 0.0;
    let mut ftp_rate = 0.0;
    let mut adj_rates_lock = vec![DEFAULT_FLOAT; 6];
    let mut adj_codes = vec![DEFAULT_INT; 6];
    let mut fixed_adj_rate = DEFAULT_FLOAT;
    let mut variable_adj_rate = DEFAULT_FLOAT;
    let mut one_acc_op = OneAccView::new();

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
    let mut out_str: String = String::new();
    let mut cf_str: String = String::new();
    let mut total_base_rate_prod = 0.0;
    let mut total_end_rate_prod = 0.0;
    let mut total_org_tenor_bal = 0.0;
    let mut total_tpr = 0.0;
    let mut base_rate = 0.0;

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

        let mut base_rate_prod = 0.0;
        let mut end_rate_prod = 0.0;
        let mut org_bal_tenor = 0.0;

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

            let mut yield_rate = half_away_from_zero(
                yieldrate_calc::calc_yieldrate(&mut lst_bm, residual_days, cpd, log),
                rate_precision,
            );

            if yield_rate < 0.0 {
                yield_rate = 0.0
            }

            for i in 0..fix_lst_adjustments.len() {
                let adj_key = AdjKey::new(cf_data_out.st_dt, fix_lst_adjustments[i]);
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
            total_ftp += cf_obj.base_rate_amount;

            cf_ftp.push(cf_obj);

            let str_cf = format!(
                "{}|{}|{}|{}|{}|{}|{}|{}|{}",
                cf_data_out.acc_id,
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
            total_ftp += cf_obj.base_rate_amount;

            ftp_rate = 0.0;

            cf_ftp.push(cf_obj);
        }

        log_debug!(
            log,
            "File does not exist's in the path : {}. Hence Base rate will be zero for the account :{}",
            full_file_path, cf_data_out.acc_id
        );
    }

    cf_data_out.total_balance = total_balance;
    cf_data_out.total_interest_ftp = total_interest_ftp;
    if total_org_tenor_bal != 0.0 {
        cf_data_out.FTP_Rate = total_end_rate_prod / total_org_tenor_bal;
        base_rate = total_base_rate_prod / total_org_tenor_bal;
    }
    cf_data_out.total_ftp = (total_balance * cf_data_out.FTP_Rate * intr_calc_days as f64)
        / (max_days_in_year as f64 * 100.0);

    //Stamping adjustment rates.
    for i in 0..fix_lst_adjustments.len() {
        let adj_key = AdjKey::new(cf_data_out.st_dt, fix_lst_adjustments[i]);
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
        fix_lst_adjustments[i], st_dt, cf_data_out.acc_id
    );
                    DEFAULT_FLOAT
                }
            },
            rate_precision,
        );

        adj_rates_lock[i] = adj_rate;
        adj_codes[i] = fix_lst_adjustments[i] as i64;

        adj_str.push_str(&format!("{}|{}|", fix_lst_adjustments[i], adj_rate));
        fixed_adj_rate += adj_rate;
    }

    //Stamping variable adjustment rates.
    for i in 0..var_lst_adjustments.len() {
        let adj_key = AdjKey::new(cf_data_out.st_dt, var_lst_adjustments[i]);
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
                    var_lst_adjustments[i],
                    st_dt,
                    cf_data_out.acc_id
                );
                    DEFAULT_FLOAT
                }
            },
            rate_precision,
        );

        adj_rates_lock[fix_lst_adjustments.len() + i] = adj_rate;
        adj_codes[fix_lst_adjustments.len() + i] = var_lst_adjustments[i] as i64;

        adj_str.push_str(&format!("{}|{}|", var_lst_adjustments[i], adj_rate));
        cf_data_out.FTP_Rate += adj_rate;
        variable_adj_rate += adj_rate;
    }

    //Load zero rate for remaining adjustments if mapped adjustments are less than 6
    let len_remain = 6 - (fix_lst_adjustments.len() + var_lst_adjustments.len()) as i32;
    if len_remain != 0 && len_remain > 0 {
        for i in 1..=len_remain {
            adj_str.push_str("|0.0|");
        }
    }
    cf_data_out.cashflows = protobuf::RepeatedField::from_vec(cf_ftp);

    let default_avg = AmbVal::new(
        DEFAULT_FLOAT,
        DEFAULT_FLOAT,
    );
    let average_balance = match avg_bal.get(&cf_data_out.acc_id) {
        Some(x) => x,
        None => {
            log_debug!(
        log,
        "Average balance is not availale for account id :{} . Hence considering zero balance for the same.", 
         cf_data_out.acc_id
    );
            &default_avg
        }
    };

    cf_data_out.int_rt = half_away_from_zero(cf_data_out.int_rt, rate_precision);
    cf_data_out.FTP_Rate = half_away_from_zero(cf_data_out.FTP_Rate, rate_precision);
    fixed_adj_rate = half_away_from_zero(fixed_adj_rate, rate_precision);
    variable_adj_rate = half_away_from_zero(variable_adj_rate, rate_precision);
    let avg_bal = half_away_from_zero(average_balance.avg_bal, bal_precision).abs();
    let int_amt = half_away_from_zero(average_balance.int_amt, bal_precision).abs();

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
    one_acc_op.outstanding_bal = half_away_from_zero(cf_data_out.pout_bal, bal_precision);
    one_acc_op.mis1 = cf_data_out.ftp_runid.to_string();

    out_str = format!(
        "{}|{}|{}|{}|{}|{}|{}|",
        avg_bal,
        cf_data_out.int_rt,
        int_amt,
        cf_data_out.FTP_Rate,
        one_acc_op.final_ftp_amt,
        basecurve,
        base_rate,
    );

    out_str.push_str(&adj_str);
    let mut adj_lock_str: String = String::new();

    for i in 0..adj_codes.len() {
        let str_lock = format!("{}|{}|", adj_codes[i], adj_rates_lock[i]);
        adj_lock_str.push_str(&str_lock);
    }

    if is_lock {
        let fix_spread = if method == 1035 {
            base_rate
        } else {
            half_away_from_zero(
                cf_data_out.int_rt - (base_rate + fixed_adj_rate),
                rate_precision,
            )
        };
        let ftp_rates_out = format!(
            "{}|{}|{}|{}|{}{}|{}|{}\n",
            cf_data_out.acc_id,
            cf_data_out.FTP_Rate,
            basecurve,
            base_rate,
            adj_lock_str,
            cf_data_out.int_rt,
            fix_spread,
            method,
        );
        write!(spread_writer, "{}", ftp_rates_out).expect("Error while write spread file.");

        one_acc_op.fx_spread = fix_spread;
        one_acc_op.var_spread = variable_adj_rate;
        out_str = format!(
            "{}{}|{}|{}|",
            out_str,
            fix_spread,
            variable_adj_rate,
            half_away_from_zero(fix_spread + variable_adj_rate, rate_precision)
        );
    } else {
        out_str = format!(
            "{}{}|{}|{}|",
            out_str, DEFAULT_FLOAT, DEFAULT_FLOAT, DEFAULT_FLOAT
        );
    }

    (cf_data_out, one_acc_op, cf_str)
}
