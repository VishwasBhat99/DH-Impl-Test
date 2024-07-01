mod structs;
use self::structs::Account;
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
use std::time::{Duration, SystemTime};

pub fn gen_summary(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
    prod_rpt_map: &HashMap<i64, f64>,
    llg_mapping: &HashMap<i64, i64>,
    exchange_rates: &HashMap<String, f64>,
) -> HashMap<i64, f64> {
    let start_time = SystemTime::now();
    let input_file_name = format!("{}-summary.txt", config_params.input_file_path());
    let output_file_name = format!("{}-converted-summary.txt", config_params.output_file_path());
    let input_rdr = match new_buf_rdr(&input_file_name) {
        Ok(r) => r,
        Err(e) => panic!(format!(
            "Cannot read file at path: '{}', Error: '{}'",
            config_params.input_file_path(),
            e
        )),
    };
    let mut output_wtr = match buf_file_wrtr(&output_file_name, None) {
        Ok(r) => r,
        Err(e) => panic!(format!(
            "Cannot write to file at path: '{}', Error: '{}'",
            config_params.output_file_path(),
            e
        )),
    };

    let mut total_account = 0;
    let mut total_failed_account = 0;
    let mut total_success_account = 0;
    let mut total_input_amount = 0.0;
    let mut total_output_amount = 0.0;
    let mut llg_distribution_ration: HashMap<i64, f64> = HashMap::new();

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
                            let mut ratio = 0.0;
                            let account_total_amount = account.amount;
                            if account.amount > limit {
                                let difference = account.amount - limit;
                                account.amount = limit;
                                ratio = account.amount / account_total_amount;
                                llg_distribution_ration.insert(account.llg_code, ratio);
                                total_output_amount += account.amount;
                                write_output(&account, &mut output_wtr, logger);
                                //get alternative llg code from the map(this llg code is for writing to file)
                                let llg_alternative = llg_mapping.get(&account.llg_code);
                                if llg_alternative.is_some() && *llg_alternative.unwrap_or(&0) != 0
                                {
                                    account.llg_code = *llg_alternative.unwrap_or(&0);
                                    account.amount = difference;
                                    ratio = account.amount / account_total_amount;
                                    llg_distribution_ration.insert(account.llg_code, ratio);
                                    total_output_amount += account.amount;
                                    write_output(&account, &mut output_wtr, logger);
                                }
                                //if alternative llg_code not found, then log the data
                                else {
                                    account.amount = difference;
                                    log_warn!(logger,"skipping account, llg mapping not found for : {}, account:{}",
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
    log_info!(
        logger,
        "total time for Summary Generation : {:#?}",
        start_time.elapsed()
    );

    println!("{}", health_report.display());
    println!(
        "total time for Summary Generation :{:?}",
        start_time.elapsed().unwrap_or(Duration::new(0, 0))
    );
    health_report.gen_health_rpt(config_params.output_file_path());
    llg_distribution_ration
}

//function to write each account to output
fn write_output(account: &Account, writer: &mut BufWriter<File>, logger: &Logger) {
    match writer.write_all(account.output_data().as_bytes()) {
        Ok(_) => {}
        Err(e) => log_error!(logger, "unable write output to output file: `{}`", e),
    }
}
