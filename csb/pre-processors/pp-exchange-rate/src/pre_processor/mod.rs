use self::derive_ex_rt_file::{append_ccy, get_ex_rt_lines};
use self::derive_fields::{append_op_line, get_op_line};
use self::io::*;
use self::structs::CurrencyConverter;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use slog::Logger;
use std::default::Default;
use std::time::SystemTime;

mod derive_ex_rt_file;
mod derive_fields;
mod io;
mod structs;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_derive_timer = SystemTime::now();
    let mut output_line: String = String::new();
    let mut tot_lines: i64 = 0;
    let mut tot_suc_lines: i64 = 0;
    let mut ex_rt_lines: String = String::new();

    let mut input_reader = read_file(config_param.input_file_path());
    for (line_num, lines) in input_reader.deserialize().enumerate().skip(1) {
        let mut ccy: CurrencyConverter =
            extract_lines(line_num, lines, config_param.input_file_path(), log);
        tot_lines += 1;
        if ccy.ex_rt == 0.0 {
            log_error!(
                log,
                "Invalid Exchange Rate format `{:?}` at line number: `{}`.",
                ccy,
                line_num + 1
            );
            continue;
        }
        if ccy.source == config_param.ccy() && ccy.target == config_param.ccy() {
            log_error!(
                log,
                "Duplicate {} currency found : `{:?}` at line number: `{}`.",
                config_param.ccy(),
                ccy,
                line_num + 1
            );
            continue;
        }
        output_line.push_str(&get_op_line(
            &mut ccy,
            *config_param.as_on_date(),
            &config_param,
        ));
        ex_rt_lines.push_str(&get_ex_rt_lines(&ccy, &config_param));
        tot_suc_lines += 1;
    }
    let end_derive_timer = SystemTime::now();
    let duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total derive process duration.");
    debug!(diag_log, "Derive Process Total Duration: {:?}.", duration);

    append_op_line(&mut output_line, &config_param);

    let start_write_timer = SystemTime::now();
    let mut writer = get_writer(config_param.output_file_path());
    output_writer(&mut writer, output_line, config_param.output_file_path());

    append_ccy(&mut ex_rt_lines, &config_param);

    let mut ex_writer = get_writer(config_param.ex_rt_file_path());
    output_writer(&mut ex_writer, ex_rt_lines, config_param.ex_rt_file_path());

    let end_write_timer = SystemTime::now();
    let duration = end_write_timer
        .duration_since(start_write_timer)
        .expect("Could not calculate total duration for writing pre-processed output and reconcilation files.");
    debug!(
        diag_log,
        "Writing Records and Reconcilation File, Total Duration: {:?}.", duration
    );

    let report_string = format!(
        "Total lines encountered: {}\n\
         Lines proccessed suceessfully: {}\n\
         Lines failed to process: {}",
        tot_lines,
        tot_suc_lines,
        tot_lines - tot_suc_lines,
    );
    let health_report = HealthReport::new(
        tot_lines,
        tot_suc_lines,
        tot_lines - tot_suc_lines,
        0.0,
        0.0,
        0,
    );
    health_report.gen_health_rpt(&config_param.output_file_path());
    log_info!(log, "{}", report_string);
    println!("{}", report_string);
}
