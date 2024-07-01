use super::*;
use normalize::account_field_names::AccFieldNames;
use rbdate::*;
use rbdate::{date_from_timestamp, get_days_from_month, timestamp, NaiveDate};
use sdb_dyn_proto_rdr::reader::account_with_cfs::{get_field_value, AccountWithCFs};
use sdb_dyn_proto_rdr::reader::Reader;

pub fn get_acc_fields(
    input_reader: &mut Reader,
    smry_aggr_data: &mut HashMap<String, AggregatedValue>,
    smry_aggr_data_report2: &mut HashMap<String, AggregatedValue_Report2>,
    account: &AccountWithCFs,
    keys: &AccFieldNames,
    as_on_dt: NaiveDate,
) {
    let val_dt = account
        .get_i64_for_key(&keys.value_date)
        .unwrap_or(timestamp(as_on_dt));
    let val_dt = date_from_timestamp(val_dt);
    let org_month = val_dt.format("%m-%Y").to_string();
    let mut aggr_key = String::new();
    aggr_key.push_str(&org_month);
    aggr_key.push('|');
    for field in keys.aggr_keys.iter() {
        aggr_key.push_str(
            &get_field_value(&account, &input_reader, field.to_string())
                .unwrap_or_else(|_| "".to_string())
                .trim()
                .to_string(),
        );
        aggr_key.push('|');
    }
    aggr_key.pop();

    let mut aggr_keys_report2 = String::new();
    for field in keys.aggr_keys_report2.iter() {
        aggr_keys_report2.push_str(
            &get_field_value(&account, &input_reader, field.to_string())
            .unwrap_or_else(|_| "".to_string())
            .trim()
            .to_string(),
        );
        aggr_keys_report2.push('|');
    }
    aggr_keys_report2.pop();

    let avg_bal = account
        .get_f64_for_key(&keys.average_balance)
        .unwrap_or(DEFAULT_FLOAT);
    let accr_int = account
        .get_f64_for_key(&keys.accr_int)
        .unwrap_or(DEFAULT_FLOAT);
    let base_rate = account
        .get_f64_for_key(&keys.base_rate)
        .unwrap_or(DEFAULT_FLOAT);
    let final_ftp_rate = account
        .get_f64_for_key(&keys.final_ftp_rate)
        .unwrap_or(DEFAULT_FLOAT);
    let current_outstanding_td = account
        .get_f64_for_key(&keys.current_outstanding_td)
        .unwrap_or(DEFAULT_FLOAT);
    let psl_amt = account
        .get_f64_for_key(&keys.psl_amt)
        .unwrap_or(DEFAULT_FLOAT);
    let ftp_with_psl_amt = account
        .get_f64_for_key(&keys.ftp_with_psl_amt)
        .unwrap_or(DEFAULT_FLOAT);
    let ftp_without_psl_amt = account
        .get_f64_for_key(&keys.ftp_without_psl_amt)
        .unwrap_or(DEFAULT_FLOAT);
    let smf_amt = account
        .get_f64_for_key(&keys.adj7)
        .unwrap_or(DEFAULT_FLOAT);
    let adj1 = account.get_f64_for_key(&keys.adj1).unwrap_or(DEFAULT_FLOAT);
    let adj2 = account.get_f64_for_key(&keys.adj2).unwrap_or(DEFAULT_FLOAT);
    let adj3 = account.get_f64_for_key(&keys.adj3).unwrap_or(DEFAULT_FLOAT);
    let adj4 = account.get_f64_for_key(&keys.adj4).unwrap_or(DEFAULT_FLOAT);
    let adj5 = account.get_f64_for_key(&keys.adj5).unwrap_or(DEFAULT_FLOAT);
    let adj6 = account.get_f64_for_key(&keys.adj6).unwrap_or(DEFAULT_FLOAT);
    let adj7 = account.get_f64_for_key(&keys.adj7).unwrap_or(DEFAULT_FLOAT);
    let adj8 = account.get_f64_for_key(&keys.adj8).unwrap_or(DEFAULT_FLOAT);
    let adj9 = account.get_f64_for_key(&keys.adj9).unwrap_or(DEFAULT_FLOAT);
    let adj10 = account.get_f64_for_key(&keys.adj10).unwrap_or(DEFAULT_FLOAT);
    let margin_amt = account.get_f64_for_key(&keys.margin_amt).unwrap_or(DEFAULT_FLOAT);
    let fixed_spread = account
        .get_f64_for_key(&keys.fixed_spread)
        .unwrap_or(DEFAULT_FLOAT);
    let variable_spread = account
        .get_f64_for_key(&keys.variable_spread)
        .unwrap_or(DEFAULT_FLOAT);
    let def_string = "NA".to_string();
    let dep_type = account.get_string_for_key(&keys.prod_code_type).unwrap_or(&def_string);
    let mis2 = account.get_string_for_key(&keys.mis2).unwrap_or(&def_string);
    let gr_ofs_gl_amt = account.get_f64_for_key(&keys.gr_ofs_gl_amt).unwrap_or(DEFAULT_FLOAT);
    let ui_ofs_gl_amt = account.get_f64_for_key(&keys.ui_ofs_gl_amt).unwrap_or(DEFAULT_FLOAT);
    let re_ofs_gl_amt = account.get_f64_for_key(&keys.re_ofs_gl_amt).unwrap_or(DEFAULT_FLOAT);
    let is_ofs_gl_amt = account.get_f64_for_key(&keys.is_ofs_gl_amt).unwrap_or(DEFAULT_FLOAT);
    let int_income_gl_amt = account.get_f64_for_key(&keys.int_income_gl_amt).unwrap_or(DEFAULT_FLOAT);
    let int_on_cancellation_amt = account.get_f64_for_key(&keys.int_cancellation_gl_amt).unwrap_or(DEFAULT_FLOAT);
    let overdue_int_gl_amt = account.get_f64_for_key(&keys.overdue_int_gl_amt).unwrap_or(DEFAULT_FLOAT);
    let woff_gl_amt = account.get_f64_for_key(&keys.woff_gl_amt).unwrap_or(DEFAULT_FLOAT);
    let rate_flag = account.get_string_for_key(&keys.rate_flag).unwrap_or(&def_string);
    let no_of_day_in_ason = get_days_from_month(as_on_dt) as f64;
    let max_days_in_year =
        num_days_start_to_end(as_on_dt, increment_date_by_months(as_on_dt, (12) as u16)) as f64;

    let aggr_val = AggregatedValue {
        ttl_crnt_ost_td: current_outstanding_td,
        ttl_avg_bal: avg_bal,
        ttl_accr_int: accr_int,
        ttl_base_tpr: (base_rate * avg_bal) / 100.0,
        ttl_adjs: ((adj1 + adj2 + adj3 + adj4 + adj5 + adj6 + adj7 + adj8 + adj9 + adj10) * avg_bal) / 100.0,
        ttl_fin_tpr: (avg_bal * final_ftp_rate * no_of_day_in_ason) / (max_days_in_year * 100.0),
        ttl_margin: margin_amt,
        ttl_spread: (fixed_spread + variable_spread) * avg_bal,
        ttl_psl_amt: psl_amt,
        ttl_ftp_amt: ftp_with_psl_amt,
        ttl_ftp_without_psl: ftp_without_psl_amt,
        ttl_additional_smf: (avg_bal * smf_amt * no_of_day_in_ason) / (max_days_in_year * 100.0),
        gr_ofs_gl_amt:gr_ofs_gl_amt,
        ui_ofs_gl_amt:ui_ofs_gl_amt,
        re_ofs_gl_amt:re_ofs_gl_amt,
        is_ofs_gl_amt:is_ofs_gl_amt,
        int_income_gl_amt:int_income_gl_amt,
        overdue_int_gl_amt:overdue_int_gl_amt,
        int_on_cancellation_gl_amt:int_on_cancellation_amt,
        woff_gl_amt:woff_gl_amt,
        count: 1,
    };

    let aggr_val_report_2 = AggregatedValue_Report2 {
        dep_type:dep_type.to_string(),
        mis2:mis2.to_string(),
        rate_flag:rate_flag.to_string(),
        ttl_avg_bal: avg_bal,
        ttl_accr_int: accr_int,
        ttl_base_tpr: base_rate,
        ttl_margin: margin_amt,
        ttl_psl: adj4+adj6+adj7,
        ttl_fin_tpr:final_ftp_rate,
        days_in_month: no_of_day_in_ason,
        days_in_year: max_days_in_year,
        weighted_yield: ((accr_int/avg_bal) * (max_days_in_year/no_of_day_in_ason)),
        weighted_base_ftp_rate:(base_rate / avg_bal) * 100.0,
        weighted_psl_rate:((adj4 + adj6 + adj7) /avg_bal) * 100.0,
        weighted_final_ftp_rate:(final_ftp_rate/avg_bal)*100.0,
        weighted_total_spread:((accr_int/avg_bal) * (max_days_in_year/no_of_day_in_ason)) - (final_ftp_rate/avg_bal)*100.0,
    };
    smry_aggr_data
        .entry(aggr_key)
        .and_modify(|data| data.add(aggr_val))
        .or_insert(aggr_val);

    smry_aggr_data_report2
        .entry(aggr_keys_report2)
        .and_modify(|data| data.add(aggr_val_report_2.clone()))
        .or_insert(aggr_val_report_2);

}
