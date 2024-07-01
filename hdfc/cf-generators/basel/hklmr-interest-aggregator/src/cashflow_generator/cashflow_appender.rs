use super::account_as_cashflows::Account;
use super::derive_fields::get_derived_fields;
use cashflow_generator::account_field_names::AccFieldNames;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::{NaiveDate, NaiveDateTime};
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use statics::*;
use std::collections::HashMap;

pub fn append_data<'a>(
    mut account: AccountWithCFs,
    keys: &AccFieldNames,
    config_params: &ConfigurationParameters,
    diag_log: &Logger,
    mut to_bucket: i64,
    mut from_bucket: i64,
    int_basis: &str,
    cust_master: &HashMap<String, String>,
    deal_id_map: &mut HashMap<String, String>,
) -> Result<Account, String> {
    let mut mat_tenor: i64 = DEFAULT_INT;
    let mut int_tenor: i64 = DEFAULT_INT;
    let mut prin_amt: f64 = DEFAULT_FLOAT;
    let mut int_amt: f64 = DEFAULT_FLOAT;
    let mut int_amt_30_days: f64 = DEFAULT_FLOAT;
    let mut tot_int_amt: f64 = DEFAULT_FLOAT;

    let mut out_acc = Account::new();

    let deal_id = account
        .get_string_for_key(&keys.deal_id)
        .expect("Error while reading deal id.");
    if deal_id_map.contains_key(&deal_id.to_string()) {
        return Err("Duplicate record: Skip".to_string());
    } else {
        deal_id_map.insert(deal_id.to_string(), deal_id.to_string());
    }
    let ost_prin_amt = account
        .get_f64_for_key(&keys.ost_prin_amt)
        .expect("Error while reading outstanding principal amt.");
    let int_amt_aip = account
        .get_f64_for_key(&keys.int_amt_aip)
        .expect("Error while reading interest amount/AIP.");
    let int_rate = account
        .get_f64_for_key(&keys.int_rate)
        .expect("Error while reading int rate.");
    let mat_dt = account
        .get_i64_for_key(&keys.mat_dt)
        .expect("Error while reading maturity date.");
    let next_reset_dt = account
        .get_i64_for_key(&keys.next_reset_dt)
        .expect("Error while reading next reset date.");
    let cust_type: String;
    if cust_master.contains_key(deal_id) {
        cust_type = cust_master.get(deal_id).unwrap().to_string();
    } else {
        cust_type = "NONE".to_string();
    }
    // TODO: Change this unwrap_or to expect whhen penalty_amt comes in cf file
    let penalty_amt = account.get_f64_for_key(&keys.penalty_amt).unwrap_or(0.0);
    let ccy = account
        .get_string_for_key(&keys.ccy)
        .expect("Error while reading deal id.");

    get_derived_fields(
        &ost_prin_amt,
        &int_amt_aip,
        &int_rate,
        &mat_dt,
        &next_reset_dt,
        &penalty_amt,
        &mut mat_tenor,
        &mut int_tenor,
        &mut prin_amt,
        &mut int_amt,
        &mut int_amt_30_days,
        &mut tot_int_amt,
        &mut to_bucket,
        &mut from_bucket,
        &mut int_basis
            .parse::<i64>()
            .expect("could not convert int_basis to i64"),
        config_params.as_on_date(),
    );
    out_acc.deal_id = deal_id.to_string();
    out_acc.ost_prin_amt = ost_prin_amt;
    out_acc.int_amt_aip = int_amt_aip;
    out_acc.int_rate = int_rate;
    out_acc.mat_dt = mat_dt;
    out_acc.next_reset_dt = next_reset_dt;
    out_acc.mat_tenor = mat_tenor;
    out_acc.int_tenor = int_tenor;
    out_acc.prin_amt = prin_amt;
    out_acc.int_amt = int_amt;
    out_acc.int_amt_30_days = int_amt_30_days;
    out_acc.tot_int_amt = tot_int_amt;
    out_acc.ccy = ccy.to_string();
    out_acc.cust_type = cust_type;
    out_acc.pt_str_1 = account
        .get_string_for_key(&keys.pt_str_1)
        .unwrap_or(&String::from("NA"))
        .to_string();
    out_acc.pt_str_2 = account
        .get_string_for_key(&keys.pt_str_2)
        .unwrap_or(&String::from("NA"))
        .to_string();
    out_acc.pt_str_3 = account
        .get_string_for_key(&keys.pt_str_3)
        .unwrap_or(&String::from("NA"))
        .to_string();
    out_acc.pt_str_4 = account
        .get_string_for_key(&keys.pt_str_4)
        .unwrap_or(&String::from("NA"))
        .to_string();
    out_acc.pt_str_5 = account
        .get_string_for_key(&keys.pt_str_5)
        .unwrap_or(&String::from("NA"))
        .to_string();
    out_acc.pt_int_1 = account
        .get_i64_for_key(&keys.pt_int_1)
        .unwrap_or(DEFAULT_INT);
    out_acc.pt_int_2 = account
        .get_i64_for_key(&keys.pt_int_2)
        .unwrap_or(DEFAULT_INT);
    out_acc.pt_int_3 = account
        .get_i64_for_key(&keys.pt_int_3)
        .unwrap_or(DEFAULT_INT);
    out_acc.pt_int_4 = account
        .get_i64_for_key(&keys.pt_int_4)
        .unwrap_or(DEFAULT_INT);
    out_acc.pt_int_5 = account
        .get_i64_for_key(&keys.pt_int_5)
        .unwrap_or(DEFAULT_INT);
    out_acc.pt_f64_1 = account
        .get_f64_for_key(&keys.pt_f64_1)
        .unwrap_or(DEFAULT_FLOAT);
    out_acc.pt_f64_2 = account
        .get_f64_for_key(&keys.pt_f64_2)
        .unwrap_or(DEFAULT_FLOAT);
    out_acc.pt_f64_3 = account
        .get_f64_for_key(&keys.pt_f64_3)
        .unwrap_or(DEFAULT_FLOAT);
    out_acc.pt_f64_4 = account
        .get_f64_for_key(&keys.pt_f64_4)
        .unwrap_or(DEFAULT_FLOAT);
    out_acc.pt_f64_5 = account
        .get_f64_for_key(&keys.pt_f64_5)
        .unwrap_or(DEFAULT_FLOAT);
    Ok(out_acc)
}

fn naivedate_from_timestamp(t: i64) -> NaiveDate {
    let naive_date_time = NaiveDateTime::from_timestamp(t, 0);
    naive_date_time.date()
}
