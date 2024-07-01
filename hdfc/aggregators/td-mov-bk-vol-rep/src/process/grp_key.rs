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
    exrt: &f64,
    source: &str,
    account: AccountWithCFs,
    input_reader: &mut Reader,
    keys: &AccFieldNames,
    config_params: &ConfigurationParameters,
) -> AccData {
    let def_val = "NA".to_string();
    let opn_dt = match get_field_value(&account, &input_reader, keys.open_date.to_string()) {
        Ok(val) => to_i64(val),
        Err(_err) => panic!("{}", _err),
    };
    let mat_dt = match get_field_value(&account, &input_reader, keys.mat_date.to_string()) {
        Ok(val) => to_i64(val),
        Err(_err) => panic!("{}", _err),
    };
    let as_on_dt = config_params.as_on_date();
    let mat_date = date_from_timestamp(mat_dt);
    let open_date = date_from_timestamp(opn_dt);
    let days_res = num_days_start_to_end(*as_on_dt, mat_date);
    let days_org_ia = num_days_start_to_end(open_date, mat_date);
    let alm_line = match get_field_value(&account, &input_reader, keys.alm_line.to_string()) {
        Ok(val) => val,
        Err(_err) => panic!("{}", _err),
    };
    let bal_lcy = match get_field_value(&account, &input_reader, keys.bal_lcy.to_string()) {
        Ok(val) => to_f64(val),
        Err(_err) => panic!("{}", _err),
    };
    let cust_id = match get_field_value(&account, &input_reader, keys.cust_id.to_string()) {
        Ok(val) => val,
        Err(_err) => panic!("{}", _err),
    };
    let prod_code = match get_field_value(&account, &input_reader, keys.prod_code.to_string()) {
        Ok(val) => val,
        Err(_err) => panic!("{}", _err),
    };

    let int_comp = match get_field_value(&account, &input_reader, keys.int_comp.to_string()) {
        Ok(val) => to_f64(val),
        Err(_err) => panic!("{}", _err),
    };

    let amount = bal_lcy + int_comp;
    let grp_key = LLGKey::new(
        *config_params.as_on_date(),
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
            Ok(code) => code,
            Err(_err) => panic!("{}", _err),
        },
        match get_field_value(&account, &input_reader, keys.gl_liab.to_string()) {
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
        match get_field_value(&account, &input_reader, keys.div.to_string()) {
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
        get_tenor_desc(days_org_ia, org_tenor_map),
        get_tenor_desc(days_res, res_tenor_map),
        get_tenor_desc(days_org_ia, ia_tenor_map),
        get_amt_cat(amount.to_string(), bucket_map),
        cat_map
            .entry(prod_code.to_string())
            .or_insert(def_val.to_string())
            .to_string(),
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
        bal_lcy: bal_lcy,
        int_comp: int_comp,
        rate: rate,
        rate_var: rate_var,
        rate_var2: rate_var2,
        amt: amount,
        yld: yld,
    };
    data.values_multiplied_by(*exrt);
    AccData { grp_key, data }
}
