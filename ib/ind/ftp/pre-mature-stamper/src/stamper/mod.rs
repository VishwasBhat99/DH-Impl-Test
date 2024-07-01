use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use rbdate::num_days_start_to_end;
use rbdate::{decr_dt_by_mon_presrv_eom, NaiveDate};
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use statics::*;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::time::SystemTime;

pub fn process(config_param: ConfigurationParameters, _log: &Logger, diag_log: &Logger) {
    let start_process_time = SystemTime::now();
    let input_file = match new_buf_rdr(config_param.input_stamper_file_path()) {
        Ok(input_file) => input_file,
        Err(_error) => panic!("Error while getting input directory path"),
    };
    let asondt = config_param.as_on_date().format("%d%m%Y").to_string();
    let asondt_one_month_less = decr_dt_by_mon_presrv_eom(*config_param.as_on_date(), 1).unwrap().format("%d%m%Y").to_string();
    let master_file_path = config_param.master_stamper_file_path().replace(&asondt, &asondt_one_month_less);
    let master_file = match new_buf_rdr(&master_file_path) {
        Ok(master_file) => master_file,
        Err(_error) => panic!("Error while getting master file path"),
    };
    let output_file = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(_error) => panic!("Error while getting output directory path"),
    };

    let reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);
    let master_reader = BufReader::new(master_file);
    let mut tot_acc_encntrd = DEFAULT_INT;
    let tot_amt = DEFAULT_FLOAT;

    //Reading Master Data File
    let mut current_month_map: HashMap<String, String> = HashMap::new();

    for line in reader.lines() {
        let acc_info: String = match line {
            Ok(acc_info) => acc_info,
            Err(error) => {
                panic!("Cannot read line from input file: {:?}", error);
            }
        };
        let fields: Vec<&str> = acc_info.split('|').collect();
        tot_acc_encntrd += 1;

        current_month_map.insert(fields[1].to_string(), acc_info);
        
    }
    let def_data = String::new();
    let start_writer_time = SystemTime::now();
    for line in master_reader.lines() {
        let acc_info: String = match line {
            Ok(acc_info) => acc_info,
            Err(error) => {
                panic!("Cannot read line from master file: {:?}", error);
            }
        };
        let fields: Vec<&str> = acc_info.split('|').collect();

        let acid = fields[1].to_string();
        if !current_month_map.contains_key(&acid) {
            continue;
        }
        let curr_month_data = current_month_map.remove(&acid).unwrap_or(def_data.clone());
        let mut curr_month_fields: Vec<String> = curr_month_data.split('|').map(String::from).collect();

        if fields.len() < 60 {
            continue;
        }
        let method_id = &curr_month_fields[10].clone();
        if method_id == "Matched Term 1" && fields[10] == "Matched Term 1" {
            curr_month_fields[10] = "PMT Matched Term 1".to_string()
        } else if method_id == "Matched Term 2" && fields[10] == "Matched Term 2" {
            curr_month_fields[10] = "PMT Matched Term 2".to_string()
        } else if method_id == "Matched Term 3" && fields[10] == "Matched Term 3" {
            curr_month_fields[10] = "PMT Matched Term 3".to_string()
        }

        if method_id == fields[10] {
            let bal_amt_ccy_open = fields[5].to_string().parse().unwrap_or(0.0);
            let bal_amt_hcy_open = fields[6].to_string().parse().unwrap_or(0.0);
            let ftp_int_rate_open = fields[33].to_string().parse().unwrap_or(0.0);
            let ftp_int_rate_closed = curr_month_fields[33].to_string().parse().unwrap_or(0.0);
            let asondt = NaiveDate::parse_from_str(&curr_month_fields[0], "%d-%m-%Y")
                .unwrap_or(*config_param.as_on_date());
            let tenor_start_date_applied =
                NaiveDate::parse_from_str(&curr_month_fields[52], "%d-%m-%Y")
                    .unwrap_or(*config_param.as_on_date());
            let tenor_start_date_applied_open = NaiveDate::parse_from_str(fields[52], "%d-%m-%Y")
                .unwrap_or(*config_param.as_on_date());
            let tenor_end_date_applied =
                NaiveDate::parse_from_str(&curr_month_fields[53], "%d-%m-%Y")
                    .unwrap_or(*config_param.as_on_date());

            let asondt_one_mon_less = decr_dt_by_mon_presrv_eom(asondt, 1)
                .expect("Cannot decrement 1 month from ason date");

            let _num_days_btwn =
                num_days_start_to_end(tenor_start_date_applied, asondt_one_mon_less) as f64;
            let num_days_btwn_closed =
                num_days_start_to_end(tenor_start_date_applied_open, asondt_one_mon_less) as f64;
            let num_days =
                num_days_start_to_end(tenor_start_date_applied, tenor_end_date_applied) as f64;

            let ftp_amt_ccy = (( ((bal_amt_ccy_open * ftp_int_rate_closed * num_days) / 36500.0 )-(bal_amt_ccy_open * ftp_int_rate_open * num_days_btwn_closed) / 36500.0 )).to_string();
            let ftp_amt_hcy = (((bal_amt_hcy_open * ftp_int_rate_closed * num_days) / 36500.0 )-((bal_amt_hcy_open * ftp_int_rate_open * num_days_btwn_closed) / 36500.0 )).to_string();
            curr_month_fields[35] = ftp_amt_ccy;
            curr_month_fields[36] = ftp_amt_hcy;
        }
        let final_vec = curr_month_fields.clone();
        let mut output_line = final_vec.join("|");
        output_line = format!("{}\n", output_line.clone());
        match writer.write_all(output_line.as_bytes()) {
            Ok(val) => val,
            Err(error) => {
                panic!("Error writing processed data: {:?}", error);
            }
        }
    }


    for (_acc, acc_line) in current_month_map {
        let output_line = format!("{}\n", acc_line);
        match writer.write_all(output_line.as_bytes()) {
            Ok(val) => val,
            Err(error) => {
                panic!("Error writing processed data: {:?}", error);
            }
        }
    }

    let end_process_time = SystemTime::now();
    let duration = end_process_time
        .duration_since(start_process_time)
        .expect("Could not calculate total process duration.");
    info!(diag_log, "Process Total Duration: {:?}.", duration);

    let end_writer_time = SystemTime::now();
    let duration = end_writer_time
        .duration_since(start_writer_time)
        .expect("Could not calculate total write process duration.");
    info!(diag_log, "Write Process Total Duration: {:?}.", duration);

    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd,
        0,
        tot_amt,
        tot_amt,
        DEFAULT_INT,
    );
    health_report.gen_health_rpt(config_param.output_file_path());
}


