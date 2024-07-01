use self::io::*;
use configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod io;
use health_report::HealthReport;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let mut op_writer = get_writer(&config_params.output_file_path());

    let input_file = File::open(&config_params.input_file()).expect("Could Not Read File");
    let input_reader = BufReader::new(input_file);

    let io_rule_file =
        File::open(&config_params.inflow_outflow_rule_file()).expect("Could Not Read File");
    let io_rule_reader = BufReader::new(io_rule_file);

    let net_rule_file = File::open(&config_params.net_rule_file()).expect("Could Not Read File");
    let net_rule_reader = BufReader::new(net_rule_file);

    let mut io_rule_fields: HashMap<String, String> = HashMap::new();
    for (index, line) in io_rule_reader.lines().enumerate() {
        let record = line
            .expect("Could not read inflow outflow rule line")
            .to_string();
        let record_fields: Vec<&str> = record.split('|').collect();
        io_rule_fields.insert(record_fields[0].to_string(), record_fields[1].to_string());
    }

    let mut net_rule_fields: HashMap<String, String> = HashMap::new();
    for (index, line) in net_rule_reader.lines().enumerate() {
        let record = line.expect("Could not read net rule line").to_string();
        let record_fields: Vec<&str> = record.split('|').collect();
        net_rule_fields.insert(record_fields[0].to_string(), record_fields[1].to_string());
    }
    let mut tot_acc_encntrd = 0;
    let mut acc_pro_suc = 0;
    let mut tot_amt = 0.0;

    let mut net_amt = 0.0;
    for (index, line) in input_reader.lines().enumerate() {
        tot_acc_encntrd += 1;
        let line = line.expect("Could Not Read Line").to_string();
        let input_fields: Vec<&str> = line.split('|').collect();
        acc_pro_suc += 1;
        tot_amt += input_fields[5]
            .to_string()
            .parse::<f64>()
            .expect("Could not find value");
        if !io_rule_fields.contains_key(input_fields[3]) {
            continue;
        }
        let flow_type = io_rule_fields
            .get(input_fields[3])
            .expect("Could not find flow type.");
        if flow_type == "INFLOW" {
            net_amt += input_fields[5]
                .to_string()
                .parse::<f64>()
                .expect("Could not find value.");
            continue;
        }
        net_amt -= input_fields[5]
            .to_string()
            .parse::<f64>()
            .expect("Could not find value.");
    }
    let llg_id = if net_amt >= 0.0 {
        net_rule_fields.get("POSITIVE")
    } else {
        net_rule_fields.get("NEGATIVE")
    };
    write!(
        op_writer,
        "{}|{}|{}|{}|{}|{}",
        &config_params.as_on_date().format("%d-%m-%Y"),
        "INDIA",
        &config_params.currency(),
        llg_id.expect("Could not find net llg id"),
        net_amt.abs(),
        net_amt.abs()
    );
    let health_report = HealthReport::new(
        tot_acc_encntrd,
        acc_pro_suc,
        tot_acc_encntrd - acc_pro_suc,
        tot_amt,
        tot_amt,
        0,
    );
    health_report.gen_health_rpt(&config_params.output_file_path());
}
