extern crate serde;
mod exrt;
mod structs;

use self::exrt::*;
use self::structs::*;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::num_days_start_to_end;
use rbdate::DateParser;
use rbdate::NaiveDate;
use sdb_day_convention::accrued_days_with_convn;
use sdb_day_convention::conventions::Conventions;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::io::BufWriter;
use std::time::SystemTime;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let output_file = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{} Cannot read output file path", error),
    };

    let mut tot_input_acc_encntrd: i64 = 0;
    let mut tot_succ_rec: i64 = 0;
    let mut writer = BufWriter::new(output_file);
    let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);

    let start_ex_rt_read_timer = SystemTime::now();

    let mut ex_rt_map: HashMap<ExrtKey, f64> = HashMap::new();
    let ex_rt_file = match new_buf_rdr(config_param.exchange_rate_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.exchange_rate_file(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in ex_rt_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.exchange_rate_file(),
                line_num + 1,
                error
            ),
        };
        let derived_fields: Vec<&str> = line.split("|").collect();
        let ex_rt_key = ExrtKey::new(derived_fields[0].to_string(), derived_fields[1].to_string());
        ex_rt_map.insert(ex_rt_key, derived_fields[2].parse::<f64>().unwrap_or(1.0));
    }
    let end_ex_rt_read_timer = SystemTime::now();
    let duration = end_ex_rt_read_timer
        .duration_since(start_ex_rt_read_timer)
        .expect("Could not calculate total duration for read timer.");
    log_debug!(
        log,
        "Reading EXCHANGE RATE File, Total Duration: {:?}.",
        duration
    );

    let bond_reader = match new_buf_rdr(config_param.bond_master_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.bond_master_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut bond_master_map: HashMap<BondMasterKey, BondMasterValue> = HashMap::new();
    for (line_num, lines) in bond_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.common_code_file_path(),
                line_num + 1,
                error
            ),
        };

        let bond_data: Vec<&str> = line.split("|").collect();

        let key = BondMasterKey {
            isin: bond_data[2].to_string(),
            country_code: bond_data[1].to_string(),
        };

        let value = BondMasterValue {
            guarantee_type: bond_data[4].to_string(),
            rating_id: bond_data[5].to_string(),
            is_financial: bond_data[6].to_string(),
        };

        bond_master_map.insert(key, value);
    }

    let common_code_reader = match new_buf_rdr(config_param.common_code_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.common_code_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut common_code_map: HashMap<CommonCode, i64> = HashMap::new();
    for (line_num, lines) in common_code_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.common_code_file_path(),
                line_num + 1,
                error
            ),
        };

        let data: Vec<&str> = line.split("|").collect();
        let code_key = CommonCode {
            code_type: data[0].to_string(),
            cm_code: data[1].to_string(),
        };
        common_code_map.insert(code_key, data[2].parse::<i64>().unwrap_or(0));
    }
    let residual_30d = *common_code_map
        .get(&CommonCode {
            code_type: "Residual_days".to_string(),
            cm_code: "30D".to_string(),
        })
        .unwrap_or(&0);

    let residual_U6M = *common_code_map
        .get(&CommonCode {
            code_type: "Residual_days".to_string(),
            cm_code: "U6M".to_string(),
        })
        .unwrap_or(&0);

    let residual_U1Y = *common_code_map
        .get(&CommonCode {
            code_type: "Residual_days".to_string(),
            cm_code: "U1Y".to_string(),
        })
        .unwrap_or(&0);

    let blrms_file_reader = match new_buf_rdr(config_param.blrms_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.blrms_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in blrms_file_reader.lines().enumerate() {
        tot_input_acc_encntrd += 1;

        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.blrms_file_path(),
                line_num + 1,
                error
            ),
        };
        let blrms_data: Vec<&str> = line.split(",").collect();

        let mat_date = NaiveDate::parse_from_str(&blrms_data[4], "%d-%m-%Y")
            .unwrap_or(config_param.as_on_date);

        let accrued_days = accrued_days_with_convn(
            config_param.as_on_date,
            mat_date,
            6,
            &config_param.day_convention(),
        )
        .unwrap()
        .days_btw_dts
        .abs();

        let bond_mat_date = NaiveDate::parse_from_str(&blrms_data[7], "%d-%m-%Y")
            .unwrap_or(config_param.as_on_date);

        let bond_residual_days = num_days_start_to_end(*config_param.as_on_date(), bond_mat_date);

        let deal_date = NaiveDate::parse_from_str(&blrms_data[2], "%d-%m-%Y")
            .unwrap_or(config_param.as_on_date);

        let contract_residual_days = num_days_start_to_end(deal_date, mat_date);

        let bond_res_bkt =
            process_cd_desc(bond_residual_days, residual_30d, residual_U6M, residual_U1Y);

        let value_date = NaiveDate::parse_from_str(&blrms_data[3], "%d-%m-%Y")
            .unwrap_or(config_param.as_on_date);
        let contract_value_days = num_days_start_to_end(value_date, mat_date);

        let contract_res_bkt = process_cd_desc(
            contract_value_days,
            residual_30d,
            residual_U6M,
            residual_U1Y,
        );

        let mat_days_diff = num_days_start_to_end(*config_param.as_on_date(), mat_date);

        let repo_rev_bkt = process_cd_desc(mat_days_diff, residual_30d, residual_U6M, residual_U1Y);

        let default_bond_data = BondMasterValue {
            guarantee_type: "NA".to_string(),
            rating_id: "NA".to_string(),
            is_financial: "NA".to_string(),
        };

        let value_res_bkt = process_cd_desc(
            num_days_start_to_end(*config_param.as_on_date(), value_date),
            residual_30d,
            residual_U6M,
            residual_U1Y,
        );

        let blrms_key = BondMasterKey {
            isin: blrms_data[5].to_string(),
            country_code: config_param.country_code().to_string(),
        };
        if bond_master_map.contains_key(&blrms_key) {
            let bond_data = bond_master_map
                .get(&blrms_key)
                .unwrap_or(&default_bond_data);

            let op_line = format!(
                "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
                blrms_data[0],
                blrms_data[1],
                blrms_data[2],
                blrms_data[3],
                blrms_data[4],
                blrms_data[5],
                blrms_data[6],
                blrms_data[7],
                blrms_data[8],
                blrms_data[9],
                blrms_data[10],
                blrms_data[11],
                blrms_data[12],
                blrms_data[13],
                blrms_data[14],
                blrms_data[15],
                blrms_data[16],
                blrms_data[17],
                accrued_days,
                bond_residual_days,
                contract_residual_days,
                bond_res_bkt,
                contract_res_bkt,
                repo_rev_bkt,
                value_res_bkt,
                blrms_data[10],
                blrms_data[12],
                blrms_data[11],
                bond_data.guarantee_type,
                bond_data.is_financial,
                bond_data.rating_id,
                blrms_data[1],
                blrms_data[13],
                config_param.base_currency()
            );
            tot_succ_rec += 1;
            match writer.write_all(op_line.as_bytes()) {
                Ok(val) => val,
                Err(error) => {
                    panic!("Error writing processed data: {:?}", error);
                }
            }
        }
    }

    let nslr_repo_reader = match new_buf_rdr(config_param.nslr_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.nslr_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in nslr_repo_reader.lines().enumerate() {
        tot_input_acc_encntrd += 1;

        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.nslr_file_path(),
                line_num + 1,
                error
            ),
        };
        let nslr_data: Vec<&str> = line.split("|").collect();

        let mat_date =
            NaiveDate::parse_from_str(&nslr_data[4], "%d-%m-%Y").unwrap_or(config_param.as_on_date);

        let accrued_days = accrued_days_with_convn(
            config_param.as_on_date,
            mat_date,
            6,
            &config_param.day_convention(),
        )
        .unwrap()
        .days_btw_dts
        .abs();

        let bond_mat_date =
            NaiveDate::parse_from_str(&nslr_data[7], "%d-%m-%Y").unwrap_or(config_param.as_on_date);

        let bond_residual_days = num_days_start_to_end(*config_param.as_on_date(), bond_mat_date);

        let deal_date =
            NaiveDate::parse_from_str(&nslr_data[2], "%d-%m-%Y").unwrap_or(config_param.as_on_date);

        let contract_residual_days = num_days_start_to_end(deal_date, mat_date);

        let bond_res_bkt =
            process_cd_desc(bond_residual_days, residual_30d, residual_U6M, residual_U1Y);

        let value_date =
            NaiveDate::parse_from_str(&nslr_data[3], "%d-%m-%Y").unwrap_or(config_param.as_on_date);
        let contract_value_days = num_days_start_to_end(value_date, mat_date);

        let contract_res_bkt = process_cd_desc(
            contract_value_days,
            residual_30d,
            residual_U6M,
            residual_U1Y,
        );

        let mat_days_diff = num_days_start_to_end(*config_param.as_on_date(), mat_date);

        let repo_rev_bkt = process_cd_desc(mat_days_diff, residual_30d, residual_U6M, residual_U1Y);

        let default_bond_data = BondMasterValue {
            guarantee_type: "NA".to_string(),
            rating_id: "NA".to_string(),
            is_financial: "NA".to_string(),
        };

        let value_res_bkt = process_cd_desc(
            num_days_start_to_end(*config_param.as_on_date(), value_date),
            residual_30d,
            residual_U6M,
            residual_U1Y,
        );

        let blrms_key = BondMasterKey {
            isin: nslr_data[5].to_string(),
            country_code: config_param.country_code().to_string(),
        };
        if bond_master_map.contains_key(&blrms_key) {
            let bond_data = bond_master_map
                .get(&blrms_key)
                .unwrap_or(&default_bond_data);

            let op_line = format!(
                "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
                nslr_data[0],
                nslr_data[1],
                nslr_data[2],
                nslr_data[3],
                nslr_data[4],
                nslr_data[5],
                nslr_data[6],
                nslr_data[7],
                nslr_data[8],
                nslr_data[9],
                nslr_data[10],
                nslr_data[11],
                nslr_data[12],
                nslr_data[13],
                nslr_data[14],
                nslr_data[15],
                nslr_data[16],
                nslr_data[17],
                accrued_days,
                bond_residual_days,
                contract_residual_days,
                bond_res_bkt,
                contract_res_bkt,
                repo_rev_bkt,
                value_res_bkt,
                nslr_data[10],
                nslr_data[12],
                nslr_data[11],
                bond_data.guarantee_type,
                bond_data.is_financial,
                bond_data.rating_id,
                nslr_data[1],
                nslr_data[13],
                config_param.base_currency()
            );
            tot_succ_rec += 1;
            match writer.write_all(op_line.as_bytes()) {
                Ok(val) => val,
                Err(error) => {
                    panic!("Error writing processed data: {:?}", error);
                }
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

fn process_cd_desc(days: i64, residual_30d: i64, residual_U6M: i64, residual_U1Y: i64) -> String {
    let result = match days {
        d if d <= residual_30d => "30D".to_string(),
        d if d <= residual_U6M => "U6M".to_string(),
        d if d <= residual_U1Y => "U1Y".to_string(),
        _ => "A1Y".to_string(),
    };

    result
}
