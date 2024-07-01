use std::collections::{HashMap, HashSet};

use chrono::{self, Datelike};
use math::round::half_away_from_zero;
use rbdate::{num_days_start_to_end, timestamp_to_naivedate, NaiveDate, NaiveDateTime};
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use stamp_ftp::one_acc_view::One_acc_view;

use crate::statics::DEFAULT_FLOAT;

use super::{cfinput::AccFieldNames, CFout::AccountWithCashflows};

pub fn append_out(
    one_acc_op: &One_acc_view,
    bal_precision: i8,
    to_date: &NaiveDate,
    from_date: &NaiveDate,
) -> String {
    let mut op_line: String = String::new();
    let days_in_month = rbdate::num_days_start_to_end(*from_date, *to_date) + 1;
    let days_in_year = rbdate::num_days_start_to_end(
        *to_date,
        rbdate::increment_date_by_months(*to_date, (12) as u16),
    );
    let yld_to_call: f64 = (one_acc_op.accr_int / one_acc_op.average_balance)
        * 100.0 as f64
        * (days_in_year as f64 / days_in_month as f64);
    op_line = format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|\
    {}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        one_acc_op.account_number,
        one_acc_op.cust_name,
        one_acc_op.average_balance,
        one_acc_op.accr_int,
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
        one_acc_op.repr_spread,
        one_acc_op.source_file_name,
        one_acc_op.ccy,
        one_acc_op.gl,
        one_acc_op.cust_id,
        one_acc_op.final_ftp_amt,
        one_acc_op.alm_line,
        one_acc_op.trade_dt,
        one_acc_op.init_dep_amt,
        one_acc_op.outstanding_bal,
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
    );

    op_line
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
    ubs_lock_map: &HashMap<String, String>,
) -> String {
    let mut op_line: String = String::new();
    let days_in_month = rbdate::num_days_start_to_end(*from_date, *to_date) + 1;
    let days_in_year = rbdate::num_days_start_to_end(
        *to_date,
        rbdate::increment_date_by_months(*to_date, (12) as u16),
    );
    let yld_to_call: f64 = (one_acc_op.accr_int / one_acc_op.average_balance)
        * 100.0 as f64
        * (days_in_year as f64 / days_in_month as f64);

    let asonmonth = asondate.format("%m-%Y").to_string();
    let def_str = "NA".to_string();
    let bdp_div = acc_data
        .get_string_for_key(&inputfields.bdp_division)
        .unwrap_or(&def_str)
        .to_string();
    let bdp_coa = acc_data
        .get_string_for_key(&inputfields.bdp_coa)
        .unwrap_or(&def_str)
        .to_string();
    let concat = acc_data
        .get_string_for_key(&inputfields.concat)
        .unwrap_or(&def_str)
        .to_string();
    let concat_two_point = acc_data
        .get_string_for_key(&inputfields.concat_two_point)
        .unwrap_or(&def_str)
        .to_string();
    let psl_category = acc_data
        .get_string_for_key(&inputfields.psl_category)
        .unwrap_or(&def_str)
        .to_string();
    let retail_wholesale = acc_data
        .get_string_for_key(&inputfields.retail_wholesale)
        .unwrap_or(&def_str)
        .to_string();
    let benchmark = acc_data
        .get_string_for_key(&inputfields.benchmark)
        .unwrap_or(&def_str)
        .to_string();
    let benchmark_manual = acc_data
        .get_f64_for_key(&inputfields.benchmark_manual)
        .unwrap_or(0.0)
        .to_string();
    let lcy_amount = acc_data
        .get_f64_for_key(&inputfields.lcy_amount)
        .unwrap_or(0.0)
        .to_string();
    let benchmark_spread = acc_data
        .get_string_for_key(&inputfields.benchmark_spread)
        .unwrap_or(&def_str)
        .to_string();
    let call_dt = acc_data.get_i64_for_key(&inputfields.call_dt).unwrap_or(0);
    let put_dt = acc_data.get_i64_for_key(&inputfields.put_dt).unwrap_or(0);
    let gl_description = acc_data
        .get_string_for_key(&inputfields.gl_description)
        .unwrap_or(&def_str)
        .to_string();
    let cntr_party = acc_data
        .get_string_for_key(&inputfields.cntr_party)
        .unwrap_or(&def_str)
        .to_string();
    let prod_desc = acc_data
        .get_string_for_key(&inputfields.prod_desc)
        .unwrap_or(&def_str)
        .to_string();
    let user_def_stats = acc_data
        .get_string_for_key(&inputfields.user_def_stats)
        .unwrap_or(&def_str)
        .to_string();
    let book_dt = acc_data
    .get_i64_for_key(&inputfields.book_dt)
    .unwrap_or(0);
    let lst_rpr_dt_udf = acc_data
        .get_i64_for_key(&inputfields.lst_reprice_dt_udf)
        .unwrap_or(0);
    let nxt_rpr_dt_udf = acc_data
        .get_i64_for_key(&inputfields.nxt_reprice_dt_udf)
        .unwrap_or(0);
    let lst_rpr_dt = acc_data
        .get_i64_for_key(&inputfields.lst_repricing_dt)
        .unwrap_or(0);
    let nxt_rpr_dt = acc_data
        .get_i64_for_key(&inputfields.nxt_repricing_dt)
        .unwrap_or(0);
    let der_int_rate = acc_data
        .get_f64_for_key(&inputfields.der_int_rate)
        .unwrap_or(0.0);
    let yld_grp_al = acc_data
        .get_string_for_key(&inputfields.yld_grp_al)
        .unwrap_or(&def_str)
        .to_string();
    let frequency = acc_data
        .get_string_for_key(&inputfields.frequency)
        .unwrap_or(&def_str)
        .to_string();
    let old_rt_typ = acc_data
        .get_string_for_key(&inputfields.old_rt_typ)
        .unwrap_or(&def_str)
        .to_string();
    let der_rate_flag = acc_data
        .get_string_for_key(&inputfields.der_rate_flag)
        .unwrap_or(&def_str)
        .to_string();
    let source_file_name = acc_data
        .get_string_for_key(&inputfields.source_file_name)
        .unwrap_or(&def_str)
        .to_string();
    let days = days_in_asonmonth(asondate);
    let ftp_wth_psl_amt =
        (one_acc_op.average_balance * one_acc_op.final_ftp_rate * (days as f64 / (days_in_year as f64 * 100.00))); 
    let adj_vec: Vec<&str> = adj_string.split('|').collect();
    let mut adj_map: HashMap<String, String> = HashMap::new();
    let mut i = 0;
    while i < adj_vec.len() - 1 {
        adj_map.insert(adj_vec[i].to_string(), adj_vec[i + 1].to_string());
        i += 2;
    }

    let (final_adj_str, amts) = get_final_adj(&adj_map, config_map);
    let adj4_amt = amts.get(0).unwrap_or(&DEFAULT_FLOAT);
    let adj5_amt = amts.get(1).unwrap_or(&DEFAULT_FLOAT);
    let adj6_amt = amts.get(2).unwrap_or(&DEFAULT_FLOAT);

    let anchor_spread = ubs_lock_map.get(&cf_data.reference).unwrap_or(&def_str);
    let psl_amt = (one_acc_op.average_balance * (*adj4_amt + *adj5_amt + *adj6_amt) * (days as f64 / (days_in_year as f64 * 100.00)));

    let ftp_wthout_psl_amt = ftp_wth_psl_amt - psl_amt;
    op_line = format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|\
    {}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|\
    {}|{}|{}|{}|{}|{}|{}|{}|{}",
        asonmonth,
        one_acc_op.account_number,
        one_acc_op.cust_name,
        one_acc_op.int_rate,
        benchmark,
        benchmark_manual,
        benchmark_spread,
        get_date_str(book_dt),
        get_date_str(one_acc_op.value_date),
        get_date_str(one_acc_op.maturity_date),
        get_date_str(lst_rpr_dt_udf),
        get_date_str(nxt_rpr_dt_udf),
        get_date_str(call_dt),
        get_date_str(put_dt),
        get_date_str(lst_rpr_dt),
        get_date_str(nxt_rpr_dt),
        frequency,
        one_acc_op.mis1,
        one_acc_op.mis2,
        one_acc_op.prod_type,
        old_rt_typ,
        one_acc_op.ccy,
        one_acc_op.gl,
        gl_description,
        cntr_party,
        lcy_amount,
        one_acc_op.outstanding_bal,
        prod_desc,
        user_def_stats,
        bdp_div,
        bdp_coa,
        one_acc_op.average_balance,
        one_acc_op.accr_int,
        half_away_from_zero(yld_to_call, bal_precision),
        one_acc_op.base_rate,
        one_acc_op.final_ftp_rate,
        der_rate_flag,
        source_file_name,
        one_acc_op.final_ftp_amt,
        gl_description,
        der_int_rate - one_acc_op.final_ftp_rate,
        anchor_spread,
        timestamp_to_naivedate(one_acc_op.value_date)
            .format("%m-%Y")
            .to_string(),
        one_acc_op.accr_int - ftp_wthout_psl_amt,
        yld_grp_al,
        concat,
        concat_two_point,
        one_acc_op.accr_int,
        "",
        one_acc_op.int_rate - one_acc_op.final_ftp_rate,
        psl_category,
        "",
        "",
        retail_wholesale,
        one_acc_op.method,
        ftp_wth_psl_amt,
        psl_amt,
        ftp_wthout_psl_amt,
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
