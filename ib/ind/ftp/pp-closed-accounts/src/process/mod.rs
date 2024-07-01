use self::config::*;
use chrono::Datelike;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::*;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::collections::HashSet;
use std::fs;
use std::io::prelude::*;
use std::io::BufWriter;
use std::{env::current_dir, fs::File};

mod config;

pub fn process(config_params: ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let mut acc_enc = 0;
    let mut tot_ip_amt = 0.0;
    let mut tot_op_amt = 0.0;
    let mut skp_rec = 0;

    let mut accs_key_set: HashSet<String> = HashSet::new();
    let config_data = get_files(config_params.config_file_path());

    let mut op_writer = get_writer(config_params.output_file_path());

    //Check the Input Data is for EOMONTH:
    if !is_month_end_date(config_params.as_on_date) {
        panic!("AsOn is not a EOMONTH Date");
    }

    //Reading Input File
    let input_reader =
        fs::read_to_string(config_params.input_file_path()).expect("Could Not Read Input File 1");
    for (line_no, line) in input_reader.lines().enumerate() {
        acc_enc += 1;
        let inp_vec: Vec<&str> = line
            .split(
                &config_data
                    .field_separator
                    .clone()
                    .unwrap_or("|".to_string()),
            )
            .collect::<Vec<&str>>();
        let acc_id = inp_vec[config_data.key_column.parse::<usize>().unwrap_or(1) - 1].to_string();
        tot_ip_amt += inp_vec[config_data
            .amount_column
            .parse::<usize>()
            .expect("Error Reading Amount Column")]
        .to_string()
        .parse()
        .unwrap_or(0.0);
        if accs_key_set.contains(&acc_id) {
            log_warn!(
                log,
                "Skipping Repeated Account: {} from EOMonth File",
                acc_id
            );
            skp_rec += 1;
            continue;
        }
        tot_op_amt += inp_vec[config_data
            .amount_column
            .parse::<usize>()
            .expect("Error Reading Amount Column")]
        .to_string()
        .parse()
        .unwrap_or(0.0);
        writeln!(op_writer, "{}", line)
            .unwrap_or_else(|_| panic!("Error in Writing Output at Line-Num: {}", line_no));

        accs_key_set.insert(acc_id);
    }

    let mut prev_date = config_params.as_on_date().day() - 1;
    while prev_date != 0 {
        let asondate = format!(
            "{:02}-{:02}-{}",
            prev_date,
            config_params.as_on_date().month(),
            config_params.as_on_date().year()
        );
        println!(
            "{:?} and {:?} and {:?}",
            asondate,
            config_params
                .as_on_date()
                .format("%d-%m-%Y")
                .to_string()
                .replace('-', ""),
            asondate.replace('-', "")
        );
        let input_file = &config_params.input_file_path().replace(
            &config_params
                .as_on_date()
                .format("%d-%m-%Y")
                .to_string()
                .replace('-', ""),
            &asondate.replace('-', ""),
        );
        let input_reader = fs::read_to_string(input_file)
            .unwrap_or_else(|_| panic!("Error in Reading Input File: {}", input_file));
        println!("{:?}", input_file);
        for (line_no, line) in input_reader.lines().enumerate() {
            acc_enc += 1;
            let inp_vec: Vec<&str> = line
                .split(
                    &config_data
                        .field_separator
                        .clone()
                        .unwrap_or("|".to_string()),
                )
                .collect::<Vec<&str>>();
            let acc_id =
                inp_vec[config_data.key_column.parse::<usize>().unwrap_or(1) - 1].to_string();
            if accs_key_set.contains(&acc_id) {
                skp_rec += 1;
                continue;
            }
            log_info!(
                log,
                "Found Missing Account: {} from File: {}",
                acc_id,
                input_file
            );
            tot_ip_amt += inp_vec[config_data
                .amount_column
                .parse::<usize>()
                .expect("Error Reading Amount Column")]
            .to_string()
            .parse()
            .unwrap_or(0.0);
            let mut output_data = String::new();
            let mut data_len = inp_vec.len();
            for (index, _) in inp_vec.iter().enumerate() {
                output_data.push_str(
                    if index
                        == config_data
                            .amount_column
                            .parse::<usize>()
                            .expect("Error getting Amount Field Column Number")
                            - 1
                    {
                        "0.0"
                    } else {
                        inp_vec[index]
                    },
                );
                output_data.push('|');
            }
            while data_len
                < config_data
                    .expected_column_count
                    .parse::<usize>()
                    .unwrap_or(data_len)
            {
                output_data.push_str(
                    if config_data
                        .date_field_columns
                        .clone()
                        .unwrap_or(Vec::new())
                        .contains(&data_len.to_string())
                    {
                        &asondate
                    } else {
                        inp_vec[data_len]
                    },
                );
                output_data.push('|');
                data_len += 1;
            }
            output_data.pop();
            writeln!(op_writer, "{}", output_data).unwrap_or_else(|_| {
                panic!(
                    "Error in Writing Output at Line-Num: {} for file: {:?}",
                    line_no, input_file
                )
            });
            accs_key_set.insert(acc_id);
        }
        prev_date -= 1;
    }

    let health_report = HealthReport::new(
        acc_enc,
        acc_enc - skp_rec,
        skp_rec,
        tot_ip_amt,
        tot_op_amt,
        0,
    );
    println!("{}", health_report.display());
    log_info!(log, "{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file_path());
}

pub fn get_writer(file_path: &str) -> BufWriter<File> {
    match buf_file_wrtr(file_path, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create file `{}` on location `{}` : {}",
            file_path,
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    }
}
