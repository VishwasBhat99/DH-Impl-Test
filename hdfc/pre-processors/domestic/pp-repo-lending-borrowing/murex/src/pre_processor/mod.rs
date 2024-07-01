use self::appender::get_op_line;
use self::input_account::InputAccount;
use self::manual_handler::remove_comma;
use self::reconcilation::ReconKey;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::SystemTime;
use rbdate::NaiveDate;

mod appender;
mod input_account;
mod manual_handler;
mod reconcilation;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let st_tm_read = SystemTime::now();
    let mut op_line: String = String::new();
    let mut ttl_rec = 0;
    let mut skp_rec = 0;
    let mut recon: HashMap<ReconKey, f64> = HashMap::new();
    let input_file = match new_buf_rdr(config_param.input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.input_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };
    let mut tot_amt = 0.0;
    for (line_num, lines) in input_file.lines().enumerate() {
        let line = match lines {
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
        ttl_rec += 1;
        let fields: Vec<&str> = line.split("~#~").collect();
        if fields.len() != 43 {
            skp_rec += 1;
            continue;
        }
        let input_account: InputAccount = InputAccount::new(&fields);
        if remove_comma(&input_account.deal_no).parse::<i64>().is_err() {
            continue;
        }
        if input_account.entity != config_param.entity() {
            skp_rec += 1;
            continue;
        }
        let amt = remove_comma(&input_account.settle_amt_1st_leg)
            .parse::<f64>()
            .unwrap_or(0.0);

        let recon_key = ReconKey::new(
            input_account.currency.to_string(),
            "MUR-REPO-LEND-BORR".to_string(),
            "gl".to_string(), //missing gl
        );
        recon
            .entry(recon_key)
            .and_modify(|val| *val += amt)
            .or_insert(amt);

        tot_amt += amt;
        let val_dt = match NaiveDate::parse_from_str(&input_account.value_dt_1st_leg, "%d %b %Y") {
            Ok(dt) => dt.format("%Y-%m-%d").to_string(),
            Err(_) => String::new(),
        };
        let as_on = *config_param.as_on_date();

        // to exclude the future value dated deals
        if val_dt > as_on.to_string() {
            skp_rec += 1;
            continue;
        }

        op_line.push_str(&get_op_line(
            &input_account,
            *config_param.as_on_date(),
            log,
        ));
    }

    let ed_tm_read = SystemTime::now();
    let duration = ed_tm_read
        .duration_since(st_tm_read)
        .expect("Could not calculate total read process duration.");
    debug!(diag_log, "Read Process Total Duration: {:?}.", duration);

    let st_tm_writer = SystemTime::now();

    let mut output_writer = match buf_file_wrtr(config_param.output_file_path(), None) {
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

    match output_writer.write_all(op_line.as_bytes()) {
        Ok(_) => println!("Successfully processed all accounts."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}` : {}",
            config_param.output_file_path(),
            error
        ),
    }

    let mut recon_writer = match buf_file_wrtr(config_param.rec_output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create reconcilation file `{}` on location `{}` : {}",
            config_param.rec_output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    let mut recon_op_line = String::new();
    for (key, value) in recon {
        let op = format!(
            "{}|{}|{}|{}|{}|{}",
            config_param.as_on_date().format("%d-%m-%Y"),
            "p_code_018_total",
            key.gl_type,
            key.gl_code,
            value,
            key.currency,
        );
        recon_op_line.push_str(&op[..]);
        recon_op_line.push_str("\n");
    }
    match recon_writer.write_all(recon_op_line.as_bytes()) {
        Ok(_) => println!("Successfully written reconcilation file."),
        Err(error) => panic!(
            "Unable to write reconcilation lines to file `{}`: {}.",
            config_param.rec_output_file_path(),
            error
        ),
    };
    let health_report = HealthReport::new(ttl_rec, ttl_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    health_report.gen_health_rpt(&config_param.output_file_path());

    let ed_tm_writer = SystemTime::now();
    let duration = ed_tm_writer
        .duration_since(st_tm_writer)
        .expect("Could not calculate total duration for write process.");
    debug!(
        diag_log,
        "Writing Repo Borrowings and Lendings Records, Total Duration: {:?}.", duration
    );

    let report_string = format!(
        "Accounts Encountered: {}\n\
         Total accounts processed: {:.2} \n\
         Total accounts skipped: {:.2}",
        ttl_rec,
        ttl_rec - skp_rec,
        skp_rec,
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);
}
