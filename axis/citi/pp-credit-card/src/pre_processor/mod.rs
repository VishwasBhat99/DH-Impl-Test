use self::format::*;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::io::BufWriter;
use std::time::SystemTime;
mod format;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let output_file = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{} Cannot read output file path", error),
    };

    let mut tot_input_acc_encntrd: i64 = 0;
    let mut tot_succ: i64 = 0;
    let mut tot_amt: f64 = 0.0;
    let mut writer = BufWriter::new(output_file);
    let npa_data_file = match new_buf_rdr(config_param.npa_data_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found npa data file: `{}` on location `{}` : {}.",
            config_param.npa_data_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let start_datafile_derive_timer = SystemTime::now();
    let mut npa_data_map: HashMap<String, (String, f64)> = HashMap::new();
    for (line_num, lines) in npa_data_file.lines().enumerate() {
        let npa_data_line = match lines {
            Ok(npa_data_line) => npa_data_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.npa_data_file_path(),
                line_num + 1,
                error
            ),
        };

        let npa_data_fields = npa_data_line.split('|').collect::<Vec<&str>>();
        if npa_data_fields.len() >= 5 {
            //Store the LoanAccNo and NPA_Classification as key-value pairs.
            npa_data_map.insert(
                npa_data_fields[0].to_string(),
                (
                    npa_data_fields[2].to_string(),
                    npa_data_fields[4]
                        .to_string()
                        .trim()
                        .parse::<f64>()
                        .unwrap_or(0.0),
                ),
            );
        } else {
            log_error!(
                log,
                "Found line :{} at line number {} defective in NPA data file.",
                line_num,
                npa_data_line
            );
        }
    }
    let end_datafile_derive_timer = SystemTime::now();
    let duration = end_datafile_derive_timer
        .duration_since(start_datafile_derive_timer)
        .expect("Could not calculate NPA Data File derive process duration.");
    debug!(
        diag_log,
        "NPA Data File Derive Process Total Duration: {:?}.", duration
    );

    let npa_live_file = match new_buf_rdr(config_param.npa_live_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found npa live file: `{}` on location `{}` : {}.",
            config_param.npa_live_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let start_livefile_derive_timer = SystemTime::now();
    let mut npa_live_hashmap: HashMap<String, String> = HashMap::new();

    for (line_num, lines) in npa_live_file.lines().enumerate() {
        let npa_live_line = match lines {
            Ok(npa_live_line) => npa_live_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.npa_live_file_path(),
                line_num + 1,
                error
            ),
        };

        let npa_live_fields = npa_live_line.split('|').collect::<Vec<&str>>();
        if npa_live_fields.len() >= 7 {
            //Store the Finacle_CustID and cust_hlth_code as key-value pairs.
            npa_live_hashmap.insert(
                npa_live_fields[0].to_string(),
                npa_live_fields[2].to_string(),
            );
        } else {
            log_error!(
                log,
                "Found line :{} at line number {} defective in NPA Live file.",
                line_num,
                npa_live_line
            );
        }
    }
    let end_livefile_derive_timer = SystemTime::now();
    let duration = end_livefile_derive_timer
        .duration_since(start_livefile_derive_timer)
        .expect("Could not calculate NPA Live File derive process duration.");
    debug!(
        diag_log,
        "NPA Live File Derive Process Total Duration: {:?}.", duration
    );

    let npa_config_file = match new_buf_rdr(config_param.npa_config_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found npa config file: `{}` on location `{}` : {}.",
            config_param.npa_config_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let start_configfile_derive_timer = SystemTime::now();

    let mut npa_config_hashmap: HashMap<String, String> = HashMap::new();
    for (line_num, lines) in npa_config_file.lines().enumerate() {
        let npa_config_line = match lines {
            Ok(npa_config_line) => npa_config_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.npa_config_file_path(),
                line_num + 1,
                error
            ),
        };

        let npa_config_fields = npa_config_line.split('|').collect::<Vec<&str>>();
        npa_config_hashmap.insert(
            npa_config_fields[0].to_string(),
            npa_config_fields[1].to_string(),
        );
    }
    let end_configfile_derive_timer = SystemTime::now();
    let duration = end_configfile_derive_timer
        .duration_since(start_configfile_derive_timer)
        .expect("Could not calculate NPA Config File derive process duration.");
    debug!(
        diag_log,
        "NPA Config File Derive Process Total Duration: {:?}.", duration
    );

    let input_file = match new_buf_rdr(config_param.input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input data file: `{}` on location `{}` : {}.",
            config_param.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_no, lines) in input_file.lines().enumerate() {
        let fields = match lines {
            Ok(data) => data,
            Err(error) => panic!(
                "Unable to read file `{}` at line no:{}: {}",
                config_param.input_file_path(),
                line_no + 1,
                error
            ),
        };
        let fields = fields.replace('\"', "");
        tot_input_acc_encntrd += 1;
        let input_account: Vec<&str> = fields.split('|').collect();
        let acc_no = input_account[1].trim().to_string();
        let cust_id = input_account[8].trim().to_string();
        let npa_classification = match npa_data_map.get(&acc_no) {
            Some(typ) => typ.to_owned().0,
            None => "0".to_string(),
        };
        let npa_amt = match npa_data_map.get(&acc_no) {
            Some(npa_amt) => npa_amt.1,
            None => input_account[3].to_string().parse().unwrap_or(0.0),
        };
        tot_amt += npa_amt;
        let cust_health_code = match npa_live_hashmap.get(&cust_id) {
            Some(code) => {
                if !code.is_empty() {
                    code.to_owned()
                } else {
                    "S".to_string()
                }
            }
            None => "0".to_string(),
        };

        let cust_npa_class = if cust_health_code == *"S" {
            "S".to_string()
        } else {
            npa_config_hashmap
                .get(&cust_health_code.to_owned())
                .unwrap_or(&"0".to_string())
                .to_owned()
        };

        let final_npa_class = if npa_classification == *"SS" {
            "S".to_string()
        } else if npa_classification != *"0" {
            npa_classification.to_owned()
        } else {
            cust_npa_class.to_string()
        };
        let op_line = get_op_line(
            input_account.to_owned(),
            npa_classification,
            npa_amt,
            cust_health_code,
            cust_npa_class,
            final_npa_class,
        );
        tot_succ += 1;
        match writer.write_all(op_line.as_bytes()) {
            Ok(val) => val,
            Err(error) => {
                panic!("Error writing processed data: {:?}", error);
            }
        }
    }
    let health_report = HealthReport::new(
        tot_input_acc_encntrd,
        tot_succ,
        tot_input_acc_encntrd - tot_succ,
        tot_amt,
        tot_amt,
        0,
    );
    health_report.gen_health_rpt(config_param.output_file_path());
}
