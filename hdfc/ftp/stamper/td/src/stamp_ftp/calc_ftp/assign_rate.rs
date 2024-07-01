use calc_ftp::get_out_type;
use macros;
use math::round::half_away_from_zero;
use rbdate::date_from_timestamp;
use rbdate::timestamp;
use rbdate::NaiveDate;
use rbdate::NaiveDateTime;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_agg_rules_adj::agg_rules::AggRules_adj;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use stamp_ftp::aggr_key::Customer;
use stamp_ftp::bm_reader;
use stamp_ftp::bm_reader::yieldrate_calc;
use stamp_ftp::bm_reader::{BmKey, IntermediateBmPoints};
use stamp_ftp::calc_ftp::{assign_rate, margin_method};
use stamp_ftp::cfinput;
use stamp_ftp::cfinput::AccFieldNames;
use stamp_ftp::one_acc_view::One_acc_view;
use stamp_ftp::read_adjustments::Adj_key;
use stamp_ftp::rule_stamper;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::time::Instant;

use super::margin_method::get_aggr_bal;

pub fn assign_rate_1(
    acc_data: &mut AccountWithCFs,
    m_rules: &AggRules,
    bc_rules: &AggRules,
    fix_adj_rules: &AggRules_adj,
    var_adj_rules: &AggRules_adj,
    bc_file_path: String,
    inputfields: &AccFieldNames,
    log: &Logger,
    diag_log: &Logger,
    ftprunid: i64,
    from_date: &NaiveDate,
    to_date: &NaiveDate,
    out_path: &str,
    ftp_rates: &mut HashMap<String, Vec<f64>>,
    lock_adjs: &HashMap<i32, String>,
    adj_rates: &HashMap<Adj_key, f64>,
    avg_bal: &HashMap<String, f64>,
    ftp_rates_file_path: &str,
    default_method: i32,
    default_basecurve: i32,
    fix_adj_count: i32,
    var_adj_count: i32,
    mut saved_bm_rates: &mut HashMap<BmKey, Vec<IntermediateBmPoints>>,
    mut spread_writer: &mut BufWriter<File>,
    rate_precision: i8,
    bal_precision: i8,
    basecurve: i32,
    input_field_names: &cfinput::AccFieldNames,
    fix_lst_adjustments: Vec<i32>,
    var_lst_adjustments: Vec<i32>,
    aggr_bal: &HashMap<Customer, f64>,
) -> (String, String) {
    let mut residual_days = 0;
    let mut yield_rate = DEFAULT_FLOAT;

    let mut adj_rates_counter = 0;
    let mut adj_rates_vec: Vec<f64> = [0.0; 6].to_vec();

    let mut final_ftp_amt = DEFAULT_FLOAT;
    let acc_no = match acc_data.get_string_for_key(&input_field_names.account_number) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();
    let mat_date = date_from_timestamp(
        match acc_data.get_i64_for_key(&input_field_names.maturity_date) {
            Ok(result) => result,
            Err(_) => DEFAULT_INT,
        },
    );
    let acc_start_date = date_from_timestamp(
        match acc_data.get_i64_for_key(&input_field_names.account_start_date) {
            Ok(result) => result,
            Err(_) => DEFAULT_INT,
        },
    );
    let accr_int = match acc_data.get_f64_for_key(&input_field_names.rat_acct_int) {
        Ok(result) => result,
        Err(e) => DEFAULT_FLOAT,
    };
    let mut aggr_balance = get_aggr_bal(aggr_bal, acc_data, input_field_names);
    let full_file_path = format!(
        "{}{}_{}.txt",
        bc_file_path,
        from_date.format("%d-%m-%Y"),
        basecurve
    );
    let bm_key = BmKey {
        date: from_date.format("%d-%m-%Y").to_string(),
        base_curve_id: basecurve,
    };
    let max_days_in_year = rbdate::num_days_start_to_end(
        *to_date,
        rbdate::increment_date_by_months(*to_date, (12) as u16),
    );
    //Added +1 -- It includes both from date and to dates.
    let run_duration = rbdate::num_days_start_to_end(*from_date, *to_date) + 1;
    let mut lst_bm: Vec<IntermediateBmPoints> = Vec::new();
    let mut intr_calc_days = DEFAULT_INT;
    if Path::new(&full_file_path).exists() {
        intr_calc_days = run_duration;
        if saved_bm_rates.contains_key(&bm_key) {
            lst_bm = bm_reader::get_new_bm_points(saved_bm_rates, bm_key).to_vec();
        } else {
            bm_reader::get_bm_points(&full_file_path, *from_date, log, &mut lst_bm);
            saved_bm_rates.insert(bm_key, lst_bm.clone());
        }

        if mat_date > acc_start_date {
            residual_days = rbdate::num_days_start_to_end(acc_start_date, mat_date);
        }

        if residual_days <= 0 {
            residual_days = 1;
        }
        yield_rate = half_away_from_zero(
            yieldrate_calc::calc_yieldrate(&mut lst_bm, residual_days, *from_date, log),
            rate_precision,
        );
        if yield_rate < 0.0 {
            yield_rate = 0.0
        }
    }
    let mut final_ftp_rate = yield_rate;
    let avg_bal = match avg_bal.get(&acc_no) {
        Some(x) => x,
        None => &DEFAULT_FLOAT,
    };
    let out_type = get_out_type(aggr_balance);
    //Stamping fixed adjustment rates.
    for i in 0..fix_lst_adjustments.len() {
        let adj_key = Adj_key::new(timestamp(acc_start_date), fix_lst_adjustments[i]);
        let adj_rate = half_away_from_zero(
            match adj_rates.get(&adj_key) {
                Some(x) => *x,
                None => {
                    let st_dt = NaiveDateTime::from_timestamp(timestamp(acc_start_date), 0)
                        .date()
                        .format("%d-%m-%Y");

                    log_debug!(
                        log,
                        "Adjustments does not exists for adjustment id :{}, 
                        date : {}, account id :{}",
                        fix_lst_adjustments[i],
                        st_dt,
                        acc_no
                    );
                    DEFAULT_FLOAT
                }
            },
            rate_precision,
        );
        final_ftp_rate += adj_rate;
        adj_rates_vec[adj_rates_counter] = adj_rate;
        adj_rates_counter += 1;
    }
    //Stamping variable adjustment rates.
    for i in 0..var_lst_adjustments.len() {
        let adj_key = Adj_key::new(timestamp(acc_start_date), var_lst_adjustments[i]);
        let adj_rate = half_away_from_zero(
            match adj_rates.get(&adj_key) {
                Some(x) => *x,
                None => {
                    let st_dt = NaiveDateTime::from_timestamp(timestamp(acc_start_date), 0)
                        .date()
                        .format("%d-%m-%Y");

                    log_debug!(
                    log,
                    "Adjustments does not exists for adjustment id :{}, date : {}, account id :{}",
                    var_lst_adjustments[i],
                    st_dt,
                    acc_no
                );
                    DEFAULT_FLOAT
                }
            },
            rate_precision,
        );
        final_ftp_rate += adj_rate;
        adj_rates_vec[adj_rates_counter] = adj_rate;
        adj_rates_counter += 1;
    }

    final_ftp_amt = half_away_from_zero(
        (avg_bal * final_ftp_rate * intr_calc_days as f64) / (max_days_in_year as f64 * 100.0),
        bal_precision,
    );

    let days_in_month = rbdate::num_days_start_to_end(*from_date, *to_date) + 1;
    let days_in_year = rbdate::num_days_start_to_end(
        *to_date,
        rbdate::increment_date_by_months(*to_date, (12) as u16),
    );
    let yld_to_call: f64 =
        (accr_int / avg_bal) * 100.0 as f64 * (days_in_year as f64 / days_in_month as f64);

    //get field values
    let cust_name = match acc_data.get_string_for_key(&input_field_names.client_name) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();
    let int_rate = match acc_data.get_f64_for_key(&input_field_names.int_rate) {
        Ok(result) => result,
        Err(e) => DEFAULT_FLOAT,
    };
    let origination_date = match acc_data.get_i64_for_key(&input_field_names.origination_date) {
        Ok(result) => result,
        Err(e) => DEFAULT_INT,
    };
    let org_date = NaiveDateTime::from_timestamp(origination_date, 0)
        .date()
        .format("%d-%m-%Y");
    let fin_cost_ftp = match acc_data.get_string_for_key(&input_field_names.fin_cost_ftp) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();
    let prod_code = match acc_data.get_string_for_key(&input_field_names.deposit_type) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    let rate_flag = match acc_data.get_string_for_key(&input_field_names.rate_flag) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();
    let currency = match acc_data.get_string_for_key(&input_field_names.institution) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();
    let new_gl = match acc_data.get_string_for_key(&input_field_names.new_gl) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    let cust_id = match acc_data.get_i64_for_key(&input_field_names.customer_id) {
        Ok(result) => result,
        Err(e) => DEFAULT_INT,
    };

    let original_balance = match acc_data.get_f64_for_key(&input_field_names.original_balance) {
        Ok(result) => result,
        Err(e) => DEFAULT_FLOAT,
    };
    let two_point_concat = match acc_data.get_string_for_key(&input_field_names.two_point_concat) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();
    let four_point_concat = match acc_data.get_string_for_key(&input_field_names.four_point_concat)
    {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();
    let outstanding_balance =
        match acc_data.get_f64_for_key(&input_field_names.current_book_balance) {
            Ok(result) => result,
            Err(e) => DEFAULT_FLOAT,
        };

    let out_str = format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|\
            {}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        acc_no,
        cust_name,
        avg_bal,
        accr_int,
        half_away_from_zero(yld_to_call, bal_precision),
        int_rate,
        yield_rate,
        final_ftp_rate,
        org_date,
        mat_date,
        "",
        "",
        fin_cost_ftp,
        "",
        "",
        prod_code,
        rate_flag,
        "",
        "TD",
        currency,
        new_gl,
        cust_id,
        final_ftp_amt,
        "",
        "",
        original_balance,
        outstanding_balance,
        yield_rate,
        adj_rates_vec[0],
        adj_rates_vec[1],
        adj_rates_vec[2],
        adj_rates_vec[3],
        adj_rates_vec[4],
        adj_rates_vec[5],
        "",
        "",
        "",
        "Assign Rate method",
        "Int Rate",
        "",
        "",
        "",
        "",
        "",
        get_date_str(DEFAULT_INT),
        get_date_str(DEFAULT_INT),
        get_date_str(DEFAULT_INT),
        get_date_str(DEFAULT_INT),
        get_date_str(DEFAULT_INT),
        get_date_str(DEFAULT_INT),
        two_point_concat,
        four_point_concat,
    );

    (out_str, out_type)
}

fn get_date_str(date: i64) -> String {
    let start_date = NaiveDateTime::from_timestamp(date, 0)
        .date()
        .format("%d-%m-%Y");

    start_date.to_string()
}
