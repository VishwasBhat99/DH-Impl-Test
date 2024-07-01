use super::{
    input::{HqlaData, ManualData},
    required_manual_fields::ReqManualFields,
};
use crate::configuration_parameters::ConfigurationParameters;
use chrono::NaiveDate;
use rbdate::DateParser;
use slog::Logger;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_manual(config_params: &ConfigurationParameters, logger: &Logger) -> ManualData {
    let man_input =
        File::open(config_params.input_manual_file_path()).expect("Could Not Read manual File");
    let man_file = BufReader::new(man_input);

    let mut slrgsec_maintained = 0.0;
    let mut slrrequired = 0.0;
    let mut lending_to_nbfchfc = 0.0;
    let mut fallcrceiling = 0.0;

    let req_manual_fields =
        ReqManualFields::new_from_path(config_params.required_manual_fields_file_path());
    for (line_num, line) in man_file.lines().enumerate() {
        let line_info = match line {
            Ok(line_info) => line_info,
            Err(error) => {
                info!(
                    logger,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_params.input_manual_file_path(),
                    line_num + 1,
                    error
                );
                continue;
            }
        };
        let fields: Vec<String> = line_info
            .split(config_params.input_delimiter())
            .map(|s| s.trim().to_string())
            .collect();
        if fields.len() < 4 {
            info!(
                logger,
                "No of fields are less than required(four) at line-no: `{}` in manual input file.",
                line_num + 1
            );
            continue;
        }
        let field_name: String = fields[2].to_lowercase();
        if fields[3].replace(',', "").parse::<f64>().is_err() {
            info!(
                logger,
                "Not a valid decimal field-value at line-no: `{}` in manual input file.",
                line_num + 1
            );
        }
        if field_name.eq(&req_manual_fields.slr_gsec_maintained.trim().to_lowercase()) {
            slrgsec_maintained = fields[3].replace(',', "").parse::<f64>().unwrap_or(0.0);
            if config_params.is_perf_diagnostics_enabled() {
                info!(
                    logger,
                    "`{}` in input, `{}` in config, `{}` is value",
                    field_name,
                    &req_manual_fields.slr_gsec_maintained.trim().to_lowercase(),
                    slrgsec_maintained
                );
            }
        } else if field_name.eq(&req_manual_fields.lending_to_nbfchfc.trim().to_lowercase()) {
            lending_to_nbfchfc = fields[3].replace(',', "").parse::<f64>().unwrap_or(0.0);
            if config_params.is_perf_diagnostics_enabled() {
                info!(
                    logger,
                    "`{}` in input, `{}` in config, `{}` is value",
                    field_name,
                    &req_manual_fields.lending_to_nbfchfc.trim().to_lowercase(),
                    lending_to_nbfchfc
                );
            }
        } else if field_name.eq(&req_manual_fields.slr_required.trim().to_lowercase()) {
            slrrequired = fields[3].replace(',', "").parse::<f64>().unwrap_or(0.0);
            if config_params.is_perf_diagnostics_enabled() {
                info!(
                    logger,
                    "`{}` in input, `{}` in config, `{}` is value",
                    field_name,
                    &req_manual_fields.slr_required.trim().to_lowercase(),
                    slrrequired
                );
            }
        } else if field_name.eq(&req_manual_fields.fallcr_ceiling.trim().to_lowercase()) {
            fallcrceiling = fields[3].replace(',', "").parse::<f64>().unwrap_or(0.0);
            if config_params.is_perf_diagnostics_enabled() {
                info!(
                    logger,
                    "`{}` in input, `{}` in config, `{}` is value",
                    field_name,
                    &req_manual_fields.fallcr_ceiling.trim().to_lowercase(),
                    fallcrceiling
                );
            }
        }
    }

    ManualData::new(
        slrgsec_maintained,
        slrrequired,
        lending_to_nbfchfc,
        fallcrceiling,
    )
}

pub fn read_hqla(config_params: &ConfigurationParameters, logger: &Logger) -> HqlaData {
    let hqla_input =
        File::open(config_params.input_hqla_file_path()).expect("Could Not Read hqla File");
    let hqla_file = BufReader::new(hqla_input);

    //Initialize default value as 1 to avoid divide by zero error.
    let mut req_slr_perc = 1.0;
    let mut req_msf_perc = 1.0;
    let mut req_fallcr_perc = 1.0;

    for (line_num, line) in hqla_file.lines().enumerate() {
        let line_info = match line {
            Ok(line_info) => line_info,
            Err(error) => {
                info!(
                    logger,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_params.input_hqla_file_path(),
                    line_num + 1,
                    error
                );
                continue;
            }
        };
        let fields: Vec<String> = line_info
            .split(config_params.input_delimiter())
            .map(|s| s.trim().to_string())
            .collect();
        if fields.len() < 6 {
            info!(
                logger,
                "No of fields are less than five at line no: `{}` in hqla file.",
                line_num + 1
            );
            continue;
        }

        let cntry_id = fields[0].to_lowercase();

        //Checking mat and repo_mat_date from input:
        let app_st_dt_fmt = get_valid_format(config_params, &fields[1]);
        let app_end_dt_fmt = get_valid_format(config_params, &fields[2]);

        if app_st_dt_fmt.is_none() {
            info!(
                logger,
                "Could not parse app-start-date date at line-no: `{}` in hqla file.",
                line_num + 1
            );
            continue;
        }
        if app_end_dt_fmt.is_none() {
            info!(
                logger,
                "Could not parse app-end-date date at line-no: `{}` in hqla file.",
                line_num + 1
            );
            continue;
        };
        let app_st_dt = &DateParser::new(app_st_dt_fmt.unwrap(), false).parse(&fields[1]);
        let app_end_dt = &DateParser::new(app_end_dt_fmt.unwrap(), false).parse(&fields[2]);
        if cntry_id.eq(&config_params.country_id().to_lowercase())
            && app_st_dt <= config_params.as_on_date()
            && app_end_dt >= config_params.as_on_date()
        {
            req_slr_perc = fields[3]
                .replace(',', "")
                .parse::<f64>()
                .expect("Cannot parse req_slr_percentage from hqla table.");
            req_msf_perc = fields[4]
                .replace(',', "")
                .parse::<f64>()
                .expect("Cannot parse req_msf_percentage from hqla table.");
            req_fallcr_perc = fields[5]
                .replace(',', "")
                .parse::<f64>()
                .expect("Cannot parse req_fallcr_percentage from hqla table.");
            if config_params.is_perf_diagnostics_enabled() {
                info!(
                    logger,
                    "Got all req percentages at line-no: `{}` in hqla file.",
                    line_num + 1
                );
            }
        }
    }

    HqlaData::new(req_slr_perc, req_msf_perc, req_fallcr_perc)
}

pub fn get_valid_format(config_params: &ConfigurationParameters, date: &str) -> Option<String> {
    for frmt in config_params.input_date_formats() {
        // print!("format:{:?}",frmt);
        if NaiveDate::parse_from_str(date, frmt).is_ok() {
            return Some(frmt.to_string());
        }
    }

    None
}
