pub mod dep_grouping;
pub mod io;
pub mod structs;
use config_params::ConfigurationParameters;
use health_report::HealthReport;
use processing::dep_grouping::get_liability_bal;
use processing::dep_grouping::read_file_and_create_op_data;
use processing::dep_grouping::write_output;
use slog::Logger;
use std::default::Default;
use std::time::SystemTime;

pub fn process(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut ttl_accnts: usize = 0;
    let mut ttl_suc_accnts: usize = 0;
    let mut ttl_bal_ip = 0.0;
    let mut ttl_bal_op = 0.0;

    let mut start_timer = SystemTime::now();
    // Reading liability balance file
    let liability_bal = get_liability_bal(config_params, log, diag_log);
    let mut end_timer = SystemTime::now();
    let mut duration = end_timer
        .duration_since(start_timer)
        .expect("Could not calculate total duration read timer.");
    info!(
        diag_log,
        "Reading liability balance file, Total Duration: {:?}.", duration
    );

    start_timer = SystemTime::now();
    // Reading input file and creating an ouput string
    let sig_cntrprty_data = read_file_and_create_op_data(
        &config_params,
        liability_bal,
        &mut ttl_accnts,
        &mut ttl_suc_accnts,
        &mut ttl_bal_ip,
        &mut ttl_bal_op,
        log,
        diag_log,
    );
    end_timer = SystemTime::now();
    duration = end_timer
        .duration_since(start_timer)
        .expect("Could not calculate total duration read timer.");
    info!(
        diag_log,
        "Reading pre-processed deposits file & creating output data, Total Duration: {:?}.",
        duration
    );

    start_timer = SystemTime::now();
    // Writing output data into file.
    write_output(config_params, sig_cntrprty_data, diag_log);
    end_timer = SystemTime::now();
    duration = end_timer
        .duration_since(start_timer)
        .expect("Could not calculate total duration writer timer.");
    info!(
        diag_log,
        "Writing significant counterparty data into file, Total Duration: {:?}.", duration
    );

    // Generating health report.
    let health_report = HealthReport::new(
        ttl_accnts as i64,
        ttl_suc_accnts as i64,
        (ttl_accnts - ttl_suc_accnts) as i64,
        ttl_bal_ip,
        ttl_bal_op,
        0,
    );
    health_report.display();
    health_report.gen_health_rpt(&config_params.output_file());
}
