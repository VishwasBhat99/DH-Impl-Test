use crate::process::input_account::{FinMap, MocAccount};
use crate::process::ConfigurationParameters;
use std::collections::HashMap;

pub fn get_output_writer(
    config_params: &ConfigurationParameters,
    input: &Vec<&str>,
    cf_type: &str,
    is_acc_gl: &str,
    alm_line: &str,
    fin_map: &HashMap<String, FinMap>,
    output: &mut String,
) {
    let fin_acc = &FinMap::init();
    let fin_acc = &*fin_map.get(&input[0].to_string()).unwrap_or(&fin_acc);
    output.push_str(&input[0].to_string());
    output.push('|');
    output.push_str(&input[4].to_string());
    output.push('|');
    let mut bal_str = String::new();
    match cf_type {
        "C" => bal_output(
            &mut bal_str,
            &0.0,
            &input[6].to_string().parse::<f64>().unwrap_or(0.0),
            &(0.0 - input[6].to_string().parse::<f64>().unwrap_or(0.0)),
        ),
        "D" => bal_output(
            &mut bal_str,
            &input[5].to_string().parse::<f64>().unwrap_or(0.0),
            &0.0,
            &(input[5].to_string().parse::<f64>().unwrap_or(0.0) - 0.0),
        ),
        _ => bal_output(
            &mut bal_str,
            &input[5].to_string().parse::<f64>().unwrap_or(0.0),
            &input[6].to_string().parse::<f64>().unwrap_or(0.0),
            &(&input[5].to_string().parse::<f64>().unwrap_or(0.0)
                - input[6].to_string().parse::<f64>().unwrap_or(0.0)),
        ),
    };
    output.push_str(&bal_str);
    output.push('|');
    output.push_str(&cf_type);
    output.push('|');
    if &input[2].to_string() != "" {
        output.push_str(&input[2].to_string());
    } else {
        output.push_str(&config_params.currency());
    }
    output.push('|');
    output.push_str(&is_acc_gl);
    output.push('|');
    output.push_str(&alm_line);
    output.push('|');
    output.push_str(&fin_acc.code_desc);
    output.push('|');
    output.push_str(&fin_acc.group2);
    output.push('|');
    output.push_str(&fin_acc.group3);
    output.push('|');
    output.push_str(&fin_acc.line);
    output.push('|');
    output.push(input[0].to_string().chars().nth(0).unwrap());
    output.push_str("\n");
}

pub fn get_moc_writer(
    config_params: &ConfigurationParameters,
    moc_input: &MocAccount,
    cf_type: &str,
    is_acc_gl: &str,
    output: &mut String,
) {
    output.push_str(&moc_input.gl_code);
    output.push('|');
    output.push_str(&moc_input.branch_code);
    output.push('|');
    let mut bal_str = String::new();
    match cf_type {
        "C" => bal_output(
            &mut bal_str,
            &0.0,
            &moc_input.cr_bal,
            &(0.0 - moc_input.cr_bal),
        ),
        "D" => bal_output(
            &mut bal_str,
            &moc_input.dr_bal,
            &0.0,
            &(moc_input.dr_bal - 0.0),
        ),
        _ => bal_output(
            &mut bal_str,
            &moc_input.dr_bal,
            &moc_input.cr_bal,
            &(moc_input.dr_bal - moc_input.cr_bal),
        ),
    };
    output.push_str(&bal_str);
    output.push('|');
    output.push_str(&cf_type);
    output.push('|');
    if &moc_input.ccy != "" {
        output.push_str(&moc_input.ccy);
    } else {
        output.push_str(&config_params.currency());
    }
    output.push('|');
    output.push_str(&is_acc_gl);
    output.push('|');
    output.push_str(&moc_input.alm_line);
    output.push('|');
    output.push_str(&moc_input.alm_line);
    output.push('|');
    output.push_str(&moc_input.alm_line);
    output.push('|');
    output.push_str(&moc_input.alm_line);
    output.push('|');
    output.push_str(&moc_input.alm_line);
    output.push('|');
    output.push(moc_input.gl_code.to_string().chars().nth(0).unwrap_or(' '));
    output.push_str("\n");
}

pub fn bal_output(bal_output: &mut String, dr_bal: &f64, cr_bal: &f64, net_bal: &f64) {
    bal_output.push_str(&dr_bal.to_string());
    bal_output.push('|');
    bal_output.push_str(&cr_bal.to_string());
    bal_output.push('|');
    bal_output.push_str(&net_bal.to_string());
}
