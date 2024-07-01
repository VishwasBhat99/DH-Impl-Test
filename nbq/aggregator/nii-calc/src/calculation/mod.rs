use self::derived_fields::{llg_for_account, read_field};
use self::structs::{BMRatesKey, BMRatesValue, OutputData};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::NaiveDate;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::Reader;
use sdb_io::buf_file_wrtr;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::env;
use std::env::current_dir;

use std::io::prelude::*;
pub mod config;
mod derived_fields;
mod req_fields;
pub mod structs;

pub fn process(config_params: ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let mut writer = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(wrtr) => wrtr,
        Err(error) => {
            panic!(
                "Could not create file: `{}` on location `{}` : {:?}.",
                config_params.output_file_path(),
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    // Read Files Configuration
    let files_config = config::get_files(config_params.config_file_path());
    // Read IPIntBMRates File
    let parse_date_from_str = NaiveDate::parse_from_str;
    let bm_rates_file = match new_buf_rdr(&files_config.bm_rates_file_path) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            files_config.bm_rates_file_path,
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut bm_rate_map: HashMap<BMRatesKey, BMRatesValue> = HashMap::new();
    for (line_num, lines) in bm_rates_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                &files_config.bm_rates_file_path,
                line_num + 1,
                error
            ),
        };
        let line_det: Vec<&str> = line.split('|').collect();
        let bm_key = BMRatesKey {
            as_on: parse_date_from_str(line_det[0], "%d-%m-%Y")
                .expect("Cannot read date in dd-mm-yyyy format."),
            bm_code: line_det[2].to_string(),
        };
        let bm_value = BMRatesValue {
            analysis_no: line_det[1].to_string(),
            rate: line_det[3]
                .parse()
                .expect("Cannot read rate from bm rate file."),
        };
        bm_rate_map.insert(bm_key, bm_value);
    }
    log_debug!(diag_logger, "{:#?}", bm_rate_map);

    let mut acc_enc = 0;
    let mut acc_succ = 0;

    for file in files_config.files {
        // Read cashflow file
        let mut file_rdr: Reader =
            reader::Reader::new_at_path(&file.metadata_file_path, &file.input_file_path);
        let rules = AggRules::new_from_path(&file.balm_rules_file_path, &file_rdr);
        let req_fields = req_fields::ReqFields::new_from_path(&file.req_fields_file_path);
        for account in file_rdr.iter() {
            acc_enc += 1;
            let acc_id = read_field(&account, &req_fields.acc_id);
            let bm_code = match account.get_string_for_key(&req_fields.int_bm_code) {
                Ok(val) => val,
                Err(_) => {
                    log_error!(
                        logger,
                        "Skipping Account: BM Code not available for {} in {}.",
                        acc_id,
                        file.source_name
                    );
                    continue;
                }
            };
            let bm_key = BMRatesKey {
                as_on: config_params.as_on_date(),
                bm_code: bm_code.to_string(),
            };
            let def_bm_val = BMRatesValue {
                analysis_no: "DEF".to_string(),
                rate: 0.0,
            };
            let bm_data: &BMRatesValue = match bm_rate_map.get(&bm_key) {
                Some(data) => data,
                None => {
                    log_warn!(
                        logger,
                        "BM Rates not found for: {} - {:#?}, using default data set.",
                        file.source_name,
                        bm_key
                    );
                    &def_bm_val
                }
            };
            let int_rate = match account.get_f64_for_key(&req_fields.int_rate) {
                Ok(val) => val,
                Err(_) => {
                    log_error!(
                        logger,
                        "Skipping Account: Int Rate not available for {} in {}.",
                        acc_id,
                        file.source_name
                    );
                    continue;
                }
            };
            let min_rate = match account.get_f64_for_key(&req_fields.min_rate) {
                Ok(val) => val,
                Err(_) => {
                    log_warn!(
                        logger,
                        "Min Rate not available for {} in {}",
                        acc_id,
                        file.source_name
                    );
                    0.0
                }
            };
            let spread: f64 = match account.get_f64_for_key(&req_fields.spread) {
                Ok(val) => val,
                Err(_) => match account.get_string_for_key(&req_fields.spread) {
                    Ok(val) => val.parse().unwrap_or(0.0),
                    Err(_) => {
                        log_warn!(
                            logger,
                            "Spread not available for {} in {}",
                            acc_id,
                            file.source_name
                        );
                        0.0
                    }
                },
            };
            let new_rate = int_rate + bm_data.rate;
            let rate = if min_rate == 0.0 {
                int_rate + spread
            } else {
                min_rate
            };

            let mut new_eff_rate = 0.0;
            if min_rate != 0.0 {
                if new_rate > rate {
                    new_eff_rate = new_rate;
                } else {
                    new_eff_rate = rate;
                }
            }

            let rate_diff = if min_rate == 0.0 {
                new_rate - rate
            } else {
                new_eff_rate - rate
            };
            let amount_lcy = match account.get_f64_for_key(&req_fields.amount_lcy) {
                Ok(val) => val,
                Err(_) => {
                    log_error!(
                        logger,
                        "Skipping Account: Amount LCY not available for {} in {}.",
                        acc_id,
                        file.source_name
                    );
                    continue;
                }
            };
            let impact_amt = (rate_diff * amount_lcy) / 100.0;
            let op = OutputData {
                source_name: file.source_name.to_string(),
                as_on: config_params.as_on_date().format("%d-%m-%Y").to_string(),
                analysis_no: bm_data.analysis_no.to_string(),
                llg_id: llg_for_account(&account, &rules, file.default_llg_id),
                acc_id: acc_id,
                branch_id: read_field(&account, &req_fields.branch_id),
                customer_id: read_field(&account, &req_fields.customer_id),
                prd_code: read_field(&account, &req_fields.prd_code),
                gl_code: read_field(&account, &req_fields.gl_code),
                currency: read_field(&account, &req_fields.currency),
                amount_lcy: amount_lcy,
                amount_ccy: read_field(&account, &req_fields.amount_ccy),
                int_rate: int_rate,
                int_bm_code: bm_code.to_string(),
                int_bm_rate: read_field(&account, &req_fields.int_bm_rate),
                spread: spread,
                min_rate: min_rate,
                max_rate: read_field(&account, &req_fields.max_rate),
                new_rate: new_rate,
                rate_diff: rate_diff,
                impact_amt: impact_amt,
            };
            writer
                .write(format!("{}", op).as_bytes())
                .expect("Could not write data to output file!!");
            acc_succ += 1;
        }
    }
    let health_stat = HealthReport::new(acc_enc, acc_succ, acc_enc - acc_succ, 0.0, 0.0, 0);
    health_stat.gen_health_rpt(config_params.output_file_path());
}
