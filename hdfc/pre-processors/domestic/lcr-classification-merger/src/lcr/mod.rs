use self::io::*;
use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use calamine::{open_workbook_auto, Reader};
use health_report::HealthReport;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::Write;
use std::time::SystemTime;

pub mod config;
mod io; 
mod process;

pub fn lcr_merger(config_params: ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let start_time = SystemTime::now();
    let mut output_file = get_writer(config_params.output_file_path());

    let mut lcr_map: HashMap<String, String> = HashMap::new();
    let mut lcr_master_excel =
        open_workbook_auto(config_params.lcr_master()).expect("Unable to open lcr Master File.");
    if let Some(Ok(reader)) = lcr_master_excel.worksheet_range(config_params.lcr_sheet()) {
        for row in reader.rows().skip(1) {
            lcr_map.insert(row[0].to_string(), row[1].to_string());
        }
    }
    let mut acc_enc = 0;
    let mut acc_succ = 0;
    let mut ip_amt = 0.0;
    let mut op_amt = 0.0;

    let files_config = config::get_files(config_params.config_file_path());
    for file in files_config.files {
        let input_file = read_file(&file);
        for (line_num, lines) in input_file.lines().enumerate() {
            let line = extract_lines(line_num, lines, &file);
            let output_line = process::append_data(&line,&mut lcr_map);
            acc_enc+=1;
            write!(output_file, "{}", output_line)
                    .expect("Unable to write summary file.");
            acc_succ+=1;
        }
    }

    let health_report = HealthReport::new(acc_enc, acc_succ, acc_enc - acc_succ, ip_amt, op_amt, 0);
    health_report.gen_health_rpt(&config_params.output_file_path());
    let total_duration = print_return_time_since!(start_time);
    log_info!(logger, "Total time for aggregation: {:?}", total_duration);
}
