use crate::aggregator::implementation::{write_output, OutputType};
use crate::aggregator::structs::{AggrKey, AggrValues};
use aggregator::config::read_config_file;
use aggregator::env::current_dir;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::buf_file_wrtr;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::env;
use std::io::prelude::*;

mod config;
mod implementation;
mod structs;

pub fn aggregate(config_params: ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let mut tot_rec = 0;
    let mut succ_rec = 0;

    let date_folder = config_params.as_on_date().format("%d%m%Y").to_string();

    let config_file = read_config_file(config_params.config_file_path());

    let output_file = config_file
        .output_file_path
        .replace("{ddmmyyyy}", &date_folder);
    let mut writer = match buf_file_wrtr(&output_file, None) {
        Ok(val) => val,
        Err(error) => {
            panic!(
                "Could not create Output file: `{}` on location `{}` : {:?}.",
                &output_file,
                env::current_exe()
                    .unwrap_or_else(|error| {
                        panic!("Unable to find current directory path: {}", error);
                    })
                    .display(),
                error
            );
        }
    };

    if config_file.dim_item_id_position > 60 || config_file.rlg_item_id_position > 60 {
        panic!("Dim Item ID or Rlg Item ID position mentioned is greater than 60, expected position is less than or equal to 60.")
    }

    let mut aggr_map: HashMap<AggrKey, AggrValues> = HashMap::new();

    let stamper_files = config_file.stamper_files.stamper_file_paths;
    for file in stamper_files {
        let input_stamper_file_path = file.replace("{ddmmyyyy}", &date_folder);
        let input_stamper_file = match new_buf_rdr(&input_stamper_file_path) {
            Ok(file) => file,
            Err(error) => panic!(
                "Could not found stamper file: `{}` on location `{}` : {}.",
                input_stamper_file_path,
                current_dir()
                    .unwrap_or_else(|error| {
                        panic!("Error while getting current directory path: {}", error);
                    })
                    .display(),
                error
            ),
        };

        for (line_num, line) in input_stamper_file.lines().enumerate() {
            let line = match line {
                Ok(line) => line,
                Err(error) => {
                    log_error!(
                        log,
                        "Unable to read file `{}` at line number: `{}` : {}",
                        input_stamper_file_path,
                        line_num + 1,
                        error
                    );
                    continue;
                }
            };

            tot_rec += 1;
            let fields: Vec<&str> = line.split('|').collect();

            if fields.len() < 60 {
                log_debug!(
                    log,
                    "Insufficient fields detected for Acid {}: Expected 60 fields, found {} at line num {}.",
                    fields.get(1).unwrap_or(&""),
                    fields.len(),
                    line_num + 1
                );
                continue;
            }

            let aorl = fields[37].to_string();

            if aorl.is_empty() {
                if config_file.is_aorl_null == "N" {
                    let err_msg = format!("Is AorL flag is N and found aorl value as null for AccID:{} in file: {}, hence skipping record",
                    &fields[1],
                    &input_stamper_file_path);
                    panic!("{}", err_msg);
                } else {
                    log_error!(
                        log,
                        "Is AorL flag is Y and found aorl value as null for AccID:{} in file: {}, hence skipping record",
                        fields[1],
                        input_stamper_file_path
                    );
                    continue;
                }
            }
            let (dim1, dim2, dim3, dim4) = (fields[38], fields[39], fields[40], fields[41]);

            let (bal_amt_hcy, int_rate, int_amt, ftp_rate, ftp_amt) = (
                fields[6].parse().unwrap_or(0.0),
                fields[7].parse().unwrap_or(0.0),
                fields[9].parse().unwrap_or(0.0),
                fields[33].parse().unwrap_or(0.0),
                fields[36].parse().unwrap_or(0.0),
            );

            let sum_prod_int_rt_bal = bal_amt_hcy * int_rate;
            let sum_prod_ftp_rt_bal = bal_amt_hcy * ftp_rate;

            let dim_item_id = fields[config_file.dim_item_id_position - 1].to_string();
            let rlg_item_id = fields[config_file.rlg_item_id_position - 1].to_string();

            if dim_item_id.is_empty() || rlg_item_id.is_empty() {
                log_debug!(
                    log,
                    "Skipping record as found dimId as null for AccID:{} in file: {}, dimsIds are as follows dim_item_id: {}, rlg_item_id: {}",
                    fields[1],
                    input_stamper_file_path,
                    dim_item_id,
                    rlg_item_id
                );
                continue;
            }


            let key = AggrKey {
                dim_item_id,
                rlg_item_id,
                aorl,
            };

            let values = aggr_map.entry(key).or_insert_with(AggrValues::new);
            values.average_balance += bal_amt_hcy;
            values.ftp_amount += ftp_amt;
            values.interest_amount += int_amt;
            values.sum_prod_ftp_rt_bal_amt += sum_prod_ftp_rt_bal;
            values.sum_prod_int_rt_bal_amt += sum_prod_int_rt_bal;

            succ_rec += 1;
        }
    }

    if config_file.is_weighted_rate_required {
        write_output(
            &aggr_map,
            config_params.as_on_date(),
            config_file.dimid,
            config_file.rlgid,
            &mut writer,
            OutputType::Weighted,
        )
    } else {
        write_output(
            &aggr_map,
            config_params.as_on_date(),
            config_file.dimid,
            config_file.rlgid,
            &mut writer,
            OutputType::Average,
        )
    }
    let health_report = HealthReport::new(tot_rec, succ_rec, tot_rec - succ_rec, 0.0, 0.0, 0);
    log_info!(log, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&output_file);
}
