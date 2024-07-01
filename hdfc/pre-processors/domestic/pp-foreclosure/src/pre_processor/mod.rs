use self::manual_handler::remove_comma;
use self::{derive_fields::get_op_line, structs::ForeClosureAmounts, structs::InputAccount};
use configuration_parameters::ConfigurationParameters;
use csv::ReaderBuilder;
use health_report::HealthReport;
use macros;
use rbdate::NaiveDate;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use statics::*;
use std::{
    collections::HashMap, default::Default, env::current_dir, io::prelude::*, time::SystemTime,
};

mod derive_fields;
mod manual_handler;
mod structs;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let st_tm_read = SystemTime::now();
    let mut op_line_fc_pos: String = String::new();
    let mut tot_rec = DEFAULT_INT;
    let mut is_header: bool = true;
    let mut skp_rec = DEFAULT_INT;
    let mut fore_clsr_aggr: HashMap<NaiveDate, ForeClosureAmounts> = HashMap::new();
    let mut reader = match ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path(config_param.input_file_path())
    {
        Ok(read) => read,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut tot_amt = DEFAULT_FLOAT;
    for (line_num, lines) in reader.deserialize().enumerate() {
        let input_account: InputAccount = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    log,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.input_file_path(),
                    line_num + 1,
                    error
                );
                Default::default()
            }
        };
        tot_rec += 1;
        if is_header {
            is_header = false;
            skp_rec += 1;
            continue;
        }
        let amt = remove_comma(&input_account.fc_pos)
            .parse::<f64>()
            .unwrap_or(DEFAULT_FLOAT);
        tot_amt += amt;
        get_op_line(&input_account, &mut fore_clsr_aggr);
    }

    let ed_tm_read = SystemTime::now();
    let duration = ed_tm_read
        .duration_since(st_tm_read)
        .expect("Could not calculate total read process duration.");
    debug!(diag_log, "Read Process Total Duration: {:?}.", duration);

    let st_tm_writer = SystemTime::now();
    for (date, amts) in fore_clsr_aggr.drain() {
        op_line_fc_pos.push_str(&format!(
            "{}|{}|{}|DIM1|DIM2|DIM3|DIM4|DIM5|{}|0.0\n",
            config_param.llg_code(),
            date.format("%d-%m-%Y"),
            config_param.currency,
            amts.fc_pos
        ));
    }

    let mut op_writer_fc_pos = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create file `{}` on location `{}` : {}",
            config_param.output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match op_writer_fc_pos.write_all(op_line_fc_pos.as_bytes()) {
        Ok(_) => println!("Successfully processed all accounts."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}` : {}",
            config_param.output_file_path(),
            error
        ),
    }
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
        "Writing Repo Borrowings and Lendings Records, Total Duration: {:?}.", duration
    );
}
