use self::account::OutputData;
use self::io::*;
use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use crate::process::account_field_names::AccFieldNames;
use health_report::HealthReport;
use sdb_dyn_proto_rdr::reader;
use slog::Logger;
use std::collections::HashMap;
use std::io::Write;
use std::time::SystemTime;

mod account;
mod account_field_names;
pub mod config;
mod io;

pub fn process(config_params: ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let start_time = SystemTime::now();
    let mut output_path = String::new();
    output_path.push_str(config_params.output_file_path());
    output_path.push_str(".txt");
    let mut output_file = get_writer(&output_path);

    let mut acc_enc = 0;
    let mut acc_succ = 0;
    let mut ip_amt = 0.0;
    let mut op_amt = 0.0;
    let mut output_map: HashMap<String, OutputData> = HashMap::new();
    let files_config = config::get_files(config_params.config_file_path());

    for file in files_config.files {
        let keys = AccFieldNames::new_from_path(&file.required_fields_file_path);
        let input_file_path = get_file_path(file.input_file_path, *config_params.as_on_date());
        let mut account_reader =
            reader::Reader::new_at_path(&file.metadata_file_path, &input_file_path);

        for account in account_reader.iter() {
            let cust_type = account
                .get_string_for_key(&keys.basel_class)
                .unwrap_or(&"NA".to_string())
                .to_string();
            acc_enc += 1;
            let output_val = OutputData::new(
                account
                    .get_string_for_key(&keys.curr)
                    .unwrap_or(&"INR".to_string())
                    .to_string(),
                account
                    .get_f64_for_key(&keys.amt)
                    .unwrap_or(0.0),
            );
            output_map
                .entry(cust_type.clone())
                .and_modify(|data| data.append_data(output_val.clone()))
                .or_insert(output_val);
            acc_succ += 1;
            ip_amt += account.get_f64_for_key(&keys.amt).unwrap_or(0.0);
        }
    }
    for (key, data) in output_map.drain() {
        op_amt += data.amt;
        write!(
            output_file,
            "{}|{}|{}|{}|{}\n",
            &config_params.as_on_date(),
            &config_params.country_code(),
            data.curr,
            key,
            data.amt
        )
        .expect("Unable to write summary file.");
    }
    let health_report = HealthReport::new(acc_enc, acc_succ, acc_enc - acc_succ, ip_amt, op_amt, 0);
    health_report.gen_health_rpt(&config_params.output_file_path());
    let total_duration = print_return_time_since!(start_time);
    log_info!(logger, "Total time for aggregation: {:?}", total_duration);
}
