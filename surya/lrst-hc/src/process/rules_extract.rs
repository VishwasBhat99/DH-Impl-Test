use configuration_parameters::ConfigurationParameters;
use dbpool::OracleConnectionManager;
use macros;
use r2d2::Conn;
use r2d2::Pool;
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::str;

#[derive(Debug, Eq, Default, Clone, Hash, PartialEq)]
pub struct RuleType {
    pub rules_hc: String,
    pub rules_pid: String,
}

impl RuleType {
    pub fn new(rules_hc: String, rules_pid: String) -> RuleType {
        RuleType {
            rules_hc: rules_hc,
            rules_pid: rules_pid,
        }
    }
}

pub fn extract(
    pool: Pool<OracleConnectionManager>,
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) -> HashMap<String, RuleType> {
    let mut rule_filenames_list: HashMap<String, RuleType> = HashMap::new();
    let conn = &pool
        .get()
        .expect("Failed to get connection from pool.")
        .conn;
    match conn {
        Some(db) => {
            let mut scen_ids: Vec<String> = Vec::new();
            let sql_1 = "select \"StrsScenId\" from \"StrsScenarioDef\" where \"IsActive\"='Y'";
            let rows = db
                .conn
                .query(sql_1, &[])
                .expect("Query Failed to Fetch Data from StrsScenarioDef.");

            for row_result in &rows {
                let row = row_result.expect("Failed to read query output from DHRules.");
                log_debug!(diag_logger, "row: {:?}", row);
                let scen_id: String = match row.get("StrsScenId") {
                    Ok(id) => id,
                    Err(err) => {
                        log_error!(
                            logger,
                            "Could not extract data from StrsScenarioDef. Error:{}",
                            err
                        );
                        continue;
                    }
                };
                scen_ids.push(scen_id);
            }
            for scen_id in scen_ids {
                let rule_type_code = &format!("LRSTHC~{}", scen_id);
                let prod_rule_code = &format!("LRSTPROD~{}", scen_id);
                let hc_rules = extract_rules(
                    rule_type_code.to_string(),
                    &config_params,
                    &logger,
                    &diag_logger,
                    db,
                );
                let prod_rules = extract_rules(
                    prod_rule_code.to_string(),
                    &config_params,
                    &logger,
                    &diag_logger,
                    db,
                );
                let ruletype = RuleType::new(hc_rules.to_string(), prod_rules.to_string());
                rule_filenames_list.insert(scen_id, ruletype);
            }
        }
        None => {}
    }
    rule_filenames_list
}

pub fn extract_rules(
    type_code: String,
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
    db: &Conn<dbpool::oracle::Connection>,
) -> String {
    let sql_2 = "select \"RuleFile\" from \"DHRules\" where \"RuleTypeCd\"=:1";
    let rows = db
        .conn
        .query(sql_2, &[&type_code])
        .expect("Query Failed to Fetch Data from DHRules.");
    for row_result in &rows {
        let row = row_result.expect("Failed to read query output from DHRules.");
        log_debug!(diag_logger, "row: {:?}", row);
        let rule_file: Vec<u8> = match row.get("RuleFile") {
            Ok(data) => data,
            Err(err) => {
                log_error!(
                    logger,
                    "Could not extract data for :{}. Error:{}",
                    type_code,
                    err
                );
                continue;
            }
        };

        match str::from_utf8(&rule_file) {
            Ok(_val) => {
                let mut filepath = config_params.rules_output_path().to_string();
                let filename = format!("{}.txt", type_code);
                filepath.push_str(&filename);
                let mut extract_file = File::create(&filepath)
                    .expect("Failed to create extracted output file in path.");
                extract_file
                    .write_all(&rule_file)
                    .expect("Failed to write to final output file.");
                log_info!(logger, "File:{} extracted successfully.", &filepath);
            }
            Err(err) => {
                panic!("{}.", err);
            }
        }
    }
    return type_code;
}
