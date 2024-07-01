use aggregator::account_field_names::AccFieldNames;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::NaiveDate;
use sdb_dyn_proto_rdr::reader;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::BufWriter;
use std::time::SystemTime;

mod account_field_names;
pub mod config;

pub fn process(config_params: ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let start_time = SystemTime::now();
    let files_config = config::get_files(config_params.config_file_path());
    let mut output_path = String::new();
    output_path.push_str(config_params.output_file_path());
    let mut output_writer = match buf_file_wrtr(&output_path, None) {
        Ok(wrtr) => wrtr,
        Err(error) => {
            panic!(
                "Could not create file: `{}` at location `{}` : {:?}.",
                config_params.output_file_path(),
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    let mut acc_enc = 0;
    let mut acc_skip_op_line = String::new();

    let mut acc_skip_map: HashMap<String, String> = HashMap::new();
    let mut acc_skip_op_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(config_params.matured_accounts_file().clone())
        .unwrap();

    let ason = config_params.as_on_date();
    let ason = ason.format("%d-%m-%Y").to_string();
    let acc_skip_file = match new_buf_rdr(&config_params.matured_accounts_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not find matured accounts file: `{}`  : {}.",
            config_params.matured_accounts_file(),
            error
        ),
    };
    for (line_num, lines) in acc_skip_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.matured_accounts_file(),
                line_num + 1,
                error
            ),
        };
        acc_skip_op_line.push_str(&line);
        let fields: Vec<&str> = line.split("|").collect();
        // acc_no and mat_dt combined together is the key to lookup
        let key = fields[0].to_string() + "|" + fields[4];
        acc_skip_map.insert(key, fields[2].to_string());
    }
    for file in files_config.input_files {
        let keys = AccFieldNames::new_from_path(&file.req_fields_file_path);
        let mut input_file_reader =
            reader::Reader::new_at_path(&file.metadata_file_path, &file.input_file);
        for account in input_file_reader.iter() {
            let mat_dt = account.get_i64_for_key(&keys.mat_dt).unwrap_or(0);
            let mat_dt = naivedate_from_timestamp(mat_dt)
                .format("%d-%m-%Y")
                .to_string();
            let acc_no = account
                .get_string_for_key(&keys.acc_no)
                .unwrap_or(&"acc_no".to_string())
                .to_string();
            let acc_skip_lookup = acc_no.to_string() + "|" + &mat_dt;
            let mat_dt_vec: Vec<&str> = mat_dt.split('-').collect();
            let ason_vec: Vec<&str> = ason.split('-').collect();
            acc_enc += 1;

            // If matured account already exists with matching acc_no and mat_dt (or)
            // if the month and year of ason and mat_dt are not same skip the account.
            if acc_skip_map.contains_key(&acc_skip_lookup)
                || mat_dt_vec[1].to_string() + mat_dt_vec[2]
                    != ason_vec[1].to_string() + ason_vec[2]
            {
                continue;
            }

            let value_dt = account.get_i64_for_key(&keys.value_dt).unwrap();
            let value_dt = naivedate_from_timestamp(value_dt)
                .format("%d-%m-%Y")
                .to_string();
            let op_line = format!(
                "{}|{}|{}|{}|{}\n",
                &acc_no, &file.source, &ason, &value_dt, &mat_dt,
            );
            acc_skip_op_line.push_str(&op_line);
            let skip = acc_no.to_string() + "|" + &mat_dt;
            acc_skip_map.insert(skip, ason.to_string());

            write_data(&mut output_writer, op_line.to_string(), logger);
            acc_skip_op_file.write_all(op_line.as_bytes());
        }
    }

    println!("Total accounts encountered: {}", acc_enc);
    let total_duration = print_return_time_since!(start_time);
    log_info!(logger, "Total time for aggregation: {:?}", total_duration);

    pub fn write_data(writer: &mut BufWriter<File>, op: String, logger: &Logger) {
        let output_as_bytes = op.as_bytes();
        match writer.write(output_as_bytes) {
            Ok(_val) => {}
            Err(err) => {
                log_info!(logger, "Error writing to output file. Error: {}", err);
            }
        }
    }

    fn naivedate_from_timestamp(t: i64) -> NaiveDate {
        let naive_date_time = rbdate::NaiveDateTime::from_timestamp(t, 0);
        naive_date_time.date()
    }
}
