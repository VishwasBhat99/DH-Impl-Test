mod account_as_cashflows;
mod account_field_names;
mod account_writer;
mod cashflow_appender;
mod io;
mod structs;

use self::cashflow_appender::append_data;
use self::io::*;
use self::structs::*;
use cashflow_generator::account_field_names::AccFieldNames;
use cashflow_generator::account_writer::AccountWriter;
use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::incr_dt_by_mon_presrv_eom;
use sdb_dyn_proto_rdr::reader;
use slog::Logger;
use statics::DEFAULT_FLOAT;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct RangeSlab {
    id: String,
    from: f64,
    to: f64,
}

pub struct RWData {
    rw_prcnt: f64,
}

pub struct RFFlag {
    rf_flag: String,
}

pub struct RMFlag {
    rm_flag: String,
}

pub fn generate(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    // init account reader
    let mut account_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );

    // init req fields
    let keys = AccFieldNames::new_from_path(config_params.req_fields_file_path());
    let mut cust_details: HashMap<String, CustFields> = HashMap::new();
    for account in account_reader.iter() {
        if let Ok(cust_id) = account.get_string_for_key(&keys.cust_id) {
            cust_details.insert(cust_id.to_string(), CustFields::new());
        }
    }

    // read cust master file
    let mut cust_master: HashMap<String, String> = HashMap::new();
    let cust_master_file = read_file(config_params.cust_master_file_path());
    for (line_num, lines) in cust_master_file.lines().enumerate() {
        let line = extract_lines(line_num, lines, config_params.cust_master_file_path());
        let fields: Vec<&str> = line.split(',').collect();
        cust_master.insert(fields[0].trim().to_string(), fields[1].trim().to_string());
    }

    // read risk weight master file
    let rw_master_file = read_file(config_params.rw_master_file_path());
    let mut rw_master: HashMap<String, RWData> = HashMap::new();
    for (line_num, lines) in rw_master_file.lines().enumerate() {
        let line = extract_lines(line_num, lines, config_params.rw_master_file_path());
        let fields: Vec<&str> = line.split('|').collect();
        let data = RWData {
            rw_prcnt: fields[1].trim().parse().unwrap_or(DEFAULT_FLOAT),
        };
        rw_master.insert(fields[0].trim().to_string(), data);
    }

    // read restructured flag master file
    let rf_master_file = read_file(config_params.restructured_flag_file_path());
    let mut rf_master: HashMap<String, RFFlag> = HashMap::new();
    for (line_num, lines) in rf_master_file.lines().enumerate() {
        let line = extract_lines(line_num, lines, config_params.restructured_flag_file_path());
        let fields: Vec<&str> = line.split('|').collect();
        let data = RFFlag {
            rf_flag: fields[1].trim().parse().unwrap_or("".to_string()),
        };
        rf_master.insert(fields[0].trim().to_string(), data);
    }

    // read residential mortgage flag master file
    let rm_master_file = read_file(config_params.residential_mortgage_file_path());
    let mut rm_master: HashMap<String, RMFlag> = HashMap::new();
    for (line_num, lines) in rm_master_file.lines().enumerate() {
        let line = extract_lines(
            line_num,
            lines,
            config_params.residential_mortgage_file_path(),
        );
        let fields: Vec<&str> = line.split('|').collect();
        let data = RMFlag {
            rm_flag: fields[1].trim().parse().unwrap_or("".to_string()),
        };
        rm_master.insert(fields[0].trim().to_string(), data);
    }

    // read cust file
    let cust_file = read_file(config_params.cust_file_path());
    for (line_num, lines) in cust_file.lines().enumerate().skip(1) {
        let line = extract_lines(line_num, lines, config_params.cust_file_path());
        let fields: Vec<&str> = line.split("~#~").collect();
        if fields.len() == 4 {
            if let Some(mut cust_fields) = cust_details.remove(fields[0]) {
                cust_fields.update_cust_basel_val_code(fields[1]);
                cust_details.insert(fields[0].trim().to_string(), cust_fields);
            }
        }
    }

    // read biu file
    let biu_file = read_file(config_params.biu_file_path());
    for (line_num, lines) in biu_file.lines().enumerate() {
        let line = extract_lines(line_num, lines, config_params.biu_file_path());
        let fields: Vec<&str> = line.split('|').collect();
        if fields.len() == 6 {
            if let Some(mut cust_fields) = cust_details.remove(fields[0]) {
                cust_fields.update_div(fields[5]);
                cust_details.insert(fields[0].trim().to_string(), cust_fields);
            }
        }
    }

    // read cust id file
    let cust_id_file = read_file(config_params.cust_id_file_path());
    for (line_num, lines) in cust_id_file.lines().enumerate().skip(1) {
        let line = extract_lines(line_num, lines, config_params.cust_id_file_path());
        let fields: Vec<&str> = line.split("~#~").collect();
        if fields.len() == 4 {
            if let Some(mut cust_fields) = cust_details.remove(fields[0]) {
                cust_fields.update_bus_and_txt_desc(&fields);
                cust_details.insert(fields[0].trim().to_string(), cust_fields);
            }
        }
    }

    // read class file
    let class_file = read_file(config_params.class_file_path());
    for (line_num, lines) in class_file.lines().enumerate() {
        let line = extract_lines(line_num, lines, config_params.class_file_path());
        let fields: Vec<&str> = line.split('|').collect();
        if fields.len() == 45 {
            if let Some(mut cust_fields) = cust_details.remove(fields[1]) {
                cust_fields.update_sme_class(fields[0]);
                cust_details.insert(fields[1].trim().to_string(), cust_fields);
            }
        }
    }
    // read ea_master file
    let mut ea_master: HashMap<String, NaiveDate> = HashMap::new();
    let ea_master_file = read_file(config_params.ea_master_file_path());
    for (line_num, lines) in ea_master_file.lines().enumerate() {
        let line = extract_lines(line_num, lines, config_params.ea_master_file_path());
        let fields: Vec<&str> = line.split('|').collect();
        if fields.len() == 3 {
            let pledge_end_date = NaiveDate::parse_from_str(fields[2], "%d-%m-%Y")
                .unwrap_or(*config_params.as_on_date());
            ea_master.insert(fields[1].trim().to_string(), pledge_end_date);
        } else {
            log_debug!(
                log,
                "skipped the account from ea_master due to the line length is not equals to 3"
            );
        }
    }
    // init writer
    let mut writer = AccountWriter::new(config_params.output_file_path(), log);
    let prd_slabs: Vec<RangeSlab> =
        get_prd_slabs(config_params.slabs_file_path(), config_params.as_on_date());
    let mut tot_rec = 0;
    let skp_rec = 0;
    let tot_amt = 0.0;
    // init account reader
    let mut account_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );
    let method_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );
    for account in account_reader.iter() {
        tot_rec += 1;
        let acc_id = account
            .get_string_for_key(&keys.acc_id)
            .expect("Error while reading account id.");
        let account_data = append_data(
            acc_id.to_string(),
            account,
            &method_reader,
            &keys,
            &prd_slabs,
            &cust_master,
            &rw_master,
            &rf_master,
            &rm_master,
            &config_params,
            &cust_details,
            &ea_master,
            diag_log,
        );
        writer.write(account_data);
    }

    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    log_info!(log, "{}", health_report.display());
    health_report.gen_health_rpt(&config_params.output_file_path());
}

pub fn get_prd_slabs(path: &str, as_on_date: &NaiveDate) -> Vec<RangeSlab> {
    let mut slabs: Vec<RangeSlab> = Vec::new();
    let input_file = match File::open(path) {
        Ok(input_file) => input_file,
        Err(error) => panic!("{}", error),
    };
    let reader = BufReader::new(input_file);
    for line in reader.lines() {
        match line {
            Ok(slab_info) => {
                let info: Vec<&str> = slab_info.split('|').collect();
                //TODO: Change this approach of handling cases like "-1D"
                let from_days = if info[1].contains("-") {
                    -1.0 * get_days(info[1].replace("-", "").as_str(), as_on_date)
                } else {
                    get_days(info[1], as_on_date)
                };
                let to_days = if info[2].contains("-") {
                    -1.0 * get_days(info[2].replace("-", "").as_str(), as_on_date)
                } else {
                    get_days(info[2], as_on_date)
                };
                let new_slab = RangeSlab {
                    id: info[0].to_string(),
                    from: from_days,
                    to: to_days,
                };
                slabs.push(new_slab)
            }
            Err(error) => {
                panic!("Cannot read line from input file: {:?}", error);
            }
        };
    }
    slabs
}

fn get_days(info: &str, as_on_date: &NaiveDate) -> f64 {
    let mut alpha_code: Vec<&str> = info.split(|c: char| c.is_numeric()).collect();
    alpha_code.retain(|&x| x != "");
    let mut num_code: Vec<&str> = info.split(|c: char| c.is_alphabetic()).collect();
    num_code.retain(|&x| x != "");
    let mut days = 0.0;
    for (i, num_val) in num_code.iter().enumerate() {
        let period = num_val.to_string() + alpha_code[i];
        days += num_days(&period, as_on_date);
    }
    days
}
fn num_days(info: &str, as_on_date: &NaiveDate) -> f64 {
    if info.contains("D") {
        let period: i64 = info
            .trim_matches('D')
            .parse::<i64>()
            .expect("Invalid from day format");
        return period as f64;
    } else if info.contains("M") {
        let period: usize = info
            .trim_matches('M')
            .parse::<usize>()
            .expect("Invalid from month format");
        let new_date = incr_dt_by_mon_presrv_eom(*as_on_date, period)
            .expect("Cannot add month to as on date as per prd slab config");
        return rbdate::num_days_start_to_end(*as_on_date, new_date) as f64;
    } else if info.contains("Y") {
        let period: usize = info
            .trim_matches('Y')
            .parse::<usize>()
            .expect("Invalid from year format");
        let new_date = incr_dt_by_mon_presrv_eom(*as_on_date, period * 12)
            .expect("Cannot add month to as on date as per prd slab config");
        return rbdate::num_days_start_to_end(*as_on_date, new_date) as f64;
    } else {
        panic!("Invalid period type in prd config file.");
    }
}
