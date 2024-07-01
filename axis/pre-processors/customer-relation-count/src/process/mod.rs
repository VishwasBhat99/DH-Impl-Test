use self::io::*;
use configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod io;
mod structs;
use self::structs::{op_data_format, OutputData};
use health_report::HealthReport;
use macros;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let mut op_writer = get_writer(config_params.output_file_path());
    let tot_amt = 0.0;

    let input_file =
        File::open(config_params.input_file_path()).expect("Failed to open input file at path");

    let salary_input_reader = buff_reader(config_params.salary_pension_data());

    //SALARY PENSION DATA
    let mut pension_map: HashMap<String, Vec<String>> = HashMap::new();
    for (_index, line) in salary_input_reader.lines().enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.salary_pension_data(),
                _index + 1,
                error
            ),
        };

        let data: Vec<&str> = line.split('|').collect();
        if data.len() == 4{
            pension_map
            .entry(data[2].to_string())
            .and_modify(|pen| pen.push(data[3].to_string()))
            .or_insert(vec![data[3].to_string()]);
        }else {
            log_info!(logger,"Salary pension data does not contains required fields. Record: `{}`",line);
        }
        
    }

    //INPUT FILE
    let input_reader = BufReader::new(input_file);

    let mut op_str = String::new();
    op_str.push_str("AsOnDt|CustID|Country|AdvanceRelation|LiabilityRelation|SalaryRelation|ConstitutionCode|TotalAmt|InsuredAmt|UninsuredAmt|LCRCategory");
    writeln!(op_writer, "{}", op_str).unwrap_or_else(|error| {
        log_info!(logger, "Unable to write to the output file: `{}`", error);
    });
    for (_index, line) in input_reader.lines().enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.input_file_path(),
                _index + 1,
                error
            ),
        };
        let input_fields: Vec<&str> = line.split('|').collect();
        let default = vec!["NA".to_string()];
        let mut op_value = OutputData::new();

        if input_fields[16] == "3" || input_fields[16] == "1" {
            let mut liability_count = 0;

            op_value.as_on_dt = *config_params.as_on_date();
            op_value.cust_id = input_fields[0].to_string();
            op_value.country = config_params.country().to_string();
            op_value.advances_relation = input_fields[3].to_string();
            op_value.liability_relation = {
                (input_fields[4] == "Y") as i64
                    + (input_fields[6] == "Y") as i64
                    + (input_fields[7] == "Y") as i64
                    + (input_fields[8] == "Y") as i64
            };
            op_value.salary_relation = {
                let pen_data = pension_map.get(input_fields[0]).unwrap_or(&default);
                pen_data
                    .iter()
                    .filter(|data| *data == "SAL" || *data == "P")
                    .count() as i64
            };
            op_value.constitutio_code = input_fields[13].to_string();
            op_value.total_amount = check_nan(input_fields[40]);
            op_value.insured_amount = {
                check_nan(input_fields[51])
                    + check_nan(input_fields[55])
                    + check_nan(input_fields[59])
                    - check_nan(input_fields[67])
            };
            op_value.uninsured_amount = {
                check_nan(input_fields[52])
                    + check_nan(input_fields[56])
                    + check_nan(input_fields[60])
                    - check_nan(input_fields[68])
            };
            op_value.lcr_category = {
                if input_fields[16] == "1" {
                    "RETAIL".to_string()
                } else {
                    "SBC".to_string()
                }
            };

            op_str = op_data_format(op_value);
            writeln!(op_writer, "{}", op_str).unwrap_or_else(|error| {
                log_info!(logger, "Unable to write to the output file: `{}`", error);
            });
        } else if input_fields[16] == "4" || input_fields[16] == "5" {
            let mut liability_count = 0;

            op_value.as_on_dt = *config_params.as_on_date();
            op_value.cust_id = input_fields[0].to_string();
            op_value.country = config_params.country().to_string();
            op_value.advances_relation = input_fields[3].to_string();
            op_value.liability_relation = {
                (input_fields[4] == "Y") as i64
                    + (input_fields[6] == "Y") as i64
                    + (input_fields[7] == "Y") as i64
                    + (input_fields[8] == "Y") as i64
            };
            op_value.salary_relation = {
                let pen_data = pension_map.get(input_fields[0]).unwrap_or(&default);
                pen_data
                    .iter()
                    .filter(|data| *data == "SAL" || *data == "P")
                    .count() as i64
            };
            op_value.constitutio_code = input_fields[13].to_string();
            op_value.total_amount = check_nan(input_fields[19]) + check_nan(input_fields[23]);
            op_value.insured_amount = check_nan(input_fields[53]) + check_nan(input_fields[57]);
            op_value.uninsured_amount = check_nan(input_fields[54]) + check_nan(input_fields[58]);
            op_value.lcr_category = "OPDEP".to_string();
            op_str = structs::op_data_format(op_value);
            writeln!(op_writer, "{}", op_str).unwrap_or_else(|error| {
                log_info!(logger, "Unable to write to the output file: `{}`", error);
            });
        } else {
            log_debug!(logger, "Record does not contains 1,3,4,5 for LCRClassification, hence skipping record. Record contains `{}` for LCRClassification" , input_fields[16]);
            continue;
        }
    }

    let health_report = HealthReport::new(0, 0, 0, tot_amt, tot_amt, 0);
    health_report.gen_health_rpt(config_params.output_file_path());
}

pub fn check_nan(amt: &str) -> f64 {
    if amt.parse::<f64>().unwrap_or(0.0).is_nan() {
        0.0
    } else {
        amt.parse::<f64>().unwrap_or(0.0)
    }
}
