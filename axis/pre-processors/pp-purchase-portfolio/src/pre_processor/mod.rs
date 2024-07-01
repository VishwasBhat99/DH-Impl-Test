extern crate serde;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::DateParser;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::io::BufWriter;
use std::time::SystemTime;
pub struct NpaData {
    pub classification: String,
    pub npa_amt: f64,
}

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let output_file = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{} Cannot read output file path", error),
    };

    let mut tot_input_acc_encntrd: i64 = 0;
    let mut tot_acc_skippd: i64 = 0;
    let mut tot_succ_rec: i64 = 0;
    let as_on_date = config_param.as_on_date();
    let mut writer = BufWriter::new(output_file);
    let date_parser = DateParser::new("%d-%b-%Y".to_string(), false);
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
    let mut npa_data_map: HashMap<String, NpaData> = HashMap::new();
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
                NpaData {
                    classification: npa_data_fields[2].to_string(),
                    npa_amt: npa_data_fields[4].parse::<f64>().unwrap_or(0.0),
                },
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
        tot_input_acc_encntrd += 1;
        let input_account: Vec<&str> = fields.split('|').collect();
        let principal_os = input_account[4].parse::<f64>().unwrap_or(0.0);
        if principal_os <= 0.0 || date_parser.parse(input_account[38]) < *as_on_date {
            //Skip the records.
            tot_acc_skippd += 1;
            continue;
        }
        let account_no = input_account[1].trim().to_string();
        let cust_id = input_account[2].trim().to_string();
        let bank_share = input_account[32].parse::<f64>().unwrap_or(0.0);
        let cmonth_emi_due = input_account[27].parse::<f64>().unwrap_or(0.0);
        let payment_type = input_account[16].trim().to_uppercase();
        let payment_frequency = match payment_type.as_str() {
            "MONTHLY" => 1,
            "QUARTERLY" => 3,
            "SEMI-ANNUALLY" => 6,
            "ANNUAL" => 12,
            _ => 1,
        };
        let mut derived_principal = principal_os * bank_share / 100.0;
        let npa_classification;
        let customer_od_bank_share = input_account[34].parse::<f64>().unwrap_or(0.0);
        match npa_data_map.get(&account_no) {
            Some(typ) => {
                npa_classification = typ.classification.to_owned();
                derived_principal = typ.npa_amt;
            }
            None => {
                npa_classification = "0".to_string();
                //Deduct overdue from derived principal.
                if customer_od_bank_share != 0.0 {
                    derived_principal -= customer_od_bank_share.abs();
                }
            }
        };
        let cust_hlth_code = match npa_live_hashmap.get(&cust_id) {
            Some(code) => code.to_owned(),
            None => "0".to_string(),
        };
        let cust_npa_class = match npa_config_hashmap.get(&cust_hlth_code) {
            Some(class) => class.to_owned(),
            None => "0".to_string(),
        };
        let final_npa_class = if npa_classification != "0".to_string() {
            npa_classification.to_owned()
        } else if cust_npa_class != "0".to_string() {
            cust_npa_class.to_owned()
        } else {
            "0".to_string()
        };

        let op_line = format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            input_account[0],
            account_no,
            cust_id,
            input_account[3],
            input_account[4].parse::<f64>().unwrap_or(0.0),
            date_parser.parse(input_account[5]).format("%d-%m-%Y"),
            date_parser.parse(input_account[6]).format("%d-%m-%Y"),
            date_parser.parse(input_account[7]).format("%d-%m-%Y"),
            input_account[8].parse::<f64>().unwrap_or(0.0),
            input_account[9].parse::<f64>().unwrap_or(0.0),
            input_account[10],
            input_account[11],
            input_account[12].parse::<f64>().unwrap_or(0.0),
            input_account[13],
            input_account[14],
            input_account[15].parse::<f64>().unwrap_or(0.0),
            payment_frequency,
            input_account[17],
            input_account[18],
            date_parser.parse(input_account[19]).format("%d-%m-%Y"),
            input_account[20],
            input_account[21],
            input_account[22],
            input_account[23],
            input_account[24],
            date_parser.parse(input_account[25]).format("%d-%m-%Y"),
            input_account[26].parse::<f64>().unwrap_or(0.0),
            cmonth_emi_due,
            input_account[28].parse::<f64>().unwrap_or(0.0),
            input_account[29].parse::<f64>().unwrap_or(0.0),
            input_account[30].parse::<f64>().unwrap_or(0.0),
            input_account[31],
            bank_share,
            input_account[33].parse::<f64>().unwrap_or(0.0),
            customer_od_bank_share,
            input_account[35].parse::<f64>().unwrap_or(0.0),
            input_account[36].parse::<f64>().unwrap_or(0.0),
            input_account[37].parse::<f64>().unwrap_or(0.0),
            date_parser.parse(input_account[38]).format("%d-%m-%Y"),
            input_account[39],
            date_parser.parse(input_account[40]).format("%d-%m-%Y"),
            date_parser.parse(input_account[41]).format("%d-%m-%Y"),
            input_account[42],
            derived_principal,
            bank_share * cmonth_emi_due /100.0,
            npa_classification,
            cust_hlth_code,
            cust_npa_class,
            final_npa_class,
            config_param.currency()
        );
        tot_succ_rec += 1;
        match writer.write_all(op_line.as_bytes()) {
            Ok(val) => val,
            Err(error) => {
                panic!("Error writing processed data: {:?}", error);
            }
        }
    }
    let health_report = HealthReport::new(
        tot_input_acc_encntrd,
        tot_succ_rec,
        tot_acc_skippd,
        0.0,
        0.0,
        0,
    );
    health_report.gen_health_rpt(config_param.output_file_path());
}
