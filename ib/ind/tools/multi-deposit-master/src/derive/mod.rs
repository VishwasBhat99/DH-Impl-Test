extern crate csv;
extern crate serde;
use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use hashbrown::{HashMap, HashSet};
use health_report::HealthReport;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::fs;
use std::io::prelude::*;
use std::time::SystemTime;

pub fn process(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_process_timer = SystemTime::now();
    let mut tot_rec = 0;
    let mut succ_rec = 0;
    let mut multi_deposit_rec = 0;

    let casatd_master_out_file = format!("{}/CASATD_Master.txt", config_params.output_file_path());
    let multi_dep_out_file = format!("{}/MultiDeposit.txt", config_params.output_file_path());
    let is_cust_repeated = config_params.is_cust_repeated();

    let mut casatd_output_file = match buf_file_wrtr(casatd_master_out_file.as_str(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{}", error),
    };
    let mut multi_deposit_output_file = match buf_file_wrtr(multi_dep_out_file.as_str(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{}", error),
    };

    let mut td_cust_no_set: HashSet<String> = HashSet::new();
    let mut ca_cust_no_set: HashSet<String> = HashSet::new();
    let mut sa_cust_no_set: HashSet<String> = HashSet::new();
    let mut printed_cust_no_set: HashSet<String> = HashSet::new();

    let td_reader = fs::read_to_string(config_params.td_input_file_path())
        .expect("Could Not Read TD input file path");
    let ca_reader = fs::read_to_string(config_params.ca_input_file_path())
        .expect("Could Not Read CA input file path");
    let sa_reader = fs::read_to_string(config_params.sa_input_file_path())
        .expect("Could Not Read SA input file path");


    for line in ca_reader.lines() {
        tot_rec += 1;
        let ca_fields = line.split('|').collect::<Vec<&str>>();
        let ca_key = get_str_from_txt(config_params.ca_input_file_path(), &ca_fields, 0);
        let ca_cust_no = get_str_from_txt(config_params.ca_input_file_path(), &ca_fields, 7);
        let ca_prod_cd = get_str_from_txt(config_params.ca_input_file_path(), &ca_fields, 41);
        let ca_tier_cust_type =
            get_str_from_txt(config_params.ca_input_file_path(), &ca_fields, 37);

        writeln!(
            casatd_output_file,
            "{}|{}|{}|{}",
            &ca_key, &ca_cust_no, &ca_prod_cd, &ca_tier_cust_type
        )
        .expect("Unable to generate CASATD Master file.");

        succ_rec += 1;
        if is_cust_repeated {
            if ca_cust_no_set.contains(&ca_cust_no) {
                printed_cust_no_set.insert(ca_cust_no.to_string());
            }
        }
        ca_cust_no_set.insert(ca_cust_no.to_string());
    }

    for line in sa_reader.lines() {
        tot_rec += 1;
        let sa_fields = line.split('|').collect::<Vec<&str>>();
        let sa_key = get_str_from_txt(config_params.sa_input_file_path(), &sa_fields, 0);
        let sa_cust_no = get_str_from_txt(config_params.sa_input_file_path(), &sa_fields, 7);
        let sa_prod_cd = get_str_from_txt(config_params.sa_input_file_path(), &sa_fields, 41);
        let sa_tier_cust_type =
            get_str_from_txt(config_params.sa_input_file_path(), &sa_fields, 37);

        writeln!(
            casatd_output_file,
            "{}|{}|{}|{}",
            &sa_key, &sa_cust_no, &sa_prod_cd, &sa_tier_cust_type
        )
        .expect("Unable to generate CASATD Master file.");

        succ_rec += 1;
        if is_cust_repeated {
            if sa_cust_no_set.contains(&sa_cust_no) {
                printed_cust_no_set.insert(sa_cust_no.to_string());
            }
        }
        if ca_cust_no_set.contains(&sa_cust_no) {
            printed_cust_no_set.insert(sa_cust_no.to_string());
        }
        sa_cust_no_set.insert(sa_cust_no.to_string());
    }
    for line in td_reader.lines() {
        tot_rec += 1;
        let td_fields = line.split('|').collect::<Vec<&str>>();
        let td_key = td_fields[0];
        let td_cust_no = td_fields[7];
        let td_prod_cd = td_fields[45];
        let td_tier_cust_type = td_fields[41];

        td_cust_no_set.insert(td_cust_no.to_string());
        if ca_cust_no_set.contains(td_cust_no) || sa_cust_no_set.contains(td_cust_no) {
            printed_cust_no_set.insert(td_cust_no.to_string());
        }
        writeln!(
            casatd_output_file,
            "{}|{}|{}|{}",
            &td_key, &td_cust_no, &td_prod_cd, &td_tier_cust_type
        )
        .expect("Unable to generate CASATD Master file.");
        succ_rec += 1;
    }

    for ele in printed_cust_no_set {
        writeln!(multi_deposit_output_file, "{}", &ele).expect("Error writing multi deposit file");
    }

    let end_process_timer = SystemTime::now();
    let duration = end_process_timer
        .duration_since(start_process_timer)
        .expect("Could not calculate total duration for the process.");
    debug!(
        diag_log,
        "Total Duration for Reading and Writing Records: {:?}.", duration
    );
    log_info!(
        log,
        "Total Records Found as Multi Deposit {}",
        multi_deposit_rec
    );
    let health_report = HealthReport::new(tot_rec, succ_rec, tot_rec - succ_rec, 0.0, 0.0, 0);
    health_report.gen_health_rpt(casatd_master_out_file.as_str());
}

pub fn get_str_from_txt(input_file: &str, data: &[&str], index: usize) -> String {
    data.get(index)
        .unwrap_or_else(|| {
            panic!(
                "Could not get data at column-no: `{}` from File: {}",
                index + 1,
                input_file,
            )
        })
        .trim()
        .trim_matches(|pat| pat == ' ' || pat == '"')
        .to_string()
}
