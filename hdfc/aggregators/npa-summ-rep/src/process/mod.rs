use crate::configuration_parameters::ConfigurationParameters;
use data_appender::*;
use health_report::HealthReport;
use input_account::config::get_files;
use input_account::read_files;
use input_account::stamper::StamperData;
use input_account::structs::{
    AccData, COAData, DivisionMapping, FinnoneProdToDiv, LNMAlternateAccs, NPAData, WriteOff,
};
use output::{AggrKey, AggrVal};
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::collections::HashMap;
use std::env;
use std::io::Write;
use std::time::SystemTime;

mod data_appender;
mod input_account;
mod output;

pub struct StoreData {
    pub stamper_data: HashMap<String, StamperData>,
    pub coa_data: HashMap<String, COAData>,
    pub finnone_data: HashMap<String, FinnoneProdToDiv>,
    pub lnm_data: HashMap<String, LNMAlternateAccs>,
    pub npa_prev_month_data: HashMap<String, NPAData>,
    pub npa_ason_data: HashMap<String, NPAData>,
    pub npa_prev_year_data: HashMap<String, NPAData>,
    pub writeoff_prev_month_data: HashMap<String, WriteOff>,
    pub writeoff_ason_data: HashMap<String, WriteOff>,
    pub writeoff_prev_year_data: HashMap<String, WriteOff>,
    pub division_data: HashMap<String, DivisionMapping>,
    pub accs_src_sys: HashMap<String, AccData>,
}

pub fn process(config_params: ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let start_time = SystemTime::now();
    let account_output = format!("{}-accounts.txt", config_params.output_file_path());
    let summary_output = format!("{}-summary.txt", config_params.output_file_path());
    let mut output_map: HashMap<AggrKey, AggrVal> = HashMap::new();
    let mut output_account_file = match buf_file_wrtr(&account_output, None) {
        Ok(init) => init,
        Err(error) => {
            panic!(
                "Could not create output file: `{}-accounts.txt` on location `{}`: {}.",
                config_params.output_file_path(),
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    let mut output_summary_file = match buf_file_wrtr(&summary_output, None) {
        Ok(init) => init,
        Err(error) => {
            panic!(
                "Could not create output file: `{}-summary.txt` on location `{}`: {}.",
                config_params.output_file_path(),
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    let files_config = get_files(config_params.config_file_path());

    let files_map = read_files(files_config, logger, _diag_logger);
    let read_duration = print_return_time_since!(start_time);
    println!(
        "Total time taken to Read and Store the files: {:?}",
        read_duration
    );
    info!(
        logger,
        "Total time taken to Read and Store the files: {:?}", read_duration
    );

    let process_start_time = SystemTime::now();
    let mut acc_enc = 0;
    for (key, val) in files_map.accs_src_sys.iter() {
        acc_enc += 1;
        let (aggr_key, aggr_val) = get_output(key.to_string(), val, &config_params, &files_map);
        writeln!(output_account_file, "{}|{}|{}", key, aggr_key, aggr_val)
            .expect("Unable to write account output file.");

        output_map
            .entry(aggr_key.clone())
            .and_modify(|data| data.append_data(aggr_val.clone(), *config_params.as_on_date()))
            .or_insert(aggr_val);
    }
    let process_timer = SystemTime::now();
    let process_duration = process_timer
        .duration_since(process_start_time)
        .expect("Could not calculate total duration.");
    println!(
        "Time for process and write account output: {:?}",
        process_duration
    );
    info!(
        logger,
        "Time for process and write account output: {:?}", process_duration
    );
    for (key, data) in output_map.drain() {
        writeln!(output_summary_file, "{}|{}", key, data)
            .expect("Unable to write summary output file.");
    }
    let write_timer = SystemTime::now();
    let write_duration = write_timer
        .duration_since(process_timer)
        .expect("Could not calculate total duration.");
    println!("Time for writing aggregated data: {:?}", write_duration);
    info!(
        logger,
        "Time for writing aggregated data: {:?}", write_duration
    );
    let health_report = HealthReport::new(acc_enc, acc_enc, 0, 0.0, 0.0, 0);
    health_report.gen_health_rpt(config_params.output_file_path());
    let total_duration = print_return_time_since!(start_time);
    info!(logger, "Total time for the process: {:?}", total_duration);
}
