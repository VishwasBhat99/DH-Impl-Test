use super::split::split_by_defesance;
use super::TradingAccount;
use macros;
use rbdate::NaiveDate;
use slog::Logger;
use std::collections::HashMap;

pub fn apply_defesance(
    desc: String,
    acc_infos: Vec<TradingAccount>,
    mut defeasance: HashMap<String, Vec<f64>>,
    log: &Logger,
    as_on_date: NaiveDate,
    mut op_line: &mut String,
) {
    let def_date: Vec<i64> = vec![1, 4, 11, 21, 29, 46, 77];
    let def_value_opt = defeasance.get_mut(&desc.to_string());
    let mut def_index = 0;
    if def_value_opt.is_none() {
        log_warn!(log, "Defesance values not found for : {}", desc);
        for account in acc_infos {
            let fields: Vec<&str> = account.acc_pt.split("|").collect();
            let amt = fields[16].parse::<f64>().unwrap_or(0.0);
            split_by_defesance(
                &fields,
                fields[0].to_string(),
                amt,
                account.mat_dt.format("%d-%m-%Y").to_string(),
                account.mat_dt.format("%d-%m-%Y").to_string(),
                &mut op_line,
                as_on_date,
            );
        }
    } else {
        let def_values = def_value_opt.expect("Unexpected unwrap error with defeasance map.");
        for account in acc_infos {
            let mut split_count = 0;
            let fields: Vec<&str> = account.acc_pt.split("|").collect();
            let mut account_no: String = String::new();
            account_no.push_str(fields[0]);
            account_no.push('-');
            let mut total_outstanding_amt = fields[16].parse::<f64>().unwrap_or(0.0);
            while total_outstanding_amt > 0.0 {
                split_count += 1;
                let mut new_acc_no = String::new();
                new_acc_no.push_str(&account_no);
                new_acc_no.push_str(&split_count.to_string());
                let cf_dt = {
                    if def_index >= 7 {
                        account.mat_dt
                    } else {
                        as_on_date + chrono::Duration::days(def_date[def_index])
                    }
                };
                if def_index >= 7 {
                    def_index = 7;
                }
                if total_outstanding_amt < def_values[def_index] {
                    split_by_defesance(
                        &fields,
                        new_acc_no,
                        total_outstanding_amt,
                        cf_dt.format("%d-%m-%Y").to_string(),
                        account.mat_dt.format("%d-%m-%Y").to_string(),
                        &mut op_line,
                        as_on_date,
                    );
                    def_values[def_index] -= total_outstanding_amt;
                    total_outstanding_amt = 0.0;
                } else {
                    let amt = def_values[def_index];
                    split_by_defesance(
                        &fields,
                        new_acc_no,
                        amt,
                        cf_dt.format("%d-%m-%Y").to_string(),
                        account.mat_dt.format("%d-%m-%Y").to_string(),
                        &mut op_line,
                        as_on_date,
                    );
                    total_outstanding_amt -= amt;
                    def_values[def_index] = 0.0;
                    def_index += 1;
                }
            }
        }
    }
}
