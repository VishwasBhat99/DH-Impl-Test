use self::derive_fields::*;
use self::io::*;
use self::structs::{input_account::*, *};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use slog::Logger;
use statics::*;
use std::collections::HashMap;
use std::default::Default;
use std::time::SystemTime;

mod derive_fields;
mod io;
mod structs;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let st_tm_read = SystemTime::now();
    let mut op_line: String = String::new();
    let mut tot_rec = DEFAULT_INT;
    let skp_rec = DEFAULT_INT;

    let mut non_core_reader = read_file(config_param.non_core_input_file());
    let mut tot_amt = DEFAULT_FLOAT;
    let mut non_core_map: HashMap<String, InputAccount> = HashMap::new();
    for (line_num, lines) in non_core_reader.deserialize().enumerate() {
        let non_core_acc: InputAccount =
            extract_lines(line_num, lines, config_param.non_core_input_file(), log);
        tot_rec += 1;
        tot_amt += non_core_acc.average_balance;
        non_core_map.insert(non_core_acc.account_number.to_string(), non_core_acc);
    }

    let mut core_reader = read_file(config_param.core_input_file());
    for (line_num, lines) in core_reader.deserialize().enumerate() {
        let mut core_acc: InputAccount =
            extract_lines(line_num, lines, config_param.core_input_file(), log);
        tot_rec += 1;
        tot_amt += core_acc.average_balance;
        op_line.push_str(&get_op_line(&mut core_acc, &mut non_core_map));
    }
  
    //Reading restructured stamper output
    let mut tot_rec_add = 0;
    let mut tot_amt_add: f64 = 0.0;
    let mut non_core_add_reader = read_file(config_param.non_core_add_input_file());
    let mut tot_amt = DEFAULT_FLOAT;
    let mut non_core_add_map: HashMap<String, InputAccountAdditional> = HashMap::new();
    for (line_num, lines) in non_core_add_reader.deserialize().enumerate() {
        let non_core_acc: InputAccountAdditional =
            extract_lines(line_num, lines, config_param.non_core_add_input_file(), log);
        tot_rec_add += 1;
        tot_amt_add += non_core_acc.avg_bal;
        non_core_add_map.insert(non_core_acc.acc_num.to_string(), non_core_acc);
    }

    let mut add_op_line: String = String::new();
    let mut core_add_reader = read_file(config_param.core_add_input_file());
    for (line_num, lines) in core_add_reader.deserialize().enumerate() {
        let mut core_acc: InputAccountAdditional =
            extract_lines(line_num, lines, config_param.core_add_input_file(), log);
        tot_rec_add += 1;
        tot_amt_add += core_acc.avg_bal;
        add_op_line.push_str(&&get_op_line_add(&mut core_acc, &mut non_core_add_map));
    }
    let ed_tm_read = SystemTime::now();
    let duration = ed_tm_read
        .duration_since(st_tm_read)
        .expect("Could not calculate total read process duration.");
    debug!(diag_log, "Read Process Total Duration: {:?}.", duration);

    let st_tm_writer = SystemTime::now();
    let op_file_path = format!("{}.txt", config_param.output_file_path());
    let mut op_writer_core = get_writer(&op_file_path);
    output_writer(
        &mut op_writer_core,
        op_line,
        config_param.output_file_path(),
    );
    let additional_op_file_path = format!("{}_additional.txt", config_param.output_file_path());
    let mut op_add_writer_core = get_writer(&additional_op_file_path);
    output_writer(
        &mut op_add_writer_core,
        add_op_line,
        &additional_op_file_path,
    );

    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    log_info!(log, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_param.output_file_path());
    let ed_tm_writer = SystemTime::now();
    let duration = ed_tm_writer
        .duration_since(st_tm_writer)
        .expect("Could not calculate total duration for write process.");
    debug!(diag_log, "Writing OD, Total Duration: {:?}.", duration);
}
