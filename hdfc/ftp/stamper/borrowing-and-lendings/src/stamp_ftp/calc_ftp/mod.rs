use rbdate::NaiveDate;
use rbdate::NaiveDateTime;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use stamp_ftp::cfinput::AccFieldNames;
use stamp_ftp::read_adjustments::Adj_key;
use stamp_ftp::CFout::AccountWithCashflows;
use stamp_ftp::CFout::Cashflow;
use std::collections::HashMap;
use std::time::Instant;
mod ftp_calc_acc_level;
mod ftp_calc_cflevel;
mod ftp_calc_lock;

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
) -> (AccountWithCashflows, String, String) {
    let mut outstr: String = String::new();
    let mut cf_out_latest: String = String::new();
    match method {
        1001 => {
            //Matched Term1 Method implementation
            //cpd : Last Reprice Date, TSD: Last Reprice Date, TED: Maturity Date
            let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.val_date, 0);
            let mut cpd = cpd_naive_date_time.date();
            let ted_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.maturity_dt, 0);
            let mut ted = ted_naive_date_time.date();

            if cpd <= NaiveDate::from_ymd(1970, 01, 01) {
                let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.val_date, 0);
                cpd = cpd_naive_date_time.date();
            }

            let (cf_data_out1, outstr_rn, cf_out) = ftp_calc_acc_level::calc_ftp_cflevel(
                &mut acc_data_in,
                cf_data_out,
                inputfieldnames,
                basecurve,
                lst_adjustments,
                basecurve_file_path,
                log,
                ftprunid,
                from_date,
                to_date,
                cpd,
                cpd,
                ted,
                false,
                false,
                out_path,
                ftp_rates_file_path,
                adj_rates,
            );

            cf_data_out = cf_data_out1;
            cf_out_latest = cf_out;
            outstr = format!("{}Matched Term1|", outstr_rn);
        }
        1002 => {
            //Matched Term2 Method implementation
            //cpd : Last Reprice Date, TSD: Last Reprice Date, TED: Next Reprice Date
            let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.val_date, 0);
            let mut cpd = cpd_naive_date_time.date();
            let ted_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.maturity_dt, 0);
            let mut ted = ted_naive_date_time.date();

            if cpd <= NaiveDate::from_ymd(1970, 01, 01) {
                let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.val_date, 0);
                cpd = cpd_naive_date_time.date();
            }

            if ted <= NaiveDate::from_ymd(1970, 01, 01) {
                let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.maturity_dt, 0);
                ted = cpd_naive_date_time.date();
            }

            let (cf_data_out1, outstr_rn, cf_out) = ftp_calc_acc_level::calc_ftp_cflevel(
                &mut acc_data_in,
                cf_data_out,
                inputfieldnames,
                basecurve,
                lst_adjustments,
                basecurve_file_path,
                log,
                ftprunid,
                from_date,
                to_date,
                cpd,
                cpd,
                ted,
                false,
                false,
                out_path,
                ftp_rates_file_path,
                adj_rates,
            );
            cf_data_out = cf_data_out1;
            cf_out_latest = cf_out;
            outstr = format!("{}Matched Term2|", outstr_rn);;
        }
        1003 => {
            //Matched Term3 method
            //cpd : Start Date, TSD: Start Date, TED: Maturity Date
            let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.val_date, 0);
            let cpd = cpd_naive_date_time.date();
            let ted_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.maturity_dt, 0);
            let ted = ted_naive_date_time.date();

            let (cf_data_out1, outstr_rn, cf_out) = ftp_calc_acc_level::calc_ftp_cflevel(
                &mut acc_data_in,
                cf_data_out,
                inputfieldnames,
                basecurve,
                lst_adjustments,
                basecurve_file_path,
                log,
                ftprunid,
                from_date,
                to_date,
                cpd,
                cpd,
                ted,
                false,
                false,
                out_path,
                ftp_rates_file_path,
                adj_rates,
            );

            cf_data_out = cf_data_out1;
            cf_out_latest = cf_out;
            outstr = format!("{}Matched Term3|", outstr_rn);
        }
        1011 => {
            //Cashflow1 Method
            //cpd: Last reprice date , TSD: Last Reprice date, TED: Cashflow date
            let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.val_date, 0);
            let mut cpd = cpd_naive_date_time.date();

            if cpd <= NaiveDate::from_ymd(1970, 01, 01) {
                let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.val_date, 0);
                cpd = cpd_naive_date_time.date();
            }

            let (cf_data_out1, outstr_rn, cf_out) = ftp_calc_cflevel::calc_ftp_cflevel(
                &mut acc_data_in,
                cf_data_out,
                inputfieldnames,
                basecurve,
                lst_adjustments,
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
            );
            cf_data_out = cf_data_out1;
            cf_out_latest = cf_out;
            outstr = format!("{}Cashflow 1|", outstr_rn);
        }
        1012 => {
            //Cashflow2 Method
            //cpd: Start Date , TSD: Start Date, TED: Cashflow date
            let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.val_date, 0);
            let cpd = cpd_naive_date_time.date();

            let (cf_data_out1, outstr_rn, cf_out) = ftp_calc_cflevel::calc_ftp_cflevel(
                &mut acc_data_in,
                cf_data_out,
                inputfieldnames,
                basecurve,
                lst_adjustments,
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
            );
            cf_data_out = cf_data_out1;
            cf_out_latest = cf_out;
            outstr = format!("{}Cashflow 2|", outstr_rn);
        }
        1021 => {
            //Assign Rate1 Method
            //cpd: AsOn Date , TSD: AsOn Date, TED: Cashflow date

            let (cf_data_out1, outstr_rn, cf_out) = ftp_calc_cflevel::calc_ftp_cflevel(
                &mut acc_data_in,
                cf_data_out,
                inputfieldnames,
                basecurve,
                lst_adjustments,
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
            );
            cf_data_out = cf_data_out1;
            cf_out_latest = cf_out;
            outstr = format!("{}Assign Rate 1|", outstr_rn);
        }
        1022 => {
            //Assign Rate2 Method
            //cpd: AsOn Date , TSD: AsOn Date, TED: Maturity Date

            let ted_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.maturity_dt, 0);
            let ted = ted_naive_date_time.date();

            let (cf_data_out1, outstr_rn, cf_out) = ftp_calc_acc_level::calc_ftp_cflevel(
                &mut acc_data_in,
                cf_data_out,
                inputfieldnames,
                basecurve,
                lst_adjustments,
                basecurve_file_path,
                log,
                ftprunid,
                from_date,
                to_date,
                *from_date,
                *from_date,
                ted,
                false,
                false,
                out_path,
                ftp_rates_file_path,
                adj_rates,
            );
            cf_data_out = cf_data_out1;
            cf_out_latest = cf_out;
            outstr = format!("{}Assign Rate 2|", outstr_rn);
        }
        1031 => {
            //Assign Rate with Lock1 Method
            //cpd: AsOn Date , TSD: AsOn Date, TED: Cashflow date
            if !(ftp_rates.is_empty()) && ftp_rates.contains_key(&cf_data_out.deal_id) {
                let rates = ftp_rates.get(&cf_data_out.deal_id).unwrap();
                let ted_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.maturity_dt, 0);
                let ted = ted_naive_date_time.date();

                let (cf_data_out1, outstr_rn) = ftp_calc_lock::calc_ftp_lock(
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
                );

                cf_data_out = cf_data_out1;
                outstr = format!("{}Assign Rate with lock 1|", outstr_rn);
            } else {
                let (cf_data_out1, outstr_rn, cf_out) = ftp_calc_cflevel::calc_ftp_cflevel(
                    &mut acc_data_in,
                    cf_data_out,
                    inputfieldnames,
                    basecurve,
                    lst_adjustments,
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
                );
                cf_data_out = cf_data_out1;
                cf_out_latest = cf_out;
                outstr = format!("{}Assign Rate with lock 1|", outstr_rn);
            }
        }
        1032 => {
            //Assign Rate with Lock2 Method
            //CPD: AsOn Date , TSD: AsOn Date, TED: Maturity date
            if ftp_rates.contains_key(&cf_data_out.deal_id) {
                let rates = ftp_rates.get(&cf_data_out.deal_id).unwrap();

                let (cf_data_out1, outstr_rn) = ftp_calc_lock::calc_ftp_lock(
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
                );
                cf_data_out = cf_data_out1;
                outstr = format!("{}Assign Rate with lock 2|", outstr_rn);
            } else {
                let ted_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.maturity_dt, 0);
                let ted = ted_naive_date_time.date();

                let (cf_data_out1, outstr_rn, cf_out) = ftp_calc_acc_level::calc_ftp_cflevel(
                    &mut acc_data_in,
                    cf_data_out,
                    inputfieldnames,
                    basecurve,
                    lst_adjustments,
                    basecurve_file_path,
                    log,
                    ftprunid,
                    from_date,
                    to_date,
                    *from_date,
                    *from_date,
                    ted,
                    false,
                    true,
                    out_path,
                    ftp_rates_file_path,
                    adj_rates,
                );
                cf_data_out = cf_data_out1;
                cf_out_latest = cf_out;
                outstr = format!("{}Assign Rate with lock 2|", outstr_rn);
            }
        }
        1033 => {
            //Assign Rate with Lock3 Method
            //CPD: Start Date , TSD: Start Date, TED: Maturity date
            if !(ftp_rates.is_empty()) && ftp_rates.contains_key(&cf_data_out.deal_id) {
                let rates = ftp_rates.get(&cf_data_out.deal_id).unwrap();

                let (cf_data_out1, outstr_rn) = ftp_calc_lock::calc_ftp_lock(
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
                );
                cf_data_out = cf_data_out1;
                outstr = format!("{}Assign Rate with lock 3|", outstr_rn);
            } else {
                let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.val_date, 0);
                let cpd = cpd_naive_date_time.date();
                let ted_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.maturity_dt, 0);
                let ted = ted_naive_date_time.date();

                let (cf_data_out1, outstr_rn, cf_out) = ftp_calc_acc_level::calc_ftp_cflevel(
                    &mut acc_data_in,
                    cf_data_out,
                    inputfieldnames,
                    basecurve,
                    lst_adjustments,
                    basecurve_file_path,
                    log,
                    ftprunid,
                    from_date,
                    to_date,
                    cpd,
                    cpd,
                    ted,
                    false,
                    true,
                    out_path,
                    ftp_rates_file_path,
                    adj_rates,
                );
                cf_data_out = cf_data_out1;
                cf_out_latest = cf_out;
                outstr = format!("{}Assign Rate with lock 3|", outstr_rn);
            }
        }
        1034 => {
            //Reprice Term with lock Method
            //CPD: Last Reprice Date , TSD: Last Reprice Date, TED: Next Reprice date
            if !(ftp_rates.is_empty()) && ftp_rates.contains_key(&cf_data_out.deal_id) {
                let rates = ftp_rates.get(&cf_data_out.deal_id).unwrap();

                let (cf_data_out1, outstr_rn) = ftp_calc_lock::calc_ftp_lock(
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
                );
                cf_data_out = cf_data_out1;
                outstr = format!("{}Reprice Term with lock|", outstr_rn);
            } else {
                let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.val_date, 0);
                let mut cpd = cpd_naive_date_time.date();
                let ted_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.maturity_dt, 0);
                let mut ted = ted_naive_date_time.date();

                if cpd <= NaiveDate::from_ymd(1970, 01, 01) {
                    let naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.val_date, 0);
                    cpd = naive_date_time.date();
                }

                if ted <= NaiveDate::from_ymd(1970, 01, 01) {
                    let naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.maturity_dt, 0);
                    ted = naive_date_time.date();
                }

                let (cf_data_out1, outstr_rn, cf_out) = ftp_calc_acc_level::calc_ftp_cflevel(
                    &mut acc_data_in,
                    cf_data_out,
                    inputfieldnames,
                    basecurve,
                    lst_adjustments,
                    basecurve_file_path,
                    log,
                    ftprunid,
                    from_date,
                    to_date,
                    cpd,
                    cpd,
                    ted,
                    false,
                    true,
                    out_path,
                    ftp_rates_file_path,
                    adj_rates,
                );
                cf_data_out = cf_data_out1;
                cf_out_latest = cf_out;
                outstr = format!("{}Reprice Term with lock|", outstr_rn);
            }
        }
        1041 => {
            //Margin Method
            let mut cf_ftp = Vec::new();
            let int_rt = acc_data_in.get_f64_for_key(&inputfieldnames.roi).unwrap();
            let mut _lst_out: Vec<String> = Vec::new();
            let mut _total_balance = 0.0;
            let mut _total_interest_ftp = 0.0;
            let mut _total_ftp = 0.0;
            let mut _ftp_rate = 0.0;

            for cf in acc_data_in
                .remove_cfs_for_key(&inputfieldnames.cashflows)
                .expect("fail")
                .iter_mut()
            {
                let mut cf_obj = Cashflow::new();
                cf_obj.interest_amount = cf.interest_amount;
                cf_obj.principal_amount = cf.principal_amount;
                cf_obj.date = cf.date;
                cf_obj.base_rate = int_rt;
                cf_obj.base_rate_amount = (cf.principal_amount * int_rt) / 36500.00;

                _total_balance += cf.principal_amount;
                _total_interest_ftp += cf.interest_amount;
                _total_ftp += cf_obj.base_rate_amount;

                let out_str = format!(
                    "{}|{}|{}|{}|{}|{}|{}|{}",
                    cf_data_out.deal_id,
                    NaiveDateTime::from_timestamp(cf.date, 0).date(),
                    cf.principal_amount,
                    cf.interest_amount,
                    cf_obj.base_rate_amount,
                    int_rt,
                    int_rt,
                    cf_obj.base_rate_amount
                );

                cf_ftp.push(cf_obj);

                _lst_out.push(out_str);
            }

            let out_str_total = format!(
                "{}|{}|{}|{}|{}",
                cf_data_out.deal_id,
                _total_balance,
                _total_interest_ftp,
                _total_ftp,
                _ftp_rate / _total_balance
            );

            _lst_out.push(out_str_total);

            cf_data_out.cashflows = protobuf::RepeatedField::from_vec(cf_ftp);
        }
        _ => {
            //TODO: Handle undefined method here
            println!("Executing unimplemented method");
        }
    }

    (cf_data_out, outstr, cf_out_latest)
}
