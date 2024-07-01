use math::round::half_away_from_zero;
use rbdate::*;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use stamp_ftp::amb_file_reader::AmbVal;
use stamp_ftp::bm_reader::{BmKey, IntermediateBmPoints};
use stamp_ftp::cfinput::AccFieldNames;
use stamp_ftp::one_acc_view::One_acc_view;
use stamp_ftp::read_adjustments::Adj_key;
use stamp_ftp::CFout::AccountWithCashflows;
use stamp_ftp::CFout::Cashflow;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;

mod ftp_calc_acc_level;
mod ftp_calc_cflevel;
mod ftp_calc_lock;
mod ftp_calc_rep_lock;

pub fn calc_ftp(
    mut acc_data_in: &mut AccountWithCFs,
    mut cf_data_out: AccountWithCashflows,
    inputfieldnames: &AccFieldNames,
    method: i32,
    basecurve: i32,
    fix_lst_adjustments: Vec<i32>,
    var_lst_adjustments: Vec<i32>,
    basecurve_file_path: String,
    log: &Logger,
    _diag_log: &Logger,
    ftprunid: i64,
    from_date: &NaiveDate,
    to_date: &NaiveDate,
    out_path: &str,
    ftp_rates: &mut HashMap<String, Vec<f64>>,
    lock_adjs: &HashMap<i32, String>,
    adj_rates: &HashMap<Adj_key, f64>,
    avg_bal: &HashMap<String, AmbVal>,
    ftp_rates_file_path: &str,
    fixed_adj_count: i32,
    var_adj_count: i32,
    mut saved_bm_rates: &mut HashMap<BmKey, Vec<IntermediateBmPoints>>,
    mut spread_writer: &mut BufWriter<File>,
    rate_precision: i8,
    bal_precision: i8,
) -> (AccountWithCashflows, One_acc_view, String, String) {
    let mut outstr: String = String::new();
    let mut cf_out_latest: String = String::new();
    let mut one_acc_op_new = One_acc_view::new();
    let mut adj_string = String::new();
    match method {
        1001 => {
            //Matched Term1 Method implementation
            //cpd : Last Reprice Date, TSD: Last Reprice Date, TED: Maturity Date
            let cpd_naive_date_time =
                NaiveDateTime::from_timestamp(cf_data_out.lst_repricing_dt, 0);
            let mut cpd = cpd_naive_date_time.date();
            let ted_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.mat_dt, 0);
            let ted = ted_naive_date_time.date();

            if cpd <= NaiveDate::from_ymd(1970, 01, 01) {
                let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.val_dt, 0);
                cpd = cpd_naive_date_time.date();
            }

            let (cf_data_out1, one_acc_op, cf_out, adj_str) = ftp_calc_acc_level::calc_ftp(
                &mut acc_data_in,
                cf_data_out,
                inputfieldnames,
                basecurve,
                fix_lst_adjustments,
                var_lst_adjustments,
                basecurve_file_path,
                log,
                ftprunid,
                from_date,
                to_date,
                cpd,
                cpd,
                ted,
                false,
                adj_rates,
                avg_bal,
                out_path,
                fixed_adj_count,
                var_adj_count,
                &mut saved_bm_rates,
                &mut spread_writer,
                rate_precision,
                bal_precision,
                method,
            );

            cf_data_out = cf_data_out1;
            cf_out_latest = cf_out;
            one_acc_op_new = one_acc_op;
            one_acc_op_new.method = "Matched Term1".to_string();
            one_acc_op_new.bc_as_on_rule = cf_data_out.lst_repricing_dt;
            one_acc_op_new.tenor_start_date_rule = cf_data_out.lst_repricing_dt;
            one_acc_op_new.tenor_end_date_rule = cf_data_out.mat_dt;
            adj_string = adj_str;
        }
        1002 => {
            //Matched Term2 Method implementation
            //cpd : Last Reprice Date, TSD: Last Reprice Date, TED: Next Reprice Date
            let cpd_naive_date_time =
                NaiveDateTime::from_timestamp(cf_data_out.lst_repricing_dt, 0);
            let mut cpd = cpd_naive_date_time.date();
            let ted_naive_date_time =
                NaiveDateTime::from_timestamp(cf_data_out.nxt_repricing_dt, 0);
            let mut ted = ted_naive_date_time.date();

            if cpd <= NaiveDate::from_ymd(1970, 01, 01) {
                let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.val_dt, 0);
                cpd = cpd_naive_date_time.date();
            }

            if ted <= NaiveDate::from_ymd(1970, 01, 01) {
                let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.mat_dt, 0);
                ted = cpd_naive_date_time.date();
            }

            let (cf_data_out1, one_acc_op, cf_out, adj_str) = ftp_calc_acc_level::calc_ftp(
                &mut acc_data_in,
                cf_data_out,
                inputfieldnames,
                basecurve,
                fix_lst_adjustments,
                var_lst_adjustments,
                basecurve_file_path,
                log,
                ftprunid,
                from_date,
                to_date,
                cpd,
                cpd,
                ted,
                false,
                adj_rates,
                avg_bal,
                out_path,
                fixed_adj_count,
                var_adj_count,
                &mut saved_bm_rates,
                &mut spread_writer,
                rate_precision,
                bal_precision,
                method,
            );
            cf_data_out = cf_data_out1;
            cf_out_latest = cf_out;
            one_acc_op_new = one_acc_op;
            one_acc_op_new.method = "Matched Term2".to_string();
            one_acc_op_new.bc_as_on_rule = cf_data_out.lst_repricing_dt;
            one_acc_op_new.tenor_start_date_rule = cf_data_out.lst_repricing_dt;
            one_acc_op_new.tenor_end_date_rule = cf_data_out.nxt_repricing_dt;
            adj_string = adj_str;
        }
        1003 => {
            //Matched Term3 method
            //cpd : Start Date, TSD: Start Date, TED: Maturity Date
            let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.val_dt, 0);
            let cpd = cpd_naive_date_time.date();
            let ted_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.mat_dt, 0);
            let ted = ted_naive_date_time.date();

            let (cf_data_out1, one_acc_op, cf_out, adj_str) = ftp_calc_acc_level::calc_ftp(
                &mut acc_data_in,
                cf_data_out,
                inputfieldnames,
                basecurve,
                fix_lst_adjustments,
                var_lst_adjustments,
                basecurve_file_path,
                log,
                ftprunid,
                from_date,
                to_date,
                cpd,
                cpd,
                ted,
                false,
                adj_rates,
                avg_bal,
                out_path,
                fixed_adj_count,
                var_adj_count,
                &mut saved_bm_rates,
                &mut spread_writer,
                rate_precision,
                bal_precision,
                method,
            );

            cf_data_out = cf_data_out1;
            cf_out_latest = cf_out;
            one_acc_op_new = one_acc_op;
            one_acc_op_new.method = "Matched Term3".to_string();
            one_acc_op_new.bc_as_on_rule = cf_data_out.val_dt;
            one_acc_op_new.tenor_start_date_rule = cf_data_out.val_dt;
            one_acc_op_new.tenor_end_date_rule = cf_data_out.mat_dt;
            adj_string = adj_str;
        }
        1011 => {
            //Cashflow1 Method
            //cpd: Last reprice date , TSD: Last Reprice date, TED: Cashflow date
            let cpd_naive_date_time =
                NaiveDateTime::from_timestamp(cf_data_out.lst_repricing_dt, 0);
            let mut cpd = cpd_naive_date_time.date();

            if cpd <= NaiveDate::from_ymd(1970, 01, 01) {
                let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.val_dt, 0);
                cpd = cpd_naive_date_time.date();
            }

            let (cf_data_out1, one_acc_op, cf_out, adj_str) = ftp_calc_cflevel::calc_ftp(
                &mut acc_data_in,
                cf_data_out,
                inputfieldnames,
                basecurve,
                fix_lst_adjustments,
                var_lst_adjustments,
                basecurve_file_path,
                log,
                ftprunid,
                from_date,
                to_date,
                cpd,
                cpd,
                false,
                ftp_rates_file_path,
                adj_rates,
                avg_bal,
                out_path,
                fixed_adj_count,
                var_adj_count,
                &mut saved_bm_rates,
                &mut spread_writer,
                rate_precision,
                bal_precision,
                method,
            );
            cf_data_out = cf_data_out1;
            cf_out_latest = cf_out;
            one_acc_op_new = one_acc_op;
            one_acc_op_new.method = "Cashflow 1".to_string();
            one_acc_op_new.bc_as_on_rule = cf_data_out.lst_repricing_dt;
            one_acc_op_new.tenor_start_date_rule = cf_data_out.lst_repricing_dt;
            adj_string = adj_str;
        }
        1012 => {
            //Cashflow2 Method
            //cpd: Start Date , TSD: Start Date, TED: Cashflow date
            let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.val_dt, 0);
            let cpd = cpd_naive_date_time.date();

            let (cf_data_out1, one_acc_op, cf_out, adj_str) = ftp_calc_cflevel::calc_ftp(
                &mut acc_data_in,
                cf_data_out,
                inputfieldnames,
                basecurve,
                fix_lst_adjustments,
                var_lst_adjustments,
                basecurve_file_path,
                log,
                ftprunid,
                from_date,
                to_date,
                cpd,
                cpd,
                false,
                ftp_rates_file_path,
                adj_rates,
                avg_bal,
                out_path,
                fixed_adj_count,
                var_adj_count,
                &mut saved_bm_rates,
                &mut spread_writer,
                rate_precision,
                bal_precision,
                method,
            );
            cf_data_out = cf_data_out1;
            cf_out_latest = cf_out;
            one_acc_op_new = one_acc_op;
            one_acc_op_new.method = "Cashflow 2".to_string();
            one_acc_op_new.bc_as_on_rule = cf_data_out.val_dt;
            one_acc_op_new.tenor_start_date_rule = cf_data_out.val_dt;
            adj_string = adj_str;
        }
        1021 => {
            //Assign Rate1 Method
            //cpd: AsOn Date , TSD: AsOn Date, TED: Cashflow date

            let (cf_data_out1, one_acc_op, cf_out, adj_str) = ftp_calc_cflevel::calc_ftp(
                &mut acc_data_in,
                cf_data_out,
                inputfieldnames,
                basecurve,
                fix_lst_adjustments,
                var_lst_adjustments,
                basecurve_file_path,
                log,
                ftprunid,
                from_date,
                to_date,
                *from_date,
                *from_date,
                false,
                ftp_rates_file_path,
                adj_rates,
                avg_bal,
                out_path,
                fixed_adj_count,
                var_adj_count,
                &mut saved_bm_rates,
                &mut spread_writer,
                rate_precision,
                bal_precision,
                method,
            );
            cf_data_out = cf_data_out1;
            cf_out_latest = cf_out;
            one_acc_op_new = one_acc_op;
            one_acc_op_new.method = "Assign Rate 1".to_string();
            one_acc_op_new.bc_as_on_rule = timestamp(*from_date);
            one_acc_op_new.tenor_start_date_rule = timestamp(*from_date);
            adj_string = adj_str;
        }
        1022 => {
            //Assign Rate2 Method
            //cpd: AsOn Date , TSD: AsOn Date, TED: Maturity Date

            let ted_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.mat_dt, 0);
            let ted = ted_naive_date_time.date();

            let (cf_data_out1, one_acc_op, cf_out, adj_str) = ftp_calc_acc_level::calc_ftp(
                &mut acc_data_in,
                cf_data_out,
                inputfieldnames,
                basecurve,
                fix_lst_adjustments,
                var_lst_adjustments,
                basecurve_file_path,
                log,
                ftprunid,
                from_date,
                to_date,
                *from_date,
                *from_date,
                ted,
                false,
                adj_rates,
                avg_bal,
                out_path,
                fixed_adj_count,
                var_adj_count,
                &mut saved_bm_rates,
                &mut spread_writer,
                rate_precision,
                bal_precision,
                method,
            );
            cf_data_out = cf_data_out1;
            cf_out_latest = cf_out;
            one_acc_op_new = one_acc_op;
            one_acc_op_new.method = "Assign Rate 2".to_string();
            one_acc_op_new.bc_as_on_rule = timestamp(*from_date);
            one_acc_op_new.tenor_start_date_rule = timestamp(*from_date);
            one_acc_op_new.tenor_end_date_rule = cf_data_out.mat_dt;
            adj_string = adj_str;
        }
        1031 => {
            //Assign Rate with Lock1 Method
            //cpd: AsOn Date , TSD: AsOn Date, TED: Cashflow date
            let def_rates = vec![0.0; 17];
            let rates = ftp_rates.get(&cf_data_out.reference).unwrap_or(&def_rates);
            let prev_method = if rates[16] == 0.0 {
                method
            } else {
                rates[16] as i32
            };
            if let (Some(rates), _) = (ftp_rates.get(&cf_data_out.reference), prev_method == method)
            {
                let ted_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.mat_dt, 0);
                let ted = ted_naive_date_time.date();

                let (cf_data_out1, one_acc_op, adj_str) = ftp_calc_lock::calc_ftp(
                    &mut acc_data_in,
                    cf_data_out,
                    inputfieldnames,
                    rates,
                    lock_adjs,
                    log,
                    ftprunid,
                    from_date,
                    to_date,
                    out_path,
                    *from_date,
                    ted,
                    var_lst_adjustments,
                    adj_rates,
                    avg_bal,
                    fixed_adj_count,
                    var_adj_count,
                    rate_precision,
                    bal_precision,
                );

                cf_data_out = cf_data_out1;
                one_acc_op_new = one_acc_op;
                one_acc_op_new.method = "Assign Rate with lock 1".to_string();
                one_acc_op_new.tenor_start_date_rule = timestamp(*from_date);
                one_acc_op_new.tenor_end_date_rule = timestamp(*from_date);
                adj_string = adj_str;
            } else {
                let (cf_data_out1, one_acc_op, cf_out, adj_str) = ftp_calc_cflevel::calc_ftp(
                    &mut acc_data_in,
                    cf_data_out,
                    inputfieldnames,
                    basecurve,
                    fix_lst_adjustments,
                    var_lst_adjustments,
                    basecurve_file_path,
                    log,
                    ftprunid,
                    from_date,
                    to_date,
                    *from_date,
                    *from_date,
                    true,
                    ftp_rates_file_path,
                    adj_rates,
                    avg_bal,
                    out_path,
                    fixed_adj_count,
                    var_adj_count,
                    &mut saved_bm_rates,
                    &mut spread_writer,
                    rate_precision,
                    bal_precision,
                    method,
                );
                cf_data_out = cf_data_out1;
                cf_out_latest = cf_out;
                one_acc_op_new = one_acc_op;
                one_acc_op_new.method = "Assign Rate with lock 1".to_string();
                one_acc_op_new.bc_as_on_rule = timestamp(*from_date);
                one_acc_op_new.tenor_start_date_rule = timestamp(*from_date);
                adj_string = adj_str;
            }
        }
        1032 => {
            //Assign Rate with Lock2 Method
            //CPD: AsOn Date , TSD: AsOn Date, TED: Maturity date
            let def_rates = vec![0.0; 17];
            let rates = ftp_rates.get(&cf_data_out.reference).unwrap_or(&def_rates);
            let prev_method = if rates[16] == 0.0 {
                method
            } else {
                rates[16] as i32
            };
            if let (Some(rates), _) = (ftp_rates.get(&cf_data_out.reference), prev_method == method)
            {
                let (cf_data_out1, one_acc_op, adj_str) = ftp_calc_lock::calc_ftp(
                    &mut acc_data_in,
                    cf_data_out,
                    inputfieldnames,
                    rates,
                    lock_adjs,
                    log,
                    ftprunid,
                    from_date,
                    to_date,
                    out_path,
                    *from_date,
                    *from_date,
                    var_lst_adjustments,
                    adj_rates,
                    avg_bal,
                    fixed_adj_count,
                    var_adj_count,
                    rate_precision,
                    bal_precision,
                );
                cf_data_out = cf_data_out1;
                one_acc_op_new = one_acc_op;
                one_acc_op_new.method = "Assign Rate with lock 2".to_string();
                one_acc_op_new.tenor_start_date_rule = timestamp(*from_date);
                one_acc_op_new.tenor_end_date_rule = timestamp(*from_date);
                adj_string = adj_str;
            } else {
                let ted_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.mat_dt, 0);
                let ted = ted_naive_date_time.date();

                let (cf_data_out1, one_acc_op, cf_out, adj_str) = ftp_calc_acc_level::calc_ftp(
                    &mut acc_data_in,
                    cf_data_out,
                    inputfieldnames,
                    basecurve,
                    fix_lst_adjustments,
                    var_lst_adjustments,
                    basecurve_file_path,
                    log,
                    ftprunid,
                    from_date,
                    to_date,
                    *from_date,
                    *from_date,
                    ted,
                    true,
                    adj_rates,
                    avg_bal,
                    out_path,
                    fixed_adj_count,
                    var_adj_count,
                    &mut saved_bm_rates,
                    &mut spread_writer,
                    rate_precision,
                    bal_precision,
                    method,
                );
                cf_data_out = cf_data_out1;
                cf_out_latest = cf_out;
                one_acc_op_new = one_acc_op;
                one_acc_op_new.method = "Assign Rate with lock 2".to_string();
                one_acc_op_new.bc_as_on_rule = timestamp(*from_date);
                one_acc_op_new.tenor_start_date_rule = timestamp(*from_date);
                one_acc_op_new.tenor_end_date_rule = cf_data_out.mat_dt;
                adj_string = adj_str;
            }
        }
        1033 => {
            //Assign Rate with Lock3 Method
            //CPD: Start Date , TSD: Start Date, TED: Maturity date
            let def_rates = vec![0.0; 17];
            let rates = ftp_rates.get(&cf_data_out.reference).unwrap_or(&def_rates);
            let prev_method = if rates[16] == 0.0 {
                method
            } else {
                rates[16] as i32
            };
            if let (Some(rates), _) = (ftp_rates.get(&cf_data_out.reference), prev_method == method)
            {
                let (cf_data_out1, one_acc_op, adj_str) = ftp_calc_lock::calc_ftp(
                    &mut acc_data_in,
                    cf_data_out,
                    inputfieldnames,
                    rates,
                    lock_adjs,
                    log,
                    ftprunid,
                    from_date,
                    to_date,
                    out_path,
                    *from_date,
                    *from_date,
                    var_lst_adjustments,
                    adj_rates,
                    avg_bal,
                    fixed_adj_count,
                    var_adj_count,
                    rate_precision,
                    bal_precision,
                );
                cf_data_out = cf_data_out1;
                one_acc_op_new = one_acc_op;
                one_acc_op_new.method = "Assign Rate with lock 3".to_string();
                one_acc_op_new.tenor_start_date_rule = timestamp(*from_date);
                one_acc_op_new.tenor_end_date_rule = timestamp(*from_date);
                adj_string = adj_str;
            } else {
                let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.val_dt, 0);
                let cpd = cpd_naive_date_time.date();
                let ted_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.mat_dt, 0);
                let ted = ted_naive_date_time.date();

                let (cf_data_out1, one_acc_op, cf_out, adj_str) = ftp_calc_acc_level::calc_ftp(
                    &mut acc_data_in,
                    cf_data_out,
                    inputfieldnames,
                    basecurve,
                    fix_lst_adjustments,
                    var_lst_adjustments,
                    basecurve_file_path,
                    log,
                    ftprunid,
                    from_date,
                    to_date,
                    *from_date,
                    cpd,
                    ted,
                    true,
                    adj_rates,
                    avg_bal,
                    out_path,
                    fixed_adj_count,
                    var_adj_count,
                    &mut saved_bm_rates,
                    &mut spread_writer,
                    rate_precision,
                    bal_precision,
                    method,
                );
                cf_data_out = cf_data_out1;
                cf_out_latest = cf_out;
                one_acc_op_new = one_acc_op;
                one_acc_op_new.method = "Assign Rate with lock 3".to_string();
                one_acc_op_new.bc_as_on_rule = timestamp(*from_date);
                one_acc_op_new.tenor_start_date_rule = cf_data_out.val_dt;
                one_acc_op_new.tenor_end_date_rule = cf_data_out.mat_dt;
                adj_string = adj_str;
            }
        }
        1034 => {
            //Reprice Term with lock 1 Method
            //CPD: Last Reprice Date , TSD: Last Reprice Date, TED: Next Reprice date
            let def_rates = vec![0.0; 17];
            let rates = ftp_rates.get(&cf_data_out.reference).unwrap_or(&def_rates);
            let prev_method = if rates[16] == 0.0 {
                method
            } else {
                rates[16] as i32
            };
            if let (Some(rates), _) = (ftp_rates.get(&cf_data_out.reference), prev_method == method)
            {
                let (cf_data_out1, one_acc_op, adj_str) = ftp_calc_lock::calc_ftp(
                    &mut acc_data_in,
                    cf_data_out,
                    inputfieldnames,
                    rates,
                    lock_adjs,
                    log,
                    ftprunid,
                    from_date,
                    to_date,
                    out_path,
                    *from_date,
                    *from_date,
                    var_lst_adjustments,
                    adj_rates,
                    avg_bal,
                    fixed_adj_count,
                    var_adj_count,
                    rate_precision,
                    bal_precision,
                );
                cf_data_out = cf_data_out1;
                one_acc_op_new = one_acc_op;
                one_acc_op_new.method = "Reprice Term with lock 1".to_string();
                one_acc_op_new.tenor_start_date_rule = timestamp(*from_date);
                one_acc_op_new.tenor_end_date_rule = timestamp(*from_date);
                adj_string = adj_str;
            } else {
                let mut cpd = date_from_timestamp(cf_data_out.lst_repricing_dt);
                let mut ted = date_from_timestamp(cf_data_out.nxt_repricing_dt);

                if cpd <= NaiveDate::from_ymd(1970, 01, 01) {
                    cpd = date_from_timestamp(cf_data_out.val_dt);
                }

                if ted <= NaiveDate::from_ymd(1970, 01, 01) {
                    ted = date_from_timestamp(cf_data_out.mat_dt);
                }

                let (cf_data_out1, one_acc_op, cf_out, adj_str) = ftp_calc_acc_level::calc_ftp(
                    &mut acc_data_in,
                    cf_data_out,
                    inputfieldnames,
                    basecurve,
                    fix_lst_adjustments,
                    var_lst_adjustments,
                    basecurve_file_path,
                    log,
                    ftprunid,
                    from_date,
                    to_date,
                    cpd,
                    cpd,
                    ted,
                    true,
                    adj_rates,
                    avg_bal,
                    out_path,
                    fixed_adj_count,
                    var_adj_count,
                    &mut saved_bm_rates,
                    &mut spread_writer,
                    rate_precision,
                    bal_precision,
                    method,
                );
                cf_data_out = cf_data_out1;
                cf_out_latest = cf_out;
                one_acc_op_new = one_acc_op;
                one_acc_op_new.method = "Reprice Term with lock 1".to_string();
                one_acc_op_new.bc_as_on_rule = cf_data_out.lst_repricing_dt;
                one_acc_op_new.tenor_start_date_rule = cf_data_out.lst_repricing_dt;
                one_acc_op_new.tenor_end_date_rule = cf_data_out.nxt_repricing_dt;
                adj_string = adj_str;
            }
        }
        1036 => {
            //Reprice Term with lock 2 Method
            //CPD: Last Reprice Date , TSD: Last Reprice Date, TED: Next Reprice date
            let def_rates = vec![0.0; 17];
            let rates = ftp_rates.get(&cf_data_out.reference).unwrap_or(&def_rates);
            let prev_method = if rates[16] == 0.0 {
                method
            } else {
                rates[16] as i32
            };
            if let (Some(rates), _) = (ftp_rates.get(&cf_data_out.reference), prev_method == method)
            {
                let (cf_data_out1, one_acc_op, adj_str) = ftp_calc_rep_lock::calc_ftp(
                    &mut acc_data_in,
                    cf_data_out,
                    inputfieldnames,
                    rates,
                    lock_adjs,
                    log,
                    ftprunid,
                    from_date,
                    to_date,
                    out_path,
                    *from_date,
                    *from_date,
                    var_lst_adjustments,
                    adj_rates,
                    avg_bal,
                    fixed_adj_count,
                    var_adj_count,
                    rate_precision,
                    bal_precision,
                    &mut spread_writer,
                    method,
                );
                cf_data_out = cf_data_out1;
                one_acc_op_new = one_acc_op;
                one_acc_op_new.method = "Reprice Term with lock 2".to_string();
                one_acc_op_new.tenor_start_date_rule = timestamp(*from_date);
                one_acc_op_new.tenor_end_date_rule = timestamp(*from_date);
                adj_string = adj_str;
            } else {
                let mut cpd = date_from_timestamp(cf_data_out.lst_repricing_dt);
                let mut ted = date_from_timestamp(cf_data_out.nxt_repricing_dt);

                if cpd <= NaiveDate::from_ymd(1970, 01, 01) {
                    cpd = date_from_timestamp(cf_data_out.val_dt);
                }

                if ted <= NaiveDate::from_ymd(1970, 01, 01) {
                    ted = date_from_timestamp(cf_data_out.mat_dt);
                }

                let (cf_data_out1, one_acc_op, cf_out, adj_str) = ftp_calc_acc_level::calc_ftp(
                    &mut acc_data_in,
                    cf_data_out,
                    inputfieldnames,
                    basecurve,
                    fix_lst_adjustments,
                    var_lst_adjustments,
                    basecurve_file_path,
                    log,
                    ftprunid,
                    from_date,
                    to_date,
                    cpd,
                    cpd,
                    ted,
                    true,
                    adj_rates,
                    avg_bal,
                    out_path,
                    fixed_adj_count,
                    var_adj_count,
                    &mut saved_bm_rates,
                    &mut spread_writer,
                    rate_precision,
                    bal_precision,
                    method,
                );
                cf_data_out = cf_data_out1;
                cf_out_latest = cf_out;
                one_acc_op_new = one_acc_op;
                one_acc_op_new.method = "Reprice Term with lock 2".to_string();
                one_acc_op_new.bc_as_on_rule = cf_data_out.lst_repricing_dt;
                one_acc_op_new.tenor_start_date_rule = cf_data_out.lst_repricing_dt;
                one_acc_op_new.tenor_end_date_rule = cf_data_out.nxt_repricing_dt;
                adj_string = adj_str;
            }
        }
        1041 => {
            //Margin Method
            let mut cf_ftp = Vec::new();
            let norm_int_rt = half_away_from_zero(
                acc_data_in
                    .get_f64_for_key(&inputfieldnames.norm_int_rt)
                    .unwrap_or(0.0),
                rate_precision,
            );
            let mut _lst_out: Vec<String> = Vec::new();
            let mut total_balance = 0.0;
            let mut total_interest_ftp = 0.0;
            let mut total_ftp = 0.0;
            let mut ftp_rate = 0.0;

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

                total_balance += cf.principal_amount;
                total_interest_ftp += cf.interest_amount;
                total_ftp += cf_obj.base_rate_amount;

                let out_str = format!(
                    "{}|{}|{}|{}|{}|{}|{}|{}",
                    cf_data_out.gl,
                    NaiveDateTime::from_timestamp(cf.date, 0).date(),
                    cf.principal_amount,
                    cf.interest_amount,
                    cf_obj.base_rate_amount,
                    norm_int_rt,
                    norm_int_rt,
                    cf_obj.base_rate_amount
                );

                cf_ftp.push(cf_obj);

                _lst_out.push(out_str);
            }

            let out_str_total = format!(
                "{}|{}|{}|{}|{}",
                cf_data_out.gl,
                total_balance,
                total_interest_ftp,
                total_ftp,
                ftp_rate / total_balance
            );

            _lst_out.push(out_str_total);

            cf_data_out.cashflows = protobuf::RepeatedField::from_vec(cf_ftp);
        }
        _ => {
            //TODO: Handle undefined method here
        }
    }

    (cf_data_out, one_acc_op_new, cf_out_latest, adj_string)
}
