use crate::{cp::CP, stamp_ftp::restructured_op::additional_struct::get_str};
use chrono::prelude::*;
use rbdate::*;
use sdb_dyn_proto_rdr::reader::account_with_cfs::{get_field_value, AccountWithCFs};
use stamp_ftp::read_adjustments::AdjKey;
use statics::*;
use std::collections::{HashMap, HashSet};

use self::additional_struct::AmbData;

use super::cfinput::AccFieldNames;

pub mod additional_struct;

pub fn calc_restructured_ftp(
    mut existing_op_str: String,
    amb_map: &HashMap<String, AmbData>,
    config_map: &HashMap<String, HashSet<String>>,
    cp: &CP,
    adj_rates: &HashMap<AdjKey, f64>,
    ews_flag: String,
    bdp_division: String,
    bdp_coa: String,
    adj_string: String,
    from_date: &NaiveDate,
    to_date: &NaiveDate,
) -> String {
    existing_op_str.pop();
    let fields: Vec<&str> = existing_op_str.split('|').collect();
    let as_on_date: &NaiveDate = cp.as_on_date();
    let as_on_month = cp.as_on_date().format("%b-%Y").to_string();
    let acct_id: String = fields[0].trim().to_string();
    let cust_name = fields[1].trim().to_string();
    let average_balance = fields[2].trim().to_string().parse::<f64>().unwrap_or(0.0);

    let default_amb_data = &Default::default();
    let amb_data: &AmbData = amb_map.get(&acct_id).unwrap_or(default_amb_data);

    let accr_int = amb_data.accr_int.clone();

    let days_in_months = rbdate::num_days_start_to_end(*from_date, *to_date) + 1;
    let days_in_year = rbdate::num_days_start_to_end(
        *to_date,
        rbdate::increment_date_by_months(*to_date, (12) as u16),
    );
    let accr_int_rate =
        (accr_int / average_balance) * (days_in_year as f64 / days_in_months as f64) * 100 as f64;

    let adj_vec: Vec<&str> = adj_string.split('|').collect();
    let mut adj_map: HashMap<String, String> = HashMap::new();
    let mut i = 0;
    while i < adj_vec.len() - 1 {
        adj_map.insert(adj_vec[i].to_string(), adj_vec[i + 1].to_string());
        i += 2;
    }

    let (final_adj_str, amts) = get_final_adj(&adj_map, config_map);
    let yield_to_call = fields[4].trim().to_string();
    let int_rate = fields[5].to_string();
    let base_rate = fields[6].to_string().parse::<f64>().unwrap_or(0.0);
    let final_adj_vec: Vec<&str> = final_adj_str.split('|').collect();
    let mut final_ftp_rates = base_rate;
    let mut adj_indx = 0;
    for ele in final_adj_vec.clone() {
        let amt = ele.parse::<f64>().unwrap_or(0.0);
        if adj_indx == 10 {
            break;
        }
        final_ftp_rates += amt;
        adj_indx += 1;
    }
    let mut ews_flag_up = "N".to_string();
    let adj4_amt_psl = amts.get(0).unwrap_or(&DEFAULT_FLOAT);
    let adj5_amt_ews = amts.get(1).unwrap_or(&DEFAULT_FLOAT);
    const val: f64 = -0.4;
    if adj5_amt_ews == &val {
        ews_flag_up = "Y".to_string();
    }
    let adj6_amt_smf = amts.get(2).unwrap_or(&DEFAULT_FLOAT);
    let adj1_amt_lp = amts.get(3).unwrap_or(&DEFAULT_FLOAT);
    let final_rate_without_psl = final_ftp_rates - (adj4_amt_psl + adj5_amt_ews + adj6_amt_smf);
    let final_ftp_amount = fields[22].to_string().parse::<f64>().unwrap_or(0.0);
    let psl_amt = (average_balance
        * (*adj4_amt_psl + *adj5_amt_ews + *adj6_amt_smf)
        * (days_in_months as f64 / (days_in_year as f64))
        / 100 as f64);
    let ftp_amount_without_psl = final_ftp_amount - psl_amt;
    let margin_amt = accr_int - ftp_amount_without_psl;
    let base_tpr_amt =
        (average_balance * base_rate * (days_in_months as f64 / (days_in_year as f64)))
            / 100 as f64;
    let tot_lp_amt =
        (average_balance * adj1_amt_lp * (days_in_months as f64 / (days_in_year as f64)))
            / 100 as f64;
    let tot_psl_amt_without_ews_smf: f64 =
        (adj4_amt_psl * average_balance * (days_in_months as f64 / (days_in_year as f64)))
            / 100 as f64;
    let tot_ews_amt =
        (adj5_amt_ews * average_balance * (days_in_months as f64 / (days_in_year as f64)))
            / 100 as f64;
    let tot_smf_amt =
        (adj6_amt_smf * average_balance * (days_in_months as f64 / (days_in_year as f64)))
            / 100 as f64;
    let margin_rate = (margin_amt / average_balance)
        / (days_in_months as f64 / (days_in_year as f64))
        * 100 as f64;

    let val_date = fields[8].trim().to_string();
    let mat_date = fields[9].trim().to_string();
    let last_rep_date = fields[10].trim().to_string();
    let next_rep_date = fields[11].trim().to_string();
    let mis1 = fields[12].trim().to_string();
    let mis2 = fields[13].trim().to_string();
    let psl_code = fields[14].trim().to_string();
    let prod_code = fields[15].trim().to_string();
    let rate_flag = fields[16].trim().to_string();
    let branch = fields[17].to_string();
    let src_file_name = fields[18].to_string();
    let currency = fields[19].to_string();
    let gl_code = fields[20].to_string();
    let cust_id = fields[21].to_string();
    let alm_line = fields[23].to_string();
    let trade_date = fields[24].to_string();
    let intial_dep_amt = fields[25].to_string();
    let currenct_outs_balance = fields[26].to_string();
    let input_benchmark = fields[34].to_string();
    let pdo = fields[35].to_string();
    let npa = fields[36].to_string();
    let ftp_method = fields[37].to_string();
    let ftp_rate_curve = fields[38].to_string();
    let org_tenor = fields[39].to_string();
    let rep_tenor = fields[40].to_string();
    let fixed_spread = fields[41].to_string();
    let variable_spread = fields[42].to_string();
    let first_month_ftp = fields[43].to_string();
    let bc_as_on_rule = fields[44].to_string();
    let tenor_start_date_rule = fields[45].to_string();
    let tenor_end_date_rule = fields[46].to_string();
    let bc_as_on_applied = fields[47].to_string();
    let tenor_start_date_applied = fields[48].to_string();
    let tenor_end_date_applied = fields[49].to_string();
    let concat_4_points = fields[50].to_string();
    let concat_2_points = fields[51].to_string();

    let add_op_str = format!("{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
      as_on_month,
      acct_id,
      cust_name,
      average_balance,
      accr_int,
      accr_int_rate,
      yield_to_call,
      int_rate,
      base_rate,
      final_adj_vec[0].to_string(),
      final_adj_vec[1].to_string(),
      final_adj_vec[2].to_string(),
      final_adj_vec[3].to_string(),
      final_adj_vec[4].to_string(),
      final_adj_vec[5].to_string(),
      final_adj_vec[6].to_string(),
      final_adj_vec[7].to_string(),
      final_adj_vec[8].to_string(),
      final_adj_vec[9].to_string(),
      final_ftp_rates,
      final_rate_without_psl,
      margin_rate,
      base_tpr_amt,
      final_ftp_amount,
      ftp_amount_without_psl,
      psl_amt,
      tot_lp_amt,
      tot_psl_amt_without_ews_smf,
      tot_ews_amt,
      tot_smf_amt,
      margin_amt,
      val_date,
      mat_date,
      last_rep_date,
      next_rep_date,
      mis1,
      mis2,
      psl_code,
      prod_code,
      rate_flag,
      branch,
      src_file_name,
      currency,
      gl_code,
      cust_id,
      alm_line,
      trade_date,
      intial_dep_amt,
      currenct_outs_balance,
      input_benchmark,
      pdo,
      npa,
      ftp_method,
      ftp_rate_curve,
      org_tenor,
      rep_tenor,
      fixed_spread,
      variable_spread,
      first_month_ftp,
      bc_as_on_rule,
      tenor_start_date_rule,
      tenor_end_date_rule,
      bc_as_on_applied,
      tenor_start_date_applied,
      tenor_end_date_applied,
      concat_4_points,
      concat_2_points,
      ews_flag_up,
      bdp_division,
      bdp_coa,
      final_adj_vec[10].to_string(),
      final_adj_vec[11].to_string(),
      final_adj_vec[12].to_string(),
      final_adj_vec[13].to_string(),
      final_adj_vec[14].to_string(),
      final_adj_vec[15].to_string(),
      final_adj_vec[16].to_string(),
      final_adj_vec[17].to_string(),
      final_adj_vec[18].to_string(),
      final_adj_vec[19].to_string(),
      );
    return add_op_str;
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
    let adj1_amt = amounts.get("C1").unwrap_or(&0.0);

    let amts = vec![*adj4_amt, *adj5_amt, *adj6_amt, *adj1_amt];
    // Remove trailing "|"
    result.pop();
    result_id.pop();

    let final_str = format!("{}|{}", result, result_id);

    (final_str, amts)
}
pub fn get_date_str(date: i64) -> String {
    let start_date = NaiveDateTime::from_timestamp(date, 0)
        .date()
        .format("%d-%m-%Y");

    start_date.to_string()
}
