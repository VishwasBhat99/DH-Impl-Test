use crate::configuration_parameters::ConfigurationParameters;
use process::get_tenor_desc;
use process::{Logger, Tenor};
use process::{AGGKey, AccData, Val};
use rbdate::num_days_start_to_end;
use rbdate::NaiveDate;
use crate::macros;
use std::collections::HashMap;

#[allow(dead_code, unused_imports)]
pub fn fetch_acc_data(
    gl_map: &mut HashMap<String, String>,
    mis1_map: &mut HashMap<String, String>,
    almconcat_map: &mut HashMap<String, String>,
    div_map: &mut HashMap<String, String>,
    alm_line_map: &mut HashMap<String, String>,
    ia_line_map: &mut HashMap<String, String>,
    org_tenor_map: &mut HashMap<Tenor, String>,
    pp_tenor_map: &mut HashMap<Tenor, String>,
    ia_tenor_map: &mut HashMap<Tenor, String>,
    cat_map: &mut HashMap<String, String>,
    lcr_map: &mut HashMap<String, String>,
    wd_nwd_map: &mut HashMap<String, String>,
    split_val: Vec<&str>,
    config_params: &ConfigurationParameters,
    logger: &Logger,
) -> AccData {
    //Key Fields:
    let def_val = "NA".to_string();
    let as_on_dt = config_params.as_on_date();
    let mut field_num = 1;
    let mut acc_no = "".to_string();
    let mut acct_open_date = "";
    let mut val_date = "";
    let mut withdraw_date = "";
    let mut mat_date = "";
    let mut ccy = "";
    let mut prod_code = "";
    let mut cust_id = "";

    //Value Fields:
    let mut int_accrued = "";
    let mut int_accrued_rev = "";
    let mut penalty_amt = "";
    let mut premat_inc = "";
    let mut balance = "";
    let mut last_withdraw = "";
    let mut org_int_rt = "";
    let mut revised_rt = "";
    let mut pen_int_rt = "";

    for val in split_val {
        let sub_string = val.trim();

        match field_num {
            1 => cust_id = sub_string,
            //Format key to match lookup key in reference file.
            2 => acc_no = sub_string.to_string(),
            4 => prod_code = sub_string,
            5 => ccy = sub_string,
            7 => balance = sub_string,
            8 => last_withdraw = sub_string,
            9 => org_int_rt = sub_string,
            10 => revised_rt = sub_string,
            11 => pen_int_rt = sub_string,
            12 => int_accrued = sub_string,
            13 => int_accrued_rev = sub_string,
            14 => penalty_amt = sub_string,
            15 => premat_inc = sub_string,
            16 => acct_open_date = sub_string,
            17 => val_date = sub_string,
            18 => withdraw_date = sub_string,
            19 => mat_date = sub_string,
            _ => (),
        }
        if field_num == 19 {
            break;
        } else {
            field_num += 1;
        }
    }
    let mis1 = mis1_map.get(&acc_no).unwrap_or(&def_val);
    if !mis1_map.contains_key(&acc_no){
        log_warn!(logger,"Acc-No: {} not found in EDW ALM TD File for Cust: {}\nStamping `mis1 = NA`", acc_no, cust_id);
    }
    let source_gl = gl_map.get(&acc_no).unwrap_or(&def_val).to_string();
    if !gl_map.contains_key(&acc_no){
        log_warn!(logger,"Acc-No: {} not found in EDW ALM TD File for Cust: {}\nStamping `source_gl = NA`", acc_no, cust_id);
    }
    let alm_concat = almconcat_map
        .get(&source_gl)
        .unwrap_or(&def_val)
        .to_string();
    if !almconcat_map.contains_key(&source_gl){
        log_warn!(logger,"SourceGL: {} not found in ORAGL Master File for Acc: {} and Cust: {}\nStamping `source_gl = NA`", source_gl, acc_no, cust_id);
    }
    let division = div_map
        .get(&mis1.to_string())
        .unwrap_or(&def_val)
        .to_string();
    if !div_map.contains_key(mis1){
        log_warn!(logger,"Mis1: {} not found in MIS DESC Master File for Acc: {} and Cust: {}\nStamping `division = NA`", mis1, acc_no, cust_id);
    }
    let alm_line = alm_line_map
        .get(&alm_concat)
        .unwrap_or(&def_val)
        .to_string();
    if !alm_line_map.contains_key(&alm_concat){
        log_warn!(logger,"Alm-Concat: {} not found in Master LLG Updated File for Acc: {} and Cust: {}\nStamping `alm_line = NA`", alm_concat, acc_no, cust_id);
    }
    let ia_line = ia_line_map.get(&alm_concat).unwrap_or(&def_val).to_string();
    if !ia_line_map.contains_key(&alm_concat){
        log_warn!(logger,"Alm-Concat: {} not found in Master LLG Updated File for Acc: {} and Cust: {}\nStamping `ia_line = NA`", alm_concat, acc_no, cust_id);
    }
    let dat_acc_opn = NaiveDate::parse_from_str(acct_open_date, "%d-%b-%Y").expect(&format!(
        "Account_open date `{}` not well formatted for account: {}",
        acct_open_date, acc_no
    ));
    let dat_val = match NaiveDate::parse_from_str(val_date, "%d-%b-%Y") {
        Ok(date) => date,
        Err(_e) => dat_acc_opn,
    };
    let dat_mat = NaiveDate::parse_from_str(mat_date, "%d-%b-%Y").expect(&format!(
        "Maturity date `{}` not well formatted for account: {}",
        mat_date, acc_no
    ));
    let dat_wth = NaiveDate::parse_from_str(withdraw_date, "%d-%b-%Y").expect(&format!(
        "Withdrawal date `{}` not well formatted for account: {}",
        withdraw_date, acc_no
    ));
    let tenor = num_days_start_to_end(dat_val, dat_mat);
    let org_tenor = get_tenor_desc(tenor, org_tenor_map);
    let pp_ten_dur = num_days_start_to_end(dat_val, dat_wth);
    let pp_tenor = get_tenor_desc(pp_ten_dur, pp_tenor_map);
    let ia_tenor = get_tenor_desc(tenor, ia_tenor_map);
    let category = cat_map.get(prod_code).unwrap_or(&def_val).to_string();
    if !cat_map.contains_key(prod_code){
        log_warn!(logger,"Prod-Code: {} not found in Cust Category Master Fileor Acc: {} and and Cust: {}\nStamping `category = NA`", prod_code, acc_no, cust_id);
    }
    let lcr_classification = lcr_map.get(cust_id).unwrap_or(&def_val).to_string();
    if !lcr_map.contains_key(cust_id){
        log_warn!(logger,"Cust-ID: {} not found in TD LCR Classification File for Acc: {}\nStamping `lcr_classification = NA`", cust_id, acc_no);
    }
    let wd_nwd = wd_nwd_map.get(prod_code).unwrap_or(&def_val).to_string();
    if !wd_nwd_map.contains_key(prod_code){
        log_warn!(logger,"Prod-Code: {} not found in WD/NWD Master File for Acc: {} and Cust: {}\nStamping `wd_nwd = NA`",prod_code, acc_no, cust_id);
    }

    let grp_key = AGGKey::new(
        *as_on_dt,
        dat_acc_opn,
        dat_val,
        dat_wth,
        dat_mat,
        ccy.to_string(),
        prod_code.to_string(),
        mis1.to_string(),
        alm_concat,
        division,
        alm_line,
        ia_line,
        org_tenor,
        pp_tenor,
        ia_tenor,
        category,
        lcr_classification,
        wd_nwd,
    );

    let int_accrued = get_f64_from_string(int_accrued);
    let int_accrued_rev = get_f64_from_string(int_accrued_rev);
    let penalty_amt = get_f64_from_string(penalty_amt);
    let premat_inc = get_f64_from_string(premat_inc);
    let balance = get_f64_from_string(balance);
    let last_withdraw = get_f64_from_string(last_withdraw);
    let org_int_rt = get_f64_from_string(org_int_rt);
    let revised_rt = get_f64_from_string(revised_rt);
    let pen_int_rt = get_f64_from_string(pen_int_rt);

    let data = Val {
        int_accrued,
        int_accrued_rev,
        penalty_amt,
        premat_inc,
        balance,
        last_withdraw,
        org_int_rt,
        revised_rt,
        pen_int_rt,
    };
    AccData { grp_key, data }
}

fn get_f64_from_string(val: &str) -> f64 {
    val.parse::<f64>().unwrap_or(0.0)
}
