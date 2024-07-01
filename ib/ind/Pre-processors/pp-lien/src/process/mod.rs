use self::account::*;
use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufWriter;
use std::io::{BufReader, Write};
mod account;

pub fn process(config_params: ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let mut tot_acc_encntrd = 0;
    let mut tot_amt = 0.0;
    let mut skip_rec_count = 0;

    //Init Writer
    let mut writer = get_writer(config_params.output_file_path());

    let mut td_data_map: HashMap<String, TDVal> = HashMap::new();
    let td_input = File::open(config_params.td_input_file()).expect("Could Not Read TD Input File");
    let td_reader = BufReader::new(td_input);
    for (line_no, line) in td_reader.lines().enumerate() {
        let acc_info: String = match line {
            Ok(acc_info) => acc_info,
            Err(error) => {
                skip_rec_count += 1;
                log_error!(
                    log,
                    "Cannot read line {} from TD input file: {:?}",
                    line_no,
                    error
                );
                continue;
            }
        };
        let td_fields: Vec<&str> = acc_info.split('|').collect();
        let td_val = TDVal::new(td_fields.clone());
        let acc_id = td_fields[0].to_string();
        td_data_map.insert(acc_id, td_val);
    }
    //Processing Lien File
    let lien_input =
        File::open(config_params.lien_input_file()).expect("Could Not Read Lien Input File");
    let lien_reader = BufReader::new(lien_input);
    for (line_no, line) in lien_reader.lines().enumerate() {
        let acc_info: String = match line {
            Ok(acc_info) => acc_info,
            Err(error) => {
                skip_rec_count += 1;
                log_error!(
                    log,
                    "Cannot read line {} from Lien input file: {:?}",
                    line_no,
                    error
                );
                continue;
            }
        };
        tot_acc_encntrd += 1;
        let input_fields: Vec<&str> = acc_info.split('|').collect();
        let key_1 = input_fields[0].to_string();
        if input_fields.len() != 6 {
            skip_rec_count += 1;
            log_error!(log,
                "Cannot read line {} from input file for acc_no: {:?} due to incorrect column count {:?}",
                line_no,
                key_1,
                input_fields.len());
            continue;
        }
        let mut lien_acc = LienData::new(input_fields);
        let mat_date = get_date(&lien_acc.mat_date, *config_params.as_on_date());
        //Default Tenor would be 30 Days
        lien_acc.tenor_flag = if rbdate::incr_dt_by_days(
            *config_params.as_on_date(),
            config_params.tenor().parse::<i64>().unwrap_or_else(|_| {
                panic!(
                    "Unable to parse Tenor: {} passed as config-param.",
                    config_params.tenor()
                )
            }),
        ) < mat_date
        {
            "N"
        } else {
            "Y"
        }
        .to_string();
        (lien_acc.cust_class, lien_acc.curr) = (
            td_data_map
                .get(&format!("003{}", lien_acc.acct_no))
                .unwrap_or(&TDVal::def())
                .cust_class
                .to_string(),
            td_data_map
                .get(&format!("003{}", lien_acc.acct_no))
                .unwrap_or(&TDVal::def())
                .currency
                .to_string(),
        );
        tot_amt += lien_acc.tdrm_amt;
        lien_acc.mat_date = mat_date.format("%d-%m-%Y").to_string();
        let output_line = format_output(&lien_acc);
        writer
            .write_all(output_line.as_bytes())
            .expect("Error writing Line Account to Output File!!");
    }
    // Generate Health Check Report
    let health_report = health_report::HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - skip_rec_count,
        skip_rec_count,
        tot_amt,
        tot_amt,
        0,
    );
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file_path());
}

pub fn get_writer(file_path: &str) -> BufWriter<File> {
    match sdb_io::buf_file_wrtr(file_path, None) {
        Ok(file) => file,
        Err(error) => panic!("Unable to create output file `{}`: {}", file_path, error),
    }
}

pub fn get_date(data: &str, as_on_date: rbdate::NaiveDate) -> rbdate::NaiveDate {
    let mut date_value = data.parse::<i64>().unwrap_or(0) + 1;
    if !(1..=99998).contains(&date_value) {
        date_value = 366;
    }
    rbdate::datevalue_to_naive_date(&(date_value).to_string()).unwrap_or(as_on_date)
}
