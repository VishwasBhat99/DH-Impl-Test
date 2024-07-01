use macros;
use rbdate::*;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use stamp_ftp::cfinput::AccFieldNames;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;
use std::collections::HashMap;

pub fn form_out_data(
    acc_data_in: &mut AccountWithCFs,
    inputfieldnames: &AccFieldNames,
    base_rt: f64,
    adj1rate: f64,
    final_tpr: f64,
    lock_spread: f64,
    aggr_balance: f64,
    ftprunid: i64,
    from_date: &NaiveDate,
    to_date: &NaiveDate,
    avg_bal: &HashMap<String, f64>,
    log: &Logger,
    diag_log: &Logger,
) -> String {
    //Added +1 -- It includes both from date and to dates.
    let _run_duration = rbdate::num_days_start_to_end(*from_date, *to_date) + 1;

    let account_number = match acc_data_in.get_string_for_key(&inputfieldnames.acc_no) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    let mis1 = match acc_data_in.get_string_for_key(&inputfieldnames.cod_mis_comp1) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    let currency = match acc_data_in.get_string_for_key(&inputfieldnames.ccy) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    let maturity_date = match acc_data_in.get_i64_for_key(&inputfieldnames.mat_dt) {
        Ok(result) => result,
        Err(e) => DEFAULT_INT,
    };

    let account_start_date = match acc_data_in.get_i64_for_key(&inputfieldnames.st_dt) {
        Ok(result) => result,
        Err(e) => DEFAULT_INT,
    };

    let original_balance = match acc_data_in.get_f64_for_key(&inputfieldnames.amt) {
        Ok(result) => result,
        Err(e) => DEFAULT_FLOAT,
    };

    let outstanding_balance = match acc_data_in.get_f64_for_key(&inputfieldnames.amt) {
        Ok(result) => result,
        Err(e) => DEFAULT_FLOAT,
    };

    let origination_date = match acc_data_in.get_i64_for_key(&inputfieldnames.st_dt) {
        Ok(result) => result,
        Err(e) => DEFAULT_INT,
    };

    let int_rate = match acc_data_in.get_f64_for_key(&inputfieldnames.int_rt) {
        Ok(result) => result,
        Err(e) => DEFAULT_FLOAT,
    };

    let cust_id = match acc_data_in.get_i64_for_key(&inputfieldnames.cust_id) {
        Ok(result) => result,
        Err(e) => DEFAULT_INT,
    };

    let prod_code = match acc_data_in.get_string_for_key(&inputfieldnames.prod_code) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    let gl = match acc_data_in.get_string_for_key(&inputfieldnames.gl_no) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    let _max_days_in_year = rbdate::num_days_start_to_end(
        *from_date,
        rbdate::increment_date_by_months(*from_date, (12) as u16),
    );

    let start_date = NaiveDateTime::from_timestamp(account_start_date, 0)
        .date()
        .format("%d-%m-%Y");
    let mat_date = NaiveDateTime::from_timestamp(maturity_date, 0)
        .date()
        .format("%d-%m-%Y");
    let org_date = NaiveDateTime::from_timestamp(origination_date, 0)
        .date()
        .format("%d-%m-%Y");

    let average_balance = match avg_bal.get(&account_number) {
        Some(x) => *x,
        None => {
            log_debug!(
            log,
            "Average balance is not availale for account id :{} . Hence considering zero balance for the same.", 
             account_number
        );
            DEFAULT_FLOAT
        }
    };

    let out_str = format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|\
        {}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        account_number,
        "",
        average_balance,
        "",
        "",
        int_rate,
        base_rt,
        final_tpr,
        org_date,
        mat_date,
        "",
        "",
        mis1,
        "",
        "",
        prod_code,
        "",
        "",
        "RD",
        currency,
        gl,
        cust_id,
        (average_balance * final_tpr * _run_duration as f64) / (_max_days_in_year as f64 * 100.0),
        "",
        "",
        original_balance,
        outstanding_balance,
        base_rt,
        adj1rate,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        "",
        "",
        "",
        "Margin method",
        "Int Rate",
        "",
        "",
        "",
        "",
        "",
        get_date_str(DEFAULT_INT),
        get_date_str(DEFAULT_INT),
        get_date_str(DEFAULT_INT),
        get_date_str(DEFAULT_INT),
        get_date_str(DEFAULT_INT),
        get_date_str(DEFAULT_INT),
    );

    out_str
}

fn get_date_str(date: i64) -> String {
    let start_date = NaiveDateTime::from_timestamp(date, 0)
        .date()
        .format("%d-%m-%Y");

    start_date.to_string()
}
