extern crate serde;
mod bucket;
mod exrt;
mod structs;

use self::exrt::*;
use self::structs::DerivatieKey;
use crate::pre_processor::bucket::get_bkt_slabs;
use crate::pre_processor::structs::DerivatieValue;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::{num_days_start_to_end, DateParser, NaiveDate};
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::time::SystemTime;

pub fn process(config_param: ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let output_file = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("Cannot create output file: {}", error),
    };
    let mut writer = BufWriter::new(output_file);

    let mut tot_input_acc_encntrd: i64 = 0;
    let mut tot_succ_rec: i64 = 0;

    let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
    let mut derivatives_map: HashMap<DerivatieKey, DerivatieValue> = HashMap::new();

    let ip_file_path = match new_buf_rdr(&config_param.input_file_path()) {
        Ok(file) => BufReader::new(file),
        Err(error) => panic!(
            "Could not find input data file: `{}` on location `{}` : {}.",
            config_param.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let start_ex_rt_read_timer = SystemTime::now();
    let ex_rt_file = match new_buf_rdr(&config_param.exchange_rate_file()) {
        Ok(file) => BufReader::new(file),
        Err(error) => panic!(
            "Could not find file `{}` on location `{}` : {}.",
            config_param.exchange_rate_file(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let ex_rt_map: HashMap<ExrtKey, f64> = ex_rt_file
        .lines()
        .enumerate()
        .map(|(line_num, line)| match line {
            Ok(line) => {
                let derived_fields: Vec<&str> = line.split("|").collect();
                let ex_rt_key =
                    ExrtKey::new(derived_fields[0].to_string(), derived_fields[1].to_string());
                let ex_rt_val = match derived_fields[2].parse::<f64>() {
                    Ok(val) => val,
                    Err(error) => panic!(
                        "Invalid exchange rate value at line {}: {}",
                        line_num + 1,
                        error
                    ),
                };
                (ex_rt_key, ex_rt_val)
            }
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.exchange_rate_file(),
                line_num + 1,
                error
            ),
        })
        .collect();
    let end_ex_rt_read_timer = SystemTime::now();
    let duration = end_ex_rt_read_timer
        .duration_since(start_ex_rt_read_timer)
        .expect("Could not calculate total duration for read timer.");
    log_debug!(
        log,
        "Reading EXCHANGE RATE File, Total Duration: {:?}.",
        duration
    );

    let (residual_u6m, residual_u1y) = get_bkt_slabs(
        config_param.bucket_schema_file_path(),
        config_param.as_on_date(),
    );

    for (line_no, lines) in ip_file_path.lines().enumerate() {
        let fields = match lines {
            Ok(data) => data,
            Err(error) => panic!(
                "Unable to read file `{}` at line no:{}: {}",
                config_param.input_file_path(),
                line_no + 1,
                error
            ),
        };
        tot_input_acc_encntrd += 1;
        let input_account: Vec<&str> = fields.split('|').collect();

        let mat_date = NaiveDate::parse_from_str(&input_account[3], "%d-%m-%Y")
            .unwrap_or_else(|_| *config_param.as_on_date());
        let mat_date_diff = num_days_start_to_end(*config_param.as_on_date(), mat_date);

        let tenor = if mat_date_diff <= residual_u6m {
            "U6M"
        } else if mat_date_diff > residual_u6m && mat_date_diff <= residual_u1y {
            "U1Y"
        } else {
            "A1Y"
        };
        let ex_rate = ex_rt_map
            .get(&ExrtKey {
                from_currency: input_account[7].to_string(),
                to_currency: config_param.base_currency().to_owned(),
            })
            .unwrap_or(&1.0);

        let key = DerivatieKey {
            country: config_param.country().to_string(),
            exposure_id: input_account[4].to_string(),
            tenor: tenor.to_string(),
        };
        let der_value = DerivatieValue {
            actual_country: input_account[0].to_string(),
            fic_mis_date: date_parser.parse_opt(&input_account[0]).unwrap_or_else(|| {
                NaiveDate::parse_from_str(&input_account[0], "%d-%m-%Y")
                    .unwrap_or_else(|_| *config_param.as_on_date())
            }),
            cust_id: input_account[2].to_string(),
            d_mat_date: date_parser.parse_opt(&input_account[3]).unwrap_or_else(|| {
                NaiveDate::parse_from_str(&input_account[3], "%d-%m-%Y")
                    .unwrap_or_else(|_| *config_param.as_on_date())
            }),
            schme_code_product: input_account[5].to_string(),
            cust_name: input_account[6].to_string(),
            native_ccy: config_param.base_currency.to_string(),
            native_mtm_amt: input_account[8].parse::<f64>().unwrap_or(0.0),
            cons_mtm_amt: input_account[9].parse::<f64>().unwrap_or(0.0),
            native_amt: input_account[9].parse::<f64>().unwrap_or(0.0) / ex_rate,
            cons_amt: input_account[9].parse::<f64>().unwrap_or(0.0),
            cons_inr_mtm_amt: input_account[9].parse::<f64>().unwrap_or(0.0),
            cons_inr_notional_amt: input_account[10].parse::<f64>().unwrap_or(0.0),
            residual_days: mat_date_diff,
            tenor: tenor.to_string(),
            exchange_rt: *ex_rate,
        };

        if let Some(value) = derivatives_map.get_mut(&key) {
            *value = insert_data(value.clone(), &der_value);
        } else {
            derivatives_map.insert(key, der_value);
        };
        tot_succ_rec += 1;
    }

    for (key, value) in derivatives_map.iter() {
        let op_line = format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            key.country,
            key.exposure_id,
            value.native_ccy,
            value.native_mtm_amt,
            value.cons_mtm_amt,
            value.native_amt,
            value.cons_amt,
            value.cons_inr_mtm_amt,
            value.cons_inr_notional_amt,
            value.tenor
        );

        match writer.write_all(op_line.as_bytes()) {
            Ok(_) => {}
            Err(error) => {
                panic!("Error writing processed data: {:?}", error);
            }
        }
    }

    let health_report = HealthReport::new(
        tot_input_acc_encntrd,
        tot_succ_rec,
        tot_input_acc_encntrd - tot_succ_rec,
        0.0,
        0.0,
        0,
    );
    health_report.gen_health_rpt(config_param.output_file_path());
}

pub fn insert_data(data: DerivatieValue, der_value: &DerivatieValue) -> DerivatieValue {
    DerivatieValue {
        actual_country: data.actual_country,
        fic_mis_date: data.fic_mis_date,
        cust_id: data.cust_id,
        d_mat_date: data.d_mat_date,
        schme_code_product: data.schme_code_product,
        cust_name: data.cust_name,
        native_ccy: data.native_ccy,
        native_mtm_amt: data.native_mtm_amt + der_value.native_mtm_amt,
        cons_mtm_amt: data.cons_mtm_amt + der_value.cons_mtm_amt,
        native_amt: data.native_amt + der_value.native_amt,
        cons_amt: data.cons_amt + der_value.cons_amt,
        cons_inr_mtm_amt: data.cons_inr_mtm_amt + der_value.cons_inr_mtm_amt,
        cons_inr_notional_amt: data.cons_inr_notional_amt + der_value.cons_inr_notional_amt,
        residual_days: data.residual_days,
        tenor: data.tenor,
        exchange_rt: data.exchange_rt,
    }
}
