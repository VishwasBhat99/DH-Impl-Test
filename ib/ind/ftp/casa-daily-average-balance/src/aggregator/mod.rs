use self::io::*;
use self::util::*;
use aggregator::account_field_names::AccFieldNames;
use aggregator::input_account::DailyData;
// use base64::{decode, encode};
use chrono::Datelike;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use compressed_string::ComprString;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::account_with_cfs::get_field_value;
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
    let mut method_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );
    let mut op_line = String::new();
    let mut reader = String::new();
    let previous_day_file_path = output_path.to_string();
    let path_exists = Path::new(&previous_day_file_path).exists();
    let as_on_date = *config_params.as_on_date();
    let mut map_of_acc_data: HashMap<String, Vec<&str>> = HashMap::new();
    let mut data = Vec::new();

    let start_time = SystemTime::now();

    

    if path_exists && !is_first_day_of_month(as_on_date) {
        println!("Previous Data Found and As on Date is not first day of month!");
        reader = fs::read_to_string(previous_day_file_path)
            .expect("Could Not Read previous_day_file_path");
        for line in reader.lines() {
            let derived_field = line.split('|').collect::<Vec<&str>>();
            map_of_acc_data.insert(derived_field[0].to_string(),derived_field);
        }
        println!("Previous Data stored!");
    }
    let total_duration = print_return_time_since!(start_time);
    let mut op_writer = get_writer(&output_path);
    println!("Reading Current Day Data!");

    if is_first_day_of_month(as_on_date) {
        for account in account_reader.iter() {
            tot_acc_encntrd += 1;
            let mut acc_data = DailyData::new();
            acc_data.acc_num = get_field_value(
             &account, &method_reader, keys.account_number.to_string()).unwrap();
            acc_data.out_bal = get_field_value(
                &account, &method_reader, keys.out_bal.to_string()).unwrap();
            acc_data.int_rt = get_field_value(
                &account, &method_reader, keys.int_rt.to_string()).unwrap();
            acc_data.int_posted = get_field_value(
                &account, &method_reader, keys.int_posted.to_string()).unwrap();
            acc_data.curr_status = get_field_value(
                &account, &method_reader, keys.curr_status.to_string()).unwrap();
            acc_data.class = get_field_value(
                &account, &method_reader, keys.class.to_string()).unwrap();
            acc_data.acc_cls_dt = get_field_value(
                &account, &method_reader, keys.acc_cls_dt.to_string()).unwrap();
            acc_data.gl_cd = get_field_value(
                &account, &method_reader, keys.gl_cd.to_string()).unwrap();
            first_occurence_of_account(acc_data, as_on_date, &mut op_writer);
        }
    }
    else {
        for account in account_reader.iter() {
            tot_acc_encntrd += 1;
            let mut acc_data = DailyData::new();
            acc_data.acc_num = get_field_value(
             &account, &method_reader, keys.account_number.to_string()).unwrap();
            acc_data.out_bal = get_field_value(
                &account, &method_reader, keys.out_bal.to_string()).unwrap();
            acc_data.int_rt = get_field_value(
                &account, &method_reader, keys.int_rt.to_string()).unwrap();
            acc_data.int_posted = get_field_value(
                &account, &method_reader, keys.int_posted.to_string()).unwrap();
            acc_data.curr_status = get_field_value(
                &account, &method_reader, keys.curr_status.to_string()).unwrap();
            acc_data.class = get_field_value(
                &account, &method_reader, keys.class.to_string()).unwrap();
            acc_data.acc_cls_dt = get_field_value(
                &account, &method_reader, keys.acc_cls_dt.to_string()).unwrap();
            acc_data.gl_cd = get_field_value(
                &account, &method_reader, keys.gl_cd.to_string()).unwrap();
            let cf_acc_num = acc_data.acc_num.to_owned();
            if map_of_acc_data.contains_key(&cf_acc_num) {
                data = map_of_acc_data.get(&cf_acc_num).unwrap().to_owned();
                previous_occurence_of_account(acc_data, data, as_on_date,&mut op_writer);
                map_of_acc_data.remove(&cf_acc_num);
            } else {
                first_occurence_of_account(acc_data, as_on_date, &mut op_writer);
            }
        }
        for (key, _val) in map_of_acc_data.iter() {
            let mut data = map_of_acc_data.get(key).unwrap().clone();
            data[4] = "0.0";
            let op_str = format!("{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",data[0],data[1],data[2],data[3],data[4],data[5],data[6],data[7],data[8],data[9]);
            writeln!(op_writer, "{}", op_str).expect("Unable to generate summary file.");
        }
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
