use macros;
use rbdate::NaiveDate;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use stamp_ftp::bm_reader::IntermediateBmPoints;
use stamp_ftp::cfinput::AccFieldNames;
use stamp_ftp::CFout::AccountWithCashflows;
use stamp_ftp::CFout::Cashflow;
use std::collections::HashMap;

pub fn calc_ftp_lock(
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
) -> (AccountWithCashflows, String) {
    let mut cf_ftp = Vec::new();
    let mut lst_bm: Vec<IntermediateBmPoints> = Vec::new();
    let mut lst_out: Vec<String> = Vec::new();
    let mut total_balance = 0.0;
    let mut total_interest_ftp = 0.0;
    let mut total_ftp = 0.0;
    let mut ftp_rate = 0.0;

    //Added +1 -- It includes both from date and to dates.
    let run_duration = rbdate::num_days_start_to_end(*from_date, *to_date) + 1;
    let mut intr_calc_days = 0;
    let mut adj_str: String = String::new();

    let mut residual_days = rbdate::num_days_start_to_end(tsd, ted);
    if residual_days <= 0 {
        residual_days = 0;
    }

    if residual_days <= run_duration {
        intr_calc_days = residual_days + 1;
    } else {
        intr_calc_days = run_duration;
    }

    let total_tpr = cf_data_out.roi - ftp_rates[8];
    let total_adj =
        ftp_rates[1] + ftp_rates[2] + ftp_rates[3] + ftp_rates[4] + ftp_rates[5] + ftp_rates[6];
    let baserate = total_tpr - total_adj;

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
        cf_obj.base_rate_amount =
            (cf.principal_amount * baserate * intr_calc_days as f64) / (365.00 * 100.0);

        total_balance += cf.principal_amount;
        total_interest_ftp += cf.interest_amount;
        total_ftp += cf_obj.base_rate_amount;

        ftp_rate = ftp_rate + (ftp_rates[0] * cf.principal_amount);

        cf_ftp.push(cf_obj);
    }

    let basecurve = match lock_adjs.get(&1) {
        Some(x) => x,
        None => "",
    };
    let base_str = format!("{}|{}", basecurve, baserate);

    for i in 2..=lock_adjs.len() {
        let adj_id = match lock_adjs.get(&(i as i32)) {
            Some(x) => x,
            None => "",
        };

        adj_str.push_str(&format!("{}|{}|", adj_id, ftp_rates[i - 1]));
    }

    cf_data_out.total_balance = total_balance;
    cf_data_out.total_interest_ftp = total_interest_ftp;
    cf_data_out.total_ftp = total_ftp;
    if total_balance != 0.0 {
        cf_data_out.FTP_Rate = ftp_rate / total_balance;
    }
    cf_data_out.cashflows = protobuf::RepeatedField::from_vec(cf_ftp);

    let out_str = format!(
        "{}|{}|{}|{}|{}|{}|{}{}|",
        total_balance,
        cf_data_out.roi,
        (total_balance * total_tpr * intr_calc_days as f64) / (365.00 * 100.0),
        total_tpr,
        total_ftp,
        base_str,
        adj_str,
        ftp_rates[8]
    );

    (cf_data_out, out_str)
}
