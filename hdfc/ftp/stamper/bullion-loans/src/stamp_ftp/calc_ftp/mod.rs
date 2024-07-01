use rbdate::timestamp;
use rbdate::NaiveDate;
use rbdate::NaiveDateTime;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use stamp_ftp::cfinput::AccFieldNames;
use stamp_ftp::read_adjustments::Adj_key;
use stamp_ftp::CFout::AccountWithCashflows;
use stamp_ftp::CFout::Cashflow;
use std::collections::HashMap;
mod ftp_calc_acc_level;
mod ftp_calc_cflevel;
mod ftp_calc_lock;
use macros;
use stamp_ftp::amb_file_reader::AmbVal;
use stamp_ftp::bm_reader::{BmKey, IntermediateBmPoints};
use stamp_ftp::one_acc_view::One_acc_view;
use statics::*;
use std::fs::File;
use std::io::BufWriter;

pub fn calc_ftp(
    mut acc_data_in: &mut AccountWithCFs,
    mut cf_data_out: AccountWithCashflows,
    inputfieldnames: &AccFieldNames,
    method: i32,
    basecurve: i32,
    lst_adjustments: Vec<i32>,
    basecurve_file_path: String,
    log: &Logger,
    ftprunid: i64,
    from_date: &NaiveDate,
    to_date: &NaiveDate,
    out_path: &str,
    ftp_rates: &mut HashMap<String, Vec<f64>>,
    lock_adjs: &HashMap<i32, String>,
    adj_rates: &HashMap<Adj_key, f64>,
    avg_bal: &HashMap<String, AmbVal>,
    ftp_rates_file_path: &str,
    mut saved_bm_rates: &mut HashMap<BmKey, Vec<IntermediateBmPoints>>,
    mut spread_writer: &mut BufWriter<File>,
    rate_precision: i8,
    bal_precision: i8,
) -> (AccountWithCashflows, One_acc_view, String) {
    let mut outstr: String = String::new();
    let mut cf_out_latest: String = String::new();
    let mut one_acc_op_new = One_acc_view::new();
    match method {
        1001 => {
            //Matched Term1 Method implementation
            //cpd : Last Reprice Date, TSD: Last Reprice Date, TED: Maturity Date
            let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.st_dt, 0);
            let mut cpd = cpd_naive_date_time.date();
            let ted_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.mat_dt, 0);
            let ted = ted_naive_date_time.date();

            if cpd <= NaiveDate::from_ymd(1970, 01, 01) {
                let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.st_dt, 0);
                cpd = cpd_naive_date_time.date();
            }

            let (cf_data_out1, one_acc_op, cf_out) = ftp_calc_acc_level::calc_ftp_cflevel(
                &mut acc_data_in,
                cf_data_out,
                inputfieldnames,
                basecurve,
                lst_adjustments,
                basecurve_file_path,
                log,
                from_date,
                to_date,
                cpd,
                cpd,
                ted,
                false,
                adj_rates,
                avg_bal,
                out_path,
                &mut saved_bm_rates,
                &mut spread_writer,
                rate_precision,
                bal_precision,
            );

            cf_data_out = cf_data_out1;
            cf_out_latest = cf_out;
            one_acc_op_new = one_acc_op;
            one_acc_op_new.method = "Matched Term1".to_string();
            one_acc_op_new.bc_as_on_rule = cf_data_out.st_dt;
            one_acc_op_new.tenor_start_date_rule = cf_data_out.st_dt;
            one_acc_op_new.tenor_end_date_rule = cf_data_out.mat_dt;
        }
        1002 => {
            //Matched Term2 Method implementation
            //cpd : Last Reprice Date, TSD: Last Reprice Date, TED: Next Reprice Date
            let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.st_dt, 0);
            let mut cpd = cpd_naive_date_time.date();
            let ted_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.st_dt, 0);
            let mut ted = ted_naive_date_time.date();

            if cpd <= NaiveDate::from_ymd(1970, 01, 01) {
                let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.st_dt, 0);
                cpd = cpd_naive_date_time.date();
            }

            if ted <= NaiveDate::from_ymd(1970, 01, 01) {
                let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.mat_dt, 0);
                ted = cpd_naive_date_time.date();
            }

            let (cf_data_out1, one_acc_op, cf_out) = ftp_calc_acc_level::calc_ftp_cflevel(
                &mut acc_data_in,
                cf_data_out,
                inputfieldnames,
                basecurve,
                lst_adjustments,
                basecurve_file_path,
                log,
                from_date,
                to_date,
                cpd,
                cpd,
                ted,
                false,
                adj_rates,
                avg_bal,
                out_path,
                &mut saved_bm_rates,
                &mut spread_writer,
                rate_precision,
                bal_precision,
            );
            cf_data_out = cf_data_out1;
            cf_out_latest = cf_out;
            one_acc_op_new = one_acc_op;
            one_acc_op_new.method = "Matched Term2".to_string();
            one_acc_op_new.bc_as_on_rule = cf_data_out.st_dt;
            one_acc_op_new.tenor_start_date_rule = cf_data_out.st_dt;
            one_acc_op_new.tenor_end_date_rule = cf_data_out.mat_dt;
        }
        1003 => {
            //Matched Term3 method
            //cpd : Start Date, TSD: Start Date, TED: Maturity Date
            let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.st_dt, 0);
            let cpd = cpd_naive_date_time.date();
            let ted_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.mat_dt, 0);
            let ted = ted_naive_date_time.date();

            let (cf_data_out1, one_acc_op, cf_out) = ftp_calc_acc_level::calc_ftp_cflevel(
                &mut acc_data_in,
                cf_data_out,
                inputfieldnames,
                basecurve,
                lst_adjustments,
                basecurve_file_path,
                log,
                from_date,
                to_date,
                cpd,
                cpd,
                ted,
                false,
                adj_rates,
                avg_bal,
                out_path,
                &mut saved_bm_rates,
                &mut spread_writer,
                rate_precision,
                bal_precision,
            );

            cf_data_out = cf_data_out1;
            cf_out_latest = cf_out;
            one_acc_op_new = one_acc_op;
            one_acc_op_new.method = "Matched Term3".to_string();
            one_acc_op_new.bc_as_on_rule = cf_data_out.st_dt;
            one_acc_op_new.tenor_start_date_rule = cf_data_out.st_dt;
            one_acc_op_new.tenor_end_date_rule = cf_data_out.mat_dt;
        }
        1011 => {
            //Cashflow1 Method
            //cpd: Last reprice date , TSD: Last Reprice date, TED: Cashflow date
            let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.st_dt, 0);
            let mut cpd = cpd_naive_date_time.date();

            if cpd <= NaiveDate::from_ymd(1970, 01, 01) {
                let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.st_dt, 0);
                cpd = cpd_naive_date_time.date();
            }

            let (cf_data_out1, one_acc_op, cf_out) = ftp_calc_cflevel::calc_ftp_cflevel(
                &mut acc_data_in,
                cf_data_out,
                inputfieldnames,
                basecurve,
                lst_adjustments,
                basecurve_file_path,
                log,
                from_date,
                to_date,
                cpd,
                cpd,
                false,
                adj_rates,
                avg_bal,
                out_path,
                &mut saved_bm_rates,
                &mut spread_writer,
                rate_precision,
                bal_precision,
            );
            cf_data_out = cf_data_out1;
            cf_out_latest = cf_out;
            one_acc_op_new = one_acc_op;
            one_acc_op_new.method = "Cashflow 1".to_string();
            one_acc_op_new.bc_as_on_rule = cf_data_out.st_dt;
            one_acc_op_new.tenor_start_date_rule = cf_data_out.st_dt;
        }
        1012 => {
            //Cashflow2 Method
            //cpd: Start Date , TSD: Start Date, TED: Cashflow date
            let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.st_dt, 0);
            let cpd = cpd_naive_date_time.date();

            let (cf_data_out1, one_acc_op, cf_out) = ftp_calc_cflevel::calc_ftp_cflevel(
                &mut acc_data_in,
                cf_data_out,
                inputfieldnames,
                basecurve,
                lst_adjustments,
                basecurve_file_path,
                log,
                from_date,
                to_date,
                cpd,
                cpd,
                false,
                adj_rates,
                avg_bal,
                out_path,
                &mut saved_bm_rates,
                &mut spread_writer,
                rate_precision,
                bal_precision,
            );
            cf_data_out = cf_data_out1;
            cf_out_latest = cf_out;
            one_acc_op_new = one_acc_op;
            one_acc_op_new.method = "Cashflow 2".to_string();
            one_acc_op_new.bc_as_on_rule = cf_data_out.st_dt;
            one_acc_op_new.tenor_start_date_rule = cf_data_out.st_dt;
        }
        1021 => {
            //Assign Rate1 Method
            //cpd: AsOn Date , TSD: AsOn Date, TED: Cashflow date

            let (cf_data_out1, one_acc_op, cf_out) = ftp_calc_cflevel::calc_ftp_cflevel(
                &mut acc_data_in,
                cf_data_out,
                inputfieldnames,
                basecurve,
                lst_adjustments,
                basecurve_file_path,
                log,
                from_date,
                to_date,
                *from_date,
                *from_date,
                false,
                adj_rates,
                avg_bal,
                out_path,
                &mut saved_bm_rates,
                &mut spread_writer,
                rate_precision,
                bal_precision,
            );
            cf_data_out = cf_data_out1;
            cf_out_latest = cf_out;
            one_acc_op_new = one_acc_op;
            one_acc_op_new.method = "Assign Rate 1".to_string();
            one_acc_op_new.bc_as_on_rule = timestamp(*from_date);
            one_acc_op_new.tenor_start_date_rule = timestamp(*from_date);
        }
        1022 => {
            //Assign Rate2 Method
            //cpd: AsOn Date , TSD: AsOn Date, TED: Maturity Date

            let ted_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.mat_dt, 0);
            let ted = ted_naive_date_time.date();

            let (cf_data_out1, one_acc_op, cf_out) = ftp_calc_acc_level::calc_ftp_cflevel(
                &mut acc_data_in,
                cf_data_out,
                inputfieldnames,
                basecurve,
                lst_adjustments,
                basecurve_file_path,
                log,
                from_date,
                to_date,
                *from_date,
                *from_date,
                ted,
                false,
                adj_rates,
                avg_bal,
                out_path,
                &mut saved_bm_rates,
                &mut spread_writer,
                rate_precision,
                bal_precision,
            );
            cf_data_out = cf_data_out1;
            cf_out_latest = cf_out;
            one_acc_op_new = one_acc_op;
            one_acc_op_new.method = "Assign Rate 2".to_string();
            one_acc_op_new.bc_as_on_rule = timestamp(*from_date);
            one_acc_op_new.tenor_start_date_rule = timestamp(*from_date);
            one_acc_op_new.tenor_end_date_rule = cf_data_out.mat_dt;
        }
        1031 => {
            //Assign Rate with Lock1 Method
            //cpd: AsOn Date , TSD: AsOn Date, TED: Cashflow date
            if !(ftp_rates.is_empty()) && ftp_rates.contains_key(&cf_data_out.acc_no) {
                let rates = ftp_rates.get(&cf_data_out.acc_no).unwrap();
                let ted_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.mat_dt, 0);
                let ted = ted_naive_date_time.date();

                let (cf_data_out1, one_acc_op) = ftp_calc_lock::calc_ftp_lock(
                    &mut acc_data_in,
                    cf_data_out,
                    inputfieldnames,
                    rates,
                    lock_adjs,
                    log,
                    from_date,
                    to_date,
                    *from_date,
                    ted,
                    avg_bal,
                    rate_precision,
                    bal_precision,
                );

                cf_data_out = cf_data_out1;
                one_acc_op_new = one_acc_op;
                one_acc_op_new.method = "Assign Rate with lock 1".to_string();
                one_acc_op_new.tenor_start_date_rule = timestamp(*from_date);
                one_acc_op_new.tenor_end_date_rule = cf_data_out.mat_dt;
            } else {
                let (cf_data_out1, one_acc_op, cf_out) = ftp_calc_cflevel::calc_ftp_cflevel(
                    &mut acc_data_in,
                    cf_data_out,
                    inputfieldnames,
                    basecurve,
                    lst_adjustments,
                    basecurve_file_path,
                    log,
                    from_date,
                    to_date,
                    *from_date,
                    *from_date,
                    true,
                    adj_rates,
                    avg_bal,
                    out_path,
                    &mut saved_bm_rates,
                    &mut spread_writer,
                    rate_precision,
                    bal_precision,
                );
                cf_data_out = cf_data_out1;
                cf_out_latest = cf_out;
                one_acc_op_new = one_acc_op;
                one_acc_op_new.method = "Assign Rate with lock 1".to_string();
                one_acc_op_new.bc_as_on_rule = timestamp(*from_date);
                one_acc_op_new.tenor_start_date_rule = timestamp(*from_date);
            }
        }
        1032 => {
            //Assign Rate with Lock2 Method
            //CPD: AsOn Date , TSD: AsOn Date, TED: Maturity date
            if ftp_rates.contains_key(&cf_data_out.acc_no) {
                let rates = ftp_rates.get(&cf_data_out.acc_no).unwrap();

                let (cf_data_out1, one_acc_op) = ftp_calc_lock::calc_ftp_lock(
                    &mut acc_data_in,
                    cf_data_out,
                    inputfieldnames,
                    rates,
                    lock_adjs,
                    log,
                    from_date,
                    to_date,
                    *from_date,
                    *from_date,
                    avg_bal,
                    rate_precision,
                    bal_precision,
                );
                cf_data_out = cf_data_out1;
                one_acc_op_new = one_acc_op;
                one_acc_op_new.method = "Assign Rate with lock 2".to_string();
                one_acc_op_new.tenor_start_date_rule = timestamp(*from_date);
                one_acc_op_new.tenor_end_date_rule = timestamp(*from_date);
            } else {
                let ted_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.mat_dt, 0);
                let ted = ted_naive_date_time.date();

                let (cf_data_out1, one_acc_op, cf_out) = ftp_calc_acc_level::calc_ftp_cflevel(
                    &mut acc_data_in,
                    cf_data_out,
                    inputfieldnames,
                    basecurve,
                    lst_adjustments,
                    basecurve_file_path,
                    log,
                    from_date,
                    to_date,
                    *from_date,
                    *from_date,
                    ted,
                    true,
                    adj_rates,
                    avg_bal,
                    out_path,
                    &mut saved_bm_rates,
                    &mut spread_writer,
                    rate_precision,
                    bal_precision,
                );
                cf_data_out = cf_data_out1;
                cf_out_latest = cf_out;
                one_acc_op_new = one_acc_op;
                one_acc_op_new.method = "Assign Rate with lock 2".to_string();
                one_acc_op_new.bc_as_on_rule = timestamp(*from_date);
                one_acc_op_new.tenor_start_date_rule = timestamp(*from_date);
                one_acc_op_new.tenor_end_date_rule = cf_data_out.mat_dt;
            }
        }
        1033 => {
            //Assign Rate with Lock3 Method
            //CPD: Start Date , TSD: Start Date, TED: Maturity date
            if !(ftp_rates.is_empty()) && ftp_rates.contains_key(&cf_data_out.acc_no) {
                let rates = ftp_rates.get(&cf_data_out.acc_no).unwrap();

                let (cf_data_out1, one_acc_op) = ftp_calc_lock::calc_ftp_lock(
                    &mut acc_data_in,
                    cf_data_out,
                    inputfieldnames,
                    rates,
                    lock_adjs,
                    log,
                    from_date,
                    to_date,
                    *from_date,
                    *from_date,
                    avg_bal,
                    rate_precision,
                    bal_precision,
                );
                cf_data_out = cf_data_out1;
                one_acc_op_new = one_acc_op;
                one_acc_op_new.method = "Assign Rate with lock 3".to_string();
                one_acc_op_new.tenor_start_date_rule = timestamp(*from_date);
                one_acc_op_new.tenor_end_date_rule = timestamp(*from_date);
            } else {
                let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.st_dt, 0);
                let cpd = cpd_naive_date_time.date();
                let ted_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.mat_dt, 0);
                let ted = ted_naive_date_time.date();

                let (cf_data_out1, one_acc_op, cf_out) = ftp_calc_acc_level::calc_ftp_cflevel(
                    &mut acc_data_in,
                    cf_data_out,
                    inputfieldnames,
                    basecurve,
                    lst_adjustments,
                    basecurve_file_path,
                    log,
                    from_date,
                    to_date,
                    *from_date,
                    cpd,
                    ted,
                    true,
                    adj_rates,
                    avg_bal,
                    out_path,
                    &mut saved_bm_rates,
                    &mut spread_writer,
                    rate_precision,
                    bal_precision,
                );
                cf_data_out = cf_data_out1;
                cf_out_latest = cf_out;
                one_acc_op_new = one_acc_op;
                one_acc_op_new.method = "Assign Rate with lock 3".to_string();
                one_acc_op_new.bc_as_on_rule = timestamp(*from_date);
                one_acc_op_new.tenor_start_date_rule = cf_data_out.st_dt;
                one_acc_op_new.tenor_end_date_rule = cf_data_out.mat_dt;
            }
        }
        1034 => {
            //Reprice Term with lock Method
            //CPD: Last Reprice Date , TSD: Last Reprice Date, TED: Next Reprice date
            if !(ftp_rates.is_empty()) && ftp_rates.contains_key(&cf_data_out.acc_no) {
                let rates = ftp_rates.get(&cf_data_out.acc_no).unwrap();

                let (cf_data_out1, one_acc_op) = ftp_calc_lock::calc_ftp_lock(
                    &mut acc_data_in,
                    cf_data_out,
                    inputfieldnames,
                    rates,
                    lock_adjs,
                    log,
                    from_date,
                    to_date,
                    *from_date,
                    *from_date,
                    avg_bal,
                    rate_precision,
                    bal_precision,
                );
                cf_data_out = cf_data_out1;
                one_acc_op_new = one_acc_op;
                one_acc_op_new.method = "Reprice Term with lock".to_string();
                one_acc_op_new.tenor_start_date_rule = timestamp(*from_date);
                one_acc_op_new.tenor_end_date_rule = timestamp(*from_date);
            } else {
                let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.st_dt, 0);
                let mut cpd = cpd_naive_date_time.date();
                let ted_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.mat_dt, 0);
                let mut ted = ted_naive_date_time.date();

                if cpd <= NaiveDate::from_ymd(1970, 01, 01) {
                    let naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.st_dt, 0);
                    cpd = naive_date_time.date();
                }

                if ted <= NaiveDate::from_ymd(1970, 01, 01) {
                    let naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.mat_dt, 0);
                    ted = naive_date_time.date();
                }

                let (cf_data_out1, one_acc_op, cf_out) = ftp_calc_acc_level::calc_ftp_cflevel(
                    &mut acc_data_in,
                    cf_data_out,
                    inputfieldnames,
                    basecurve,
                    lst_adjustments,
                    basecurve_file_path,
                    log,
                    from_date,
                    to_date,
                    cpd,
                    cpd,
                    ted,
                    true,
                    adj_rates,
                    avg_bal,
                    out_path,
                    &mut saved_bm_rates,
                    &mut spread_writer,
                    rate_precision,
                    bal_precision,
                );
                cf_data_out = cf_data_out1;
                cf_out_latest = cf_out;
                one_acc_op_new = one_acc_op;
                one_acc_op_new.method = "Reprice Term with lock".to_string();
                one_acc_op_new.bc_as_on_rule = cf_data_out.st_dt;
                one_acc_op_new.tenor_start_date_rule = cf_data_out.st_dt;
                one_acc_op_new.tenor_end_date_rule = cf_data_out.mat_dt;
            }
        }
        1041 => {
            //Margin Method
            let mut cf_ftp = Vec::new();
            let norm_int_rt = acc_data_in
                .get_f64_for_key(&inputfieldnames.int_rt)
                .unwrap();
            let mut _lst_out: Vec<String> = Vec::new();
            let mut _total_balance = 0.0;
            let mut _total_interest_ftp = 0.0;
            let mut _total_ftp = 0.0;
            let mut _ftp_rate = 0.0;
            let mut adj_str: String = String::new();
            let mut adj_rates_lock = vec![DEFAULT_FLOAT; 6];
            let run_duration = rbdate::num_days_start_to_end(*from_date, *to_date) + 1;
            let max_days_in_year = rbdate::num_days_start_to_end(
                *to_date,
                rbdate::increment_date_by_months(*to_date, (12) as u16),
            );

            for cf in acc_data_in
                .remove_cfs_for_key(&inputfieldnames.cashflows)
                .expect("fail")
                .iter_mut()
            {
                let mut cf_obj = Cashflow::new();
                cf_obj.interest_amount = cf.interest_amount;
                cf_obj.principal_amount = cf.principal_amount;
                cf_obj.date = cf.date;
                cf_obj.base_rate = norm_int_rt;
                cf_obj.base_rate_amount = (cf.principal_amount * norm_int_rt) / 36500.00;

                _total_balance += cf.principal_amount;
                _total_interest_ftp += cf.interest_amount;
                _total_ftp += cf_obj.base_rate_amount;

                cf_ftp.push(cf_obj);
            }

            for i in 0..lst_adjustments.len() {
                let adj_key = Adj_key::new(cf_data_out.st_dt, lst_adjustments[i]);
                let adj_rate = match adj_rates.get(&adj_key) {
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
                };

                adj_str.push_str(&format!("{}|{}|", lst_adjustments[i], adj_rate));
                cf_data_out.FTP_Rate += adj_rate;
                adj_rates_lock[i] = adj_rate;
            }

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

            one_acc_op_new.average_balance = average_balance.avg_bal;
            one_acc_op_new.int_rate = cf_data_out.int_rt;
            one_acc_op_new.accr_int = average_balance.int_amt;
            one_acc_op_new.final_ftp_rate = cf_data_out.FTP_Rate + cf_data_out.int_rt;
            one_acc_op_new.final_ftp_amt = (average_balance.avg_bal
                * (cf_data_out.FTP_Rate + cf_data_out.int_rt)
                * run_duration as f64)
                / (max_days_in_year as f64 * 100.0);
            one_acc_op_new.rate_curve = basecurve.to_string();
            one_acc_op_new.base_rate = cf_data_out.int_rt;
            one_acc_op_new.adj1 = adj_rates_lock[0];
            one_acc_op_new.adj2 = adj_rates_lock[1];
            one_acc_op_new.adj3 = adj_rates_lock[2];
            one_acc_op_new.adj4 = adj_rates_lock[3];
            one_acc_op_new.adj5 = adj_rates_lock[4];
            one_acc_op_new.adj6 = adj_rates_lock[5];
        }
        _ => {
            //TODO: Handle undefined method here
        }
    }

    (cf_data_out, one_acc_op_new, cf_out_latest)
}
