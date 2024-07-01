extern crate csv;
extern crate serde;
use self::csv::ReaderBuilder;
use self::output_field::OutputData;
use calamine::{open_workbook_auto, Reader};
use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::DateParser;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::default;
use std::env::current_dir;
use std::io::{BufRead, BufWriter, Write};
use std::path::PathBuf;
use std::vec;

use self::input_field::{CashflowData, MasterFileData, YieldData};

mod input_field;
mod output_field;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut tot_acc_enc = 0;
    let mut acc_enc = 0;
    let mut master_file_path = open_workbook_auto(config_param.master_file_path())
        .expect("Unable to open the master xlsx file.");
    println!(
        "Sheets present in Master-File: `{:?}`",
        master_file_path.sheet_names()
    );
    if !master_file_path
        .sheet_names()
        .contains(&config_param.sec_loan_cashflows_sheet_name().to_string())
    {
        panic!(
            "Sheet passed: `{}` not present in Master-File: `{}`",
            config_param.sec_loan_cashflows_sheet_name(),
            config_param.master_file_path
        );
    }
    println!(
        "Reading Sheet: `{}` from Master-File",
        config_param.sec_loan_cashflows_sheet_name(),
    );
    if !master_file_path
        .sheet_names()
        .contains(&config_param.sec_loan_master_sheet_name().to_string())
    {
        panic!(
            "Sheet passed: `{}` not present in Master-File: `{}`",
            config_param.sec_loan_master_sheet_name(),
            config_param.master_file_path
        );
    }
    println!(
        "Reading Sheet: `{}` from Master-File",
        config_param.sec_loan_master_sheet_name(),
    );
    let mut cashflow_map: HashMap<String, Vec<CashflowData>> = HashMap::new();
    if let Some(Ok(cashflow_file_reader)) =
        master_file_path.worksheet_range(&config_param.sec_loan_cashflows_sheet_name())
    {
        for (row_no, row) in cashflow_file_reader.rows().enumerate().skip(1) {
            let cashflow_data = CashflowData::new_from_xlsx(row);
            let ubs_account = cashflow_data.ubs_account_number.to_string();
            if let Some(vec) = cashflow_map.get_mut(&cashflow_data.ubs_account_number) {
                vec.push(cashflow_data);
            } else {
                let mut new_vec = Vec::new();
                new_vec.push(cashflow_data);
                cashflow_map.insert(ubs_account, new_vec);
            }
        }
    }
    let mut reader = match ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .from_path(config_param.yield_file_path())
    {
        Ok(read) => read,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.yield_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut yield_map: HashMap<String, f64> = HashMap::new();
    for (line_num, lines) in reader.deserialize().enumerate() {
        let yield_data: YieldData = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    log,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.yield_file_path(),
                    line_num + 1,
                    error
                );
                Default::default()
            }
        };
        yield_map.insert(yield_data.account_no, yield_data.yield_rate);
    }
    let mut non_concat_op = String::new();
    let mut output_data: BTreeMap<String, Vec<OutputData>> = BTreeMap::new();
    let mut tot_acc_enc = 0;
    if let Some(Ok(master_file_reader)) =
        master_file_path.worksheet_range(&config_param.sec_loan_master_sheet_name())
    {
        for (row_no, row) in master_file_reader.rows().enumerate().skip(1) {
            tot_acc_enc = tot_acc_enc + 1;
            let master_data = MasterFileData::new_from_xlsx(row);
            let ubs_account = master_data.ubs_account_number.clone();
            if !ubs_account.is_empty() {
                if cashflow_map.contains_key(&ubs_account.clone()) {
                    acc_enc = acc_enc + 1;
                    let default_field = CashflowData::default();
                    let vec = vec![default_field];
                    let cash_vec = cashflow_map.get(&ubs_account).unwrap_or(&vec);
                    for cashflows in cash_vec {
                        let deal_start_date =
                            rbdate::datevalue_to_naive_date(&master_data.deal_start_date)
                                .unwrap_or(config_param.as_on_date)
                                .format("%d-%m-%Y")
                                .to_string();
                        let cf_end_date = rbdate::datevalue_to_naive_date(&master_data.cf_end_date)
                            .unwrap_or(config_param.as_on_date)
                            .format("%d-%m-%Y")
                            .to_string();
                        let expected_rate_reset_date =
                            rbdate::datevalue_to_naive_date(&master_data.expected_rate_reset_date)
                                .unwrap_or(config_param.as_on_date)
                                .format("%d-%m-%Y")
                                .to_string();
                        let date1 = rbdate::datevalue_to_naive_date(&cashflows.date_1)
                            .unwrap_or(config_param.as_on_date)
                            .format("%d-%m-%Y")
                            .to_string();
                        let principal_amount =
                            cashflows.principal_payment.parse::<f64>().unwrap_or(0.0);
                        let intrest_portion =
                            cashflows.intrestportion.parse::<f64>().unwrap_or(0.0);
                        let yield_data =
                            yield_map.get(&cashflows.ubs_account_number).unwrap_or(&0.0);
                        let output_field = OutputData {
                            ubs_account_number: ubs_account.to_string(),
                            cust_id: master_data.cust_id.parse::<i64>().unwrap_or(0),
                            deal_name: master_data.deal_name.to_string(),
                            principal_amount: master_data
                                .principal_amount
                                .parse::<f64>()
                                .unwrap_or(0.0),
                            deal_start_date,
                            cf_end_date,
                            accrued_interest: master_data
                                .accrued_interest
                                .parse::<f64>()
                                .unwrap_or(0.0),
                            compounding_frequency: master_data.compounding_frequency.to_string(),
                            deal_value: master_data.deal_value.parse::<i64>().unwrap_or(0),
                            gl: master_data.gl.parse::<i64>().unwrap_or(0),
                            system: master_data.system.to_string(),
                            curr_nominal_int_rate: master_data
                                .curr_nominal_int_rate
                                .parse::<i64>()
                                .unwrap_or(0),
                            ratings: master_data.ratings.to_owned(),
                            rating_aggency: master_data.rating_aggency.to_owned(),
                            asset_class: master_data.asset_class.to_owned(),
                            division: master_data.division.to_owned(),
                            typ: master_data.typ.to_owned(),
                            originator: master_data.originator.to_owned(),
                            contract_yield: master_data
                                .contract_yield
                                .parse::<f64>()
                                .unwrap_or(0.0),
                            current_annual_yield: master_data
                                .current_annual_yield
                                .parse::<f64>()
                                .unwrap_or(0.0),
                            reset_frequency: master_data.reset_frequency.to_owned(),
                            interest_rate_type: master_data.interest_rate_type.to_owned(),
                            expected_rate_reset_date,
                            date1: date1,
                            intrest_portion: intrest_portion,
                            principal_payment: principal_amount,
                            yld: *yield_data,
                            mis_code_1: master_data.mis_code_1.parse::<i64>().unwrap_or(0),
                            mis_code_2: master_data.mis_code_2.parse::<i64>().unwrap_or(0),
                        };
                        if let Some(vec) = output_data.get_mut(&ubs_account.clone()) {
                            vec.push(output_field);
                        } else {
                            let mut new_vec = Vec::new();
                            new_vec.push(output_field);
                            output_data.insert(ubs_account.clone(), new_vec);
                        }
                    }
                } else {
                    non_concat_op.push_str(&format!("{}\n", ubs_account.clone()))
                }
            }
        }
    }
    let mut op_line = String::new();
    for (account_number, account_data) in output_data {
        for account_data in account_data {
            op_line.push_str(&format!(
            "{}~{}~{}~{}~{}~{}~{}~{}~{}~{}~{}~{}~{}~{}~{}~{}~{}~{}~{}~{}~{}~{}~{}~{}~{}~{}~{}~{}~{}\n",
             account_number,
             account_data.cust_id,
             account_data.deal_name,
             account_data.principal_amount,
             account_data.deal_start_date ,
             account_data.cf_end_date ,
             account_data.accrued_interest,
             account_data.compounding_frequency,
             account_data.deal_value,
             account_data.gl,
             account_data.system,
             account_data.curr_nominal_int_rate,
             account_data.ratings,
             account_data.rating_aggency,
             account_data.asset_class,
             account_data.division,
             account_data.typ,
             account_data.originator,
             account_data.contract_yield,
             account_data.current_annual_yield,
             account_data.reset_frequency,
             account_data.interest_rate_type,
             account_data.expected_rate_reset_date,
             account_data.date1,
             account_data.intrest_portion,
             account_data.principal_payment,
             account_data.yld,
             account_data.mis_code_1,
             account_data.mis_code_2

        ));
        }
    }

    let mut op_writer = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create Output file: `{}` on location `{}` : {}",
            config_param.output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match op_writer.write_all(op_line.as_bytes()) {
        Ok(_) => info!(log, "Successfully written outputfile."),
        Err(error) => panic!(
            "Unable to write processed lines to file `{}`: {}.",
            config_param.output_file_path(),
            error
        ),
    }
    let mut non_concat_op_writer = match buf_file_wrtr(config_param.non_concat_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create Output file: `{}` on location `{}` : {}",
            config_param.non_concat_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };
    match non_concat_op_writer.write_all(non_concat_op.as_bytes()) {
        Ok(_) => info!(log, "Successfully written outputfile."),
        Err(error) => panic!(
            "Unable to write processed lines to file `{}`: {}.",
            config_param.output_file_path(),
            error
        ),
    }
    let health_report = HealthReport::new(tot_acc_enc, acc_enc, tot_acc_enc - acc_enc, 0.0, 0.0, 0);
    println!("{}", health_report.display());
}
