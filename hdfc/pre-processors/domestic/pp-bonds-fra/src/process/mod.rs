use self::io::*;
use configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod io;
use calamine::{open_workbook_auto, Reader};
use health_report::HealthReport;
use rbdate::DateParser;
use std::collections::HashMap;
use std::fs::File;
use std::io::*;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let op_path = format!("{}.txt", config_params.output_file_path());
    let mut op_writer = get_writer(&op_path);
    let op_path_nd = format!("{}_nd.txt", config_params.output_file_path());
    let mut op_writer_nd = get_writer(&op_path_nd);
    let mut tot_acc_encntrd = 0;
    let mut acc_pro_suc = 0;
    let tot_amt = 0.0;
    let mut ref_vec: Vec<String> = Vec::new();
    let mut ref_excel =
        open_workbook_auto(config_params.ref_file_path()).expect("Unable to open Reference File.");
    if let Some(Ok(ref_reader)) = ref_excel.worksheet_range(config_params.sheet_name()) {
        for row in ref_reader.rows().skip(1) {
            ref_vec.push(row[6].to_string());
        }
    }
    let input_file =
        File::open(&config_params.input_file_path()).expect("Unable to open `Input File`.");
    let input_reader = BufReader::new(input_file);
    for (index, line) in input_reader.lines().enumerate().skip(1) {
        let line = line.expect("Could Not Read Line").to_string();
        let input_fields: Vec<&str> = line.split('|').collect();
        tot_acc_encntrd += 1;
        let data_format = format!(
                "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
                input_fields[0], input_fields[1], input_fields[2], input_fields[3], input_fields[4], input_fields[5],input_fields[6],input_fields[7],input_fields[8],input_fields[11],input_fields[12],input_fields[13],input_fields[14],input_fields[15],input_fields[16],input_fields[17],input_fields[18],input_fields[19],input_fields[20],input_fields[21],input_fields[22],input_fields[23],input_fields[24],input_fields[25],input_fields[26],input_fields[27],input_fields[28],input_fields[29],input_fields[35],input_fields[36],input_fields[37],input_fields[38],input_fields[39],input_fields[40],input_fields[41],input_fields[42],input_fields[43],input_fields[44],input_fields[45],input_fields[46],input_fields[47],input_fields[48],input_fields[49],input_fields[50]
            );
        if ref_vec.contains(&input_fields[6].to_string()) {
            writeln!(op_writer_nd, "{}", data_format);
        } else {
            writeln!(op_writer, "{}", data_format);
        }
        acc_pro_suc += 1;
    }

    let health_report = HealthReport::new(
        tot_acc_encntrd,
        acc_pro_suc,
        tot_acc_encntrd - acc_pro_suc,
        tot_amt,
        tot_amt,
        0,
    );
    health_report.gen_health_rpt(&config_params.output_file_path());
}
