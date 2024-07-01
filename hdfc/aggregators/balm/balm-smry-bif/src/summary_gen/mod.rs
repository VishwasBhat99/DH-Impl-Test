mod structs;
use calamine::{open_workbook, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use statics::{DEFAULT_FLOAT, DEFAULT_INT};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufWriter, Write};
use summary_gen::structs::Account;
pub fn gen_summary(config_params: ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let input_rdr = match new_buf_rdr(config_params.input_file_path()) {
        Ok(r) => r,
        Err(e) => panic!(format!(
            "Cannot read file at path: '{}', Error: '{}'",
            config_params.input_file_path(),
            e
        )),
    };
    let mut output_wtr = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(r) => r,
        Err(e) => panic!(format!(
            "Cannot write to file at path: '{}', Error: '{}'",
            config_params.output_file_path(),
            e
        )),
    };
    let prod_rpt_map = get_product_rpt_map(config_params.product_rpt_file_path(), logger);
    let llg_mapping = get_llg_mapping(config_params.llg_mapping_file_path());
    let exchange_rates = get_exchnage_rates(config_params.currency_conversion_file_path());
    let mut total_account = 0;
    let mut total_failed_account = 0;
    let mut total_success_account = 0;
    let mut total_input_amount = 0.0;
    let mut total_output_amount = 0.0;
    for line in input_rdr.lines() {
        match line {
            //if the line has no errors
            Ok(each_line) => {
                match Account::new_from_line(each_line.to_owned()) {
                    //if theres no error when line was parsed as account struct
                    Ok(mut account) => {
                        total_account += 1;
                        total_success_account += 1;
                        let limit_amount = prod_rpt_map.get(&account.llg_code);
                        total_input_amount += account.amount;
                        //if the limit_amount was found in the prod_map hashmap
                        if limit_amount.is_some() && limit_amount.unwrap_or(&0.0) != &0.0 {
                            let mut limit = *limit_amount.unwrap_or(&0.0);
                            //if the conversation from INT->INR no need to lookup in hashmap
                            let mut conversion = 0.0;
                            if !account.currency.eq("INR") {
                                //if from->to (ccy) not found, take default as 1(means no conversion)
                                conversion = *exchange_rates.get(&account.currency).unwrap_or(&1.0);
                                limit *= conversion;
                            }
                            //if the account amount greater than limit
                            // write limit first and then difference amount
                            if account.amount > limit {
                                let diffrence = account.amount - limit;
                                account.amount = limit;
                                total_output_amount += account.amount;
                                write_output(&account, &mut output_wtr, logger);
                                //get alternative llg code from the map(this llg code is for writing to file)
                                let llg_alternative = llg_mapping.get(&account.llg_code);
                                if llg_alternative.is_some() {
                                    account.llg_code = *llg_alternative.unwrap_or(&0);
                                    account.amount = diffrence;
                                    total_output_amount += account.amount;
                                    write_output(&account, &mut output_wtr, logger);
                                }
                                //if alternative llg_code not found, then log the data
                                else {
                                    log_info!(logger,"skipping account, llg mapping nout found for : {}, account:{}",
                                account.llg_code, account.output_data());
                                }
                            }
                            //if the account amount < limit, then write account as it is
                            else {
                                total_output_amount += account.amount;
                                write_output(&account, &mut output_wtr, logger);
                            }
                        }
                        //if the limit amount was not found this llgcode, write line as it is
                        else {
                            total_output_amount += account.amount;
                            write_output(&account, &mut output_wtr, logger);
                        }
                    }
                    Err(e) => {
                        total_account += 1;
                        total_failed_account += 1;
                        log_error!(logger, "Couldn't parse InputAccount: {}", e);
                    }
                }
            }
            Err(..) => {}
        }
    }
    let health_report = HealthReport::new(
        total_account,
        total_success_account,
        total_failed_account,
        total_input_amount,
        total_output_amount,
        0,
    );
    log_info!(logger, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file_path());
}

//this function reads the product_report excel and gets llg -> limit_amount
pub fn get_product_rpt_map(product_rpt_file: &str, logger: &Logger) -> HashMap<i64, f64> {
    let mut pdt_rpt_map: HashMap<i64, f64> = HashMap::new();
    let mut pdt_rpt_excel: Xlsx<_> =
        open_workbook(product_rpt_file).expect("Error while opening `product report file`.");
    let sheet_name = pdt_rpt_excel
        .sheet_names()
        .first()
        .unwrap_or(&"Sheet1".to_string())
        .to_owned();
    if let Some(Ok(reader)) = pdt_rpt_excel.worksheet_range(sheet_name.as_str()) {
        for row in reader.rows().skip(1) {
            let limit: f64;
            if row[3].is_empty() {
                continue;
            }
            if row[11].is_empty() {
                limit = 0.0;
            } else {
                limit = str_to_flt(row[11].to_string().as_str())
            }
            pdt_rpt_map.insert(str_to_int(row[3].to_string().as_str()), limit);
        }
    }
    pdt_rpt_map
}

//this function gets exchnage rates for INR -> Anything
pub fn get_exchnage_rates(exchange_rate_file: &str) -> HashMap<String, f64> {
    let mut exchanges_rates: HashMap<String, f64> = HashMap::new();
    let rdr = match new_buf_rdr(exchange_rate_file) {
        Ok(r) => r,
        Err(e) => panic!(format!(
            "Cannot read file at path: '{}', Error: '{}'",
            exchange_rate_file, e
        )),
    };
    for line in rdr.lines() {
        if let Ok(each_line) = line {
            let line_contents: Vec<&str> = each_line.split("|").collect();
            if line_contents.len() < 3 {
                continue;
            }
            if line_contents[0].eq("INR") {
                exchanges_rates.insert(line_contents[1].to_string(), str_to_flt(line_contents[2]));
            }
        }
    }
    exchanges_rates
}

pub fn str_to_flt(num: &str) -> f64 {
    num.parse().unwrap_or(DEFAULT_FLOAT)
}
pub fn str_to_int(num: &str) -> i64 {
    num.parse().unwrap_or(DEFAULT_INT)
}

//this function reads the llgmaping file and return a hashmap<from_llg, to_llog>
fn get_llg_mapping(llg_mapping_file: &str) -> HashMap<i64, i64> {
    let mut llg_map: HashMap<i64, i64> = HashMap::new();
    let rdr = match new_buf_rdr(llg_mapping_file) {
        Ok(r) => r,
        Err(e) => panic!(format!(
            "Cannot read file at path: '{}', Error: '{}'",
            llg_mapping_file, e
        )),
    };
    for line in rdr.lines() {
        if let Ok(each_line) = line {
            let line_contents: Vec<&str> = each_line.split("|").collect();
            if line_contents.len() < 2 {
                continue;
            }
            llg_map.insert(str_to_int(line_contents[0]), str_to_int(line_contents[1]));
        }
    }
    llg_map
}

//function to write each account to output
fn write_output(account: &Account, writer: &mut BufWriter<File>, logger: &Logger) {
    match writer.write_all(account.output_data().as_bytes()) {
        Ok(_) => {}
        Err(e) => log_error!(logger, "unable write output to output file: `{}`", e),
    }
}
