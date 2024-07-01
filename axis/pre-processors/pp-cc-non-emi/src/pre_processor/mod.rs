extern crate serde;
use self::format::*;
use configuration_parameters::ConfigurationParameters;
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
    let mut tot_data_acc_encntrd: i64 = 0;
    let mut tot_live_acc_encntrd: i64 = 0;
    let mut tot_config_acc_encntrd: i64 = 0;
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
    let mut npa_data_map: HashMap<String, String> = HashMap::new();
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

        tot_data_acc_encntrd += 1;
        let npa_data_fields = npa_data_line.split('|').collect::<Vec<&str>>();
        if npa_data_fields.len() >= 5 {
            //Store the LoanAccNo and NPA_Classification as key-value pairs.
            npa_data_map.insert(
                npa_data_fields[0].to_string(),
                npa_data_fields[2].to_string(),
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

        tot_live_acc_encntrd += 1;

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

        tot_config_acc_encntrd += 1;

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
    let date_parser = rbdate::DateParser::new("%d-%b-%y".to_string(), false);
    let as_on_date = config_param.as_on_date();
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
        tot_input_acc_encntrd += 1;
        let input_account: Vec<&str> = fields.split('|').collect();
        let acid = input_account[0].trim().to_string();
        let cust_id = input_account[1].trim().to_string();
        let overdue_date = date_parser
            .parse_opt(input_account[3])
            .unwrap_or(*as_on_date);
        let mut duedate = date_parser
            .parse_opt(input_account[7])
            .unwrap_or(*as_on_date);
        let npa_classification = match npa_data_map.get(&acid) {
            Some(npa_val) => npa_val.to_string(),
            None => '0'.to_string(),
        };
        let cust_hlth_code = match npa_live_hashmap.get(&cust_id) {
            Some(val) => val.to_string(),
            None => '0'.to_string(),
        };
        let cust_npa_class = match npa_config_hashmap.get(&cust_hlth_code) {
            Some(val) => val.to_string(),
            None => '0'.to_string(),
        };
        let final_npa_class = if npa_classification != '0'.to_string() {
            npa_classification.to_owned()
        } else if cust_npa_class != '0'.to_string() {
            cust_npa_class.to_owned()
        } else {
            '0'.to_string()
        };
        let description_type = if input_account[5].to_uppercase().contains("CORPORATE") {
            "C"
        } else {
            "R"
        };
        let mut closing_bal = input_account[9].parse::<f64>().unwrap_or(0.0);
        let overdue_amt = input_account[2].parse::<f64>().unwrap_or(0.0);
        let mut op_line = String::new();
        if overdue_amt != 0.0 {
            closing_bal -= overdue_amt;
            op_line = get_op_line(
                input_account.to_owned(),
                acid.to_owned(),
                cust_id.to_owned(),
                overdue_amt.to_owned(),
                overdue_date.to_owned(),
                duedate.to_owned(),
                closing_bal,
                npa_classification.to_owned(),
                cust_hlth_code.to_owned(),
                cust_npa_class.to_owned(),
                final_npa_class.to_owned(),
                description_type.to_string(),
            );
            closing_bal = overdue_amt;
            duedate = overdue_date;
            let op_string1 = get_op_line(
                input_account,
                acid,
                cust_id,
                overdue_amt,
                overdue_date,
                duedate,
                closing_bal,
                npa_classification,
                cust_hlth_code,
                cust_npa_class,
                final_npa_class,
                description_type.to_string(),
            );
            op_line.push_str(&op_string1);
        } else {
            op_line = get_op_line(
                input_account.to_owned(),
                acid.to_owned(),
                cust_id.to_owned(),
                overdue_amt.to_owned(),
                overdue_date.to_owned(),
                duedate.to_owned(),
                closing_bal,
                npa_classification.to_owned(),
                cust_hlth_code.to_owned(),
                cust_npa_class.to_owned(),
                final_npa_class.to_owned(),
                description_type.to_string(),
            );
        }
        match writer.write_all(op_line.as_bytes()) {
            Ok(val) => val,
            Err(error) => {
                panic!("Error writing processed data: {:?}", error);
            }
        }
    }

    println!(
        "Total Accounts present in input File: {}\n\
        Total Accounts present in NPA Data File: {}\n\
        Total Accounts present in NPA Live File: {}\n\
        Total Accounts present in NPA Config File: {}",
        tot_input_acc_encntrd, tot_data_acc_encntrd, tot_live_acc_encntrd, tot_config_acc_encntrd
    );
}
