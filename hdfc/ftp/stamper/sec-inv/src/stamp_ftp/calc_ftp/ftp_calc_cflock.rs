use macros;
use math::round::half_away_from_zero;
use rbdate::timestamp;
use rbdate::NaiveDate;
use rbdate::NaiveDateTime;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use stamp_ftp::amb_file_reader::AmbVal;
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

pub fn calc_ftp_lock_cf_level(
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
    var_lst_adjustments: Vec<i32>,
    adj_rates: &HashMap<AdjKey, f64>,
    avg_bal: &HashMap<String, AmbVal>,
    fixed_adj_count: i32,
    var_adj_count: i32,
    rate_precision: i8,
    bal_precision: i8,
    mut spread_writer: &mut BufWriter<File>,
    method: i32,
) -> (AccountWithCashflows, String, OneAccView) {
    let mut cf_ftp = Vec::new();
    let mut total_balance = 0.0;
    let mut total_interest_ftp = 0.0;
    let mut total_ftp = 0.0;
    let mut ftp_rate = 0.0;
    let mut adj_rates_lock = vec![DEFAULT_FLOAT; 6];
    let mut adj_codes = vec![DEFAULT_INT; 6];
    let mut total_adj = DEFAULT_FLOAT;
    let mut adj_rates_lock = vec![DEFAULT_FLOAT; 6];
    let mut adj_codes = vec![DEFAULT_INT; 6];
    let mut total_adj = DEFAULT_FLOAT;
    let mut var_adj_rate = DEFAULT_FLOAT;
    let mut total_tpr = 0.0;
    let mut one_acc_op = OneAccView::new();

    one_acc_op.tenor_start_date_applied = timestamp(tsd);
    one_acc_op.tenor_end_date_applied = timestamp(ted);

    let max_days_in_year = rbdate::num_days_start_to_end(
        *to_date,
        rbdate::increment_date_by_months(*to_date, (12) as u16),
    );

    //Added +1 -- It includes both from date and to dates.
    let run_duration = rbdate::num_days_start_to_end(*from_date, *to_date) + 1;
    let intr_calc_days: i64;
    let mut adj_str: String = String::new();

    let mut residual_days = rbdate::num_days_start_to_end(tsd, ted);

    if residual_days <= 0 {
        residual_days = 0;
    }

    intr_calc_days = run_duration;

    for i in 0..fixed_adj_count as i64 {
        let rate = half_away_from_zero(ftp_rates[(i + 1) as usize], rate_precision);
        let adjid = adj_codes[(i + 2) as usize];

        adj_rates_lock[i as usize] = rate;
        total_adj += rate;
        adj_codes[i as usize] = adjid;
    }

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
            var_lst_adjustments[i], st_dt, cf_data_out.acc_no
        );
                    0.0
                }
            },
            rate_precision,
        );

        adj_rates_lock[(fixed_adj_count as usize + i)] = adj_rate;
        total_adj += adj_rate;
        var_adj_rate += adj_rate;
        adj_codes[(fixed_adj_count as usize + i)] = var_lst_adjustments[i] as i64;
    }

    for i in 0..adj_codes.len() {
        let str_lock = format!("{}|{}|", adj_codes[i], adj_rates_lock[i]);
        adj_str.push_str(&str_lock);
    }

    let baserate = half_away_from_zero(ftp_rates[8], rate_precision);
    let basecurve = ftp_rates[9];

    let base_str = format!("{}|{}", basecurve, baserate);
    let total_tpr = half_away_from_zero(baserate + total_adj, rate_precision);

    total_balance = half_away_from_zero(total_balance, bal_precision);
    cf_data_out.total_balance = total_balance;
    cf_data_out.total_interest_ftp = half_away_from_zero(total_interest_ftp, bal_precision);
    cf_data_out.total_ftp = half_away_from_zero(total_ftp, bal_precision);
    if total_balance != 0.0 {
        cf_data_out.ftp_rate = ftp_rate / total_balance;
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

    cf_data_out.int_rt = half_away_from_zero(cf_data_out.int_rt, rate_precision);
    cf_data_out.ftp_rate = half_away_from_zero(cf_data_out.ftp_rate, rate_precision);
    let spread = half_away_from_zero(ftp_rates[8], rate_precision);
    var_adj_rate = half_away_from_zero(var_adj_rate, rate_precision);
    let avg_bal = half_away_from_zero(average_balance.avg_bal, bal_precision).abs();
    let int_amt = half_away_from_zero(average_balance.int_amt, bal_precision).abs();

    one_acc_op.average_balance = avg_bal;
    one_acc_op.int_rate = cf_data_out.int_rt;
    one_acc_op.accr_int = int_amt;
    one_acc_op.final_ftp_rate = total_tpr;
    one_acc_op.final_ftp_amt = half_away_from_zero(
        (avg_bal * total_tpr * intr_calc_days as f64) / (max_days_in_year as f64 * 100.0),
        rate_precision,
    );
    one_acc_op.rate_curve = basecurve.to_string();
    one_acc_op.adj1 = adj_rates_lock[0];
    one_acc_op.adj2 = adj_rates_lock[1];
    one_acc_op.adj3 = adj_rates_lock[2];
    one_acc_op.adj4 = adj_rates_lock[3];
    one_acc_op.adj5 = adj_rates_lock[4];
    one_acc_op.adj6 = adj_rates_lock[5];
    one_acc_op.base_rate = baserate;
    one_acc_op.fx_spread = spread;
    one_acc_op.var_spread = var_adj_rate;

    let out_str = format!(
        "{}|{}|{}|{}|{}|{}|{}{}|{}|{}|",
        avg_bal,
        cf_data_out.int_rt,
        int_amt,
        total_tpr,
        half_away_from_zero(
            (avg_bal * total_tpr * intr_calc_days as f64) / (max_days_in_year as f64 * 100.0),
            rate_precision
        ),
        base_str,
        adj_str,
        spread,
        var_adj_rate,
        half_away_from_zero((cf_data_out.int_rt - total_tpr), rate_precision),
    );

    if cf_data_out.acc_no == cf_data_out.fc_ubs_acc {
        let mut rates_out = String::new();
        let mut id_index = 9;
        for rate_index in 0..7 {
            id_index = 9 + rate_index;
            rates_out.push_str(&format!(
                "{}|{}|",
                ftp_rates[id_index] as i64, ftp_rates[rate_index],
            ));
        }
        let ftp_rates_out = format!(
            "{}|{}|{}{}|{}|{}\n",
            cf_data_out.ubs_acct_num, ftp_rates[15], rates_out, ftp_rates[7], ftp_rates[8], method,
        );
        write!(spread_writer, "{}", ftp_rates_out).expect("Error while write spread file.");
    }

    (cf_data_out, out_str, one_acc_op)
}
