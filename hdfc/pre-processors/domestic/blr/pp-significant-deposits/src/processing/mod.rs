pub mod dep_grouping;
pub mod structs;

use config_params::ConfigurationParameters;
use health_report::HealthReport;
use processing::dep_grouping::create_output;
use processing::dep_grouping::get_dep_map;
use processing::dep_grouping::get_sorted_data;
use processing::dep_grouping::get_summarised_data;
use processing::dep_grouping::get_ucic_cust_map;
use processing::dep_grouping::write_output;
use slog::Logger;
use std::time::SystemTime;

pub fn process_data(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut ttl_accnts: usize = 0;
    let mut ttl_suc_accnts: usize = 0;
    let mut ttl_bal_ip = 0.0;
    let mut ttl_bal_op = 0.0;
    let mut start_timer = SystemTime::now();
    // This hashmap contains customer id as key and ucic id and ucic name as value.
    let ucic_cust_map = get_ucic_cust_map(&config_params, &log, &diag_log);
    let mut end_timer = SystemTime::now();
    let mut duration = end_timer
        .duration_since(start_timer)
        .expect("Could not calculate total duration read timer.");
    info!(
        diag_log,
        "Reading ucic maping File, Total Duration: {:?}.", duration
    );
    start_timer = SystemTime::now();
    // This hashmap contains customer id as key and ca amount, sa amount & td amount as value.
    let deposits_det = get_dep_map(
        &config_params,
        &mut ttl_accnts,
        &mut ttl_suc_accnts,
        &mut ttl_bal_ip,
        &log,
        &diag_log,
    );
    end_timer = SystemTime::now();
    duration = end_timer
        .duration_since(start_timer)
        .expect("Could not calculate total duration read timer.");
    info!(
        diag_log,
        "Reading deposits data File, Total Duration: {:?}.", duration
    );
    start_timer = SystemTime::now();
    // This hashmap contains ucicid as key and entire data required for topnd as value.
    let summarised_data = get_summarised_data(ucic_cust_map, deposits_det, diag_log);
    // Getting the data in descending order.
    let top_n_vec = get_sorted_data(summarised_data, &diag_log);
    // Creating output string for all the records to write into file.
    let top_n_op_data = create_output(&top_n_vec, &mut ttl_bal_op, &config_params, &diag_log);
    end_timer = SystemTime::now();
    duration = end_timer
        .duration_since(start_timer)
        .expect("Could not calculate total duration process timer.");
    info!(
        diag_log,
        "Processing deposits data File, Total Duration: {:?}.", duration
    );
    start_timer = SystemTime::now();

    // Writing output data into file.
    write_output(&config_params, top_n_op_data, &log, &diag_log);
    end_timer = SystemTime::now();
    duration = end_timer
        .duration_since(start_timer)
        .expect("Could not calculate total duration writer timer.");
    info!(
        diag_log,
        "Writing topn deposits data into File, Total Duration: {:?}.", duration
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
