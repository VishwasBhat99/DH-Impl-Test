extern crate serde;
use self::account_field_names::AccFieldNames;
use self::input_account::{NpaDataAccount, NpaLiveAccount};
use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use sdb_dyn_proto_rdr::reader;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::io::BufWriter;
use std::time::SystemTime;
mod account_field_names;
mod input_account;

pub fn process(config_param: ConfigurationParameters, _log: &Logger, diag_log: &Logger) {
    let output_file = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{} Cannot read output file path", error),
    };
    let start_derive_timer = SystemTime::now();
    let mut op_line: String = String::new();
    let mut tot_input_acc_encntrd: i64 = 0;
    let mut tot_data_acc_encntrd: i64 = 0;
    let mut tot_live_acc_encntrd: i64 = 0;
    let mut tot_config_acc_encntrd: i64 = 0;
    let mut writer = BufWriter::new(output_file);

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
    let cif_report_file = match new_buf_rdr(config_param.cif_report()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found CIF config file: `{}` on location `{}` : {}.",
            config_param.cif_report(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let input_reader = reader::Reader::new_at_path(
        config_param.input_metadata_file(),
        config_param.input_file_path(),
    );

    let start_datafile_derive_timer = SystemTime::now();
    let mut npa_data_hashmap: HashMap<String, NpaDataAccount> = HashMap::new();
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

        let mut npa_data_acc = NpaDataAccount::new();
        let npa_data_fields = npa_data_line.split('|').collect::<Vec<&str>>();
        if npa_data_fields.len() >= 5 {
            npa_data_acc.npa_class = npa_data_fields[2].to_string();
            npa_data_acc.seg_cd = npa_data_fields[3].to_string();
            npa_data_acc.amount = npa_data_fields[4].to_string();
            npa_data_hashmap.insert(npa_data_fields[0].to_string(), npa_data_acc);
        }
    }

    let keys = AccFieldNames::new_from_path(config_param.req_fields_file_path());

    let end_datafile_derive_timer = SystemTime::now();
    let duration = end_datafile_derive_timer
        .duration_since(start_datafile_derive_timer)
        .expect("Could not calculate NPA Data File derive process duration.");
    debug!(
        diag_log,
        "NPA Data File Derive Process Total Duration: {:?}.", duration
    );

    let start_livefile_derive_timer = SystemTime::now();
    let mut npa_live_hashmap: HashMap<String, NpaLiveAccount> = HashMap::new();
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
        let mut npa_live_acc = NpaLiveAccount::new();
        if npa_live_fields.len() >= 7 {
            npa_live_acc.finnacle_cust_id = npa_live_fields[0].to_string();
            npa_live_acc.seg_class = npa_live_fields[1].to_string();
            npa_live_acc.cust_hlth_cd = npa_live_fields[2].to_string();
            npa_live_acc.cust_const = npa_live_fields[3].to_string();
            npa_live_acc.npa_date = NaiveDate::parse_from_str(npa_live_fields[4], "%d/%m/%Y")
                .unwrap_or_else(|_| NaiveDate::from_ymd(1970, 1, 1))
                .format("%d-%m-%Y")
                .to_string();

            npa_live_acc.v_ucif_code = npa_live_fields[5].to_string();
            npa_live_acc.reporting_date = npa_live_fields[6].to_string();
            npa_live_hashmap.insert(npa_live_acc.finnacle_cust_id.to_owned(), npa_live_acc);
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

    //CIF REPORT
    //AgreementNo|CustomerID|CIFNo
    let mut cif_hashmap: HashMap<String, String> = HashMap::new();
    for (line_num, lines) in cif_report_file.lines().enumerate() {
        let cif_line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.cif_report(),
                line_num + 1,
                error
            ),
        };

        tot_config_acc_encntrd += 1;

        let cif_fields = cif_line.split('|').collect::<Vec<&str>>();
        cif_hashmap.insert(cif_fields[0].to_string(), cif_fields[2].to_string());
    }

    let end_configfile_derive_timer = SystemTime::now();
    let duration = end_configfile_derive_timer
        .duration_since(start_configfile_derive_timer)
        .expect("Could not calculate NPA Config File derive process duration.");
    debug!(
        diag_log,
        "NPA Config File Derive Process Total Duration: {:?}.", duration
    );

    let mut npa_class;
    let mut seg_cd;
    let mut amount;
    let mut seg_class;
    let mut cust_hlth_cd;
    let mut cust_const;
    let mut npa_date;
    let mut v_ucif_cd;
    let mut npa_report_dt;
    let mut cust_npa_class;
    let mut final_npa_class;

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
        //Get the field names from input file based on metadata and required fields file.
        let mut input_pos = match input_reader.get_field_pos(&keys.acid) {
            Some(val) => val,
            None => panic!("Cannot get the field position for the field given in req fields file."),
        };
        let acid = input_account[input_pos - 1];
        input_pos = match input_reader.get_field_pos(&keys.foracid) {
            Some(val) => val,
            None => panic!("Cannot get the field position for the field given in req fields file."),
        };
        let foracid = input_account[input_pos - 1];
        input_pos = match input_reader.get_field_pos(&keys.cust_id) {
            Some(val) => val,
            None => panic!("Cannot get the field position for the field given in req fields file."),
        };
        let cust_name = input_account[input_pos - 1];
        input_pos = match input_reader.get_field_pos(&keys.schm_code) {
            Some(val) => val,
            None => panic!("Cannot get the field position for the field given in req fields file."),
        };
        let schm_code = input_account[input_pos - 1];
        input_pos = match input_reader.get_field_pos(&keys.schm_type) {
            Some(val) => val,
            None => panic!("Cannot get the field position for the field given in req fields file."),
        };
        let schm_type = input_account[input_pos - 1];
        input_pos = match input_reader.get_field_pos(&keys.acct_crncy_code) {
            Some(val) => val,
            None => panic!("Cannot get the field position for the field given in req fields file."),
        };
        let acct_crncy_code = input_account[input_pos - 1];
        input_pos = match input_reader.get_field_pos(&keys.acct_open_date) {
            Some(val) => val,
            None => panic!("Cannot get the field position for the field given in req fields file."),
        };
        let acct_open_date = input_account[input_pos - 1];
        input_pos = match input_reader.get_field_pos(&keys.out_bal_amt_con) {
            Some(val) => val,
            None => panic!("Cannot get the field position for the field given in req fields file."),
        };
        let out_bal_amt_con = input_account[input_pos - 1];
        input_pos = match input_reader.get_field_pos(&keys.nfs) {
            Some(val) => val,
            None => panic!("Cannot get the field position for the field given in req fields file."),
        };
        let nfs = input_account[input_pos - 1];

        if npa_data_hashmap.contains_key(&foracid.to_string()) {
            let npa_data_acc = npa_data_hashmap
                .get(&foracid.to_string())
                .expect("Cannot get NPA data for a foracid.");
            npa_class = npa_data_acc.npa_class.to_owned();
            seg_cd = npa_data_acc.seg_cd.to_owned();
            amount = npa_data_acc.amount.to_owned();
        } else {
            npa_class = "0".to_string();
            seg_cd = "0".to_string();
            amount = "0".to_string();
        }
        //Derive custid from CIF file.
        let cif_no = cif_hashmap
            .get(acid)
            .unwrap_or(&"NA".to_string())
            .to_owned();
        if npa_live_hashmap.contains_key(&cif_no.to_string()) {
            let npa_live_acc = npa_live_hashmap
                .get(&cif_no.to_string())
                .expect("Could not fetch npa data for an input account.");
            seg_class = npa_live_acc.seg_class.to_owned();
            cust_hlth_cd = match npa_live_acc.cust_hlth_cd.to_owned().as_str() {
                "" => "S".to_string(),
                _ => npa_live_acc.cust_hlth_cd.to_owned(),
            };

            cust_const = npa_live_acc.cust_const.to_owned();
            npa_date = npa_live_acc.npa_date.to_owned();
            v_ucif_cd = npa_live_acc.v_ucif_code.to_owned();
            npa_report_dt = npa_live_acc.reporting_date.to_owned();
        } else {
            seg_class = "0".to_string();
            cust_hlth_cd = "0".to_string();
            cust_const = "0".to_string();
            npa_date = "".to_string();
            v_ucif_cd = "0".to_string();
            npa_report_dt = "".to_string();
        }
        if cust_hlth_cd == *"S" {
            cust_npa_class = "S".to_string();
        } else if npa_config_hashmap.contains_key(&cust_hlth_cd) {
            cust_npa_class = npa_config_hashmap
                .get(&cust_hlth_cd)
                .unwrap_or(&"0".to_string())
                .to_string();
        } else {
            cust_npa_class = "0".to_string();
        }
        if npa_class == *"SS" {
            final_npa_class = "S".to_string();
        } else if npa_class != *"0" {
            final_npa_class = npa_class.to_owned();
        } else {
            final_npa_class = cust_npa_class.to_owned();
        }

        let temp_string = format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            acid,
            foracid,
            cif_no,
            cust_name.replace('\"', ""),
            schm_code,
            schm_type,
            acct_crncy_code,
            acct_open_date,
            npa_class,
            seg_cd,
            amount,
            seg_class,
            cust_hlth_cd,
            cust_const,
            npa_date,
            v_ucif_cd,
            npa_report_dt,
            cust_npa_class,
            final_npa_class,
            out_bal_amt_con,
            seg_cd,
            nfs,
        );
        op_line.push_str(temp_string.as_str());
        op_line.push('\n');
    }

    println!(
        "Total Accounts present in input File: {}\n\
        Total Accounts present in NPA Data File: {}\n\
        Total Accounts present in NPA Live File: {}\n\
        Total Accounts present in NPA Config File: {}",
        tot_input_acc_encntrd, tot_data_acc_encntrd, tot_live_acc_encntrd, tot_config_acc_encntrd
    );
    let end_derive_timer = SystemTime::now();
    let duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total derive process duration.");
    debug!(diag_log, "Derive Process Total Duration: {:?}.", duration);
    let start_writer_time = SystemTime::now();
    match writer.write_all(op_line.as_bytes()) {
        Ok(val) => val,
        Err(error) => {
            panic!("Error writing processed data: {:?}", error);
        }
    }
    let end_writer_time = SystemTime::now();
    let duration = end_writer_time
        .duration_since(start_writer_time)
        .expect("Could not calculate total write process duration.");
    info!(diag_log, "Write Process Total Duration: {:?}.", duration);
}
