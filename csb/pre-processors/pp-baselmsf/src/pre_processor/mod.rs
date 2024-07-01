use self::derive_fields::*;
use self::io::*;
use self::structs::{param_download::*, percent_values::*, values_download::*};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use slog::Logger;
use statics::*;
use std::default::Default;
use std::time::SystemTime;

mod derive_fields;
mod io;
mod structs;

pub fn process(config_param: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let st_tm_read = SystemTime::now();
    let mut op_line: String = String::new();
    let mut tot_rec = DEFAULT_INT;
    let skp_rec = DEFAULT_INT;
    let mut msf_percent = DEFAULT_FLOAT;
    let mut slr_ndtl = DEFAULT_FLOAT;
    let mut percentage = DEFAULT_FLOAT;
    let mut tot_amt = DEFAULT_FLOAT;
    let mut input_reader = read_file(config_param.param_file_path());
    for (line_num, lines) in input_reader.deserialize().enumerate().skip(2) {
        let param_download: ParamDownloadInput =
            extract_lines(line_num, lines, config_param.param_file_path(), log);
        tot_rec += 1;
        msf_percent = param_download
            .param_value
            .parse::<f64>()
            .unwrap_or(DEFAULT_FLOAT);

        break;
    }
    input_reader = read_file(config_param.values_file_path());
    for (line_num, lines) in input_reader.deserialize().enumerate().skip(2) {
        let values_download: ValuesDownloadInput =
            extract_lines(line_num, lines, config_param.values_file_path(), log);
        tot_rec += 1;
        slr_ndtl = values_download
            .slr_ndtl
            .parse::<f64>()
            .unwrap_or(DEFAULT_FLOAT);
        tot_amt += slr_ndtl;
        break;
    }
    input_reader = read_file(config_param.percent_file_path());
    for (line_num, lines) in input_reader.deserialize().enumerate().skip(2) {
        let percent_download: PercentInput =
            extract_lines(line_num, lines, config_param.percent_file_path(), log);
        percentage = percent_download
            .percent
            .parse::<f64>()
            .unwrap_or(DEFAULT_FLOAT);
        break;
    }
    op_line.push_str(&get_op_line(
        &config_param,
        msf_percent,
        slr_ndtl,
        percentage,
    ));

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
    debug!(diag_log, "Writing LCBG, Total Duration: {:?}.", duration);
}
