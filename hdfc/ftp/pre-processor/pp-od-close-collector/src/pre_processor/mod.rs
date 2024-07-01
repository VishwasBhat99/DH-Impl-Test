use self::io::*;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use slog::Logger;
use statics::*;
use std::io::prelude::*;
use std::time::SystemTime;

mod io;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let st_tm_read = SystemTime::now();
    let mut tot_rec = DEFAULT_INT;
    let skp_rec = DEFAULT_INT;

    let mut amb_data: Vec<String> = Vec::new();
    let amb_file_reader = read_file(config_param.amb_file_path());
    let amb_file_delimeter = config_param.amb_file_delimeter();
    for (line_num, lines) in amb_file_reader.lines().enumerate().skip(1) {
        let line = extract_lines(line_num, lines, config_param.amb_file_path());
        let fields: Vec<&str> = line.split(amb_file_delimeter).collect();
        if fields.len() >= 8 {
            let dr_bal = fields[2].parse().unwrap_or(DEFAULT_FLOAT);
            if dr_bal != 0.0 {
                amb_data.push(fields[0].to_string());
            }
        }
    }

    let stamper_reader = read_file(config_param.stamper_file_path());
    amb_data.sort();
    for (line_num, lines) in stamper_reader.lines().enumerate() {
        let line = extract_lines(line_num, lines, config_param.stamper_file_path());
        let fields: Vec<&str> = line.split('|').collect();
        tot_rec += 1;
        if let Ok(index) = amb_data.binary_search(&fields[0].to_string()) {
            amb_data.remove(index);
        }
    }

    let ed_tm_read = SystemTime::now();
    let duration = ed_tm_read
        .duration_since(st_tm_read)
        .expect("Could not calculate total read process duration.");
    debug!(diag_log, "Read Process Total Duration: {:?}.", duration);

    let st_tm_writer = SystemTime::now();
    let mut op_writer = get_writer(config_param.output_file_path());
    for accs in amb_data.iter() {
        write!(op_writer, "{}\n", accs).expect("Error while writing output.");
    }

    let health_report = HealthReport::new(
        tot_rec,
        tot_rec - skp_rec,
        skp_rec,
        DEFAULT_FLOAT,
        DEFAULT_FLOAT,
        0,
    );
    log_info!(log, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_param.output_file_path());

    let ed_tm_writer = SystemTime::now();
    let duration = ed_tm_writer
        .duration_since(st_tm_writer)
        .expect("Could not calculate total duration for write process.");
    debug!(diag_log, "Writing OD, Total Duration: {:?}.", duration);
}
