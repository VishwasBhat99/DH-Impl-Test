extern crate csv;
extern crate serde;
use self::csv::ReaderBuilder;
use self::input_account::InputAccount;
use configuration_parameters::ConfigurationParameters;
use macros;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env::current_dir;
use std::io::prelude::*;
use std::io::BufWriter;
use std::time::SystemTime;
mod input_account;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let output_file = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{}", error),
    };
    let start_derive_timer = SystemTime::now();
    let mut op_line: String = String::new();
    let mut tot_acc_encntrd: i64 = 0;
    let mut writer = BufWriter::new(output_file);
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
    let mut tenor_map: HashMap<String, f64> = HashMap::new();
    let mut prin_out_map: HashMap<String, f64> = HashMap::new();
    let mut inp_vec: Vec<InputAccount> = Vec::new();
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
        let bal_term = input_account
            .balance_term
            .parse::<f64>()
            .unwrap_or(0.0)
            .to_owned();
        let prin_out_amt = input_account
            .principal_ouststanding_amount
            .parse::<f64>()
            .unwrap_or(0.0)
            .to_owned();
        let weighted_tenor = (bal_term * prin_out_amt).to_owned();
        tot_acc_encntrd += 1;
        if input_account.maturity_date != "NULL".to_string() {
            tenor_map
                .entry(input_account.productcode.to_owned())
                .and_modify(|amt| *amt += weighted_tenor)
                .or_insert(weighted_tenor);
            prin_out_map
                .entry(input_account.productcode.to_owned())
                .and_modify(|amt| *amt += prin_out_amt)
                .or_insert(prin_out_amt);
        }
        inp_vec.push(input_account);
    }
    for inp_acc in inp_vec {
        let prin_out_amt = inp_acc
            .principal_ouststanding_amount
            .parse::<f64>()
            .unwrap_or(0.0)
            .to_owned();
        let mat_dt = inp_acc.maturity_date.to_owned();
        let prod_cd = inp_acc.productcode.to_owned();
        op_line.push_str(InputAccount::acc_to_string(inp_acc).as_str());
        op_line.push('|');
        if tenor_map.contains_key(&prod_cd) {
            let weight_tenor = tenor_map.get(&prod_cd).unwrap();
            let sum_prin_out = prin_out_map.get(&prod_cd).unwrap();
            let avg_tenor = weight_tenor / sum_prin_out;
            let factor = prin_out_amt / avg_tenor;
            op_line.push_str(weight_tenor.to_string().as_str());
            op_line.push('|');
            op_line.push_str(sum_prin_out.to_string().as_str());
            op_line.push('|');
            op_line.push_str(avg_tenor.to_string().as_str());
            op_line.push('|');
            op_line.push_str(factor.to_string().as_str());
        } else {
            op_line.push_str("0.0|0.0|0.0|0.0");
        }
        op_line.push('\n');
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
    let end_writer_time = SystemTime::now();
    let duration = end_writer_time
        .duration_since(start_writer_time)
        .expect("Could not calculate total write process duration.");
    info!(diag_log, "Write Process Total Duration: {:?}.", duration);
}
