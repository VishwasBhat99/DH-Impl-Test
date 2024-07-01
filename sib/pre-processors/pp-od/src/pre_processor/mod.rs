extern crate csv;
extern crate serde;
use self::csv::ReaderBuilder;
use self::derive_fields::get_op_line;
use self::input_account::{
    AdditionLoanFile, InputAccount, IntRateData, NPAData, RateCode, TblCodes,
};
use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::{HashMap, HashSet};
use std::env::current_dir;
use std::io::prelude::*;
use std::io::BufWriter;
use std::time::SystemTime;
mod derive_fields;
mod input_account;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let output_file = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{} Cannot read output file path", error),
    };
    let start_derive_timer = SystemTime::now();
    let mut op_line: String = String::new();
    let mut tot_acc_encntrd: i64 = 0;
    let mut tot_succ = 0;
    let mut writer = BufWriter::new(output_file);
    let mut reader = match ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path(config_param.input_file_path())
    {
        Ok(read) => read,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut int_rate_reader = match ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path(config_param.int_rate_file_path())
    {
        Ok(read) => read,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.int_rate_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut ratecode_reader = match ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path(config_param.ratecode_file_path())
    {
        Ok(read) => read,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.ratecode_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut tbl_code_reader = match ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path(config_param.tbl_code_file_path())
    {
        Ok(read) => read,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.tbl_code_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let npa_file = match new_buf_rdr(config_param.npa_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            config_param.npa_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let config_file = match new_buf_rdr(config_param.config_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            config_param.config_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let loan_file = match new_buf_rdr(config_param.additional_loan_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            config_param.config_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let mut int_rate_map: HashMap<String, IntRateData> = HashMap::new();
    for (line_num, lines) in int_rate_reader.deserialize().enumerate() {
        let int_rate_data: IntRateData = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    log,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.int_rate_file_path(),
                    line_num + 1,
                    error
                );
                Default::default()
            }
        };
        int_rate_map.insert(int_rate_data.acid.to_owned(), int_rate_data);
    }
    let mut ratecode_map: HashMap<String, RateCode> = HashMap::new();
    for (line_num, lines) in ratecode_reader.deserialize().enumerate() {
        let ratecode_data: RateCode = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    log,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.ratecode_file_path(),
                    line_num + 1,
                    error
                );
                Default::default()
            }
        };
        ratecode_map.insert(ratecode_data.intrate_code.to_owned(), ratecode_data);
    }
    let mut tblcodes_set: HashSet<String> = HashSet::new();
    for (line_num, lines) in tbl_code_reader.deserialize().enumerate() {
        let tbl_code_data: TblCodes = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    log,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.tbl_code_file_path(),
                    line_num + 1,
                    error
                );
                Default::default()
            }
        };
        tblcodes_set.insert(tbl_code_data.int_tbl_code);
    }
    let mut npa_map: HashMap<String, NPAData> = HashMap::new();
    for (line_num, lines) in npa_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.npa_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        npa_map.insert(
            fields[1].to_string(),
            NPAData {
                npa_classification: fields[8].to_string(),
                cust_hlth_code: fields[12].to_string(),
                cust_npa_class: fields[17].to_string(),
                final_npa_class: fields[18].to_string(),
                npa_amount: fields[10].to_string(),
            },
        );
    }
    let mut config_vec: Vec<String> = Vec::new();
    for (_line_num, line) in config_file.lines().enumerate() {
        config_vec.push(line.unwrap_or_else(|_| "".to_string()));
    }
    
    let mut additional_loan_file_map: HashMap<String, AdditionLoanFile> = HashMap::new();
    for (line_num, lines) in loan_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.npa_file_path(),
                line_num + 1,
                error
            ),
        };
        let additional_loan_file_vec: Vec<&str> = line.split('|').collect();
        additional_loan_file_map.insert(
            additional_loan_file_vec[0].to_string(),
            AdditionLoanFile {
                gnt_type: additional_loan_file_vec[3].to_string(),
                status_code: additional_loan_file_vec[4].to_string(),
                occupation_code: additional_loan_file_vec[5].to_string(),
                sector: additional_loan_file_vec[6].to_string(),
                sector_code: additional_loan_file_vec[7].to_string(),
                subsector_code: additional_loan_file_vec[8].to_string(),
                staffflag: additional_loan_file_vec[9].to_string(),
                cre_free_text_1: additional_loan_file_vec[10].to_string(),
                pres_val_sec:additional_loan_file_vec[13].parse::<f64>().unwrap_or(0.0).abs(),
                paripassu_perc:additional_loan_file_vec[13].parse::<f64>().unwrap_or(0.0).abs(),
                prov_percent: additional_loan_file_vec[13].parse::<f64>().unwrap_or(0.0).abs(),
                dumm2: additional_loan_file_vec[15].parse::<f64>().unwrap_or(0.0).abs(),
                dumm3: additional_loan_file_vec[16].parse::<f64>().unwrap_or(0.0).abs(),
                dumm4: additional_loan_file_vec[17].parse::<f64>().unwrap_or(0.0).abs(),
                dumm5: additional_loan_file_vec[18].to_string(),
                dumm6: additional_loan_file_vec[19].to_string(),
                dumm7: additional_loan_file_vec[20].to_string(),
                dumm8: additional_loan_file_vec[21].to_string(),
                dumm9: NaiveDate::parse_from_str(&additional_loan_file_vec[22].to_string(), "%d-%m-%Y").unwrap_or(NaiveDate::from_ymd_opt(1970,1, 1).expect("Unable to get Def-Date")),
                dumm10: NaiveDate::parse_from_str(&additional_loan_file_vec[23].to_string(), "%d-%m-%Y").unwrap_or(NaiveDate::from_ymd_opt(1970,1, 1).expect("Unable to get Def-Date")),
                const_catgory_code: additional_loan_file_vec[24].to_string(),
                rating_agc: additional_loan_file_vec[25].to_string(),
                rating: additional_loan_file_vec[26].to_string(),
                super_annuation_flag: additional_loan_file_vec[27].to_string(),
                turn_amt1: additional_loan_file_vec[28].parse::<f64>().unwrap_or(0.0).abs(),
                turn_amt2: additional_loan_file_vec[29].parse::<f64>().unwrap_or(0.0).abs(),
                turn_amt_3: additional_loan_file_vec[30].parse::<f64>().unwrap_or(0.0).abs(),
                ftp_char1: additional_loan_file_vec[31].to_string(),
                ftp_char2: additional_loan_file_vec[32].to_string(),
                ftp_amt1: additional_loan_file_vec[33].parse::<f64>().unwrap_or(0.0).abs(),
                ftp_amt2: additional_loan_file_vec[34].parse::<f64>().unwrap_or(0.0).abs(),
                ftp_date1: NaiveDate::parse_from_str(&additional_loan_file_vec[35].to_string(), "%d-%m-%Y").unwrap_or(NaiveDate::from_ymd_opt(1970,1, 1).expect("Unable to get Def-Date")),
                ftp_date2: NaiveDate::parse_from_str(&additional_loan_file_vec[35].to_string(), "%d-%m-%Y").unwrap_or(NaiveDate::from_ymd_opt(1970, 1, 1).expect("Unable to get Def-Date")),
            },
        );
    }
    for (line_num, lines) in reader.deserialize().enumerate() {
        let input_account: InputAccount = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    log,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.input_file_path(),
                    line_num + 1,
                    error
                );
                Default::default()
            }
        };
        tot_acc_encntrd += 1;
    let temp_string = get_op_line(
        &input_account,
        &int_rate_map,
        &ratecode_map,
        &tblcodes_set,
        &npa_map,
        &additional_loan_file_map,
        &config_vec,
        *config_param.as_on_date(),
    );
    op_line.push_str(temp_string.as_str());
    op_line.push('\n');
    tot_succ += 1;
}
    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_succ,
        tot_acc_encntrd - tot_succ,
        0.0,
        0.0,
        0,
    );
    health_report.gen_health_rpt(config_param.output_file_path());
    let end_derive_timer = SystemTime::now();
    let duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total derive process duration.");
    debug!(diag_log, "Derive Process Total Duration: {:?}.", duration);
    let start_writer_time = SystemTime::now();
    match writer.write_all(op_line.as_bytes()) {
        Ok(val) => val,
        Err(error) => {
            panic!("Error writing processed data: {:?}", error);
        }
    }

    let end_writer_time = SystemTime::now();
    let duration = end_writer_time
        .duration_since(start_writer_time)
        .expect("Could not calculate total write process duration.");
    info!(diag_log, "Write Process Total Duration: {:?}.", duration);
}
