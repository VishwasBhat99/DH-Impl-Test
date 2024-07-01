use self::derive_fields::*;
use self::io::*;
use self::structs::{cust_master::*, extra_fields::*, input_account::InputAccount};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use slog::Logger;
use statics::*;
use std::collections::HashMap;
use std::default::Default;
use std::io::BufRead;
use std::time::SystemTime;

mod derive_fields;
mod io;
mod structs;

pub fn process(config_param: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let st_tm_read = SystemTime::now();
    let mut op_line: String = String::new();
    let mut tot_rec = DEFAULT_INT;
    let skp_rec = DEFAULT_INT;

    let mut cust_master_reader = read_file(config_param.cust_master());
    let mut cust_master: CustMasterMap = CustMasterMap::new();
    for (line_num, lines) in cust_master_reader.deserialize().enumerate() {
        let cust_master_input: CustMasterInput =
            extract_lines(line_num, lines, config_param.cust_master(), log);
        get_cust_master_data(cust_master_input, &mut cust_master);
    }
    log_info!(
        log,
        "Number of records for Cust Master File: {}",
        cust_master.store.len()
    );
    let mut input_reader = read_file(config_param.input_file_path());
    let mut tot_amt = 0.0;
    let mut extra_field_reader = read_file(config_param.extra_fields_file_path());
    let mut extra_field_map: HashMap<String, ExtraFieldData> = HashMap::new();
    for (_, lines) in extra_field_reader.deserialize().enumerate() {
        let extra_field_rec: ExtraFieldData =
            lines.expect("Cannot read line into Extra Fields Data struct!");
        extra_field_map.insert(extra_field_rec.acc_id.to_string(), extra_field_rec);
    }
    log_info!(
        log,
        "Number of records for Extra Fields File: {}",
        extra_field_map.len()
    );
    // Init LTV Reader
    let ltv_reader =
        sdb_io::new_buf_rdr(config_param.ltv_file_path()).expect("Cannot open LTV file for read!");
    let mut ltv_map: HashMap<String, String> = HashMap::new();
    for (_, lines) in ltv_reader.lines().enumerate() {
        let line = lines.expect("Cannot read data from LTV file!");
        let line_info: Vec<&str> = line.split('|').collect();
        let acc_no = line_info[0].to_string();
        let ltv = line_info[1].to_string();
        ltv_map.insert(acc_no, ltv);
    }
    log_info!(
        log,
        "Number of records for LTV Fields File: {}",
        ltv_map.len()
    );
    for (line_num, lines) in input_reader.deserialize().enumerate().skip(1) {
        let mut input_account: InputAccount =
            extract_lines(line_num, lines, config_param.input_file_path(), log);
        tot_rec += 1;
        let amt = input_account
            .ccod_ualimit
            .parse::<f64>()
            .unwrap_or(DEFAULT_FLOAT)
            + input_account
                .tl_ualimit
                .parse::<f64>()
                .unwrap_or(DEFAULT_FLOAT);
        tot_amt += amt;

        op_line.push_str(&get_op_line(
            &mut input_account,
            &mut cust_master,
            &extra_field_map,
            &ltv_map,
        ));
    }

    let ed_tm_read = SystemTime::now();
    let duration = ed_tm_read
        .duration_since(st_tm_read)
        .expect("Could not calculate total read process duration.");
    debug!(diag_log, "Read Process Total Duration: {:?}.", duration);

    let st_tm_writer = SystemTime::now();
    let mut op_writer = get_writer(config_param.output_file_path());
    output_writer(&mut op_writer, op_line, config_param.output_file_path());

    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    log_info!(log, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_param.output_file_path());

    let ed_tm_writer = SystemTime::now();
    let duration = ed_tm_writer
        .duration_since(st_tm_writer)
        .expect("Could not calculate total duration for write process.");
    debug!(diag_log, "Writing LCBG, Total Duration: {:?}.", duration);
}
