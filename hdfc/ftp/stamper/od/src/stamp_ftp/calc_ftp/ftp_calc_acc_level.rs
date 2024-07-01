use macros;
use math::round::half_away_from_zero;
use rbdate::*;
use sdb_dyn_proto_rdr::reader::account_with_cfs::{get_field_value, AccountWithCFs};
use sdb_dyn_proto_rdr::reader::Reader;
use slog::Logger;
use stamp_ftp::append_output::append_out;
use stamp_ftp::bm_reader;
use stamp_ftp::bm_reader::yieldrate_calc;
use stamp_ftp::bm_reader::{BmKey, IntermediateBmPoints};
use stamp_ftp::cfinput::AccFieldNames;
use stamp_ftp::one_acc_view::One_acc_view;
use stamp_ftp::read_adjustments::AdjKey;
use statics::*;
use std::collections::HashMap;
use std::path::Path;

use crate::stamp_ftp::restructured_op::additional_struct::AmbData;

pub fn calc_ftp(
    input_reader: &Reader,
    acc_data_in: &AccountWithCFs,
    inputfieldnames: &AccFieldNames,
    basecurve: i32,
    fix_lst_adjustments: Vec<i32>,
    var_lst_adjustments: Vec<i32>,
    basecurve_file_path: String,
    log: &Logger,
    from_date: &NaiveDate,
    to_date: &NaiveDate,
    cpd: NaiveDate,
    ted: NaiveDate,
    adj_rates: &HashMap<AdjKey, f64>,
    fixed_adj_count: i32,
    saved_bm_rates: &mut HashMap<BmKey, Vec<IntermediateBmPoints>>,
    rate_precision: i8,
    bal_precision: i8,
    method: i32,
    method_name: &str,
    is_closed: bool,
    amb_map: &HashMap<String, AmbData>,
) -> (String, String) {
    let run_duration = num_days_start_to_end(*from_date, *to_date) + 1;
    let mut one_acc_op = One_acc_view::new();
    let mut adj_str: String = String::new();
    let acc_id = acc_data_in
        .get_string_for_key(&inputfieldnames.cod_acc_no)
        .unwrap_or(&String::default())
        .to_string();
    one_acc_op.account_number = acc_id.clone();
    let default_amb_data = &Default::default();
    let amb_data: &AmbData = amb_map.get(&acc_id).unwrap_or(default_amb_data);

    let accr_int = amb_data.accr_int.clone();
    one_acc_op.accr_int = accr_int;
    one_acc_op.ccy = acc_data_in
        .get_string_for_key(&inputfieldnames.inst)
        .unwrap_or(&String::default())
        .to_string();

    one_acc_op.maturity_date = acc_data_in
        .get_i64_for_key(&inputfieldnames.dt_acc_close)
        .unwrap_or(DEFAULT_INT);

    one_acc_op.nxt_rep_dt = acc_data_in
        .get_i64_for_key(&inputfieldnames.nxt_rep_dt)
        .unwrap_or(DEFAULT_INT);

    one_acc_op.lst_rep_dt = acc_data_in
        .get_i64_for_key(&inputfieldnames.lst_rep_dt)
        .unwrap_or(DEFAULT_INT);

    one_acc_op.init_dep_amt = acc_data_in
        .get_f64_for_key(&inputfieldnames.bal_book_lcy)
        .unwrap_or(DEFAULT_FLOAT);

    if !is_closed {
        one_acc_op.outstanding_bal = acc_data_in
            .get_f64_for_key(&inputfieldnames.crnt_book_bal)
            .unwrap_or(DEFAULT_FLOAT);
    }

    one_acc_op.value_date = acc_data_in
        .get_i64_for_key(&inputfieldnames.dt_open_acc)
        .unwrap_or(DEFAULT_INT);

    let int_rate = acc_data_in
        .get_f64_for_key(&inputfieldnames.int_rt)
        .unwrap_or(DEFAULT_FLOAT);

    one_acc_op.gl = acc_data_in
        .get_string_for_key(&inputfieldnames.gl_acc_no)
        .unwrap_or(&String::default())
        .to_string();

    one_acc_op.cust_id = acc_data_in
        .get_i64_for_key(&inputfieldnames.cod_collat_id)
        .unwrap_or(DEFAULT_INT)
        .to_string();

    one_acc_op.cust_name = acc_data_in
        .get_string_for_key(&inputfieldnames.cod_acc_title)
        .unwrap_or(&String::default())
        .to_string();

    one_acc_op.mis1 = acc_data_in
        .get_i64_for_key(&inputfieldnames.mis1)
        .unwrap_or(DEFAULT_INT)
        .to_string();

    one_acc_op.prod_type = acc_data_in
        .get_string_for_key(&inputfieldnames.cod_prod)
        .unwrap_or(&String::default())
        .to_string();

    one_acc_op.rate_flag = acc_data_in
        .get_string_for_key(&inputfieldnames.rt_flg)
        .unwrap_or(&String::default())
        .to_string();

    one_acc_op.mis2 = acc_data_in
        .get_i64_for_key(&inputfieldnames.mis2)
        .unwrap_or(DEFAULT_INT)
        .to_string();

    one_acc_op.npa = acc_data_in
        .get_string_for_key(&inputfieldnames.npa_flg)
        .unwrap_or(&String::default())
        .to_string();

    one_acc_op.alm_line = acc_data_in
        .get_string_for_key(&inputfieldnames.alm_line)
        .unwrap_or(&String::default())
        .to_string();

    one_acc_op.input_benchmark = acc_data_in
        .get_string_for_key(&inputfieldnames.bm_id_lookup)
        .unwrap_or(&String::default())
        .to_string();

    one_acc_op.bc_as_on_rule = timestamp(cpd);
    one_acc_op.tenor_start_date_rule = timestamp(cpd);
    one_acc_op.tenor_end_date_rule = timestamp(ted);
    one_acc_op.bc_as_on_applied = timestamp(cpd);
    one_acc_op.tenor_end_date_rule = timestamp(cpd);
    one_acc_op.tenor_end_date_applied = timestamp(ted);
    one_acc_op.alm_concat = acc_data_in
        .get_string_for_key(&inputfieldnames.alm_concat)
        .unwrap_or(&String::default())
        .to_string();
    one_acc_op.two_point_concat = acc_data_in
        .get_string_for_key(&inputfieldnames.two_point_concat)
        .unwrap_or(&String::default())
        .to_string();

    let average_balance = acc_data_in
        .get_f64_for_key(&inputfieldnames.avg_bal)
        .unwrap_or(DEFAULT_FLOAT);

    let mut lst_bm: Vec<IntermediateBmPoints> = Vec::new();
    let mut _lst_out: Vec<String> = Vec::new();
    let mut _total_balance = 0.0;
    let mut _total_interest_ftp = 0.0;
    let mut _total_ftp = 0.0;
    let mut _ftp_rate = 0.0;
    let mut adj_rates_lock = vec![DEFAULT_FLOAT; 6];
    let mut adj_codes = vec![DEFAULT_INT; 6];

    let full_file_path = format!(
        "{}{}_{}.txt",
        basecurve_file_path,
        cpd.format("%d-%m-%Y"),
        basecurve
    );
    let bm_key = BmKey {
        date: cpd.format("%d-%m-%Y").to_string(),
        base_curve_id: basecurve,
    };
    let max_days_in_year =
        num_days_start_to_end(*to_date, increment_date_by_months(*to_date, (12) as u16));

    let mut base_rate = 0.0;
    let mut residual_days = 0;

    let mut intr_calc_days = 0;
    if Path::new(&full_file_path).exists() {
        if saved_bm_rates.contains_key(&bm_key) {
            lst_bm = bm_reader::get_new_bm_points(saved_bm_rates, bm_key).to_vec();
        } else {
            bm_reader::get_bm_points(&full_file_path, cpd, log, &mut lst_bm);
            saved_bm_rates.insert(bm_key, lst_bm.clone());
        }

        if ted > cpd {
            residual_days = num_days_start_to_end(cpd, ted);
        }

        if residual_days <= 0 {
            residual_days = 0;
        }

        if method == 1023 {
            residual_days = run_duration;
        }

        intr_calc_days = run_duration;

        let mut yield_rate: f64 = half_away_from_zero(
            yieldrate_calc::calc_yieldrate(&mut lst_bm, residual_days, cpd, log),
            rate_precision,
        );

        if yield_rate < 0.0 {
            yield_rate = 0.0
        }

        base_rate = yield_rate;
    } else {
        log_debug!(
            log,
            "File does not exist's in the path : {}. Hence Base rate will be zero for the account :{}",
            full_file_path, one_acc_op.account_number
        );
    }

    let mut fin_ftp_rate = base_rate;
    //Stamping adjustment rates.
    for i in 0..fix_lst_adjustments.len() {
        let adj_key = AdjKey::new(timestamp(*to_date), fix_lst_adjustments[i]);
        let mut is_present = false;
        let adj_rate = half_away_from_zero(
            match adj_rates.get(&adj_key) {
                Some(x) => {
                    is_present = true;
                    *x
                }
                None => {
                    log_debug!(
            log,
            "Adjustments does not exists for adjustment id :{}, date : {}, account id :{}", 
            fix_lst_adjustments[i], to_date, one_acc_op.account_number
        );
                    0.0
                }
            },
            rate_precision,
        );

        adj_rates_lock[i] = adj_rate;
        adj_codes[i] = fix_lst_adjustments[i] as i64;
        if is_present {
            adj_str.push_str(&format!("{}|{}|", fix_lst_adjustments[i], adj_rate));
        }
        fin_ftp_rate += adj_rate;
    }

    //Stamping variable adjustment rates.
    for i in 0..var_lst_adjustments.len() {
        let adj_key = AdjKey::new(timestamp(*to_date), var_lst_adjustments[i]);
        let mut is_present = false;
        let adj_rate = half_away_from_zero(
            match adj_rates.get(&adj_key) {
                Some(x) => {
                    is_present = true;
                    *x
                }
                None => {
                    log_debug!(
                log,
                "Adjustments does not exists for adjustment id :{}, date : {}, account id :{}", 
                var_lst_adjustments[i], to_date, one_acc_op.account_number
            );
                    0.0
                }
            },
            rate_precision,
        );
        let loop_count: i32 = fixed_adj_count + i as i32;
        adj_rates_lock[(loop_count) as usize] = adj_rate;
        adj_codes[loop_count as usize] = var_lst_adjustments[i] as i64;
        if is_present {
            adj_str.push_str(&format!("{}|{}|", var_lst_adjustments[i], adj_rate));
        }
        fin_ftp_rate += adj_rate;
    }

    fin_ftp_rate = half_away_from_zero(fin_ftp_rate, rate_precision);
    base_rate = half_away_from_zero(base_rate, rate_precision);

    one_acc_op.average_balance = average_balance;
    one_acc_op.int_rate = int_rate;
    one_acc_op.final_ftp_rate = fin_ftp_rate;
    one_acc_op.final_ftp_amt = half_away_from_zero(
        (average_balance * fin_ftp_rate * intr_calc_days as f64)
            / (max_days_in_year as f64 * 100.0),
        bal_precision,
    );
    one_acc_op.rate_curve = basecurve.to_string();
    one_acc_op.adj1 = adj_rates_lock[0];
    one_acc_op.adj2 = adj_rates_lock[1];
    one_acc_op.adj3 = adj_rates_lock[2];
    one_acc_op.adj4 = adj_rates_lock[3];
    one_acc_op.adj5 = adj_rates_lock[4];
    one_acc_op.adj6 = adj_rates_lock[5];
    one_acc_op.base_rate = base_rate;
    one_acc_op.method = String::from(method_name);
    one_acc_op.source_file_name = String::from("OD");
    one_acc_op.npa = match get_field_value(
        &acc_data_in,
        &input_reader,
        inputfieldnames.npa_flg.to_string(),
    ) {
        Ok(value) => value.to_string(),
        Err(_error) => "".to_string(),
    };
    one_acc_op.input_benchmark = match get_field_value(
        &acc_data_in,
        &input_reader,
        inputfieldnames.benchmark.to_string(),
    ) {
        Ok(value) => value.to_string(),
        Err(_error) => "".to_string(),
    };

    append_out(&one_acc_op, bal_precision, to_date, from_date, &adj_str)
}
