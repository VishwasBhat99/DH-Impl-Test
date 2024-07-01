use super::*;
use crate::configuration_parameters::ConfigurationParameters;
use sdb_dyn_proto_rdr::reader::account_with_cfs::get_field_value;

pub fn append_input_fields(
    one_acc_out: &mut OneAccountView,
    account: &AccountWithCFs,
    input_reader: &mut Reader,
    keys: &AccFieldNames,
    config_params: &ConfigurationParameters,
) {
    one_acc_out.as_on_month = timestamp(*config_params.to_date());
    one_acc_out.account_id =
        match get_field_value(&account, &input_reader, keys.account_id.to_string()) {
            Ok(value) => value,
            Err(_error) => panic!("{}", _error),
        };
    one_acc_out.currency = match get_field_value(&account, &input_reader, keys.currency.to_string())
    {
        Ok(value) => value,
        Err(_error) => panic!("{}", _error),
    };
    one_acc_out.eop_balance_ccy =
        match get_field_value(&account, &input_reader, keys.eop_balance_ccy.to_string()) {
            Ok(value) => to_f64(value),
            Err(_error) => panic!("{}", _error),
        };
    one_acc_out.eop_balance_hcy =
        match get_field_value(&account, &input_reader, keys.eop_balance_hcy.to_string()) {
            Ok(value) => to_f64(value),
            Err(_error) => panic!("{}", _error),
        };

    one_acc_out.rate_flag = match config_params.rate_flag() {
        "F" => "F".to_string(),
        "V" => "V".to_string(),
        _ => match get_field_value(&account, &input_reader, keys.rate_flag.to_string()) {
            Ok(value) => value,
            Err(_error) => panic!("{}", _error),
        },
    };
    one_acc_out.val_dt = match get_field_value(&account, &input_reader, keys.val_dt.to_string()) {
        Ok(value) => to_i64(value),
        Err(_error) => panic!("{}", _error),
    };
    one_acc_out.open_dt = match get_field_value(&account, &input_reader, keys.open_dt.to_string()) {
        Ok(value) => to_i64(value),
        Err(_error) => panic!("{}", _error),
    };
    one_acc_out.mat_dt = match get_field_value(&account, &input_reader, keys.mat_dt.to_string()) {
        Ok(value) => to_i64(value),
        Err(_error) => panic!("{}", _error),
    };
    one_acc_out.lst_repricing_dt =
        match get_field_value(&account, &input_reader, keys.lst_repricing_dt.to_string()) {
            Ok(value) => to_i64(value),
            Err(_error) => panic!("{}", _error),
        };
    one_acc_out.rep_freq = match get_field_value(&account, &input_reader, keys.rep_freq.to_string())
    {
        Ok(value) => value,
        Err(_error) => panic!("{}", _error),
    };
    one_acc_out.cust_agg_bal =
        match get_field_value(&account, &input_reader, keys.cust_agg_bal.to_string()) {
            Ok(value) => to_f64(value),
            Err(_error) => panic!("{}", _error),
        };
    one_acc_out.int_rate = match get_field_value(&account, &input_reader, keys.int_rate.to_string())
    {
        Ok(value) => to_f64(value),
        Err(_error) => panic!("{}", _error),
    };
    one_acc_out.dim1 = match get_field_value(&account, &input_reader, keys.dim1.to_string()) {
        Ok(value) => value,
        Err(_error) => panic!("{}", _error),
    };
    one_acc_out.dim2 = match get_field_value(&account, &input_reader, keys.dim2.to_string()) {
        Ok(value) => value,
        Err(_error) => panic!("{}", _error),
    };
    one_acc_out.dim3 = match get_field_value(&account, &input_reader, keys.dim3.to_string()) {
        Ok(value) => value,
        Err(_error) => panic!("{}", _error),
    };
    one_acc_out.dim4 = match get_field_value(&account, &input_reader, keys.dim4.to_string()) {
        Ok(value) => value,
        Err(_error) => panic!("{}", _error),
    };
    one_acc_out.customer_id =
        match get_field_value(&account, &input_reader, keys.customer_id.to_string()) {
            Ok(value) => value,
            Err(_error) => panic!("{}", _error),
        };
    one_acc_out.rl2 = match get_field_value(&account, &input_reader, keys.rl2.to_string()) {
        Ok(value) => value,
        Err(_error) => panic!("{}", _error),
    };
    one_acc_out.rl3 = match get_field_value(&account, &input_reader, keys.rl3.to_string()) {
        Ok(value) => value,
        Err(_error) => panic!("{}", _error),
    };
    one_acc_out.gl_code = match get_field_value(&account, &input_reader, keys.gl_code.to_string()) {
        Ok(value) => value,
        Err(_error) => panic!("{}", _error),
    };
    one_acc_out.prod_code =
        match get_field_value(&account, &input_reader, keys.prod_code.to_string()) {
            Ok(value) => value,
            Err(_error) => panic!("{}", _error),
        };
    one_acc_out.div_code = match get_field_value(&account, &input_reader, keys.div_code.to_string())
    {
        Ok(value) => value,
        Err(_error) => panic!("{}", _error),
    };
    one_acc_out.mis_code_1 =
        match get_field_value(&account, &input_reader, keys.mis_code_1.to_string()) {
            Ok(value) => value,
            Err(_error) => panic!("{}", _error),
        };
    one_acc_out.mis_code_2 =
        match get_field_value(&account, &input_reader, keys.mis_code_2.to_string()) {
            Ok(value) => value,
            Err(_error) => panic!("{}", _error),
        };
    one_acc_out.mis_code_3 =
        match get_field_value(&account, &input_reader, keys.mis_code_3.to_string()) {
            Ok(value) => value,
            Err(_error) => panic!("{}", _error),
        };
}

pub fn append_rules_based_dates(one_acc_out: &mut OneAccountView, parsed_method: &ParsedMethod) {
    one_acc_out.bc_as_on_rule = timestamp(parsed_method.curve_pick_date);
    one_acc_out.bc_as_on_applied = timestamp(parsed_method.curve_pick_date);
    one_acc_out.tenor_start_date_rule = timestamp(parsed_method.tenor_start_date);
    one_acc_out.tenor_start_date_applied = timestamp(parsed_method.tenor_start_date);
    one_acc_out.tenor_end_date_rule = timestamp(parsed_method.tenor_end_date);
    one_acc_out.tenor_end_date_applied = timestamp(parsed_method.tenor_end_date);
}

pub fn to_f32(val: String) -> f32 {
    return val
        .to_string()
        .parse::<f32>()
        .unwrap_or(DEFAULT_FLOAT as f32);
}

pub fn to_f64(val: String) -> f64 {
    return val.to_string().parse::<f64>().unwrap_or(DEFAULT_FLOAT);
}

pub fn to_i64(val: String) -> i64 {
    return val.to_string().parse::<i64>().unwrap_or(DEFAULT_INT);
}
