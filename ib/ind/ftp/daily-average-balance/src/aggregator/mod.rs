use self::io::*;
use self::util::*;
use aggregator::account_field_names::AccFieldNames;
use aggregator::input_account::DailyData;
use base64::{decode, encode};
use chrono::Datelike;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use compressed_string::ComprString;
use sdb_dyn_proto_rdr::reader;
use slog::Logger;
use hashbrown::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;

mod account_field_names;
mod input_account;
mod io;
mod util;

pub fn generatesummary(
    config_params: ConfigurationParameters,
    logger: &Logger,
    _diag_logger: &Logger,
) {
    let mut tot_acc_encntrd = 0;
    let skp_acc = 0;
    let ttl_amt: f64 = 0.0;
    let keys = AccFieldNames::new_from_path(config_params.known_fields_file_path());
    let output_path = config_params.output_file_path().to_string();
    let mut account_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );

    let mut op_line = String::new();
    let mut reader = String::new();
    let previous_day_file_path = output_path.to_string();
    let path_exists = Path::new(&previous_day_file_path).exists();
    let as_on_date = *config_params.as_on_date();
    let mut map_of_acc_data: HashMap<String, ComprString> = HashMap::new();
    let mut data = String::new();

    let start_time = SystemTime::now();

    

    if path_exists && !is_first_day_of_month(as_on_date) {
        println!("Previous Data Found and As on Date is not first day of month!");
        reader = fs::read_to_string(previous_day_file_path)
            .expect("Could Not Read previous_day_file_path");
        for line in reader.lines() {
            let derived_fields = line.split('|').collect::<Vec<&str>>();
            let derived_data = ComprString::new(line);
            map_of_acc_data.insert(derived_fields[0].to_string(), derived_data);
        }
        println!("Previous Data stored!");
    }
    let mut op_writer = get_writer(&output_path);
    println!("Reading Current Day Data!");
    for account in account_reader.iter() {
        tot_acc_encntrd += 1;
        let cf_acc_num = account
            .get_string_for_key(&keys.account_number)
            .expect("Cannot get 'account number` field.")
            .to_string();
        let out_bal = account
            .get_f64_for_key(&keys.out_bal)
            .expect("Cannot get 'out_bal` field.")
            .to_string();
        let int_rt = account
            .get_f64_for_key(&keys.int_rt)
            .expect("Cannot get 'accr_int_amt` field.")
            .to_string();
        let int_posted = account
            .get_f64_for_key(&keys.int_posted)
            .expect("Cannot get 'int_rate` field.")
            .to_string();
        let curr_status = account
            .get_string_for_key(&keys.curr_status)
            .unwrap_or(&"".to_string())
            .to_string();
        let class = account
            .get_string_for_key(&keys.class)
            .unwrap_or(&"".to_string())
            .to_string();
        let acc_cls_dt = account
            .get_i64_for_key(&keys.acc_cls_dt)
            .unwrap_or(2)
            .to_string();
        let gl_cd = account
            .get_string_for_key(&keys.gl_cd)
            .unwrap_or(&"".to_string())
            .to_string();

        let mut acc_data = DailyData::new();
        acc_data.acc_num = cf_acc_num.to_owned();
        acc_data.out_bal = out_bal.to_owned();
        acc_data.int_rt = int_rt.to_owned();
        acc_data.int_posted = int_posted.to_owned();
        acc_data.curr_status = curr_status.to_owned();
        acc_data.class = class.to_owned();
        acc_data.acc_cls_dt = acc_cls_dt.to_owned();
        acc_data.gl_cd = gl_cd.to_owned();
        if is_first_day_of_month(as_on_date) {
            op_line = first_occurence_of_account(acc_data, as_on_date);
            writeln!(op_writer, "{}", op_line).expect("Unable to generate summary file.");
        } else {
            if map_of_acc_data.contains_key(&cf_acc_num) {
                data = map_of_acc_data.get(&cf_acc_num).unwrap().to_string();
                op_line = previous_occurence_of_account(acc_data, data, as_on_date);
                writeln!(op_writer, "{}", op_line).expect("Unable to generate summary file.");
                map_of_acc_data.remove(&cf_acc_num);
            } else {
                op_line = first_occurence_of_account(acc_data, as_on_date);
                writeln!(op_writer, "{}", op_line).expect("Unable to generate summary file.");
            }
        }
    }
    for (key, _val) in map_of_acc_data.iter() {
        let encoded_data = map_of_acc_data.get(key).unwrap().to_string();
        writeln!(op_writer, "{}", encoded_data).expect("Unable to generate summary file.");
    }
    let total_duration = print_return_time_since!(start_time);
    log_info!(logger, "Total time for aggregation: {:?}", total_duration);
    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - skp_acc,
        skp_acc,
        ttl_amt,
        ttl_amt,
        0,
    );
    health_report.gen_health_rpt(&config_params.output_file_path());
}
