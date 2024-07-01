use self::derive_fields::*;
use self::io::*;
use self::structs::{aggregated::*, input_records::*};
use self::writer::*;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use slog::Logger;
use statics::*;
use std::default::Default;
use std::io::Write;
use std::time::SystemTime;

mod derive_fields;
mod io;
mod structs;
mod writer;

pub fn aggregate(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let st_tm_read = SystemTime::now();
    let mut tot_rec = DEFAULT_INT;
    let skp_rec = DEFAULT_INT;

    let mut input_reader = read_file(config_param.input_file_path());
    let mut aggr_map: AggregatedMap = AggregatedMap::new();
    let mut tot_amt = 0.0;
    for (line_num, lines) in input_reader.deserialize().enumerate() {
        let input_record: InputRecord =
            extract_lines(line_num, lines, config_param.input_file_path(), log);
        tot_rec += 1;
        tot_amt += input_record.bal;
        get_aggregated_pair(&mut aggr_map, &input_record, &config_param);
    }

    let ed_tm_read = SystemTime::now();
    let duration = ed_tm_read
        .duration_since(st_tm_read)
        .expect("Could not calculate total read process duration.");
    debug!(diag_log, "Read Process Total Duration: {:?}.", duration);

    let st_tm_writer = SystemTime::now();
    let mut op_writer = get_writer(config_param.output_file_path());
    write_output(&mut aggr_map,&mut op_writer);

    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    log_info!(log, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_param.output_file_path());

    let ed_tm_writer = SystemTime::now();
    let duration = ed_tm_writer
        .duration_since(st_tm_writer)
        .expect("Could not calculate total duration for write process.");
    debug!(
        diag_log,
        "Writing IA Recon Aggregator, Total Duration: {:?}.", duration
    );
}
