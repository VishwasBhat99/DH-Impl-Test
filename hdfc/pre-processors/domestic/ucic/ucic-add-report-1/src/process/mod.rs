use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use health_report::HealthReport;
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::time::SystemTime;
use std::fs;

pub fn process(config_params: &ConfigurationParameters, _log: &Logger, diag_log: &Logger) {
    let start_process_timer = SystemTime::now();
    let mut tot_acc_encntrd = 0;
    let mut skip_rec_count = 0;

    let mut cust_bal_map: HashMap<String, f64> = HashMap::new();
    let mut edw_data_map: HashMap<String, (String,String)> = HashMap::new();
    let mut ucic_data_map: HashMap<String, Vec<String>> = HashMap::new();

    //Output file to write output:
    let output_file =
        File::create(config_params.output_file_path()).expect("Unable to Create Output File Path");
    let mut output_writer = BufWriter::new(output_file);
    let out_error = format!(
        "Could not write output in file {}",
        config_params.output_file_path
    );

    //Read cust_bal file and store cust_id -> bal map:
    let cust_bal_file = match File::open(config_params.customer_bal_file()) {
        Ok(io_config_file) => io_config_file,
        Err(_err) => {
            panic!(
                "Could not open Customer_balance file {}",
                config_params.customer_bal_file()
            );
        }
    };
    let cust_bal_reader = BufReader::new(cust_bal_file);
    for (line_no, line) in cust_bal_reader.lines().enumerate() {
        let data = match line {
            Ok(cust_bal_info) => cust_bal_info,
            Err(error) => {
                log_error!(
                    _log,
                    "Cannot read line {} from Customer Balance file: {:?}",
                    line_no + 1,
                    error
                );
                continue;
            }
        };
        let fields: Vec<&str> = data
            .split(config_params.customer_bal_file_delimiter())
            .collect();
        cust_bal_map.insert(
            fields[0].to_string(),
            fields[1].to_string().parse::<f64>().unwrap_or(0.0),
        );
    }

    //Read edw_master file and store values in map:
    let edw_master_file = match File::open(config_params.edw_master_file()) {
        Ok(io_config_file) => io_config_file,
        Err(_err) => {
            panic!(
                "Could not open Customer_balance file {}",
                config_params.edw_master_file()
            );
        }
    };
    let edw_file = fs::read_to_string(config_params.edw_master_file()).expect("Unable to read edw_master_file");
    let last_row_num = edw_file.lines().count()-1;
    drop(edw_file);
    let edw_file_reader = BufReader::new(edw_master_file);
    for (line_no, line) in edw_file_reader.lines().enumerate().skip(1) {
        if line_no == last_row_num {
            continue;
        }
        let data = match line {
            Ok(edw_info) => edw_info,
            Err(error) => {
                log_error!(
                    _log,
                    "Cannot read line {} from UCIC BIU Master file: {:?}",
                    line_no + 1,
                    error
                );
                continue;
            }
        };
        let fields: Vec<&str> = data.split(config_params.edw_file_delimiter()).collect();
        edw_data_map.insert(
            fields[0].to_string(),
            (fields[6].to_string(),format!{
                "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
                fields[6].to_string(),
                fields[1].to_string(),
                fields[2].to_string(),
                fields[3].to_string(),
                fields[4].to_string(),
                fields[5].to_string(),
                fields[7].to_string(),
                fields[8].to_string(),
                fields[9].to_string(),
                fields[10].to_string(),
            }),
        );
    }

    //read ucic_biu file and store data:
    let ucic_biu_file = match File::open(config_params.ucic_biu_file()) {
        Ok(io_config_file) => io_config_file,
        Err(_err) => {
            panic!(
                "Could not open Customer_balance file {}",
                config_params.ucic_biu_file()
            );
        }
    };
    let ucic_biu_reader = BufReader::new(ucic_biu_file);
    for (line_no, line) in ucic_biu_reader.lines().enumerate().skip(1) {
        let data = match line {
            Ok(ucic_biu_info) => ucic_biu_info,
            Err(error) => {
                log_error!(
                    _log,
                    "Cannot read line {} from UCIC BIU master file: {:?}",
                    line_no + 1,
                    error
                );
                continue;
            }
        };
        let fields: Vec<&str> = data.split(config_params.ucic_file_delimiter()).collect();
        ucic_data_map.insert(
            fields[0].to_string(),
            vec![
                fields[3].to_string(),
                fields[5].to_string()
            ],
        );
    }

    //Read input, lookup in master files and write output:
    let input_file = match File::open(config_params.input_file_path()) {
        Ok(io_config_file) => io_config_file,
        Err(_err) => {
            panic!(
                "Could not open Input file {}",
                config_params.input_file_path()
            );
        }
    };

    let input_file_reader = BufReader::new(input_file);
    for (line_no, line) in input_file_reader.lines().enumerate() {
        tot_acc_encntrd += 1;
        let data: String = match line {
            Ok(acc_info) => acc_info,
            Err(error) => {
                skip_rec_count += 1;
                log_error!(
                    _log,
                    "Cannot read line {} from Input file: {:?}",
                    line_no + 1,
                    error
                );
                continue;
            }
        };

        let fields: Vec<&str> = data.split(config_params.input_file_delimiter()).collect();
        let cust_id = fields[1].to_string();

        let default_ucic_vec = vec![
            "NA".to_string(),
            "NA".to_string(),
        ];

        let ucic_data = match ucic_data_map.get(&cust_id) {
            Some(val) => val,
            None => &default_ucic_vec,
        };
        let default_val = "NA".to_string();
        let default_date = "01-01-1990".to_string();

        if let Some((cod_cust_id,edw_record)) = edw_data_map.get(&cust_id) {
            let balance = match cust_bal_map.get(cod_cust_id) {
                Some(val) => *val,
                None => 0.0,
            };
            let op_str = format! {
                "{}|{}|{}|{}|{}",
                    data,
                    edw_record,
                    ucic_data[0],
                    ucic_data[1],
                    balance,
            };
            writeln!(output_writer, "{}", op_str).expect(&out_error);
        } else {
            let op_str = format!(
                "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
                data,
                default_val,
                default_val,
                default_val,
                default_val,
                default_val,
                default_val,
                default_val,
                default_val,
                default_date,
                default_date,
                ucic_data[0],
                ucic_data[1],
                0.0,
            );
            writeln!(output_writer, "{}", op_str).expect(&out_error);
        }
    }

    // generate health check
    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - skip_rec_count,
        skip_rec_count,
        0.0,
        0.0,
        0,
    );
    health_report.gen_health_rpt(&config_params.output_file_path);
    let end_process_timer = SystemTime::now();
    let duration = end_process_timer
        .duration_since(start_process_timer)
        .expect("Could not calculate total duration for the process.");
    debug!(
        diag_log,
        "Total Duration for Reading and Writing UCIC Report 1: {:?}.", duration
    );
}
