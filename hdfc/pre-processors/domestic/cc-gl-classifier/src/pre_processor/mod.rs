use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use calamine::{open_workbook_auto, Reader};
use health_report::HealthReport;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::io::prelude::*;
use std::io::BufWriter;

use std::time::SystemTime;

pub fn process(config_params: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let output_file = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{} Cannot read output file path", error),
    };
    let start_timer = SystemTime::now();

    let mut tot_rec = 0;
    let mut succ_rec = 0;
    let mut tot_amt_in_input = 0.0;
    let mut tot_amt_in_output = 0.0;
    let mut output_writer = BufWriter::new(output_file);
    //Mapping master File reading started
    log_debug!(log, "Mapping master File reading started");
    let mut total_amt_lcy: f64 = 0.0;
    let mut master_excel = open_workbook_auto(config_params.master_file())
        .expect("Unable to open Mapping Master File.");

    if let Some(Ok(reader)) = master_excel.worksheet_range(config_params.master_file_sheet_name()) {
        for row in reader.rows().skip(0) {
            if config_params
                .source_names()
                .contains(&row[0].to_string().trim().to_string())
            {
                total_amt_lcy += row[2].to_string().parse().unwrap_or(0.0);
            }
        }
    }
    log_debug!(log, "Master File Reading Completed");
    log_debug!(log, "Input File Reading Started");
    let input_file = match new_buf_rdr(config_params.input_file()) {
        Ok(file) => file,
        Err(_error) => panic!(
            "Could not found input_file: `{}`",
            config_params.input_file(),
        ),
    };
    let mut input_bucket_sum: Vec<f64> = vec![0.0; 29];
    //input file reading started
    for (line_num, lines) in input_file.lines().enumerate().skip(2) {
        let input_line = match lines {
            Ok(input_line) => input_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.input_file(),
                line_num + 1,
                error
            ),
        };
        tot_rec += 1;
        let input_fields: Vec<String> = input_line
            .split('~')
            .map(|s| s.trim().to_string())
            .collect();
        if input_fields.len()<64 {
            log_debug!(log,"row no: {} skipped becasue no. of col is less then 64",line_num);
            continue;
        }
        succ_rec += 1;
        let mut bucket_no_for_sum = 0;
        let mut bucket_no = 5;
        while bucket_no < 63 {
            tot_amt_in_input += input_bucket_sum[bucket_no_for_sum];
            input_bucket_sum[bucket_no_for_sum] = input_bucket_sum[bucket_no_for_sum]
                + input_fields[bucket_no].parse::<f64>().unwrap_or(0.0);
            bucket_no_for_sum += 1;
            bucket_no += 2;
        }
    }
    log_debug!(log, "Input File Reading Completed");
    let mut distribution_vec: Vec<f64> = Vec::new();
    let total_input_sum = input_bucket_sum[28];
    for val in input_bucket_sum.clone() {
        distribution_vec.push(total_amt_lcy * (val / total_input_sum));
    }
    let mut subst_amt_vec: Vec<f64> = Vec::new();
    let mut index = 0;
    for val in input_bucket_sum {
        tot_amt_in_output += val - distribution_vec[index];
        subst_amt_vec.push(val - distribution_vec[index]);
        index += 1;
    }
   
    let b1_llg_amt:f64 = subst_amt_vec[0..15].iter().sum();
    let b2_llg_amt:f64 = subst_amt_vec[15..20].iter().sum();
    let b3_llg_amt:f64= subst_amt_vec[20..28].iter().sum();    
    let as_on_date = config_params.as_on_date().format("%d-%m-%Y").to_string();
    writeln!(
        output_writer,
        "{}|{}|{}|{}|{}|{}",
        as_on_date,
        config_params.country_name(),
        config_params.ccy(),
        config_params.rf_llg(),
        total_amt_lcy,
        total_amt_lcy
    )
    .expect("rf_llg output line can not be written");
    writeln!(
        output_writer,
        "{}|{}|{}|{}|{}|{}",
        as_on_date,
        config_params.country_name(),
        config_params.ccy(),
        config_params.b1_llg(),
        b1_llg_amt,
        b1_llg_amt
    )
    .expect("b1_llg output line can not be written");
    writeln!(
        output_writer,
        "{}|{}|{}|{}|{}|{}",
        as_on_date,
        config_params.country_name(),
        config_params.ccy(),
        config_params.b2_llg(),
        b2_llg_amt,
        b2_llg_amt
    )
    .expect("b2_llg output line can not be written");
    writeln!(
        output_writer,
        "{}|{}|{}|{}|{}|{}",
        as_on_date,
        config_params.country_name(),
        config_params.ccy(),
        config_params.b3_llg(),
        b3_llg_amt,
        b3_llg_amt
    )
    .expect("b3_llg output line can not be written");
    let end_timer = SystemTime::now();
    let duration = end_timer
        .duration_since(start_timer)
        .expect("Could not calculate total process duration.");
    log_debug!(log, "Total Duration for process the data: {:?}.", duration);
    info!(
        diag_log,
        "Total Duration for process the data: {:?}.", duration
    );
    let health_report = HealthReport::new(
        tot_rec,
        succ_rec,
        tot_rec - succ_rec,
        tot_amt_in_input,
        tot_amt_in_output,
        0,
    );
    health_report.gen_health_rpt(&config_params.output_file_path());
}
