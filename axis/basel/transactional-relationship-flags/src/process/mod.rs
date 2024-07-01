use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use process::account_field_names::AccFieldNames;
use process::implementation::llg_for_cf_account;
use rbdate::num_days_start_to_end;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::account_with_cfs::get_field_value;
use sdb_io::buf_file_wrtr;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
#[allow(dead_code, unused_imports)]
use std::env;
use std::io::prelude::*;
use std::io::BufWriter;

use self::llg_key::LLGKey;
mod account_field_names;
mod implementation;
mod llg_key;

pub fn process(config_params: ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let mut acc_enc = 0;
    let mut acc_succ = 0;
    let as_on_date = config_params.as_on_date();
    //Create hashmaps for the master files.
    let mut stable_reln_map: HashMap<String, i64> = HashMap::new();
    let mut salary_pension_reln_map: HashMap<String, String> = HashMap::new();
    let mut ca_map: HashMap<String, (LLGKey, String)> = HashMap::new();
    let mut sa_map: HashMap<String, (LLGKey, String)> = HashMap::new();
    let mut td_map: HashMap<String, (LLGKey, String)> = HashMap::new();
    let mut rd_map: HashMap<String, (LLGKey, String)> = HashMap::new();
    let mut cust_id_llg_map: HashMap<String, Vec<i32>> = HashMap::new();
    let mut tbl_comp_map: HashMap<String, String> = HashMap::new();
    let mut nwd_codes: Vec<i32> = Vec::new();

    if config_params.is_nwd_code_in_use() == "true" {
        let nwd_reader = match new_buf_rdr(config_params.nwd_code_lookup()) {
            Ok(file) => file,
            Err(error) => panic!(
                "Could not found file `{}` on location `{}` : {}.",
                config_params.nwd_code_lookup(),
                env::current_exe()
                    .expect("Error while getting current directory path.")
                    .display(),
                error
            ),
        };

        for (line_num, lines) in nwd_reader.lines().enumerate() {
            let line = match lines {
                Ok(line) => line,
                Err(error) => {
                    log_error!(
                        logger,
                        "Unable to read file `{}` at line number: `{}` : {}",
                        config_params.nwd_code_lookup(),
                        line_num + 1,
                        error
                    );
                    "".to_string()
                }
            };
            nwd_codes = line
                .split(',')
                .map(|x| x.parse::<i32>().unwrap_or(0))
                .collect();
        }
    }
    let mut constitution_codes: Vec<&str> = Vec::new();
    if config_params.nwd_constitution_codes() != "NA" {
        constitution_codes = config_params.nwd_constitution_codes().split(',').collect();
    }

    let stable_reln_reader = match new_buf_rdr(config_params.stable_reln_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.stable_reln_file(),
            env::current_exe()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let salary_pension_reln_reader = match new_buf_rdr(config_params.salary_pension_reln_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.salary_pension_reln_file(),
            env::current_exe()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let tbl_comp_reader = match new_buf_rdr(config_params.tbl_dep_comp_def_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.tbl_dep_comp_def_file(),
            env::current_exe()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let cmg_file_reader = match new_buf_rdr(config_params.cmg_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.cmg_file(),
            env::current_exe()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let mut ca_file_reader =
        reader::Reader::new_at_path(config_params.ca_metadata_file(), config_params.ca_file());
    let ca_account_reader =
        reader::Reader::new_at_path(config_params.ca_metadata_file(), config_params.ca_file());
    let ca_rules = AggRules::new_from_path(config_params.ca_balm_rule_file_path(), &ca_file_reader);
    let output_file = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => {
            panic!(
                "Could not create output file: `{}` on location `{}`: {}.",
                config_params.output_file_path(),
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    let mut writer = BufWriter::new(output_file);
    for (line_num, lines) in stable_reln_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    logger,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_params.stable_reln_file(),
                    line_num + 1,
                    error
                );
                "".to_string()
            }
        };
        let fields: Vec<&str> = line.split('|').collect();
        stable_reln_map
            .entry(fields[2].trim().to_string())
            .and_modify(|data| *data += fields[4].trim().parse::<i64>().unwrap_or(0))
            .or_insert(fields[4].trim().parse::<i64>().unwrap_or(0));
    }

    for (line_num, lines) in salary_pension_reln_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    logger,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_params.salary_pension_reln_file(),
                    line_num + 1,
                    error
                );
                "".to_string()
            }
        };
        let fields: Vec<&str> = line.split('|').collect();
        salary_pension_reln_map.insert(fields[2].trim().to_string(), fields[3].trim().to_string());
    }

    for (line_num, lines) in tbl_comp_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    logger,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_params.tbl_dep_comp_def_file(),
                    line_num + 1,
                    error
                );
                "".to_string()
            }
        };
        let fields: Vec<&str> = line.split('|').collect();
        if fields[3].contains("CA")
            || fields[3].contains("SA")
            || fields[3].contains("TD")
            || fields[3].contains("TDNCHVD")
        {
            tbl_comp_map.insert(fields[1].to_string(), fields[3].to_string());
        }
    }
    let keys = AccFieldNames::new_from_path(config_params.req_fields_file_path());

    for account in ca_file_reader.iter() {
        let ca_cust_id =
            match get_field_value(&account, &ca_account_reader, keys.ca_cust_id.to_owned()) {
                Ok(val) => val.to_string(),
                Err(_) => {
                    log_error!(logger, "Could not read ca_cust_id from CA file.");
                    "".to_string()
                }
            };
        let ca_llg: LLGKey =
            llg_for_cf_account(&account, &ca_rules, &ca_cust_id, &config_params, logger);
        if !tbl_comp_map.contains_key(&ca_llg.source_code.to_string()) {
            continue;
        }
        let flag_check = "Y".to_string();
        ca_map.insert(ca_cust_id, (ca_llg, flag_check));
    }
    let mut sa_file_reader =
        reader::Reader::new_at_path(config_params.sa_metadata_file(), config_params.sa_file());
    let sa_account_reader =
        reader::Reader::new_at_path(config_params.sa_metadata_file(), config_params.sa_file());
    let sa_rules = AggRules::new_from_path(config_params.sa_balm_rule_file_path(), &sa_file_reader);
    for account in sa_file_reader.iter() {
        let sa_cust_id =
            match get_field_value(&account, &sa_account_reader, keys.sa_cust_id.to_owned()) {
                Ok(val) => val.to_string(),
                Err(_) => {
                    log_error!(logger, "Could not read sa_cust_id from sa file.");
                    "".to_string()
                }
            };

        let sa_llg = llg_for_cf_account(&account, &sa_rules, &sa_cust_id, &config_params, logger);
        if !tbl_comp_map.contains_key(&sa_llg.source_code.to_string()) {
            continue;
        }
        let flag_check = "Y".to_string();
        sa_map.insert(sa_cust_id, (sa_llg, flag_check));
    }
    drop(sa_account_reader);
    drop(sa_file_reader);
    let mut td_file_reader =
        reader::Reader::new_at_path(config_params.td_metadata_file(), config_params.td_file());
    let td_account_reader =
        reader::Reader::new_at_path(config_params.td_metadata_file(), config_params.td_file());
    let td_rules = AggRules::new_from_path(config_params.td_balm_rule_file_path(), &td_file_reader);
    for account in td_file_reader.iter() {
        let td_cust_id =
            match get_field_value(&account, &td_account_reader, keys.td_cust_id.to_owned()) {
                Ok(val) => val.to_string(),
                Err(_) => {
                    log_error!(logger, "Could not read td_cust_id from td file.");
                    "".to_string()
                }
            };
        let td_llg = llg_for_cf_account(&account, &td_rules, &td_cust_id, &config_params, logger);

        let mut llg_vec = cust_id_llg_map
            .entry(td_cust_id.clone())
            .or_insert_with(Vec::new);

        llg_vec.push(td_llg.source_code);

        if !tbl_comp_map.contains_key(&td_llg.source_code.to_string()) {
            continue;
        }
        let td_mat_date =
            naivedate_from_timestamp(account.get_i64_for_key(&keys.td_mat_date).unwrap_or(0));
        let td_const_code =
            match get_field_value(&account, &td_account_reader, keys.td_const_code.to_owned()) {
                Ok(val) => val,
                Err(_) => {
                    log_error!(logger, "Could not read td_mat_date from td file.");
                    "".to_string()
                }
            };
        let mut residual_flag = false;
        let mut const_code_flag = false;
        if num_days_start_to_end(*as_on_date, td_mat_date)
            > *config_params.nwd_residual_days_limit()
        {
            residual_flag = true;
        }

        if (!constitution_codes.is_empty() && constitution_codes.contains(&td_const_code.as_str()))
            || constitution_codes.is_empty()
        {
            const_code_flag = true;
        }

        let mut flag_check = "N".to_string();
        if residual_flag && const_code_flag {
            flag_check = "Y".to_string();
        }
        td_map.insert(td_cust_id, (td_llg, flag_check));
    }

    drop(td_account_reader);
    drop(td_file_reader);
    let mut rd_file_reader =
        reader::Reader::new_at_path(config_params.rd_metadata_file(), config_params.rd_file());
    let rd_account_reader =
        reader::Reader::new_at_path(config_params.rd_metadata_file(), config_params.rd_file());
    let rd_rules = AggRules::new_from_path(config_params.rd_balm_rule_file_path(), &rd_file_reader);
    for account in rd_file_reader.iter() {
        let rd_cust_id =
            match get_field_value(&account, &rd_account_reader, keys.rd_cust_id.to_owned()) {
                Ok(val) => val.to_string(),
                Err(_) => {
                    log_error!(logger, "Could not read rd_cust_id from rd file.");
                    "".to_string()
                }
            };
        let rd_llg = llg_for_cf_account(&account, &rd_rules, &rd_cust_id, &config_params, logger);
        if !tbl_comp_map.contains_key(&rd_llg.source_code.to_string()) {
            continue;
        }
        let mut llg_vec = cust_id_llg_map
            .entry(rd_cust_id.clone())
            .or_insert_with(Vec::new);
        llg_vec.push(rd_llg.source_code);

        let rd_mat_date =
            naivedate_from_timestamp(account.get_i64_for_key(&keys.rd_mat_date).unwrap_or(0));
        let rd_const_code =
            match get_field_value(&account, &rd_account_reader, keys.rd_const_code.to_owned()) {
                Ok(val) => val,
                Err(_) => {
                    log_error!(logger, "Could not read rd_mat_date from rd file.");
                    "".to_string()
                }
            };
        let mut residual_flag = false;
        let mut const_code_flag = false;
        if num_days_start_to_end(*as_on_date, rd_mat_date)
            > *config_params.nwd_residual_days_limit()
        {
            residual_flag = true;
        }
        if (!constitution_codes.is_empty() && constitution_codes.contains(&rd_const_code.as_str()))
            || constitution_codes.is_empty()
        {
            const_code_flag = true;
        }
        let mut flag_check = "N".to_string();
        if residual_flag && const_code_flag {
            flag_check = "Y".to_string();
        }
        rd_map.insert(rd_cust_id, (rd_llg, flag_check));
    }
    drop(rd_account_reader);
    drop(rd_file_reader);
    for (line_num, lines) in cmg_file_reader.lines().enumerate() {
        acc_enc += 1;
        let line = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    logger,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_params.cmg_file(),
                    line_num + 1,
                    error
                );
                "".to_string()
            }
        };

        if let Some((id, _)) = line.split_once('|') {
            let cmg_cust_id = id.trim().to_string();
            if ca_map.contains_key(&cmg_cust_id)
                || sa_map.contains_key(&cmg_cust_id)
                || td_map.contains_key(&cmg_cust_id)
                || rd_map.contains_key(&cmg_cust_id)
            {
                let mut salary_relationship = "N".to_string();
                let mut pension_relationship = "N".to_string();
                if ca_map.contains_key(&cmg_cust_id) || sa_map.contains_key(&cmg_cust_id) {
                    if let Some(schm_code) = salary_pension_reln_map.get(&cmg_cust_id) {
                        if schm_code.to_uppercase().contains("SAL") {
                            salary_relationship = "Y".to_string();
                        } else if schm_code.to_uppercase().contains('P') {
                            pension_relationship = "Y".to_string();
                        }
                    }
                }

                let mut asset_relationship_count = stable_reln_map.get(&cmg_cust_id).unwrap_or(&0);

                let mut ca_llg = config_params.default_llg_code();
                let mut ca_flag_check = "N".to_string();
                let mut sa_llg = config_params.default_llg_code();
                let mut sa_flag_check = "N".to_string();
                let mut td_llg = config_params.default_llg_code();
                let mut td_flag_check = "N".to_string();
                let mut rd_llg = config_params.default_llg_code();
                let mut rd_flag_check = "N".to_string();

                let ca_relation = match ca_map.get(&cmg_cust_id) {
                    Some(rel) => {
                        ca_llg = &rel.0.source_code;
                        ca_flag_check = rel.1.to_owned();
                        "Y".to_string()
                    }
                    None => "N".to_string(),
                };
                let sa_relation = match sa_map.get(&cmg_cust_id) {
                    Some(rel) => {
                        sa_llg = &rel.0.source_code;
                        sa_flag_check = rel.1.to_owned();
                        "Y".to_string()
                    }
                    None => "N".to_string(),
                };
                let td_relation = match td_map.get(&cmg_cust_id) {
                    Some(rel) => {
                        td_llg = &rel.0.source_code;
                        td_flag_check = rel.1.to_owned();
                        "Y".to_string()
                    }
                    None => "N".to_string(),
                };

                if let Some(rel) = rd_map.get(&cmg_cust_id) {
                    rd_llg = &rel.0.source_code;
                    rd_flag_check = rel.1.to_owned();
                };

                //NWD Relation:
                let mut nwd_relation = "N".to_string();
                if config_params.is_nwd_code_in_use() == "true" {
                    if td_flag_check == *"Y" || rd_flag_check == *"Y" {
                        if cust_id_llg_map
                            .get(&cmg_cust_id)
                            .unwrap_or(&Vec::new())
                            .iter()
                            .any(|llg| nwd_codes.contains(llg))
                        {
                            nwd_relation = "Y".to_string()
                        }
                    }
                } else if ca_flag_check == *"Y"
                    || sa_flag_check == *"Y"
                    || td_flag_check == *"Y"
                    || rd_flag_check == *"Y"
                {
                    nwd_relation = "Y".to_string();
                }

                let output_line = format!(
                    "{}|{}|{}|{}|{}|N|{}|{}|{}|N|N\n",
                    cmg_cust_id,
                    salary_relationship,
                    pension_relationship,
                    asset_relationship_count,
                    ca_relation,
                    sa_relation,
                    td_relation,
                    nwd_relation
                );

                writer
                    .write_all(output_line.as_bytes())
                    .expect("Could not write to the output file.");
                acc_succ += 1;
            } else {
                log_error!(logger, "Lookup not found for CMG CustID: {}", cmg_cust_id);
            }
        };
    }
    let health_report = HealthReport::new(acc_enc, acc_succ, acc_enc - acc_succ, 0.0, 0.0, 0);
    health_report.gen_health_rpt(config_params.output_file_path());
}

fn naivedate_from_timestamp(t: i64) -> NaiveDate {
    let naive_date_time = rbdate::NaiveDateTime::from_timestamp(t, 0);
    naive_date_time.date()
}
