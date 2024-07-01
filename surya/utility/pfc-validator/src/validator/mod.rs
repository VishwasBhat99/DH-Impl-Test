use self::io::*;
use calamine::{open_workbook_auto, Reader};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use slog::Logger;
use statics::*;
use std::io::prelude::*;
use std::time::SystemTime;

mod io;

pub fn validate(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let st_tm_read = SystemTime::now();
    let mut alwd_skip = config_param.footer_count();
    let mut tot_amt = DEFAULT_FLOAT;
    let skp_rec = config_param.header_count() + config_param.footer_count();
    let mut tot_rec = config_param.header_count();

    match config_param.file_format().to_lowercase().as_str() {
        "txt" | "csv" | "na" => {
            let input_reader = read_file(config_param.input_file_path(), log);
            for (line_num, lines) in input_reader
                .lines()
                .enumerate()
                .skip(config_param.header_count())
            {
                tot_rec += 1;
                let line = extract_lines(line_num, lines, config_param.input_file_path());
                let fields: Vec<&str> = line.split(config_param.input_delimeter()).collect();
                if let Some(amt) = fields.get(config_param.amt_field_pos() - 1) {
                    tot_amt += amt.parse().unwrap_or(DEFAULT_FLOAT);
                } else if alwd_skip > 0 {
                    alwd_skip -= 1;
                    continue;
                } else {
                    let err_msg = format!(
                        "File: `{}` doesn't contains any data for amount field pos: `{}` at line number: `{}`.",
                        config_param.input_file_path(),
                        config_param.amt_field_pos(),
                        line_num + 1,
                    );
                    log_error!(log, "{}", err_msg);
                    panic!("{}", err_msg);
                }
            }
        }
        "xlsx" | "xls" => {
            let mut input_reader = open_workbook_auto(config_param.input_file_path())
                .expect("Unable to open Input File.");
            if let Some(Ok(reader)) = input_reader.worksheet_range(config_param.input_sheet_name())
            {
                for row in reader.rows().skip(config_param.header_count()) {
                    tot_rec += 1;
                    if let Some(amt) = row.get(config_param.amt_field_pos() - 1) {
                        tot_amt += amt.to_string().parse().unwrap_or(DEFAULT_FLOAT);
                    } else if alwd_skip > 0 {
                        alwd_skip -= 1;
                        continue;
                    } else {
                        let err_msg = format!(
                            "File: `{}` at sheet: `{}` doesn't contains any data for amount at field position: `{}` at line number: `{}`.",
                            config_param.input_file_path(),
                            config_param.input_sheet_name(),
                            config_param.amt_field_pos(),
                            tot_rec - config_param.footer_count(),
                        );
                        log_error!(log, "{}", err_msg);
                        panic!("{}", err_msg);
                    }
                }
            }
        }
        _ => {
            let err_msg = format!(
                "Unknown File format: `{}` for `{}`.",
                config_param.file_format(),
                config_param.input_file_path(),
            );
            log_error!(log, "{}", err_msg);
            panic!("{}", err_msg);
        }
    }

    if tot_amt == 0.0 {
        let err_msg = format!(
            "File: `{}` doesn't contains any data for amount field at position: `{}`.",
            config_param.input_file_path(),
            config_param.amt_field_pos(),
        );
        log_error!(log, "{}", err_msg);
        panic!("{}", err_msg);
    }
    let ed_tm_read = SystemTime::now();
    let duration = ed_tm_read
        .duration_since(st_tm_read)
        .expect("Could not calculate total read process duration.");
    debug!(diag_log, "Read Process Total Duration: {:?}.", duration);

    let st_tm_writer = SystemTime::now();

    let health_report = HealthReport::new(
        tot_rec as i64,
        (tot_rec - skp_rec) as i64,
        skp_rec as i64,
        tot_amt,
        tot_amt,
        0,
    );
    log_info!(log, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_param.input_file_path());

    let ed_tm_writer = SystemTime::now();
    let duration = ed_tm_writer
        .duration_since(st_tm_writer)
        .expect("Could not calculate total duration for write process.");
    debug!(diag_log, "Writing OD, Total Duration: {:?}.", duration);
}
