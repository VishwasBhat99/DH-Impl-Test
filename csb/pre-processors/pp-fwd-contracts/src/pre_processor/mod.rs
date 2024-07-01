use self::derive_fields::{get_output, get_value};
use calamine::{open_workbook_auto, Reader};
use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_agg_rules_txt::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use statics::DEFAULT_FLOAT;
use std::collections::HashMap;
use std::io::{BufRead, BufWriter, Write};
use std::process::Command;
use std::time::SystemTime;

mod derive_fields;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let st_tm_read = SystemTime::now();
    let mut tot_rec = 0;
    let mut tot_succ = 0;
    let def_date = NaiveDate::from_ymd(1970, 01, 01);
    let op_path = format!("{}{}", config_param.output_file_path(), ".txt");
    let output_file = match buf_file_wrtr(&op_path, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not find Output file: `{}` : {}.",
            config_param.output_file_path(),
            error
        ),
    };
    let mut op_writer = BufWriter::new(output_file);

    let extra_fields_reader = match new_buf_rdr(config_param.exfields_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not find exfields file: `{}` : {}.",
            config_param.exfields_file_path(),
            error
        ),
    };
    let mut extra_fields: HashMap<String, String> = HashMap::new();
    for (line_num, lines) in extra_fields_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    log,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.exfields_file_path(),
                    line_num + 1,
                    error
                );
                continue;
            }
        };
        let ex_fields: Vec<&str> = line.split('|').collect();
        extra_fields.insert(ex_fields[10].to_string(), ex_fields[8].to_string());
    }

    let ip_txt_path = config_param
        .input_file_path()
        .replace(".xlsx", "_converted.txt");
    let input_file = match buf_file_wrtr(&ip_txt_path, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not find Input txt file: `{}` : {}.",
            ip_txt_path, error
        ),
    };
    let mut ip_writer = BufWriter::new(input_file);
    let mut input_file_excel =
        open_workbook_auto(config_param.input_file_path()).expect("Unable to open Input File.");
    if let Some(Ok(reader)) = input_file_excel.worksheet_range(config_param.input_sheet_name()) {
        for row in reader.rows() {
            get_output(row, &mut ip_writer, *config_param.as_on_date(), def_date);
        }
    }
    ip_writer.flush().unwrap();

    let accounts = reader::Reader::new_at_path(config_param.metadata_file_path(), &ip_txt_path);
    let rw_rules = AggRules::new_from_path(config_param.rw_file_path(), &accounts);
    let ccf_rules = AggRules::new_from_path(config_param.ccf_file_path(), &accounts);

    let input_reader = match new_buf_rdr(&ip_txt_path) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not find Input txt file: `{}` : {}.",
            ip_txt_path, error
        ),
    };
    for (line_num, line) in input_reader.lines().enumerate() {
        tot_rec += 1;
        let line = line.expect("Could Not Read Line");
        let input_fields: Vec<&str> = line.split('|').collect();
        let mut op_line = String::new();
        if input_fields.first() == Some(&"") {
            log_info!(log, "Deal_number missing skipped line: {}", line_num);
            continue;
        }
        tot_succ += 1;
        for i in 0..26 {
            op_line.push_str(&input_fields[i].to_string());
            op_line.push_str("|");
        }
        op_line.push_str(
            extra_fields
                .get(&input_fields[23].to_string())
                .unwrap_or(&"NA".to_string()),
        );
        op_line.push_str("|");
        op_line.push_str(&input_fields[27].to_string());
        op_line.push_str("|");
        let rw = get_value(&line.to_string(), &rw_rules, &accounts);
        let risk_weight = if rw.len() < 4 {
            rw
        } else {
            rw[rw.len() - 3..].to_string()
        };
        let ccf = get_value(&line.to_string(), &ccf_rules, &accounts);
        op_line.push_str(&ccf.to_string());
        op_line.push_str("|");
        let total_exp = input_fields[19].parse::<f64>().unwrap_or(DEFAULT_FLOAT)
            + input_fields[18].parse::<f64>().unwrap_or(DEFAULT_FLOAT);
        op_line.push_str(&total_exp.to_string());
        op_line.push_str("|");
        op_line.push_str(&risk_weight.to_string());
        op_line.push_str("|");
        let rw_value = total_exp * risk_weight.parse::<f64>().unwrap_or(DEFAULT_FLOAT);
        op_line.push_str(&rw_value.to_string());
        op_line.push_str("\n");
        op_writer.write_all(op_line.as_bytes()).unwrap();
    }
    //remove the temporary text file created
    Command::new("rm")
        .arg(&ip_txt_path)
        .output()
        .expect("rm command failed to start");

    let ed_tm_read = SystemTime::now();
    let duration = ed_tm_read
        .duration_since(st_tm_read)
        .expect("Could not calculate total read process duration.");
    debug!(diag_log, "Read Process Total Duration: {:?}.", duration);

    let st_tm_writer = SystemTime::now();
    let health_report = HealthReport::new(tot_rec, tot_succ, tot_rec - tot_succ, 0.0, 0.0, 0);
    log_info!(log, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_param.output_file_path());

    let ed_tm_writer = SystemTime::now();
    let duration = ed_tm_writer
        .duration_since(st_tm_writer)
        .expect("Could not calculate total duration for write process.");
    debug!(
        diag_log,
        "Writing Pre-Processor, Total Duration: {:?}.", duration
    );
}
