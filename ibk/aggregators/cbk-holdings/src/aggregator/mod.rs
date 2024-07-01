use self::bucket::Bucket;
use aggregator::account_field_names::AccFieldNames;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::NaiveDate;
use sdb_dyn_proto_rdr::reader::Reader;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::env;
use std::env::current_dir;
use std::fs::OpenOptions;
use std::io::{BufRead, Write};
use std::time::SystemTime;

mod account_field_names;
mod bucket;
pub mod config;

pub fn get_mis_reports(
    config_params: ConfigurationParameters,
    logger: &Logger,
    _diag_logger: &Logger,
) {
    let start_time = SystemTime::now();
    let files_config = config::get_files(config_params.config_file_path());
    let mut acc_enc = 0;
    for file in files_config.files {
        let mut output_file = match OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&file.output_file_path)
        {
            Ok(create) => create,
            Err(error) => {
                panic!(
                    "Could not create file: `{}` on location `{}` : {:?}.",
                    &file.output_file_path,
                    env::current_exe()
                        .expect("Unable to find current directory path!")
                        .display(),
                    error
                );
            }
        };
        let keys = AccFieldNames::new_from_path(&file.required_fields_file_path);
        let mut account_reader =
            Reader::new_at_path(&file.metadata_file_path, &file.input_file_path);

        // read common codes file
        let common_codes = match new_buf_rdr(&file.common_codes_file) {
            Ok(file) => file,
            Err(error) => panic!(
                "Could not found file `{}` on location `{}` : {}.",
                file.common_codes_file,
                current_dir()
                    .expect("Error while getting current directory path.")
                    .display(),
                error
            ),
        };
        let mut code_defn: HashMap<String, String> = HashMap::new();
        for (line_num, lines) in common_codes.lines().enumerate() {
            let line = match lines {
                Ok(line) => line,
                Err(error) => panic!(
                    "Unable to read file `{}` at line number: `{}` : {}",
                    file.common_codes_file,
                    line_num + 1,
                    error
                ),
            };
            let fields: Vec<&str> = line.split('|').collect();
            code_defn.insert(fields[1].trim().to_string(), fields[0].trim().to_string());
        }

        // read scheme defination file
        let scheme_defn_file = match new_buf_rdr(&file.scheme_defn_file) {
            Ok(file) => file,
            Err(error) => panic!(
                "Could not found file `{}` on location `{}` : {}.",
                file.scheme_defn_file,
                current_dir()
                    .expect("Error while getting current directory path.")
                    .display(),
                error
            ),
        };
        let mut scheme_defn: Vec<String> = Vec::new();
        for (line_num, lines) in scheme_defn_file.lines().enumerate() {
            let line = match lines {
                Ok(line) => line,
                Err(error) => panic!(
                    "Unable to read file `{}` at line number: `{}` : {}",
                    file.scheme_defn_file,
                    line_num + 1,
                    error
                ),
            };
            let fields: Vec<&str> = line.split('|').collect();
            scheme_defn.push(fields[0].trim().to_string());
        }

        // read bucket defination file
        let bkt_defn_file = match new_buf_rdr(&file.bkt_defn_file) {
            Ok(file) => file,
            Err(error) => panic!(
                "Could not found file `{}` on location `{}` : {}.",
                file.bkt_defn_file,
                current_dir()
                    .expect("Error while getting current directory path.")
                    .display(),
                error
            ),
        };

        let mut bkt_defn: HashMap<Bucket, String> = HashMap::new();
        for (line_num, lines) in bkt_defn_file.lines().enumerate() {
            let line = match lines {
                Ok(line) => line,
                Err(error) => panic!(
                    "Unable to read file `{}` at line number: `{}` : {}",
                    file.scheme_defn_file,
                    line_num + 1,
                    error
                ),
            };
            let fields: Vec<&str> = line.split('|').collect();
            let from_bkt_days = get_resi_days(&fields[5], fields[3].parse::<i64>().unwrap());
            let to_bkt_days = get_resi_days(&fields[6], fields[4].parse::<i64>().unwrap());

            let bucket = Bucket::new(from_bkt_days, to_bkt_days, fields[0].to_string());
            bkt_defn.insert(bucket, fields[1].to_string());
        }

        let report_id = file.report_id;

        for account in account_reader.iter() {
            let def_val = "NA".to_string();
            let def_dt = &"31-12-2099".to_string();
            let def_dim = &"99".to_string();
            acc_enc += 1;

            let deal_id = account
                .get_string_for_key(&keys.deal_id)
                .unwrap_or(&def_val);
            let amount_ccy = account.get_f64_for_key(&keys.amount_ccy).unwrap_or(0.0);
            let amount_hcy = account.get_f64_for_key(&keys.amount_hcy).unwrap_or(0.0);
            let src_yield = account.get_f64_for_key(&keys.src_yield).unwrap_or(0.0);
            let mat_dt = account
                .get_string_for_key(&keys.maturity_dt)
                .unwrap_or(&def_dt);
            let st_dt = account
                .get_string_for_key(&keys.start_date)
                .unwrap_or(&def_dt);
            let ccy_id = account.get_string_for_key(&keys.ccy_id).unwrap_or(&def_val);
            let code_type = account
                .get_string_for_key(&keys.code_type)
                .unwrap_or(&def_val);
            let dim1;
            let code_id = code_defn.get(code_type).unwrap_or(&"99".to_string()).parse::<i64>().unwrap();
            if code_id == 1 {
                dim1 = 1;
            }
            else {
                dim1 = 2;
            }

            let maturity_dt = NaiveDate::parse_from_str(mat_dt, "%d-%m-%Y").unwrap();
            let start_date = NaiveDate::parse_from_str(st_dt, "%d-%m-%Y").unwrap();
            for scheme_id in scheme_defn.iter() {
                let dim2 = scheme_id.to_string();
                let residual_days: i64 =
                    rbdate::num_days_start_to_end(start_date, maturity_dt) + 1; //get residual days
                let mut dim3 = def_dim.to_string();
                for (bucket, value) in bkt_defn.iter() {
                    let from_bkt: i64 = bucket.from_bkt_days;
                    let to_bkt: i64 = bucket.to_bkt_days;
                    if scheme_id.to_string() == bucket.scheme_id
                        && from_bkt <= residual_days
                        && to_bkt >= residual_days
                    {
                        dim3 = value.to_string();
                        break;
                    }
                }

                write!(
                    output_file,
                    "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
                    report_id,
                    config_params.as_on_date().format("%d-%m-%Y").to_string(),
                    deal_id.to_string(),
                    dim1,
                    dim2,
                    dim3.to_string(),
                    ccy_id.to_string(),
                    mat_dt.to_string(),
                    amount_ccy.to_string(),
                    amount_hcy.to_string(),
                    src_yield.to_string()
                )
                .expect("Unable to write key to summary file.");
            }
        }
        let health_report = HealthReport::new(acc_enc, 0, 0, 0.0, 0.0, 0);
        health_report.gen_health_rpt(&file.output_file_path);
        let total_duration = print_return_time_since!(start_time);
        log_info!(logger, "Total time for aggregation: {:?}", total_duration);
    }
}

pub fn get_resi_days(range: &str, days: i64) -> i64 {
    let days = match range {
        "Days" => days,
        "Month" => days * 30,
        "Years" => days * 365,
        _ => 0,
    };
    return days;
}
