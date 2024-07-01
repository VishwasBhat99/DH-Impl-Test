use super::{
    macros, AlmMaster, AlmMasterKey, BaselPassThroughs, CustMasterData, CustMasterInput,
    CustMasterMap, InputAccount, Logger, DEFAULT_FLOAT,
};
use calamine::DataType;
pub use chrono::NaiveDateTime;
use chrono::{Datelike, Duration, NaiveDate};
use std::collections::HashMap;

pub fn get_op_line(
    acc: &mut InputAccount,
    cust_master: &mut CustMasterMap,
    alm_master: &mut HashMap<AlmMasterKey, AlmMaster>,
    as_on_date: NaiveDate,
    concats: &mut String,
    log: &Logger,
) -> String {
    let mut op_line = String::new();
    let mut uniq_acc_no = String::new();
    uniq_acc_no.push_str(&acc.acc_no);
    uniq_acc_no.push('-');
    uniq_acc_no.push_str(&acc.cntrct_num);
    acc.acc_no = uniq_acc_no;
    let acc_open_dt = get_date(&acc.acc_no, &acc.acc_open_dt, "account_open_date", log);
    let effc_dt = get_date(&acc.acc_no, &acc.effc_dt, "effective_date", log);
    let mut mat_dt = get_date(&acc.acc_no, &acc.mat_dt, "maturity_date", log);

    let mut no_of_days: i64;
    let mut cf_dt = mat_dt;
    while cf_dt <= as_on_date && acc_open_dt <= mat_dt {
        no_of_days = i64::from(mat_dt.num_days_from_ce() - acc_open_dt.num_days_from_ce()) + 1;
        cf_dt += Duration::days(no_of_days);
    }
    mat_dt = cf_dt;

    acc.acc_open_dt = get_formatted_date(acc_open_dt);
    acc.effc_dt = get_formatted_date(effc_dt);
    acc.mat_dt = get_formatted_date(mat_dt);
    acc.as_on = get_formatted_date(get_date(&acc.acc_no, &acc.as_on, "as_on_date", log));
    let resid_days = num_days_start_to_end(&acc.acc_no, as_on_date, mat_dt, log);
    let cntrct_days = num_days_start_to_end(&acc.acc_no, effc_dt, mat_dt, log);
    acc.comp_freq = get_freq(&acc.comp_freq);
    acc.pay_freq = get_freq(&acc.pay_freq);
    op_line.push_str(&acc.print());

    op_line.push_str(&resid_days.to_string());
    op_line.push('|');
    op_line.push_str(&cntrct_days.to_string());
    op_line.push('|');

    let def_cust_master_data = CustMasterData::new();
    let cust_master_data = cust_master
        .store
        .entry(acc.cust_id.to_string())
        .or_insert(def_cust_master_data);
    op_line.push_str(&cust_master_data.print());

    let mut alm_master_key = AlmMasterKey::new();
    if acc.bal_os.parse().unwrap_or(DEFAULT_FLOAT) < 0.0 {
        alm_master_key.insert(acc.gl_cd.to_string(), String::from("D"));
    } else {
        alm_master_key.insert(acc.gl_cd.to_string(), String::from("C"));
    };
    let def_alm_master = AlmMaster::new();
    let alm_master = alm_master.entry(alm_master_key).or_insert(def_alm_master);
    op_line.push_str(&alm_master.print());
    let basel_passthroughs = BaselPassThroughs::new();
    op_line.push_str(&basel_passthroughs.print());
    op_line.push('\n');

    if alm_master.balm_llg == "NONE" {
        concats.push_str(&format!("TD|{}|{}|{}\n", acc.acc_no, acc.gl_cd, acc.bal_os));
    }
    op_line
}

pub fn get_alm_master_data(row: &[DataType], alm_master: &mut HashMap<AlmMasterKey, AlmMaster>) {
    fn get_data(data: &DataType) -> String {
        data.to_string().replace("\u{a0}", " ")
    }

    alm_master.insert(
        AlmMasterKey {
            gl_cd: get_data(&row[0]),
            dr_cr: get_data(&row[2]),
        },
        AlmMaster {
            w4b_cd: get_data(&row[3]),
            balm_llg: get_data(&row[5]),
            care_llg: get_data(&row[6]),
            ba_llg: get_data(&row[7]),
        },
    );
}

fn get_date(acc_no: &str, date: &str, field_name: &str, log: &Logger) -> NaiveDate {
    match NaiveDate::parse_from_str(date, "%d-%m-%Y") {
        Ok(dt) => dt,
        Err(error) => {
            log_error!(
                log,
                "`{}` is not well-formatted as `DD-MM-YYYY` for account: `{}` : `{}`.",
                field_name,
                acc_no,
                error
            );
            NaiveDate::from_ymd(1970, 01, 01)
        }
    }
}

fn get_formatted_date(dt: NaiveDate) -> String {
    dt.format("%d-%m-%Y").to_string()
}

pub fn get_cust_master_data(
    cust_master_input: CustMasterInput,
    cust_master_map: &mut CustMasterMap,
) {
    let mut cust_master_data = CustMasterData::new();
    cust_master_data.insert(cust_master_input.clone());
    cust_master_map
        .store
        .insert(cust_master_input.clients_code, cust_master_data);
}

pub fn num_days_start_to_end(acc_no: &str, start: NaiveDate, end: NaiveDate, log: &Logger) -> i64 {
    if start > end {
        log_error!(
            log,
            "`start_date`: `{}` is greater than `end_date`: `{}` for account: `{}`.",
            start,
            end,
            acc_no
        );
        return 1;
    }

    i64::from(end.num_days_from_ce() - start.num_days_from_ce())
}

fn get_freq(freq: &str) -> String {
    let freq = match freq.to_uppercase().as_str() {
        "MONTHLY" => 1,
        "BI-MONTHLY" => 2,
        "QUARTELY" => 3,
        "HALF YEARLY" => 6,
        "ANNUAL" => 12,
        _ => 0,
    };
    freq.to_string()
}
