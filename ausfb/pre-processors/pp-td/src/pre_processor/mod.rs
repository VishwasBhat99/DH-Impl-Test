extern crate csv;
extern crate serde;
use self::csv::ReaderBuilder;
use self::derive_fields::get_op_line;
use self::input_account::InputAccount;
use self::recon::ReconKey;
use configuration_parameters::ConfigurationParameters;
use macros;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::io::BufWriter;
use std::time::SystemTime;
mod derive_fields;
mod input_account;
mod recon;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let output_file = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{} Cannot read output file path", error),
    };
    let mut recon_writer = match buf_file_wrtr(config_param.rec_output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{}", error),
    };
    let mut recon_map: HashMap<ReconKey, f64> = HashMap::new();
    let asondate = config_param.as_on_date().format("%d-%m-%Y");
    let start_derive_timer = SystemTime::now();
    let mut op_line: String = String::new();
    let mut tot_acc_encntrd: i64 = 0;
    let mut writer = BufWriter::new(output_file);
    let mut reader = match ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b',')
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
    for (line_num, lines) in reader.deserialize().enumerate().skip(1) {
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
        tot_acc_encntrd += 1;
        let prd_gl = &input_account.regular_deposit_gl;
        let int_gl = &input_account.compounded_gl;
        let ccy = &input_account.currency;
        let prd_amt = &input_account
            .current_principal_balance
            .parse::<f64>()
            .unwrap_or(0.0);
        let int_amt = &input_account
            .compounded_interest
            .parse::<f64>()
            .unwrap_or(0.0);
        let temp_string = get_op_line(&input_account, config_param.as_on_date);
        op_line.push_str(temp_string.as_str());
        op_line.push('\n');

        let prd_recon_key = ReconKey::new(ccy.to_string(), "PRDGL".to_string(), prd_gl.to_string());
        let int_recon_key = ReconKey::new(ccy.to_string(), "INTGL".to_string(), int_gl.to_string());

        recon_map
            .entry(prd_recon_key)
            .and_modify(|amt| *amt += prd_amt)
            .or_insert(*prd_amt);
        recon_map
            .entry(int_recon_key)
            .and_modify(|amt| *amt += int_amt)
            .or_insert(*int_amt);
    }

    let mut recon_output_line = String::new();
    for (key, value) in recon_map {
        let op = format!(
            "{}|{}|{}|{}|{}|{}",
            asondate,
            config_param.source_name(),
            key.gl_code,
            key.gl_type,
            value,
            key.currency,
        );
        recon_output_line.push_str(&op[..]);
        recon_output_line.push_str("\n");
    }
    let end_derive_timer = SystemTime::now();
    let duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total derive process duration.");
    debug!(diag_log, "Derive Process Total Duration: {:?}.", duration);
    let start_writer_time = SystemTime::now();
    match writer.write_all(op_line.as_bytes()) {
        Ok(val) => val,
        Err(error) => {
            panic!("Error writing processed data: {:?}", error);
        }
    }

    match recon_writer.write_all(recon_output_line.as_bytes()) {
        Ok(_val) => {}
        Err(error) => {
            panic!("Cannot generate reconciliation report file: {:?}", error);
        }
    }

    let end_writer_time = SystemTime::now();
    let duration = end_writer_time
        .duration_since(start_writer_time)
        .expect("Could not calculate total write process duration.");
    info!(diag_log, "Write Process Total Duration: {:?}.", duration);
}
