use crate::macros;
use chrono::{Duration, NaiveDate};
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::io::prelude::*;
use std::{
    collections::HashMap,
    env::current_dir,
    fs::File,
    io::{BufRead, BufWriter},
};

use super::utility::{is_date_between, parse_date, parse_date_unwraped};

pub fn implement_income_expense(
    balm_fc_td_file: &str,
    tot_rec: &mut i64,
    skp_rec: &mut i64,
    log: &Logger,
    pnl_bacid_map: &HashMap<String, String>,
    first_day_of_month: NaiveDate,
    last_day_of_month: NaiveDate,
    second_day_of_month: NaiveDate,
    third_day_of_month: NaiveDate,
    curr_month_income_master_map: &HashMap<String, (f64, f64)>,
    prev_month_income_master_map: &HashMap<String, (f64, f64)>,
    td_daily_basis_input_map: &HashMap<NaiveDate, HashMap<String, (f64, f64)>>,
    currency_map: &HashMap<String, f64>,
    writer: &mut BufWriter<File>,
) {
    let default_date = NaiveDate::parse_from_str("01-01-3099", "%d-%m-%Y")
        .expect("Cannot parse '01-01-3099' as date");

    let balm_fc_td = match new_buf_rdr(balm_fc_td_file) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found BALM FC TD file: `{}` on location `{}` : {}.",
            balm_fc_td_file,
            current_dir()
                .unwrap_or_else(|error| {
                    panic!("Error while getting current directory path: {}", error);
                })
                .display(),
            error
        ),
    };

    for (line_num, lines) in balm_fc_td.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                balm_fc_td_file,
                line_num + 1,
                error
            ),
        };
        *tot_rec += 1;

        let fields: Vec<&str> = line.split('|').collect();

        if fields.len() < 58 {
            log_debug!(
                log,
                "Insufficient fields detected for Acid {}: Expected 58 fields, found {} at line num {}.",
                fields[0],
                fields.len(),
                line_num+1
            );
            *skp_rec += 1;
            continue;
        }

        let acid = fields[0];
        let open_eff_date = parse_date(fields[5], line_num, acid);
        let maturity_date = parse_date(fields[6], line_num, acid);
        let foracid = fields[33];
        let schm_code = fields[43];
        let acct_curr_code = fields[46];
        let acct_cls_flg = fields[47];
        let acct_open_date = parse_date(fields[49], line_num, acid);
        let acct_cls_date = parse_date_unwraped(fields[51], default_date);

        let pnl_bacid = match pnl_bacid_map.get(schm_code) {
            Some(value) => value,
            None => {
                log_debug!(
                    log,
                    "Unable to find Scheme Code : {} in BALM FC GSP File",
                    schm_code
                );
                "NA"
            }
        };
        let amt;
        let mut case = "18";
        if is_date_between(first_day_of_month, last_day_of_month, acct_open_date)
            && !is_date_between(first_day_of_month, last_day_of_month, open_eff_date)
            && acct_cls_flg == "N"
        {
            let (cr_amt, dr_amt) = *curr_month_income_master_map
                .get(acid)
                .unwrap_or(&(0.0, 0.0));
            amt = (dr_amt - cr_amt) * -1.0;
            case = "1"
        } else if is_date_between(first_day_of_month, last_day_of_month, acct_open_date)
            && is_date_between(first_day_of_month, last_day_of_month, open_eff_date)
            && acct_cls_flg == "N"
            && open_eff_date == acct_open_date
        {
            let (cr_amt, dr_amt) = *curr_month_income_master_map
                .get(acid)
                .unwrap_or(&(0.0, 0.0));
            amt = (dr_amt - cr_amt) * -1.0;
            case = "2"
        } else if is_date_between(first_day_of_month, last_day_of_month, acct_open_date)
            && is_date_between(first_day_of_month, last_day_of_month, open_eff_date)
            && acct_cls_flg == "N"
            && open_eff_date < acct_open_date
        {
            let (cr_amt, dr_amt) = *curr_month_income_master_map
                .get(acid)
                .unwrap_or(&(0.0, 0.0));
            amt = (dr_amt - cr_amt) * -1.0;
            case = "3"
        } else if is_date_between(first_day_of_month, last_day_of_month, acct_open_date)
            && open_eff_date < first_day_of_month
            && acct_cls_flg == "Y"
        {
            let (cr_amt, dr_amt) = *curr_month_income_master_map
                .get(acid)
                .unwrap_or(&(0.0, 0.0));
            amt = (dr_amt - cr_amt) * -1.0;
            case = "4"
        } else if is_date_between(first_day_of_month, last_day_of_month, acct_open_date)
            && open_eff_date > last_day_of_month
            && acct_cls_flg == "Y"
        {
            let (cr_amt, dr_amt) = *curr_month_income_master_map
                .get(acid)
                .unwrap_or(&(0.0, 0.0));
            amt = (dr_amt - cr_amt) * -1.0;
            case = "5"
        } else if is_date_between(first_day_of_month, last_day_of_month, acct_open_date)
            && is_date_between(first_day_of_month, last_day_of_month, open_eff_date)
            && acct_cls_flg == "Y"
        {
            let (cr_amt, dr_amt) = *curr_month_income_master_map
                .get(acid)
                .unwrap_or(&(0.0, 0.0));
            amt = (dr_amt - cr_amt) * -1.0;
            case = "6"
        } else if !is_date_between(first_day_of_month, last_day_of_month, acct_open_date)
            && !is_date_between(first_day_of_month, last_day_of_month, open_eff_date)
            && acct_cls_flg == "Y"
            && acct_cls_date != maturity_date
        {
            let (curr_month_cr_amt, curr_month_dr_amt) = *curr_month_income_master_map
                .get(acid)
                .unwrap_or(&(0.0, 0.0));
            let curr_month_amt = curr_month_dr_amt - curr_month_cr_amt;

            let (prev_month_cr_amt, prev_month_dr_amt) = *prev_month_income_master_map
                .get(acid)
                .unwrap_or(&(0.0, 0.0));
            let prev_month_amt = prev_month_dr_amt - prev_month_cr_amt;

            amt = curr_month_amt * -1.0 - prev_month_amt * -1.0;
            case = "7"
        } else if is_date_between(first_day_of_month, last_day_of_month, acct_open_date)
            && is_date_between(first_day_of_month, last_day_of_month, open_eff_date)
            && acct_cls_flg == "N"
            && open_eff_date > acct_open_date
        {
            let two_days_before_open_eff_date = open_eff_date - Duration::days(2);
            let error_msg = format!(
                "Cannot get TD input file for date : {} and open effective date is : {}",
                two_days_before_open_eff_date, open_eff_date
            );
            let td_input_map = td_daily_basis_input_map
                .get(&two_days_before_open_eff_date)
                .expect(&error_msg);

            let (curr_month_cr_amt, curr_month_dr_amt) =
                *curr_month_income_master_map.get(acid).unwrap_or(&(0.0, 0.0));
            let curr_month_amt = curr_month_dr_amt - curr_month_cr_amt;
            let (cr_amt, dr_amt) = *td_input_map.get(acid).unwrap_or(&(0.0, 0.0));
            amt = (dr_amt - cr_amt) * -1.0 + (curr_month_amt * -1.0);
            case = "8"
        } else if is_date_between(first_day_of_month, last_day_of_month, acct_open_date)
            && open_eff_date > last_day_of_month
            && acct_cls_flg == "Y"
        {
            let two_days_before_open_eff_date = open_eff_date - Duration::days(2);
            let error_msg = format!(
                "Cannot get TD input file for date : {} and open effective date is : {}",
                two_days_before_open_eff_date, open_eff_date
            );
            let td_input_map = td_daily_basis_input_map
                .get(&two_days_before_open_eff_date)
                .expect(&error_msg);

            let (curr_month_cr_amt, curr_month_dr_amt) =
                *td_input_map.get(acid).unwrap_or(&(0.0, 0.0));
            let curr_month_amt = curr_month_dr_amt - curr_month_cr_amt;
            let (cr_amt, dr_amt) = *td_input_map.get(acid).unwrap_or(&(0.0, 0.0));
            amt = (dr_amt - cr_amt) * -1.0 + (curr_month_amt * -1.0);
            case = "9"
        } else if !is_date_between(first_day_of_month, last_day_of_month, acct_open_date)
        && is_date_between(first_day_of_month, second_day_of_month, open_eff_date)
        && acct_cls_flg == "N"
    {
        let (cr_amt, dr_amt) = *curr_month_income_master_map
                .get(acid)
                .unwrap_or(&(0.0, 0.0));
            amt = (dr_amt - cr_amt) * -1.0;
        case = "10A"
    }  
        else if !is_date_between(first_day_of_month, last_day_of_month, acct_open_date)
            && is_date_between(third_day_of_month, last_day_of_month, open_eff_date)
            && acct_cls_flg == "N"
        {
            let two_days_before_open_eff_date = open_eff_date - Duration::days(2);
            let error_msg = format!(
                "Cannot get TD input file for date : {} and open effective date is : {}",
                two_days_before_open_eff_date, open_eff_date
            );
            let td_input_map = td_daily_basis_input_map
                .get(&two_days_before_open_eff_date)
                .expect(&error_msg);

            let (curr_month_cr_amt, curr_month_dr_amt) = *curr_month_income_master_map
                .get(acid)
                .unwrap_or(&(0.0, 0.0));
            let curr_month_amt = curr_month_dr_amt - curr_month_cr_amt;

            let (prev_month_cr_amt, prev_month_dr_amt) = *prev_month_income_master_map
                .get(acid)
                .unwrap_or(&(0.0, 0.0));
            let prev_month_amt = prev_month_dr_amt - prev_month_cr_amt;

            let (cr_amt, dr_amt) = *td_input_map.get(acid).unwrap_or(&(0.0, 0.0));

            amt = (dr_amt - cr_amt) * -1.0 + (curr_month_amt * -1.0) - (prev_month_amt * -1.0);
            case = "10B"
        } else if !is_date_between(first_day_of_month, last_day_of_month, acct_open_date)
            && is_date_between(first_day_of_month, last_day_of_month, open_eff_date)
            && acct_cls_flg == "Y"
            && acct_cls_date == open_eff_date
            && acct_cls_date == maturity_date
        {
            let two_days_before_open_eff_date = open_eff_date - Duration::days(2);
            let error_msg = format!(
                "Cannot get TD input file for date : {} and open effective date is : {}",
                two_days_before_open_eff_date, open_eff_date
            );
            let td_input_map = td_daily_basis_input_map
                .get(&two_days_before_open_eff_date)
                .expect(&error_msg);

            let (curr_month_cr_amt, curr_month_dr_amt) = *curr_month_income_master_map
                .get(acid)
                .unwrap_or(&(0.0, 0.0));
            let curr_month_amt = curr_month_dr_amt - curr_month_cr_amt;

            let (prev_month_cr_amt, prev_month_dr_amt) = *prev_month_income_master_map
                .get(acid)
                .unwrap_or(&(0.0, 0.0));
            let prev_month_amt = prev_month_dr_amt - prev_month_cr_amt;

            let (cr_amt, dr_amt) = *td_input_map.get(acid).unwrap_or(&(0.0, 0.0));

            amt = (dr_amt - cr_amt) * -1.0 + (curr_month_amt * -1.0) - (prev_month_amt * -1.0);
            case = "11"
        } else if !is_date_between(first_day_of_month, last_day_of_month, acct_open_date)
            && is_date_between(first_day_of_month, last_day_of_month, open_eff_date)
            && acct_cls_flg == "Y"
            && acct_cls_date != open_eff_date
            && acct_cls_date == maturity_date
        {
            let two_days_before_open_eff_date = open_eff_date - Duration::days(2);
            let error_msg = format!(
                "Cannot get TD input file for date : {} and open effective date is : {}",
                two_days_before_open_eff_date, open_eff_date
            );
            let td_input_map = td_daily_basis_input_map
                .get(&two_days_before_open_eff_date)
                .expect(&error_msg);

            let (curr_month_cr_amt, curr_month_dr_amt) = *curr_month_income_master_map
                .get(acid)
                .unwrap_or(&(0.0, 0.0));
            let curr_month_amt = curr_month_dr_amt - curr_month_cr_amt;

            let (prev_month_cr_amt, prev_month_dr_amt) = *prev_month_income_master_map
                .get(acid)
                .unwrap_or(&(0.0, 0.0));
            let prev_month_amt = prev_month_dr_amt - prev_month_cr_amt;

            let (cr_amt, dr_amt) = *td_input_map.get(acid).unwrap_or(&(0.0, 0.0));

            amt = (dr_amt - cr_amt) * -1.0 + (curr_month_amt * -1.0) - (prev_month_amt * -1.0);
            case = "12"
        } else if !is_date_between(first_day_of_month, last_day_of_month, acct_open_date)
            && is_date_between(first_day_of_month, last_day_of_month, open_eff_date)
            && acct_cls_flg == "Y"
            && acct_cls_date != open_eff_date
            && acct_cls_date != maturity_date
        {
            let two_days_before_open_eff_date = open_eff_date - Duration::days(2);
            let error_msg = format!(
                "Cannot get TD input file for date : {} and open effective date is : {}",
                two_days_before_open_eff_date, open_eff_date
            );
            let td_input_map = td_daily_basis_input_map
                .get(&two_days_before_open_eff_date)
                .expect(&error_msg);

            let (curr_month_cr_amt, curr_month_dr_amt) = *curr_month_income_master_map
                .get(acid)
                .unwrap_or(&(0.0, 0.0));
            let curr_month_amt = curr_month_dr_amt - curr_month_cr_amt;

            let (prev_month_cr_amt, prev_month_dr_amt) = *prev_month_income_master_map
                .get(acid)
                .unwrap_or(&(0.0, 0.0));
            let prev_month_amt = prev_month_dr_amt - prev_month_cr_amt;

            let (cr_amt, dr_amt) = *td_input_map.get(acid).unwrap_or(&(0.0, 0.0));

            amt = (dr_amt - cr_amt) * -1.0 + (curr_month_amt * -1.0) - (prev_month_amt * -1.0);
            case = "13"
        } else if !is_date_between(first_day_of_month, last_day_of_month, acct_open_date)
            && is_date_between(first_day_of_month, last_day_of_month, open_eff_date)
            && acct_cls_flg == "Y"
            && acct_cls_date == open_eff_date
            && acct_cls_date != maturity_date
        {
            let two_days_before_open_eff_date = open_eff_date - Duration::days(2);
            let error_msg = format!(
                "Cannot get TD input file for date : {} and open effective date is : {}",
                two_days_before_open_eff_date, open_eff_date
            );
            let td_input_map = td_daily_basis_input_map
                .get(&two_days_before_open_eff_date)
                .expect(&error_msg);

            let (curr_month_cr_amt, curr_month_dr_amt) = *curr_month_income_master_map
                .get(acid)
                .unwrap_or(&(0.0, 0.0));
            let curr_month_amt = curr_month_dr_amt - curr_month_cr_amt;

            let (prev_month_cr_amt, prev_month_dr_amt) = *prev_month_income_master_map
                .get(acid)
                .unwrap_or(&(0.0, 0.0));
            let prev_month_amt = prev_month_dr_amt - prev_month_cr_amt;

            let (cr_amt, dr_amt) = *td_input_map.get(acid).unwrap_or(&(0.0, 0.0));

            amt = (dr_amt - cr_amt) * -1.0 + (curr_month_amt * -1.0) - (prev_month_amt * -1.0);
            case = "14"
        } else if !is_date_between(first_day_of_month, last_day_of_month, acct_open_date)
            && !is_date_between(first_day_of_month, last_day_of_month, open_eff_date)
            && acct_cls_flg == "N"
        {
            let (curr_month_cr_amt, curr_month_dr_amt) = *curr_month_income_master_map
                .get(acid)
                .unwrap_or(&(0.0, 0.0));
            let curr_month_amt = curr_month_dr_amt - curr_month_cr_amt;

            let (prev_month_cr_amt, prev_month_dr_amt) = *prev_month_income_master_map
                .get(acid)
                .unwrap_or(&(0.0, 0.0));
            let prev_month_amt = prev_month_dr_amt - prev_month_cr_amt;

            amt = curr_month_amt * -1.0 - prev_month_amt * -1.0;
            case = "15"
        } else if !is_date_between(first_day_of_month, last_day_of_month, acct_open_date)
            && !is_date_between(first_day_of_month, last_day_of_month, open_eff_date)
            && acct_cls_flg == "Y"
            && maturity_date == acct_cls_date
        {
            let (curr_month_cr_amt, curr_month_dr_amt) = *curr_month_income_master_map
                .get(acid)
                .unwrap_or(&(0.0, 0.0));
            let curr_month_amt = curr_month_dr_amt - curr_month_cr_amt;

            let (prev_month_cr_amt, prev_month_dr_amt) = *prev_month_income_master_map
                .get(acid)
                .unwrap_or(&(0.0, 0.0));
            let prev_month_amt = prev_month_dr_amt - prev_month_cr_amt;

            amt = curr_month_amt * -1.0 - prev_month_amt * -1.0;
            case = "16"
        } else if !is_date_between(first_day_of_month, last_day_of_month, acct_open_date)
            && open_eff_date > last_day_of_month
            && acct_cls_flg == "Y"
        {
            let (curr_month_cr_amt, curr_month_dr_amt) = *curr_month_income_master_map
                .get(acid)
                .unwrap_or(&(0.0, 0.0));
            let curr_month_amt = curr_month_dr_amt - curr_month_cr_amt;

            let (prev_month_cr_amt, prev_month_dr_amt) = *prev_month_income_master_map
                .get(acid)
                .unwrap_or(&(0.0, 0.0));
            let prev_month_amt = prev_month_dr_amt - prev_month_cr_amt;

            amt = curr_month_amt * -1.0 - prev_month_amt * -1.0;
            case = "17"
        } else {
            let (curr_month_cr_amt, curr_month_dr_amt) = *curr_month_income_master_map
                .get(acid)
                .unwrap_or(&(0.0, 0.0));
            let curr_month_amt = curr_month_dr_amt - curr_month_cr_amt;

            let (prev_month_cr_amt, prev_month_dr_amt) = *prev_month_income_master_map
                .get(acid)
                .unwrap_or(&(0.0, 0.0));
            let prev_month_amt = prev_month_dr_amt - prev_month_cr_amt;

            amt = curr_month_amt * -1.0 - prev_month_amt * -1.0;
        }

        let exrt = *currency_map.get(acct_curr_code).unwrap_or(&1.0);
        let lcy_amt = amt * exrt;
        let op_str = format!(
            "{}|{}|{}|{}|{}|{}|{}|{}",
            acid, foracid, schm_code, acct_curr_code, pnl_bacid, amt, lcy_amt, case
        );
        writeln!(writer, "{}", op_str).unwrap_or_else(|error| {
            panic!("Unable to write to the output file: {}", error);
        });
    }
}
