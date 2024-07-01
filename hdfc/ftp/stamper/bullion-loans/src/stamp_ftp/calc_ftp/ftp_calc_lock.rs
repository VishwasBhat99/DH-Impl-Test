use macros;
use math::round::half_away_from_zero;
use rbdate::timestamp;
use rbdate::NaiveDate;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use stamp_ftp::amb_file_reader::AmbVal;
use stamp_ftp::cfinput::AccFieldNames;
use stamp_ftp::one_acc_view::One_acc_view;
use stamp_ftp::CFout::AccountWithCashflows;
use stamp_ftp::CFout::Cashflow;
use statics::DEFAULT_FLOAT;
use std::collections::HashMap;

pub fn calc_ftp_lock(
    acc_data_in: &mut AccountWithCFs,
    mut cf_data_out: AccountWithCashflows,
    inputfieldnames: &AccFieldNames,
    ftp_rates: &Vec<f64>,
    lock_adjs: &HashMap<i32, String>,
    log: &Logger,
    from_date: &NaiveDate,
    to_date: &NaiveDate,
    tsd: NaiveDate,
    ted: NaiveDate,
    avg_bal: &HashMap<String, AmbVal>,
    rate_precision: i8,
    bal_precision: i8,
) -> (AccountWithCashflows, One_acc_view) {
    let mut cf_ftp = Vec::new();
    let mut total_balance = 0.0;
    let mut total_interest_ftp = 0.0;
    let mut total_ftp = 0.0;
    let mut ftp_rate = 0.0;
    let mut one_acc_op = One_acc_view::new();

    one_acc_op.tenor_start_date_applied = timestamp(tsd);
    one_acc_op.tenor_end_date_applied = timestamp(ted);

    //Added +1 -- It includes both from date and to dates.
    let run_duration = rbdate::num_days_start_to_end(*from_date, *to_date) + 1;
    let mut intr_calc_days = 0;
    let mut adj_str: String = String::new();
    let max_days_in_year = rbdate::num_days_start_to_end(
        *to_date,
        rbdate::increment_date_by_months(*to_date, (12) as u16),
    );
    let mut residual_days = rbdate::num_days_start_to_end(tsd, ted);
    if residual_days <= 0 {
        residual_days = 0;
    }

    intr_calc_days = run_duration;

    let total_tpr = half_away_from_zero(cf_data_out.int_rt - ftp_rates[8], rate_precision);
    let total_adj = half_away_from_zero(
        ftp_rates[1] + ftp_rates[2] + ftp_rates[3] + ftp_rates[4] + ftp_rates[5] + ftp_rates[6],
        rate_precision,
    );
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

    total_balance = half_away_from_zero(total_balance, bal_precision);
    total_interest_ftp = half_away_from_zero(total_interest_ftp, bal_precision);
    total_ftp = half_away_from_zero(total_ftp, bal_precision);

    cf_data_out.total_balance = total_balance;
    cf_data_out.total_interest_ftp = total_interest_ftp;
    cf_data_out.total_ftp = total_ftp;
    if total_balance != 0.0 {
        cf_data_out.FTP_Rate = ftp_rate / total_balance;
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

    cf_data_out.FTP_Rate = half_away_from_zero(cf_data_out.FTP_Rate, rate_precision);
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
    one_acc_op.fx_spread = half_away_from_zero(ftp_rates[8], rate_precision);

    let out_str = format!(
        "{}|{}|{}|{}|{}|{}|{}{}|",
        avg_bal,
        cf_data_out.int_rt,
        int_amt,
        total_tpr,
        one_acc_op.final_ftp_amt,
        base_str,
        adj_str,
        one_acc_op.fx_spread
    );

    (cf_data_out, one_acc_op)
}
