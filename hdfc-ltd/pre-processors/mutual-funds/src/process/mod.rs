use self::io::*;
use configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod io;
use health_report::HealthReport;
use sdb_io::new_buf_rdr;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let mut op_writer = get_writer(&config_params.output_file_path());
    let mut tot_acc_encntrd = 0;
    let mut acc_pro_suc = 0;
    let mut tot_amt = 0.0;
    let input = File::open(&config_params.input_file()).expect("Could Not Read File");
    let input_reader = BufReader::new(input);

    for (index, line) in input_reader.lines().enumerate().skip(1) {
        tot_acc_encntrd += 1;
        let line = line.expect("Could Not Read Line").to_string();
        let input_fields: Vec<&str> = line.split('|').collect();
        acc_pro_suc += 1;
        write!(
            op_writer,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            input_fields[0],
            input_fields[1],
            input_fields[2],
            input_fields[3],
            input_fields[4],
            input_fields[5],
            input_fields[6],
            input_fields[7],
            input_fields[8],
            input_fields[9],
            input_fields[10],
            input_fields[11],
            input_fields[12],
            input_fields[13],
        );
    }
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
