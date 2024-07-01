use super::account::*;
use crate::configuration_parameters::ConfigurationParameters;
use crate::process::account_field_names::AccFieldNames;
use crate::process::bucket::*;
use crate::process::convert::*;
use crate::process::llg_key::LLGKey;
use crate::process::tenor::*;
use rbdate::date_from_timestamp;
use rbdate::num_days_start_to_end;
use sdb_dyn_proto_rdr::reader::account_with_cfs::{get_field_value, AccountWithCFs};
use sdb_dyn_proto_rdr::reader::Reader;
use std::collections::HashMap;

#[allow(dead_code, unused_imports)]
pub fn fetch_acc_data(
    alco_map: &mut HashMap<String, String>,
    org_tenor_map: &mut HashMap<Tenor, String>,
    res_tenor_map: &mut HashMap<Tenor, String>,
    ia_tenor_map: &mut HashMap<Tenor, String>,
    bucket_map: &mut HashMap<Bucket, String>,
    cat_map: &mut HashMap<String, String>,
    lcr_map: &mut HashMap<String, String>,
    wd_nwd_map: &mut HashMap<String, String>,
    matured_acc_map: &mut HashMap<String, String>,
    exrt: &f64,
    source: &str,
    account: &AccountWithCFs,
    input_reader: &mut Reader,
    keys: &AccFieldNames,
    config_params: &ConfigurationParameters,
    acc_no: &String,
    val_dt: &mut i64,
) -> AccData {
    let value_date = date_from_timestamp(*val_dt);
    let def_val = "NA".to_string();
    let acc_opn_dt = match get_field_value(&account, &input_reader, keys.acc_open_dt.to_string()) {
        Ok(val) => to_i64(val),
        Err(_err) => panic!("{}", _err),
    };
    if *val_dt == 0 {
        *val_dt = acc_opn_dt;
    }
    let mat_dt = match get_field_value(&account, &input_reader, keys.mat_dt.to_string()) {
        Ok(val) => to_i64(val),
        Err(_err) => panic!("{}", _err),
    };
    let mat_date = date_from_timestamp(mat_dt);

    //Keys for maps:
    let org_tenor_key = num_days_start_to_end(value_date, mat_date);
    let res_tenor_key = num_days_start_to_end(*config_params.as_on_date(), mat_date);
    let alm_line = match get_field_value(&account, &input_reader, keys.alm_line.to_string()) {
        Ok(val) => val,
        Err(_err) => panic!("{}", _err),
    };
    let cust_id = match get_field_value(&account, &input_reader, keys.cust_id.to_string()) {
        Ok(val) => to_i64(val),
        Err(_err) => panic!("{}", _err),
    };
    let prod_code = match get_field_value(&account, &input_reader, keys.prod_code.to_string()) {
        Ok(val) => val,
        Err(_err) => panic!("{}", _err),
    };
    let amount_initl_deposit =
        match get_field_value(&account, &input_reader, keys.amt_initl_deposit.to_string()) {
            Ok(val) => to_f64(val),
            Err(_err) => panic!("{}", _err),
        };
    let mut incr_roll_val = "Incremental";
    if matured_acc_map.contains_key(&acc_no.to_string()) && mat_date <= *config_params.as_on_date()
    {
        incr_roll_val = "Rollover";
    }
    let grp_key = LLGKey::new(
        *config_params.as_on_date(),
        date_from_timestamp(acc_opn_dt),
        value_date,
        mat_date,
        source.to_string(),
        match get_field_value(&account, &input_reader, keys.ccy.to_string()) {
            Ok(val) => val,
            Err(_err) => panic!("{}", _err),
        },
        match get_field_value(&account, &input_reader, keys.prod_code.to_string()) {
            Ok(val) => val,
            Err(_err) => panic!("{}", _err),
        },
        match get_field_value(&account, &input_reader, keys.mis1.to_string()) {
            Ok(val) => val,
            Err(_err) => panic!("{}", _err),
        },
        match get_field_value(&account, &input_reader, keys.gl_liability.to_string()) {
            Ok(val) => val,
            Err(_err) => panic!("{}", _err),
        },
        match get_field_value(&account, &input_reader, keys.gl_int_comp.to_string()) {
            Ok(val) => val,
            Err(_err) => panic!("{}", _err),
        },
        match get_field_value(&account, &input_reader, keys.concat.to_string()) {
            Ok(code) => code.to_string(),
            Err(_) => def_val.to_string(),
        },
        match get_field_value(&account, &input_reader, keys.division.to_string()) {
            Ok(val) => val,
            Err(_err) => panic!("{}", _err),
        },
        match get_field_value(&account, &input_reader, keys.alm_line.to_string()) {
            Ok(val) => val,
            Err(_err) => panic!("{}", _err),
        },
        match get_field_value(&account, &input_reader, keys.ia_line.to_string()) {
            Ok(val) => val,
            Err(_err) => panic!("{}", _err),
        },
        alco_map
            .entry(alm_line.to_string())
            .or_insert(def_val.to_string())
            .to_string(),
        get_tenor_desc(org_tenor_key, org_tenor_map),
        get_tenor_desc(res_tenor_key, res_tenor_map),
        get_tenor_desc(res_tenor_key, ia_tenor_map),
        get_amt_cat(amount_initl_deposit.to_string(), bucket_map),
        cat_map
            .entry(prod_code.to_string())
            .or_insert(def_val.to_string())
            .to_string(),
        incr_roll_val.to_string(),
        lcr_map
            .entry(cust_id.to_string())
            .or_insert(def_val.to_string())
            .to_string(),
        wd_nwd_map
            .entry(prod_code.to_string())
            .or_insert(def_val.to_string())
            .to_string(),
    );
    let rate = match get_field_value(&account, &input_reader, keys.rate.to_string()) {
        Ok(val) => to_f64(val),
        Err(_err) => panic!("{}", _err),
    };
    let rate_var = match get_field_value(&account, &input_reader, keys.rate_var.to_string()) {
        Ok(val) => to_f64(val),
        Err(_err) => panic!("{}", _err),
    };
    let rate_var2 = match get_field_value(&account, &input_reader, keys.rate_var2.to_string()) {
        Ok(val) => to_f64(val),
        Err(_err) => panic!("{}", _err),
    };

    let yld = rate + rate_var + rate_var2;
    let mut data = Val {
        rate,
        rate_var,
        rate_var2,
        amt_initl_dep: amount_initl_deposit,
        yld,
    };
    data.values_multiplied_by(*exrt);
    AccData { grp_key, data }
}
