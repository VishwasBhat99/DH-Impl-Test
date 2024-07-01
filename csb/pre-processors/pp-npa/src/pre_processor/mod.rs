use self::derive_fields::*;
use self::io::*;
use self::structs::npa::*;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use slog::Logger;
use statics::*;
use std::time::SystemTime;

mod derive_fields;
mod io;
mod structs;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let st_tm_read = SystemTime::now();
    let mut op_line: String = String::new();
    let mut tot_rec = DEFAULT_INT;
    let skp_rec = DEFAULT_INT;

    let mut tot_amt = 0.0;

    let mut npa_reader = read_file(config_param.npa_consolidated());
    for (line_num, lines) in npa_reader.deserialize().enumerate().skip(1) {
        tot_rec += 1;
        let npa_input: NPAInput =
            extract_lines(line_num, lines, config_param.npa_consolidated(), log);
        op_line.push_str(&get_op_line(
            npa_input,
            config_param.base_ccy(),
            &mut tot_amt,
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
    debug!(diag_log, "Writing NPA, Total Duration: {:?}.", duration);
}
