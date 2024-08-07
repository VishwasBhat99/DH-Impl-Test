use macros;
use rbdate::NaiveDate;
use rbdate::NaiveDateTime;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use stamp_ftp::bm_reader;
use stamp_ftp::bm_reader::yieldrate_calc;
use stamp_ftp::bm_reader::IntermediateBmPoints;
use stamp_ftp::cfinput::AccFieldNames;
use stamp_ftp::read_adjustments::Adj_key;
use stamp_ftp::CFout::AccountWithCashflows;
use stamp_ftp::CFout::Cashflow;
use std::collections::HashMap;
use std::fs::OpenOptions;
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
    ftprunid: i64,
    from_date: &NaiveDate,
    to_date: &NaiveDate,
    cpd: NaiveDate,
    tsd: NaiveDate,
    ted: NaiveDate,
    is_cashflow: bool,
    is_lock: bool,
    out_path: &str,
    ftp_rates_file_path: &str,
    adj_rates: &HashMap<Adj_key, f64>,
) -> (AccountWithCashflows, String, String) {
    let mut cf_ftp = Vec::new();
    let mut lst_bm: Vec<IntermediateBmPoints> = Vec::new();
    let mut lst_out: Vec<String> = Vec::new();
    let mut total_balance = 0.0;
    let mut total_interest_ftp = 0.0;
    let mut total_ftp = 0.0;
    let mut ftp_rate = 0.0;

    let full_file_path = format!(
        "{}{}_{}.txt",
        basecurve_file_path,
        cpd.format("%d-%m-%Y"),
        basecurve
    );
    let max_days_in_year =
        rbdate::num_days_start_to_end(tsd, rbdate::increment_date_by_months(tsd, (12) as u16));
    let mut adj_str: String = String::new();
    let mut lock_str: String = String::new();
    let mut lock_str_adj: String = String::new();
    let mut out_str: String = String::new();
    let mut cf_str: String = String::new();
    let mut yield_rate = 0.0;
    let mut ttl_adj_rate = 0.0;
    let mut residual_days = 0;

    //Added +1 -- It includes both from date and to dates.
    let run_duration = rbdate::num_days_start_to_end(*from_date, *to_date) + 1;

    let mut intr_calc_days = 0;
    if Path::new(&full_file_path).exists() {
        lst_bm = bm_reader::get_bm_points(&full_file_path, cpd, log);

        if ted > cpd {
            residual_days = rbdate::num_days_start_to_end(cpd, ted);
        }

        if residual_days <= 0 {
            residual_days = 0;
        }

        if residual_days <= run_duration {
            intr_calc_days = residual_days + 1;
        } else {
            intr_calc_days = run_duration;
        }

        yield_rate = yieldrate_calc::calc_yieldrate(&mut lst_bm, residual_days, cpd, log);

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
            full_file_path, cf_data_out.deal_id
        );
    }

    cf_data_out.total_balance = total_balance;
    cf_data_out.total_interest_ftp = total_interest_ftp;
    if total_balance != 0.0 {
        cf_data_out.FTP_Rate = ftp_rate / total_balance;
    }
    cf_data_out.total_ftp = (total_balance * cf_data_out.FTP_Rate * intr_calc_days as f64)
        / (max_days_in_year as f64 * 100.0);

    let base_rate = yield_rate;

    //Stamping adjustment rates.
    for i in 0..lst_adjustments.len() {
        let adj_key = Adj_key::new(cf_data_out.val_date, lst_adjustments[i]);
        let adj_rate =
            match adj_rates.get(&adj_key) {
                Some(x) => *x,
                None => {
                    let val_dt = NaiveDateTime::from_timestamp(cf_data_out.val_date, 0)
                        .date()
                        .format("%d-%m-%Y");

                    log_debug!(
            log,
            "Adjustments does not exists for adjustment id :{}, date : {}, account id :{}", 
            lst_adjustments[i], val_dt, cf_data_out.deal_id
        );
                    0.0
                }
            };

        adj_str.push_str(&format!("{}|{}|", lst_adjustments[i], adj_rate));
        lock_str.push_str(&format!("{}|", adj_rate));
        lock_str_adj.push_str(&format!("{}|", lst_adjustments[i]));
        cf_data_out.FTP_Rate += adj_rate;
        ttl_adj_rate += adj_rate;
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

    out_str = format!(
        "{}|{}|{}|{}|{}|{}|{}|",
        total_balance,
        cf_data_out.roi,
        total_interest_ftp,
        (base_rate + ttl_adj_rate),
        (total_balance * cf_data_out.FTP_Rate * intr_calc_days as f64)
            / (max_days_in_year as f64 * 100.0),
        basecurve,
        base_rate
    );

    out_str.push_str(&adj_str);

    if is_lock {
        let spread = cf_data_out.roi - (ttl_adj_rate + base_rate);
        let ftp_rates_out = format!(
            "{}|{}|{}|{}|{}|{}|{}|{}",
            cf_data_out.deal_id,
            (ttl_adj_rate + base_rate),
            base_rate,
            lock_str,
            cf_data_out.roi,
            spread,
            basecurve,
            lock_str_adj
        );
        writeftp_rates_file(ftp_rates_file_path, ftp_rates_out);

        out_str = format!("{}{}|", out_str, spread);
    } else {
        out_str = format!("{}{}|", out_str, 0.0);
    }

    (cf_data_out, out_str, cf_str)
}

pub fn writeftp_rates_file(path: &str, ouput: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)
        .unwrap();

    writeln!(file, "{}", ouput);
}
