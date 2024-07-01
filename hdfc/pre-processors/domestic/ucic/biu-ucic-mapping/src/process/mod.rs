use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use health_report::HealthReport;
use slog::Logger;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::time::SystemTime;
use structs::BiuRecord;
mod structs;

pub fn process(config_params: &ConfigurationParameters, _log: &Logger, diag_log: &Logger) {
    let start_process_timer = SystemTime::now();
    let mut tot_acc_encntrd = 0;
    let mut skip_rec_count = 0;

    // Read ucic_master.txt file and store records in a HashMap with cust_id as the key
    let ucic_file = match File::open(config_params.ucic_master_file()) {
        Ok(io_config_file) => io_config_file,
        Err(_err) => {
            panic!(
                "Could not open UCIC master file {}",
                config_params.ucic_master_file()
            );
        }
    };
    let ucic_reader = BufReader::new(ucic_file);
    let master_row = fs::read_to_string(config_params.ucic_master_file())
        .expect("Unable to read Ucic Master File");
    let last_master_row_num = master_row.lines().count() - 1;
    let mut ucic_map: HashMap<String, Vec<String>> = HashMap::new();
    for (line_no, line) in ucic_reader.lines().enumerate().skip(1) {
        let data = match line {
            Ok(ucic_info) => ucic_info,
            Err(error) => {
                log_error!(
                    _log,
                    "Cannot read line {} from Ucic Master file: {:?}",
                    line_no + 1,
                    error
                );
                continue;
            }
        };
        if line_no == last_master_row_num {
            break;
        }
        let fields: Vec<&str> = data.split(config_params.ucic_field_delimiter()).collect();
        ucic_map.insert(
            fields[0].to_string(),
            vec![
                fields[6].to_string(),
                fields[9].to_string(),
                fields[10].to_string(),
            ],
        );
    }

    //Read Customer_balance file, store balances against each cust_id:
    let customer_bal_file = match File::open(config_params.customer_bal_file()) {
        Ok(io_config_file) => io_config_file,
        Err(_err) => {
            panic!(
                "Could not open Customer Bal file {}",
                config_params.customer_bal_file()
            );
        }
    };
    let customer_bal_reader = BufReader::new(customer_bal_file);
    let master_row = fs::read_to_string(config_params.customer_bal_file())
        .expect("Unable to read Customer Balance File");
    let last_master_row_num = master_row.lines().count() - 1;
    let mut cust_bal_map: HashMap<String, f64> = HashMap::new();
    for (line_no, line) in customer_bal_reader.lines().enumerate().skip(1) {
        let data = match line {
            Ok(ucic_info) => ucic_info,
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
        if line_no == last_master_row_num && config_params.is_cust_bal_footer() {
            break;
        }
        let fields: Vec<&str> = data
            .split(config_params.customer_bal_file_delimiter())
            .collect();
        cust_bal_map.insert(
            fields[0].to_string(),
            fields[1].to_string().parse::<f64>().unwrap_or(0.0),
        );
    }

    // Read biu_detail.txt file, process records, and write output to biu_output.txt
    let biu_file = match File::open(&config_params.input_file_path) {
        Ok(inp_file) => inp_file,
        Err(_err) => {
            panic!(
                "Could not read input file {}",
                config_params.input_file_path
            );
        }
    };
    let biu_reader = BufReader::new(biu_file);
    let output_file =
        File::create(config_params.output_file_path()).expect("Unable to Create Output File Path");
    let mut output_writer = BufWriter::new(output_file);
    let out_error = format!(
        "Could not write output in file {}",
        config_params.output_file_path
    );
    for (line_no, line) in biu_reader.lines().enumerate() {
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
        if line_no == 0 {
            let header_str = format!(
                "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
                "UCIC",
                "T1",
                "T2",
                "T3",
                "T4",
                "Division",
                "Creation Date",
                "Last Modification Date",
                "Cust_ID",
                "Balance"
            );
            writeln!(output_writer, "{}", header_str).expect(&out_error);
            continue;
        }
        if let Some(biu_record) = BiuRecord::from_line(&data, config_params) {
            let c_id = biu_record.cust_id.clone();
            let balance = match cust_bal_map.get(&c_id) {
                Some(val) => *val,
                None => 0.0,
            };
            if let Some(ucic_record) = ucic_map.get(&biu_record.cust_id) {
                if !ucic_record[0].is_empty() && ucic_record[0].to_uppercase() != "NULL" {
                    let op_str = format!(
                        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
                        ucic_record[0],
                        biu_record.t1,
                        biu_record.t2,
                        biu_record.t3,
                        biu_record.t4,
                        biu_record.division,
                        ucic_record[1],
                        ucic_record[2],
                        biu_record.cust_id,
                        balance,
                    );
                    writeln!(output_writer, "{}", op_str).expect(&out_error);
                } else {
                    let op_str = format!(
                        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
                        biu_record.cust_id,
                        biu_record.t1,
                        biu_record.t2,
                        biu_record.t3,
                        biu_record.t4,
                        biu_record.division,
                        ucic_record[1],
                        ucic_record[2],
                        biu_record.cust_id,
                        balance
                    );
                    writeln!(output_writer, "{}", op_str).expect(&out_error);
                }
            } else {
                let op_str = format!(
                    "{}|{}|{}|{}|{}|{}|01-01-1990 00:00:00|01-01-1990 00:00:00|{}|{}",
                    biu_record.cust_id,
                    biu_record.t1,
                    biu_record.t2,
                    biu_record.t3,
                    biu_record.t4,
                    biu_record.division,
                    biu_record.cust_id,
                    balance
                );
                writeln!(output_writer, "{}", op_str).expect(&out_error);
            }
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
        "Total Duration for Reading and Writing UcicDatas: {:?}.", duration
    );
}
