use rbdate::*;
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
use super::amb_file_reader::AvgBalances;
use math::round::half_away_from_zero;
use stamp_ftp::bm_reader::{BmKey, IntermediateBmPoints};
use stamp_ftp::one_acc_view::One_acc_view;
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
    avg_bal: &HashMap<String, AvgBalances>,
    ftp_rates_file_path: &str,
    mut saved_bm_rates: &mut HashMap<BmKey, Vec<IntermediateBmPoints>>,
    mut spread_writer: &mut BufWriter<File>,
    rate_precision: i8,
    bal_precision: i8,
) -> (AccountWithCashflows, One_acc_view, String) {
    let mut cf_out_latest: String = String::new();
    let mut ted: NaiveDate;
    let mut cpd = date_from_timestamp(cf_data_out.val_dt);
    let mut one_acc_op_new = One_acc_view::new();
    match method {
        1001 => {
            //Matched Term1 Method implementation
            //cpd : Value Date, TSD: Value Date, TED: Maturity Date
            ted = date_from_timestamp(cf_data_out.ftp_mat_dt);

            let (cf_data_out1, one_acc_op) = ftp_calc_acc_level::calc_ftp_cflevel(
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
                out_path,
                adj_rates,
                avg_bal,
                &mut saved_bm_rates,
                &mut spread_writer,
                rate_precision,
                bal_precision,
            );

            cf_data_out = cf_data_out1;
            one_acc_op_new = one_acc_op;
            one_acc_op_new.method = "Matched Term1".to_string();
            one_acc_op_new.bc_as_on_rule = timestamp(cpd);
            one_acc_op_new.tenor_start_date_rule = timestamp(cpd);
            one_acc_op_new.tenor_end_date_rule = cf_data_out.ftp_mat_dt;
        }
        1002 => {
            //Matched Term2 Method implementation
            //cpd : Last Reprice Date, TSD: Last Reprice Date, TED: Next Reprice Date
            ted = date_from_timestamp(cf_data_out.nxt_rep_dt);
            let lrd = date_from_timestamp(cf_data_out.lst_rep_dt);

            if ted <= NaiveDate::from_ymd(1970, 01, 01) {
                ted = date_from_timestamp(cf_data_out.ftp_mat_dt);
            }

            let (cf_data_out1, one_acc_op) = ftp_calc_acc_level::calc_ftp_cflevel(
                &mut acc_data_in,
                cf_data_out,
                inputfieldnames,
                basecurve,
                lst_adjustments,
                basecurve_file_path,
                log,
                from_date,
                to_date,
                lrd,
                lrd,
                ted,
                false,
                out_path,
                adj_rates,
                avg_bal,
                &mut saved_bm_rates,
                &mut spread_writer,
                rate_precision,
                bal_precision,
            );
            cf_data_out = cf_data_out1;
            one_acc_op_new = one_acc_op;
            one_acc_op_new.method = "Matched Term2".to_string();
            one_acc_op_new.bc_as_on_rule = timestamp(lrd);
            one_acc_op_new.tenor_start_date_rule = timestamp(lrd);
            one_acc_op_new.tenor_end_date_rule = cf_data_out.nxt_rep_dt;
        }
        1003 => {
            //Matched Term3 method
            //cpd : Start Date, TSD: Start Date, TED: Maturity Date
            ted = date_from_timestamp(cf_data_out.ftp_mat_dt);

            let (cf_data_out1, one_acc_op) = ftp_calc_acc_level::calc_ftp_cflevel(
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
                out_path,
                adj_rates,
                avg_bal,
                &mut saved_bm_rates,
                &mut spread_writer,
                rate_precision,
                bal_precision,
            );

            cf_data_out = cf_data_out1;
            one_acc_op_new = one_acc_op;
            one_acc_op_new.method = "Matched Term3".to_string();
            one_acc_op_new.bc_as_on_rule = timestamp(cpd);
            one_acc_op_new.tenor_start_date_rule = timestamp(cpd);
            one_acc_op_new.tenor_end_date_rule = cf_data_out.ftp_mat_dt;
        }
        1011 => {
            //Cashflow1 Method
            //cpd: Value Date , TSD: Value Date, TED: Cashflow date

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
                ftp_rates_file_path,
                adj_rates,
                avg_bal,
                &mut saved_bm_rates,
                &mut spread_writer,
                rate_precision,
                bal_precision,
            );
            cf_data_out = cf_data_out1;
            cf_out_latest = cf_out;
            one_acc_op_new = one_acc_op;
            one_acc_op_new.method = "Cashflow 1".to_string();
            one_acc_op_new.bc_as_on_rule = timestamp(cpd);
            one_acc_op_new.tenor_start_date_rule = timestamp(cpd);
        }
        1012 => {
            //Cashflow2 Method
            //cpd: Start Date , TSD: Start Date, TED: Cashflow date

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
                ftp_rates_file_path,
                adj_rates,
                avg_bal,
                &mut saved_bm_rates,
                &mut spread_writer,
                rate_precision,
                bal_precision,
            );
            cf_data_out = cf_data_out1;
            cf_out_latest = cf_out;
            one_acc_op_new = one_acc_op;
            one_acc_op_new.method = "Cashflow 2".to_string();
            one_acc_op_new.bc_as_on_rule = timestamp(cpd);
            one_acc_op_new.tenor_start_date_rule = timestamp(cpd);
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
                ftp_rates_file_path,
                adj_rates,
                avg_bal,
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
            ted = date_from_timestamp(cf_data_out.ftp_mat_dt);

            let (cf_data_out1, one_acc_op) = ftp_calc_acc_level::calc_ftp_cflevel(
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
                out_path,
                adj_rates,
                avg_bal,
                &mut saved_bm_rates,
                &mut spread_writer,
                rate_precision,
                bal_precision,
            );
            cf_data_out = cf_data_out1;
            one_acc_op_new = one_acc_op;
            one_acc_op_new.method = "Assign Rate 2".to_string();
            one_acc_op_new.bc_as_on_rule = timestamp(cpd);
            one_acc_op_new.tenor_start_date_rule = timestamp(cpd);
            one_acc_op_new.tenor_end_date_rule = cf_data_out.ftp_mat_dt;
        }
        1031 => {
            //Assign Rate with Lock1 Method
            //cpd: AsOn Date , TSD: AsOn Date, TED: Cashflow date
            if !(ftp_rates.is_empty()) && ftp_rates.contains_key(&cf_data_out.deal_no) {
                let rates = ftp_rates.get(&cf_data_out.deal_no).unwrap();
                let ted_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.ftp_mat_dt, 0);
                let ted = ted_naive_date_time.date();

                let (cf_data_out1, one_acc_op) = ftp_calc_lock::calc_ftp_lock(
                    &mut acc_data_in,
                    cf_data_out,
                    inputfieldnames,
                    rates,
                    lock_adjs,
                    log,
                    ftprunid,
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
                one_acc_op_new.tenor_end_date_rule = cf_data_out.ftp_mat_dt;
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
                    ftp_rates_file_path,
                    adj_rates,
                    avg_bal,
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
            if ftp_rates.contains_key(&cf_data_out.deal_no) {
                let rates = ftp_rates.get(&cf_data_out.deal_no).unwrap();

                let (cf_data_out1, one_acc_op) = ftp_calc_lock::calc_ftp_lock(
                    &mut acc_data_in,
                    cf_data_out,
                    inputfieldnames,
                    rates,
                    lock_adjs,
                    log,
                    ftprunid,
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
                ted = date_from_timestamp(cf_data_out.ftp_mat_dt);

                let (cf_data_out1, one_acc_op) = ftp_calc_acc_level::calc_ftp_cflevel(
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
                    out_path,
                    adj_rates,
                    avg_bal,
                    &mut saved_bm_rates,
                    &mut spread_writer,
                    rate_precision,
                    bal_precision,
                );
                cf_data_out = cf_data_out1;
                one_acc_op_new = one_acc_op;
                one_acc_op_new.method = "Assign Rate with lock 2".to_string();
                one_acc_op_new.bc_as_on_rule = timestamp(cpd);
                one_acc_op_new.tenor_start_date_rule = timestamp(cpd);
                one_acc_op_new.tenor_end_date_rule = cf_data_out.ftp_mat_dt;
            }
        }
        1033 => {
            //Assign Rate with Lock3 Method
            //CPD: Start Date , TSD: Start Date, TED: Maturity date
            if !(ftp_rates.is_empty()) && ftp_rates.contains_key(&cf_data_out.deal_no) {
                let rates = ftp_rates.get(&cf_data_out.deal_no).unwrap();

                let (cf_data_out1, one_acc_op) = ftp_calc_lock::calc_ftp_lock(
                    &mut acc_data_in,
                    cf_data_out,
                    inputfieldnames,
                    rates,
                    lock_adjs,
                    log,
                    ftprunid,
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
                ted = date_from_timestamp(cf_data_out.ftp_mat_dt);

                let (cf_data_out1, one_acc_op) = ftp_calc_acc_level::calc_ftp_cflevel(
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
                    out_path,
                    adj_rates,
                    avg_bal,
                    &mut saved_bm_rates,
                    &mut spread_writer,
                    rate_precision,
                    bal_precision,
                );
                cf_data_out = cf_data_out1;
                one_acc_op_new = one_acc_op;
                one_acc_op_new.method = "Assign Rate with lock 3".to_string();
                one_acc_op_new.bc_as_on_rule = timestamp(cpd);
                one_acc_op_new.tenor_start_date_rule = timestamp(cpd);
                one_acc_op_new.tenor_end_date_rule = cf_data_out.ftp_mat_dt;
            }
        }
        1034 => {
            //Reprice Term with lock Method
            //CPD: Value Date , TSD: Value Date, TED: Next Reprice date
            if !(ftp_rates.is_empty()) && ftp_rates.contains_key(&cf_data_out.deal_no) {
                let rates = ftp_rates.get(&cf_data_out.deal_no).unwrap();

                let (cf_data_out1, one_acc_op) = ftp_calc_lock::calc_ftp_lock(
                    &mut acc_data_in,
                    cf_data_out,
                    inputfieldnames,
                    rates,
                    lock_adjs,
                    log,
                    ftprunid,
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
                ted = date_from_timestamp(cf_data_out.ftp_mat_dt);

                let (cf_data_out1, one_acc_op) = ftp_calc_acc_level::calc_ftp_cflevel(
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
                    out_path,
                    adj_rates,
                    avg_bal,
                    &mut saved_bm_rates,
                    &mut spread_writer,
                    rate_precision,
                    bal_precision,
                );
                cf_data_out = cf_data_out1;
                one_acc_op_new = one_acc_op;
                one_acc_op_new.method = "Reprice Term with lock".to_string();
                one_acc_op_new.bc_as_on_rule = timestamp(cpd);
                one_acc_op_new.tenor_start_date_rule = timestamp(cpd);
                one_acc_op_new.tenor_end_date_rule = cf_data_out.ftp_mat_dt;
            }
        }
        1041 => {
            //Margin Method
            let mut cf_ftp = Vec::new();
            let norm_int_rt = half_away_from_zero(
                acc_data_in
                    .get_f64_for_key(&inputfieldnames.deal_rt)
                    .unwrap_or(0.0),
                rate_precision,
            );
            let mut lst_out: Vec<String> = Vec::new();
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

                lst_out.push(out_str);
            }

            let out_str_total = format!(
                "{}|{}|{}|{}|{}",
                cf_data_out.gl,
                total_balance,
                total_interest_ftp,
                total_ftp,
                ftp_rate / total_balance
            );

            lst_out.push(out_str_total);

            cf_data_out.cashflows = protobuf::RepeatedField::from_vec(cf_ftp);
        }
        _ => {
            //TODO: Handle undefined method here
        }
    }

    (cf_data_out, one_acc_op_new, cf_out_latest)
}
