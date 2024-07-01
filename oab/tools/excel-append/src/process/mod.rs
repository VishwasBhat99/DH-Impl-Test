use calamine::{open_workbook_auto, Reader};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use slog::Logger;
use std::convert::TryInto;
use xlsxwriter::*;

pub fn excel_append(
    config_params: ConfigurationParameters,
    _logger: &Logger,
    _diag_logger: &Logger,
) {
    let mut acc_enc: i64 = 0;
    let mut acc_succ: i64 = 0;
    let input_file1 = config_params.input_file1();
    let input1_sheet = config_params.sheet_name1();
    //Splitting input file path to get path and filename.
    let input_path_split: Vec<&str> = input_file1.split('.').collect();
    let mut output_filename = input_path_split
        .first()
        .expect("Could not determine name of input file 1.")
        .to_string();
    //Remove '_1' from the input filename.
    output_filename.pop();
    output_filename.pop();
    let full_output_path = format!("{}.xlsx", output_filename);
    let mut input_file1 =
        open_workbook_auto(&input_file1).expect("Unable to open the input xlsx file 1.");
    let workbook = Workbook::new(&full_output_path);
    let mut sheet = workbook
        .add_worksheet(Some(input1_sheet))
        .expect("Could not create sheet name in output excel.");
    let mut row_index = 0;
    if let Some(Ok(reader)) = input_file1.worksheet_range(input1_sheet) {
        //First file content written as it is to output.
        for (x, row) in reader.rows().enumerate() {
            acc_enc += 1;
            let mut y = 0;
            while y < row.len() {
                sheet
                    .write_string(
                        x.try_into().unwrap_or(0),
                        y.try_into().unwrap_or(0),
                        &row[y].to_string(),
                        None,
                    )
                    .expect("Could not write to output sheet contents of file 1.");
                y += 1;
            }
            acc_succ += 1;
        }
        //Get the row index of the next line in output file after contents of first file are written.
        row_index = reader.rows().len();
    }
    let mut input_file2 = open_workbook_auto(config_params.input_file2())
        .expect("Unable to open the input xlsx file 2.");
    if let Some(Ok(reader)) = input_file2.worksheet_range(config_params.sheet_name2()) {
        for (_x, row) in reader.rows().enumerate().skip(1) {
            acc_enc += 1;
            let mut y = 0;
            let mut row_val: Vec<String> = Vec::new();
            while y < row.len() {
                let data = row[y as usize].to_string().clone();
                row_val.push(data);
                y += 1;
            }
            let pos_val_vec: Vec<String> = vec![
                row[0_usize].to_string().clone(),
                row[1_usize].to_string().clone(),
                "D".to_string().clone(),
                row[0_usize].to_string().clone(),
                row[1_usize].to_string().clone(),
                row[2_usize].to_string().clone(),
                "".to_string().clone(),
                "".to_string().clone(),
                "Net".to_string().clone(),
            ];
            for i in 1..3 {
                //To print values twice.
                for (pos, mut val) in pos_val_vec.clone().into_iter().enumerate() {
                    if pos == 2 && i == 1 {
                        val = "C".to_string();
                    }
                    sheet
                        .write_string(
                            row_index.try_into().unwrap_or(0),
                            pos.try_into().unwrap_or(0),
                            &val,
                            None,
                        )
                        .expect("Could not write to output sheet contents of file 2.");
                }
                row_index += 1;
            }
            acc_succ += 1;
        }
    }
    workbook.close().expect("Failed to close workbook.");
    let health_report = HealthReport::new(acc_enc, acc_succ, acc_enc - acc_succ, 0.0, 0.0, 0);
    health_report.gen_health_rpt(&full_output_path);
}
