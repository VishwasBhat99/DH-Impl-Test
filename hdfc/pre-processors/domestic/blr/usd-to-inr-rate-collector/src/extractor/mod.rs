use self::get_dates::Dates;
use self::io::*;
use self::reader::get_data;
use self::structs::exchange_rate::*;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::NaiveDate;
use slog::Logger;
use statics::*;
use std::collections::HashMap;
use std::default::Default;
use std::io::Write;
use std::time::SystemTime;

mod get_dates;
mod io;
mod reader;
mod structs;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let st_tm_read = SystemTime::now();
    let tot_rec = DEFAULT_INT;
    let skp_rec = DEFAULT_INT;

    let dates = Dates::new(config_param.as_on_date());
    let mut rates_data: HashMap<NaiveDate, f64> = HashMap::new();
    get_data(&dates, &config_param, &mut rates_data, log);

    let ed_tm_read = SystemTime::now();
    let duration = ed_tm_read
        .duration_since(st_tm_read)
        .expect("Could not calculate total read process duration.");
    debug!(diag_log, "Read Process Total Duration: {:?}.", duration);

    let st_tm_writer = SystemTime::now();
    let mut op_writer = get_writer(config_param.output_file());

    for (date, rates) in rates_data.drain() {
        write!(op_writer, "{}|{}\n", date.format("%d-%m-%Y"), rates)
            .expect("Error while writing exchange_rate file.");
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
    health_report.gen_health_rpt(&config_param.output_file());

    let ed_tm_writer = SystemTime::now();
    let duration = ed_tm_writer
        .duration_since(st_tm_writer)
        .expect("Could not calculate total duration for write process.");
    debug!(
        diag_log,
        "Writing USD to INR Rate Collector, Total Duration: {:?}.", duration
    );
}
