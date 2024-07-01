use std::collections::HashMap;
use std::collections::HashSet;

use chrono::Datelike;
use math::round::half_away_from_zero;
use rbdate::NaiveDate;
use rbdate::NaiveDateTime;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use stamp_ftp::one_acc_view::One_acc_view;

use crate::ftp_parameters;

use super::cfinput::AccFieldNames;
use super::CFout::AccountWithCashflows;

pub fn append_out(
    one_acc_op: &One_acc_view,
    acc_data: &AccountWithCFs,
    inputfields: &AccFieldNames,
    bal_precision: i8,
    to_date: &NaiveDate,
    from_date: &NaiveDate,
) -> String {
    let def_double = 0.0;
    let days_in_month = rbdate::num_days_start_to_end(*from_date, *to_date) + 1;
    let days_in_year = rbdate::num_days_start_to_end(
        *to_date,
        rbdate::increment_date_by_months(*to_date, (12) as u16),
    );
    let gr_ofs_gl_amt = (acc_data.get_f64_for_key(&inputfields.gr_cr).unwrap_or(def_double)) - (acc_data.get_f64_for_key(&inputfields.gr_dr).unwrap_or(def_double));
    let ui_ofs_gl_amt = (acc_data.get_f64_for_key(&inputfields.ui_cr).unwrap_or(def_double)) - (acc_data.get_f64_for_key(&inputfields.ui_dr).unwrap_or(def_double));
    let re_ofs_gl_amt = (acc_data.get_f64_for_key(&inputfields.re_cr).unwrap_or(def_double)) - (acc_data.get_f64_for_key(&inputfields.re_dr).unwrap_or(def_double));
    let is_ofs_gl_amt = (acc_data.get_f64_for_key(&inputfields.is_cr).unwrap_or(def_double)) - (acc_data.get_f64_for_key(&inputfields.is_dr).unwrap_or(def_double));

    let accr_intt = gr_ofs_gl_amt + ui_ofs_gl_amt + re_ofs_gl_amt + is_ofs_gl_amt;
    let yld_to_call: f64 = (accr_intt / one_acc_op.average_balance)
        * 100.0 as f64
        * (days_in_year as f64 / days_in_month as f64);

    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|\
        {}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        one_acc_op.account_number,
        one_acc_op.cust_name,
        half_away_from_zero(one_acc_op.average_balance, bal_precision),
        half_away_from_zero(accr_intt, bal_precision),
        half_away_from_zero(yld_to_call, bal_precision),
        one_acc_op.int_rate,
        one_acc_op.base_rate,
        one_acc_op.final_ftp_rate,
        get_date_str(one_acc_op.value_date),
        get_date_str(one_acc_op.maturity_date),
        get_date_str(one_acc_op.lst_rep_dt),
        get_date_str(one_acc_op.nxt_rep_dt),
        one_acc_op.mis1,
        one_acc_op.mis2,
        one_acc_op.psl_code,
        one_acc_op.prod_type,
        one_acc_op.rate_flag,
        one_acc_op.branch,
        one_acc_op.source_file_name,
        one_acc_op.ccy,
        one_acc_op.gl,
        one_acc_op.cust_id,
        one_acc_op.final_ftp_amt,
        one_acc_op.alm_line,
        one_acc_op.trade_dt,
        one_acc_op.orig_bal,
        half_away_from_zero(one_acc_op.outstanding_bal, bal_precision),
        one_acc_op.base_rate,
        one_acc_op.adj1,
        one_acc_op.adj2,
        one_acc_op.adj3,
        one_acc_op.adj4,
        one_acc_op.adj5,
        one_acc_op.adj6,
        one_acc_op.input_benchmark,
        one_acc_op.pdo,
        one_acc_op.npa,
        one_acc_op.method,
        one_acc_op.rate_curve,
        one_acc_op.org_tenor,
        one_acc_op.rep_tenor,
        one_acc_op.fx_spread,
        one_acc_op.var_spread,
        one_acc_op.first_ftp,
        get_date_str(one_acc_op.bc_as_on_rule),
        get_date_str(one_acc_op.tenor_start_date_rule),
        get_date_str(one_acc_op.tenor_end_date_rule),
        get_date_str(one_acc_op.bc_as_on_applied),
        get_date_str(one_acc_op.tenor_start_date_applied),
        get_date_str(one_acc_op.tenor_end_date_applied),
    )
}

pub fn additional_append_out(
    one_acc_op: &One_acc_view,
    bal_precision: i8,
    to_date: &NaiveDate,
    from_date: &NaiveDate,
    adj_string: String,
    cf_data: &AccountWithCashflows,
    acc_data: &AccountWithCFs,
    inputfields: &AccFieldNames,
    config_map: &HashMap<String, HashSet<String>>,
    asondate: &NaiveDate,
    spread_map: &HashMap<String,String>
) -> String {
    let mut op_line: String = String::new();
    let def_double = 0.0;
    let def_str="NA".to_string();
    let days_in_month = days_in_asonmonth(asondate);
    let days_in_year = rbdate::num_days_start_to_end(
        *to_date,
        rbdate::increment_date_by_months(*to_date, (12) as u16),
    );
    
    //get_fields:
    let ftp_month = asondate.format("%B-%Y").to_string();
    let scheme_id = acc_data.get_string_for_key(&inputfields.scheme_id).unwrap_or(&def_str).to_string();
    let bdp = acc_data.get_string_for_key(&inputfields.division).unwrap_or(&def_str).to_string();
    let coa = acc_data.get_string_for_key(&inputfields.coa).unwrap_or(&def_str).to_string();
    let acc_num = acc_data.get_string_for_key(&inputfields.account_number).unwrap_or(&def_str).to_string();
    let cust_name = acc_data.get_string_for_key(&inputfields.customer_name).unwrap_or(&def_str).to_string();
    let avg_bal = one_acc_op.average_balance;

    let int_rate = one_acc_op.int_rate;
    let base_rate = one_acc_op.base_rate;
    let final_ftp_rate = one_acc_op.final_ftp_rate;
    let gr_ofs_gl = acc_data.get_string_for_key(&inputfields.gr_ofs_gl).unwrap_or(&def_str).to_string();
    let ui_ofs_gl = acc_data.get_string_for_key(&inputfields.ui_ofs_gl).unwrap_or(&def_str).to_string();
    let re_ofs_gl = acc_data.get_string_for_key(&inputfields.re_ofs_gl).unwrap_or(&def_str).to_string();
    let is_ofs_gl = acc_data.get_string_for_key(&inputfields.is_ofs_gl).unwrap_or(&def_str).to_string();

    let gr_ofs_gl_amt = (acc_data.get_f64_for_key(&inputfields.gr_cr).unwrap_or(def_double)) - (acc_data.get_f64_for_key(&inputfields.gr_dr).unwrap_or(def_double));
    let ui_ofs_gl_amt = (acc_data.get_f64_for_key(&inputfields.ui_cr).unwrap_or(def_double)) - (acc_data.get_f64_for_key(&inputfields.ui_dr).unwrap_or(def_double));
    let re_ofs_gl_amt = (acc_data.get_f64_for_key(&inputfields.re_cr).unwrap_or(def_double)) - (acc_data.get_f64_for_key(&inputfields.re_dr).unwrap_or(def_double));
    let is_ofs_gl_amt = (acc_data.get_f64_for_key(&inputfields.is_cr).unwrap_or(def_double)) - (acc_data.get_f64_for_key(&inputfields.is_dr).unwrap_or(def_double));

    let accr_intt = gr_ofs_gl_amt + ui_ofs_gl_amt + re_ofs_gl_amt + is_ofs_gl_amt;
    let yield_to_call: f64 = (accr_intt / one_acc_op.average_balance)
        * 100.0 as f64
        * (days_in_year as f64 / days_in_month as f64);

    let int_income_gl = acc_data.get_string_for_key(&inputfields.int_income_gl).unwrap_or(&def_str).to_string();
    let overdue_int_gl = acc_data.get_string_for_key(&inputfields.overdue_int_gl).unwrap_or(&def_str).to_string();
    let int_on_can_gl = acc_data.get_string_for_key(&inputfields.int_on_cancellation_gl).unwrap_or(&def_str).to_string();
    let writeoff_gl = acc_data.get_string_for_key(&inputfields.writeoff_gl).unwrap_or(&def_str).to_string();

    let int_income_gl_amt = acc_data.get_f64_for_key(&inputfields.int_income_gl_amt).unwrap_or(0.0);
    let overdue_int_gl_amt = acc_data.get_f64_for_key(&inputfields.overdue_int_gl_amt).unwrap_or(0.0);
    let int_on_can_gl_amt = acc_data.get_f64_for_key(&inputfields.int_on_cancellation_gl_amt).unwrap_or(0.0);
    let writeoff_gl_amt = acc_data.get_f64_for_key(&inputfields.writeoff_gl_amt).unwrap_or(0.0);

    let value_date = one_acc_op.value_date;
    let mat_date = one_acc_op.maturity_date;
    let lrd = one_acc_op.lst_rep_dt;
    let nrd = one_acc_op.nxt_rep_dt;
    let mis1 = &one_acc_op.mis1;
    let mis2 = &one_acc_op.mis2;
    let psl_code = &one_acc_op.psl_code;
    let prod_code = acc_data.get_string_for_key(&inputfields.product_code).unwrap_or(&def_str).to_string();
    let rate_flag = acc_data.get_string_for_key(&inputfields.rate_flag).unwrap_or(&def_str).to_string();
    let source_file_name = &one_acc_op.source_file_name;
    let currency = &one_acc_op.ccy;
    let gl_code = &one_acc_op.gl;
    let cust_id = &one_acc_op.cust_id;
    let alm_line = &one_acc_op.alm_line;
    let trade_dt = &one_acc_op.trade_dt;
    let initial_dep_amt = one_acc_op.orig_bal;
    let curr_outstanding_bal = one_acc_op.outstanding_bal;
    let input_benchmark = &one_acc_op.input_benchmark;
    let pdo = &one_acc_op.pdo;
    let npa = &one_acc_op.npa;
    let ftp_method = &one_acc_op.method;
    let ftp_rate_curve = &one_acc_op.rate_curve;
    let org_tenor = one_acc_op.org_tenor;
    let rep_tenor = one_acc_op.rep_tenor;
    let fixed_spread = one_acc_op.fx_spread;
    let var_spread = one_acc_op.var_spread;
    let first_month_ftp = one_acc_op.first_ftp;
    let anchor_spread = match spread_map.get(&acc_num) {
        Some(val) => val,
        None => "NA"
    };
    let anchor_month = rbdate::date_from_timestamp(value_date).format("%B-%Y").to_string();
    let ftp_wth_psl_amt =
        (one_acc_op.average_balance * one_acc_op.final_ftp_rate * (days_in_month as f64 / (days_in_year as f64 * 100.00))); 
    let adj_vec: Vec<&str> = adj_string.split('|').collect();
    let mut adj_map: HashMap<String, String> = HashMap::new();
    let mut i = 0;
    while i < adj_vec.len() - 1 {
        adj_map.insert(adj_vec[i].to_string(), adj_vec[i + 1].to_string());
        i += 2;
    }

    let (final_adj_str, amts) = get_final_adj(&adj_map, config_map);
    let adj4_amt = amts.get(0).unwrap_or(&0.0);
    let adj5_amt = amts.get(1).unwrap_or(&0.0);
    let adj6_amt = amts.get(2).unwrap_or(&0.0);
    let psl_amt = one_acc_op.average_balance * (*adj4_amt + *adj5_amt + *adj6_amt) * (days_in_month as f64 / (days_in_year as f64 * 100.00));
    let ftp_wthout_psl_amt = ftp_wth_psl_amt - psl_amt;
    let margin_amt = accr_intt - ftp_wthout_psl_amt;
    let mut ews_flag = "N".to_string();
    if *adj5_amt == -0.4 {
        ews_flag = "Y".to_string();
    }
    op_line = format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
         ftp_month,
         scheme_id,
         bdp,
         coa,
         acc_num,
         cust_name,
         avg_bal,
         accr_intt,
         yield_to_call,
         int_rate,
         base_rate,
         final_ftp_rate,
         gr_ofs_gl,
         ui_ofs_gl,
         re_ofs_gl,
         is_ofs_gl,
         gr_ofs_gl_amt,
         ui_ofs_gl_amt,
         re_ofs_gl_amt,
         is_ofs_gl_amt,
         int_income_gl,
         overdue_int_gl,
         int_on_can_gl,
         writeoff_gl,
         int_income_gl_amt,
         overdue_int_gl_amt,
         int_on_can_gl_amt,
         writeoff_gl_amt,
         get_date_str(value_date),
         get_date_str(mat_date),
         get_date_str(lrd),
         get_date_str(nrd),
         mis1,
         mis2,
         psl_code,
         prod_code,
         rate_flag,
         source_file_name,
         currency,
         gl_code,
         cust_id,
         alm_line,
         trade_dt,
         initial_dep_amt,
         curr_outstanding_bal,
         input_benchmark,
         pdo,
         npa,
         ftp_method,
         ftp_rate_curve,
         org_tenor,
         rep_tenor,
         fixed_spread,
         var_spread,
         first_month_ftp,
         ews_flag,
         anchor_spread,
         anchor_month,
         ftp_wth_psl_amt,
         psl_amt,
         ftp_wthout_psl_amt,
         margin_amt,
         final_adj_str
    );

    op_line
}

pub fn get_date_str(date: i64) -> String {
    let start_date = NaiveDateTime::from_timestamp(date, 0)
        .date()
        .format("%d-%m-%Y");

    start_date.to_string()
}

pub fn days_in_asonmonth(date: &NaiveDate) -> i64 {
    let month = date.month();
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            let year = date.year();
            // February
            if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                29 // Leap year
            } else {
                28 // Non-leap year
            }
        }
        _ => 0, // Invalid month
    }
}

pub fn get_final_adj(
    adj_map: &HashMap<String, String>,
    config_map: &HashMap<String, HashSet<String>>,
) -> (String, Vec<f64>) {
    let mut amounts: HashMap<String, f64> = HashMap::new();
    let mut ids: HashMap<String, i32> = HashMap::new();
    let mut adj4_amt = 0.0;
    for (id, amt) in adj_map {
        for (key, ids_set) in config_map {
            if ids_set.contains(id) {
                let entry_amt = amounts.entry(key.clone()).or_insert(0.0);
                *entry_amt += amt.parse::<f64>().unwrap_or(0.0);

                let entry_id = ids.entry(key.clone()).or_insert(0);
                *entry_id = id.parse::<i32>().unwrap_or(0);
            }
        }
    }

    let mut result = String::new();
    let mut result_id = String::new();
    for key in ["C1", "C2", "C3", "C4", "C5", "C6", "C7", "C8", "C9", "C10"].iter() {
        let amt = amounts.get(*key).unwrap_or(&0.0);
        let id = ids.get(*key).unwrap_or(&0);
        result.push_str(&format!("{}|", amt));
        result_id.push_str(&format!("{}|", id));
    }
    let adj4_amt = amounts.get("C4").unwrap_or(&0.0);
    let adj5_amt = amounts.get("C5").unwrap_or(&0.0);
    let adj6_amt = amounts.get("C6").unwrap_or(&0.0);
    let amts = vec![*adj4_amt,*adj5_amt,*adj6_amt];
    // Remove trailing "|"
    result.pop();
    result_id.pop();

    let final_str = format!("{}|{}", result, result_id);

    (final_str, amts)
}
