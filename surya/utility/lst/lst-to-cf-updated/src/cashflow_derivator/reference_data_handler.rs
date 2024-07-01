use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::field_struct::Fields;
use cashflow_derivator::str_to_int;
use macros;
use slog::Logger;
use statics::{DEFAULT_FLOAT, DEFAULT_INT};
use std::collections::HashMap;

pub fn add_reference_file_values(
    out_acc: &mut AccountWithCashflows,
    fields: &Fields,
    reference_data: &HashMap<i64, Vec<String>>,
    log: &Logger,
) {
    let acc_num = str_to_int(&out_acc.get_acc_num());
    if !reference_data.contains_key(&acc_num) {
        log_warn!(
            log,
            "No Data for account: `{}` found in Reference file",
            acc_num
        );
        return;
    }
    let acc_data = reference_data
        .get(&acc_num)
        .unwrap_or(&vec!["".to_string()])
        .to_owned();
    out_acc.p1_int_1 = get_int_from_key(&acc_data, fields.p1_int_1, log, acc_num);
    out_acc.p2_int_2 = get_int_from_key(&acc_data, fields.p2_int_2, log, acc_num);
    out_acc.p3_int_3 = get_int_from_key(&acc_data, fields.p3_int_3, log, acc_num);
    out_acc.p4_int_4 = get_int_from_key(&acc_data, fields.p4_int_4, log, acc_num);
    out_acc.p5_int_5 = get_int_from_key(&acc_data, fields.p5_int_5, log, acc_num);
    out_acc.p6_flt_1 = get_flt_from_key(&acc_data, fields.p6_flt_1, log, acc_num);
    out_acc.p7_flt_2 = get_flt_from_key(&acc_data, fields.p7_flt_2, log, acc_num);
    out_acc.p8_flt_3 = get_flt_from_key(&acc_data, fields.p8_flt_3, log, acc_num);
    out_acc.p9_flt_4 = get_flt_from_key(&acc_data, fields.p9_flt_4, log, acc_num);
    out_acc.p10_flt_5 = get_flt_from_key(&acc_data, fields.p10_flt_5, log, acc_num);
    out_acc.p11_str_1 = get_str_from_key(&acc_data, fields.p11_str_1, log, acc_num);
    out_acc.p12_str_2 = get_str_from_key(&acc_data, fields.p12_str_2, log, acc_num);
    out_acc.p13_str_3 = get_str_from_key(&acc_data, fields.p13_str_3, log, acc_num);
    out_acc.p14_str_4 = get_str_from_key(&acc_data, fields.p14_str_4, log, acc_num);
    out_acc.p15_str_5 = get_str_from_key(&acc_data, fields.p15_str_5, log, acc_num);
}

fn get_int_from_key(data: &Vec<String>, mut key: i64, loger: &Logger, acc_id: i64) -> i64 {
    if key.eq(&0) {
        return DEFAULT_INT;
    }
    key -= 2;
    let req_data = data.get(key as usize);
    if req_data.is_some() {
        let actual_data = req_data.unwrap_or(&"".to_string()).to_owned();
        let converted_data = actual_data.parse::<i64>();
        if converted_data.is_ok() {
            return converted_data.unwrap_or(DEFAULT_INT);
        } else {
            log_error!(
                loger,
                "unable to parse value: `{}` , position: `{}` to int",
                actual_data,
                key + 2
            );
        }
    } else {
        log_error!(
            loger,
            "position `{}` not found for account: `{}`, passing default values",
            key + 2,
            acc_id
        );
    }

    return DEFAULT_INT;
}

fn get_flt_from_key(data: &Vec<String>, mut key: i64, loger: &Logger, acc_id: i64) -> f64 {
    if key.eq(&0) {
        return DEFAULT_FLOAT;
    }
    key -= 2;
    let req_data = data.get(key as usize);
    if req_data.is_some() {
        let actual_data = req_data.unwrap_or(&"".to_string()).to_owned();
        let converted_data = actual_data.parse::<f64>();
        if converted_data.is_ok() {
            return converted_data.unwrap_or(DEFAULT_FLOAT);
        } else {
            log_error!(
                loger,
                "unable to parse value: `{}` , position: `{}` to float",
                actual_data,
                key + 2
            );
        }
    } else {
        log_error!(
            loger,
            "position `{}` not found for account: `{}`, passing default values",
            key + 2,
            acc_id
        );
    }

    return DEFAULT_FLOAT;
}
fn get_str_from_key(data: &Vec<String>, mut key: i64, loger: &Logger, acc_id: i64) -> String {
    if key.eq(&0) {
        return "".to_string();
    }
    key -= 2;
    let req_data = data.get(key as usize);
    if req_data.is_some() {
        let actual_data = req_data.unwrap_or(&"".to_string()).to_owned();
        return actual_data;
    } else {
        log_error!(
            loger,
            "position `{}` not found for account: `{}`, passing default values",
            key + 2,
            acc_id
        );
    }

    return "".to_string();
}
