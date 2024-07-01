use self::derive_fields::*;
use self::io::*;
use self::structs::input_account::InputAccount;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use slog::Logger;
use statics::*;
use std::default::Default;
use std::io::prelude::*;
use std::time::SystemTime;

mod derive_fields;
mod io;
mod structs;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let st_tm_read = SystemTime::now();
    let mut op_line: String = String::new();
    let mut tot_rec = DEFAULT_INT;
    let skp_rec = DEFAULT_INT;

    let overdue_reader = read_file_del(config_param.over_rate_file_path());
    let mut int_rt = String::new();
    for (line_num, lines) in overdue_reader.lines().enumerate() {
        let i_r = extract_lines_del(line_num, lines, config_param.over_rate_file_path());
        int_rt = i_r;
        break;
    }

    let mut input_reader = read_file(config_param.input_file_path());
    let mut tot_amt = 0.0;
    for (line_num, lines) in input_reader.deserialize().enumerate().skip(1) {
        let mut input_account: InputAccount =
            extract_lines(line_num, lines, config_param.input_file_path(), log);
        tot_rec += 1;

        let amt = input_account.bal_os.parse().unwrap_or(DEFAULT_FLOAT);
        tot_amt += amt;

        op_line.push_str(&get_op_line(
            &mut input_account,
            *config_param.as_on_date(),
            &int_rt,
            log,
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
    debug!(
        diag_log,
        "Writing TD Interest, Total Duration: {:?}.", duration
    );
}
