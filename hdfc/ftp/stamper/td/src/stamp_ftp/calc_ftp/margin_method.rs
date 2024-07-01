use macros;
use math::round::half_away_from_zero;
use rbdate::date_from_timestamp;
use rbdate::NaiveDate;
use rbdate::NaiveDateTime;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_agg_rules_adj::agg_rules::AggRules_adj;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use stamp_ftp::bm_reader;
use stamp_ftp::bm_reader::yieldrate_calc;
use stamp_ftp::bm_reader::{BmKey, IntermediateBmPoints};
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;
// use stamp_ftp::calc_ftp;
use calc_ftp::get_out_type;
use stamp_ftp::aggr_key::Customer;
use stamp_ftp::calc_ftp::{assign_rate, margin_method};
use stamp_ftp::cfinput;
use stamp_ftp::cfinput::AccFieldNames;
use stamp_ftp::one_acc_view::One_acc_view;
use stamp_ftp::read_adjustments::Adj_key;
use stamp_ftp::rule_stamper;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::time::Instant;

pub fn margin_method(
    method: i32,
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
    let _run_duration = rbdate::num_days_start_to_end(*from_date, *to_date) + 1;
    let base_rate = match acc_data.get_f64_for_key(&input_field_names.int_rate) {
        Ok(result) => result,
        Err(_) => DEFAULT_FLOAT,
    };
    let mut aggr_balance = get_aggr_bal(aggr_bal, acc_data, input_field_names);
    let adjrate1 = get_dep_adj_rate(aggr_balance);
    let mut final_tpr = 0.0;
    let account_number = match acc_data.get_string_for_key(&input_field_names.account_number) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();
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
    if method == 1011 {
        final_tpr = base_rate + adjrate1;
    } else if method == 1042 {
        let full_file_path = format!(
            "{}{}_{}.txt",
            bc_file_path,
            to_date.format("%d-%m-%Y"),
            basecurve
        );
        let bm_key = BmKey {
            date: from_date.format("%d-%m-%Y").to_string(),
            base_curve_id: basecurve,
        };

        let mut lst_bm: Vec<IntermediateBmPoints> = Vec::new();
        if Path::new(&full_file_path).exists() {
            if saved_bm_rates.contains_key(&bm_key) {
                lst_bm = bm_reader::get_new_bm_points(saved_bm_rates, bm_key).to_vec();
            } else {
                bm_reader::get_bm_points(&full_file_path, *from_date, log, &mut lst_bm);
                saved_bm_rates.insert(bm_key, lst_bm.clone());
            }

            yield_rate = half_away_from_zero(
                yieldrate_calc::calc_yieldrate(&mut lst_bm, residual_days, *from_date, log),
                rate_precision,
            );

            final_tpr = yield_rate;
        } else {
            log_error!(log, "{} is not present", full_file_path);
            final_tpr = int_rate;
        }
    }
    let org_date = NaiveDateTime::from_timestamp(origination_date, 0)
        .date()
        .format("%d-%m-%Y");
    let maturity_date = match acc_data.get_i64_for_key(&input_field_names.maturity_date) {
        Ok(result) => result,
        Err(e) => DEFAULT_INT,
    };
    let mat_date = NaiveDateTime::from_timestamp(maturity_date, 0)
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

    let accr_int = match acc_data.get_f64_for_key(&input_field_names.rat_acct_int) {
        Ok(result) => result,
        Err(e) => DEFAULT_FLOAT,
    };
    let original_balance = match acc_data.get_f64_for_key(&input_field_names.original_balance) {
        Ok(result) => result,
        Err(e) => DEFAULT_FLOAT,
    };

    let outstanding_balance =
        match acc_data.get_f64_for_key(&input_field_names.current_book_balance) {
            Ok(result) => result,
            Err(e) => DEFAULT_FLOAT,
        };
    let days_in_month = rbdate::num_days_start_to_end(*from_date, *to_date) + 1;
    let days_in_year = rbdate::num_days_start_to_end(
        *to_date,
        rbdate::increment_date_by_months(*to_date, (12) as u16),
    );
    let average_balance = match avg_bal.get(&account_number) {
        Some(x) => x,
        None => &DEFAULT_FLOAT,
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
    let out_type = get_out_type(aggr_balance);
    let yld_to_call: f64 =
        (accr_int / average_balance) * 100.0 as f64 * (days_in_year as f64 / days_in_month as f64);
    let _max_days_in_year = rbdate::num_days_start_to_end(
        *from_date,
        rbdate::increment_date_by_months(*from_date, (12) as u16),
    );

    let out_str = format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|\
    {}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        account_number,
        cust_name,
        average_balance,
        "",
        half_away_from_zero(yld_to_call, bal_precision),
        int_rate,
        base_rate,
        final_tpr,
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
        (average_balance * final_tpr * _run_duration as f64) / (_max_days_in_year as f64 * 100.0),
        "",
        "",
        original_balance,
        outstanding_balance,
        base_rate,
        adjrate1,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        "",
        "",
        "",
        "Margin method",
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

pub fn get_dep_adj_rate(aggr_bal: f64) -> f64 {
    let mut adj_rate: f64 = 0.0;

    if aggr_bal >= 20000000.00 && aggr_bal < 50000000.00 {
        adj_rate = 0.5;
    } else if aggr_bal >= 50000000.00 {
        adj_rate = 0.25;
    } else {
        adj_rate = 1.0;
    }

    adj_rate
}

pub fn get_aggr_bal(
    aggr_bal: &HashMap<Customer, f64>,
    acc_data_in: &mut AccountWithCFs,
    input_field_names: &cfinput::AccFieldNames,
) -> f64 {
    let cust_id = acc_data_in
        .get_i64_for_key(&input_field_names.customer_id)
        .unwrap();
    let ccy = acc_data_in
        .get_string_for_key(&input_field_names.institution)
        .unwrap()
        .to_string();
    let st_dt = acc_data_in
        .get_i64_for_key(&input_field_names.origination_date)
        .unwrap();
    let mat_dt = acc_data_in
        .get_i64_for_key(&input_field_names.maturity_date)
        .unwrap();

    let acc_bal = acc_data_in
        .get_f64_for_key(&input_field_names.original_balance)
        .unwrap();

    let cust_key = Customer::new(cust_id, st_dt, mat_dt, ccy);
    let balance = match aggr_bal.get(&cust_key) {
        Some(x) => *x,
        None => acc_bal,
    };

    balance
}

fn get_date_str(date: i64) -> String {
    let start_date = NaiveDateTime::from_timestamp(date, 0)
        .date()
        .format("%d-%m-%Y");

    start_date.to_string()
}

pub fn margin_method_2(
    method: i32,
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
    let _run_duration = rbdate::num_days_start_to_end(*from_date, *to_date) + 1;
    let base_rate = match acc_data.get_f64_for_key(&input_field_names.int_rate) {
        Ok(result) => result,
        Err(_) => DEFAULT_FLOAT,
    };
    let mut aggr_balance = get_aggr_bal(aggr_bal, acc_data, input_field_names);
    let adjrate1 = get_dep_adj_rate(aggr_balance);
    let mut final_tpr = 0.0;
    let account_number = match acc_data.get_string_for_key(&input_field_names.account_number) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();
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
    if method == 1011 {
        final_tpr = base_rate + adjrate1;
    } else if method == 1042 {
        let full_file_path = format!(
            "{}{}_{}.txt",
            bc_file_path,
            to_date.format("%d-%m-%Y"),
            basecurve
        );
        let bm_key = BmKey {
            date: from_date.format("%d-%m-%Y").to_string(),
            base_curve_id: basecurve,
        };

        let mut lst_bm: Vec<IntermediateBmPoints> = Vec::new();
        if Path::new(&full_file_path).exists() {
            if saved_bm_rates.contains_key(&bm_key) {
                lst_bm = bm_reader::get_new_bm_points(saved_bm_rates, bm_key).to_vec();
            } else {
                bm_reader::get_bm_points(&full_file_path, *from_date, log, &mut lst_bm);
                saved_bm_rates.insert(bm_key, lst_bm.clone());
            }

            yield_rate = half_away_from_zero(
                yieldrate_calc::calc_yieldrate(&mut lst_bm, residual_days, *from_date, log),
                rate_precision,
            );

            final_tpr = yield_rate;
        } else {
            log_error!(log, "{} is not present", full_file_path);
            final_tpr = int_rate;
        }
    }
    let org_date = NaiveDateTime::from_timestamp(origination_date, 0)
        .date()
        .format("%d-%m-%Y");
    let maturity_date = match acc_data.get_i64_for_key(&input_field_names.maturity_date) {
        Ok(result) => result,
        Err(e) => DEFAULT_INT,
    };
    let mat_date = NaiveDateTime::from_timestamp(maturity_date, 0)
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

    let accr_int = match acc_data.get_f64_for_key(&input_field_names.rat_acct_int) {
        Ok(result) => result,
        Err(e) => DEFAULT_FLOAT,
    };
    let original_balance = match acc_data.get_f64_for_key(&input_field_names.original_balance) {
        Ok(result) => result,
        Err(e) => DEFAULT_FLOAT,
    };

    let outstanding_balance =
        match acc_data.get_f64_for_key(&input_field_names.current_book_balance) {
            Ok(result) => result,
            Err(e) => DEFAULT_FLOAT,
        };
    let days_in_month = rbdate::num_days_start_to_end(*from_date, *to_date) + 1;
    let days_in_year = rbdate::num_days_start_to_end(
        *to_date,
        rbdate::increment_date_by_months(*to_date, (12) as u16),
    );
    let average_balance = match avg_bal.get(&account_number) {
        Some(x) => x,
        None => &DEFAULT_FLOAT,
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
    let out_type = get_out_type(aggr_balance);
    let yld_to_call: f64 =
        (accr_int / average_balance) * 100.0 as f64 * (days_in_year as f64 / days_in_month as f64);
    let _max_days_in_year = rbdate::num_days_start_to_end(
        *from_date,
        rbdate::increment_date_by_months(*from_date, (12) as u16),
    );

    let out_str = format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|\
    {}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        account_number,
        cust_name,
        average_balance,
        "",
        half_away_from_zero(yld_to_call, bal_precision),
        int_rate,
        final_tpr,
        final_tpr,
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
        (average_balance * final_tpr * _run_duration as f64) / (_max_days_in_year as f64 * 100.0),
        "",
        "",
        original_balance,
        outstanding_balance,
        final_tpr,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        "",
        "",
        "",
        "Margin method 2",
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
