use calamine::{open_workbook_auto, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::io::BufRead;
use std::path::Path;

pub fn process(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut tot_rec = 0;
    let mut skp_rec = 0;
    let mut tot_amt = 0.0;
    let mut exrt: f64 = 1.0;

    let mut output_sheet = umya_spreadsheet::new_file();

    // new worksheet
    output_sheet
        .new_sheet(config_params.output_sheet_name())
        .expect("Could not add new sheet");
    let exrt_file = match new_buf_rdr(config_params.exrt_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(format!(
            "Cannot read file at path: '{}', Error: '{}'",
            config_params.exrt_file_path(),
            error
        )),
    };
    for line in exrt_file.lines() {
        if let Ok(each_line) = line {
            let line_contents: Vec<&str> = each_line.split("|").collect();
            if line_contents.len() < 3 {
                skp_rec += 1;
                continue;
            }
            if line_contents[0].eq(config_params.from_ccy())
                && line_contents[1].eq(config_params.to_ccy())
            {
                exrt = line_contents[2].parse::<f64>().unwrap_or(1.0);
                break;
            }
        }
    }
    let mut input_file = open_workbook_auto(config_params.input_file_path()).expect(&format!(
        "Could not open file at: {}",
        config_params.input_file_path()
    ));
    if let Some(Ok(reader)) = input_file.worksheet_range(config_params.input_sheet_name()) {
        tot_rec += 1;
        let mut row_num = 1;
        for row in reader.rows() {
            let mut col_num = 1;
            let mut row_vec: Vec<String> = Vec::new();
            for index in 0..row.len() {
                if row.len() == 0 {
                    continue;
                }
                if index == 2 || index == 5 || index == 6 || index == 7 || index == 8 {
                    if row[index].to_string().parse::<f64>().is_ok() {
                        let existing_amt = row[index]
                            .to_string()
                            .parse::<f64>()
                            .expect("Invalid amount");
                        let new_amt = existing_amt * exrt;
                        row_vec.push(new_amt.to_string());
                    }
                } else {
                    row_vec.push(row[index].to_string());
                }
                if index == 2 {
                    if row[index].to_string().parse::<f64>().is_ok() {
                        tot_amt += row[index]
                            .to_string()
                            .parse::<f64>()
                            .expect("Cannot calculate total amount");
                    }
                }
            }
            for item in row_vec.iter() {
                let _ = output_sheet
                    .get_sheet_mut(1)
                    .get_cell_by_column_and_row_mut(col_num, row_num)
                    .set_value(item);
                col_num += 1;
            }
            row_num += 1;
        }
        let output_path = std::path::Path::new(config_params.output_file_path());
        umya_spreadsheet::writer::xlsx::write(&output_sheet, output_path)
            .expect("Cannot write to file");
    }

    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    log_info!(log, "{}", health_report.display());
    health_report.gen_health_rpt(&config_params.output_file_path());
}
