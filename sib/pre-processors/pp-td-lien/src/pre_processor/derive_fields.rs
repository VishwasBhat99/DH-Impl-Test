extern crate chrono;

use rbdate::{num_days_start_to_end, NaiveDate};

use slog::Logger;
use std::io::{self, *};
use std::{collections::HashMap, fs::File, io::BufRead};

use crate::configuration_parameters::ConfigurationParameters;

use super::output_account::{get_writer, OutputData};
use super::TdAccountStructValues;
use health_report::HealthReport;

pub fn derive_output(
    lien_file_paths_reader: io::BufReader<File>,
    acid_map: &HashMap<String, TdAccountStructValues>,
    config_params: &ConfigurationParameters,
    logger: &Logger,
) {
    let def_struct = TdAccountStructValues {
        acid: "".to_string(),
        currency: "".to_string(),
        clr_bal_amt: "0.0".parse::<f64>().unwrap_or(0.0),
        const_code: "".to_string(),
        gl_sub_head_code: "".to_string(),
    };
    let mut acc_enc = 0;
    let mut acc_proc = 0;
    let mut op_writer = get_writer(config_params.output_file_path());
    for (line_num, line) in lien_file_paths_reader.lines().enumerate() {
        let line = line
            .unwrap_or_else(|_| {
                panic!("Could Not Read Line from Lien File (Line-Num:{})", line_num)
            })
            .to_string();
        let input_fields: Vec<&str> = line.split('|').collect();

        acc_enc += 1;
        if acid_map.contains_key(input_fields[0]) {
            let acid_value = acid_map.get(input_fields[0]).unwrap_or(&def_struct);

            let maturity_date = NaiveDate::parse_from_str(input_fields[7], "%d-%m-%Y")
                .unwrap_or(config_params.as_on_date);

            let lien_start_date = NaiveDate::parse_from_str(input_fields[6], "%d-%m-%Y")
                .unwrap_or(config_params.as_on_date);

            let as_on_date = *config_params.as_on_date();
            let tenor = num_days_start_to_end(as_on_date, maturity_date);
            let lien_amount = input_fields[5].parse::<f64>().unwrap_or(0.0);

            let final_lien_amt = if lien_amount <= acid_value.clr_bal_amt {
                lien_amount
            } else {
                acid_value.clr_bal_amt
            };

            let op = OutputData {
                acid: input_fields[0].trim().to_string(),
                b2k_type: input_fields[1].trim().to_string(),
                b2k_id: input_fields[2].trim().to_string(),
                entity_cre_flag: input_fields[3].trim().to_string(),
                del_flag: input_fields[4].trim().to_string(),
                lien_amt: lien_amount.to_string(),
                lien_start_date: lien_start_date.format("%d-%m-%Y").to_string(),
                lien_expiry_date: maturity_date.format("%d-%m_%Y").to_string(),
                lien_reason_code: input_fields[8].trim().to_string(),
                sol_id: input_fields[9].trim().to_string(),
                currency: acid_value.currency.trim().to_string(),
                clr_bal_amt: acid_value.clr_bal_amt.to_string(),
                const_code: acid_value.const_code.trim().to_string(),
                maturity_date: maturity_date.format("%d-%m-%Y").to_string(),
                tenor: tenor.to_string(),
                gl_sub_head_code: acid_value.gl_sub_head_code.trim().to_string(),
                final_lien_amt: final_lien_amt.to_string(),
            };

            let result = writeln!(op_writer, "{}", op.format_with_separator());
            match result {
                Ok(_) => acc_proc += 1,
                Err(err) => {
                    panic!("Error in writing the output {:?}", err);
                }
            }
        } else {
            info!(
                logger,
                "Skipping Account: {:?} as lookup not found in TD Master Data", input_fields[0]
            );
        }
    }
    let health_report = HealthReport::new(acc_enc, acc_proc, acc_enc - acc_proc, 0.0, 0.0, 0);
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file_path());
}
