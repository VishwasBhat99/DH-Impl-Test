use self::config::InputPositions;
use self::config::{get_files, LookupAcc};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use math::round::half_away_from_zero;
use sdb_io::*;
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::Write;
use std::io::{BufReader, BufWriter};
use std::time::SystemTime;

static DEFAULT_INT: i64 = 0;
static DEFAULT_FLOAT: f64 = 0.0;

mod config;

pub fn override_output(config_params: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_timer = SystemTime::now();
    let files_config = get_files(config_params.config_file_path());
    for file in files_config.files {
        let mut tot_rec = DEFAULT_INT;
        let mut tot_acc_skip = DEFAULT_INT;
        let mut tot_acc_succ = DEFAULT_INT;
        let out_path = format!("{}_Override.txt", &file.output_file_path);
        let stamp_out_path = format!("{}_New.txt", &file.output_file_path);

        let mut out_writer = match buf_file_wrtr(&out_path, None) {
            Ok(file) => file,
            Err(error) => panic!(
                "Unable to create file `{}` on location `{}` : {}",
                out_path, file.output_file_path, error
            ),
        };
        let mut stamp_out_writer = match buf_file_wrtr(&stamp_out_path, None) {
            Ok(file) => file,
            Err(error) => panic!(
                "Unable to create file `{}` on location `{}` : {}",
                stamp_out_path, file.output_file_path, error
            ),
        };

        // Read the Input Override file
        let mut override_map: HashMap<LookupAcc, f64> = HashMap::new();
        let override_file =
            File::open(&file.override_file_path).expect("Could Not Read The Input Override File");
        let reader = BufReader::new(override_file);
        for (_index, line) in reader.lines().enumerate() {
            let acc_data = line.unwrap();
            let fields: Vec<&str> = acc_data.split(&file.override_delimiter).collect();
            let lookup_key = LookupAcc {
                source_name: fields[0].to_string().to_uppercase(),
                acc_no: fields[1].to_string(),
            };
            override_map.insert(lookup_key, fields[2].parse::<f64>().unwrap());
        }

        // Read the Extracted file
        let input_file =
            File::open(&file.extracted_file_path).expect("Could Not Read The Extracted File");
        let input_reader = BufReader::new(input_file);
        for (index, line) in input_reader.lines().enumerate() {
            tot_rec += 1;
            let acc_data = match line {
                Ok(line) => line,
                _ => {
                    log_info!(
                        log,
                        "Invalid record, Account skipped - line num: {}",
                        (index + 1)
                    );
                    tot_acc_skip += 1;
                    continue;
                }
            };
            tot_acc_succ += 1;
            let input_fields: Vec<&str> = acc_data.split(&file.stamper_delimiter).collect();
            write_output(
                &input_fields,
                &file.source_name,
                &file.extracted_file_positions,
                &override_map,
                &mut out_writer,
                config_params.bal_precision(),
            );
        }
        out_writer.flush().unwrap();
        let health_report = HealthReport::new(tot_rec, tot_acc_succ, tot_acc_skip, 0.0, 0.0, 0);
        health_report.gen_health_rpt(&file.output_file_path);

        let mut tot_st_rec = DEFAULT_INT;
        let mut tot_st_acc_skip = DEFAULT_INT;
        let mut tot_st_acc_succ = DEFAULT_INT;
        // Read Stamper File
        let stamper_file =
            File::open(&file.stamper_file_path).expect("Could Not Read The Input Stamper File");
        let stamper_reader = BufReader::new(stamper_file);
        for (index, line) in stamper_reader.lines().enumerate() {
            tot_st_rec += 1;
            let acc_data = match line {
                Ok(line) => line,
                _ => {
                    log_info!(
                        log,
                        "Invalid record, Account skipped - line num: {}",
                        (index + 1)
                    );
                    tot_st_acc_skip += 1;
                    continue;
                }
            };
            tot_st_acc_succ += 1;
            let input_fields: Vec<&str> = acc_data.split(&file.stamper_delimiter).collect();
            write_output(
                &input_fields,
                &file.source_name,
                &file.stamper_file_positions,
                &override_map,
                &mut stamp_out_writer,
                config_params.bal_precision(),
            );
        }
        stamp_out_writer.flush().unwrap();
        let health_report = HealthReport::new(tot_rec, tot_acc_succ, tot_acc_skip, 0.0, 0.0, 0);
        health_report.gen_health_rpt(&file.output_file_path);
        let report_string = format!(
            "Stamper Output:\n\
             Accounts Encountered: {}\n\
             Accounts Skipped: {}\n\
             Accounts Successful: {}",
            tot_st_rec, tot_st_acc_skip, tot_st_acc_succ,
        );
        println!("{}", report_string);
    }
    let end_time = SystemTime::now();
    let duration = end_time
        .duration_since(start_timer)
        .expect("Could not calculate total process duration.");
    info!(diag_log, "Process Total Duration: {:?}.", duration);
}

pub fn write_output(
    input_fields: &Vec<&str>,
    source_name: &str,
    inp_positions: &InputPositions,
    override_map: &HashMap<LookupAcc, f64>,
    writer: &mut BufWriter<File>,
    precision: i8,
) {
    let acc_no = input_fields[inp_positions.acc_num_position - 1];
    let lookup_accno = LookupAcc {
        acc_no: acc_no.to_string(),
        source_name: source_name.to_uppercase(),
    };
    let stamper_ftp_rate = input_fields[inp_positions.ftp_rate_position - 1]
        .parse::<f64>()
        .unwrap_or(DEFAULT_FLOAT);
    let ftp_amount = input_fields[inp_positions.ftp_amount_ccy_position - 1]
        .parse::<f64>()
        .unwrap_or(DEFAULT_FLOAT);

    let ftp_rate = override_map.get(&lookup_accno).unwrap_or(&stamper_ftp_rate);
    let ftp_amt_ccy = half_away_from_zero((ftp_amount * (*ftp_rate)) / stamper_ftp_rate, precision);
    let adj6 = (stamper_ftp_rate - ftp_rate).to_string();

    let mut op_line = String::new();
    for index_count in 0..input_fields.len() {
        if index_count == inp_positions.ftp_rate_position - 1 {
            op_line.push_str(&ftp_rate.to_string());
            op_line.push_str("|");
            continue;
        }
        if index_count == inp_positions.adj_rate_position - 1 {
            op_line.push_str(&adj6.to_string());
            op_line.push_str("|");
            continue;
        }
        if index_count == inp_positions.ftp_amount_ccy_position - 1
            || index_count == inp_positions.ftp_amount_hcy_position - 1
        {
            op_line.push_str(&ftp_amt_ccy.to_string());
            op_line.push_str("|");
            continue;
        }
        op_line.push_str(&input_fields[index_count].to_string());
        op_line.push_str("|");
    }
    op_line.pop();
    op_line.push_str("\n");
    writer.write_all(op_line.as_bytes()).unwrap();
}
