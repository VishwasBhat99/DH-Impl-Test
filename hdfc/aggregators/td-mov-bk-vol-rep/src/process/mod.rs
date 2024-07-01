use self::account::AccData;
use self::account::*;
use self::cf_writer::CFwrite;
use self::io::*;
use self::llg_key::LLGKey;
use super::process::cf_writer::account::Account;
use super::process::cf_writer::create_account::create_cf_acc;
use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use crate::process::account_field_names::AccFieldNames;
use crate::process::bucket::*;
use crate::process::convert::*;
use crate::process::tenor::*;
use calamine::{open_workbook_auto, Reader};
use chrono::Local;
use health_report::HealthReport;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::account_with_cfs::get_field_value;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::io::Write;
use std::time::SystemTime;

mod account;
mod account_field_names;
mod bucket;
mod cf_writer;
pub mod config;
mod convert;
mod grp_key;
mod io;
mod llg_key;
mod tenor;

pub fn process(config_params: ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let start_time = SystemTime::now();
    let mut output_path = String::new();
    output_path.push_str(config_params.output_file_path());
    output_path.push_str(".txt");
    let mut output_file = get_writer(&output_path);
    let mut cf_writer = CFwrite::new(config_params.output_file_path(), logger);

    let mut alco_map: HashMap<String, String> = HashMap::new();
    let mut alco_master_excel =
        open_workbook_auto(config_params.alco_master()).expect("Unable to open ALCO Master File.");
    if let Some(Ok(reader)) = alco_master_excel.worksheet_range(config_params.alco_sheet()) {
        for row in reader.rows().skip(1) {
            alco_map.insert(row[0].to_string(), row[1].to_string());
        }
    }

    let mut org_tenor_map: HashMap<Tenor, String> = HashMap::new();
    let mut res_tenor_map: HashMap<Tenor, String> = HashMap::new();
    let mut ia_tenor_map: HashMap<Tenor, String> = HashMap::new();
    let mut tenor_master_excel = open_workbook_auto(config_params.tenor_master())
        .expect("Unable to open Tenor Master File.");
    if let Some(Ok(reader)) = tenor_master_excel.worksheet_range(config_params.tenor_sheet()) {
        for row in reader.rows().skip(1) {
            let ia_tenor = Tenor::new(row[0].to_string(), row[1].to_string());
            let org_tenor = Tenor::new(row[0].to_string(), row[1].to_string());
            let res_tenor = Tenor::new(row[0].to_string(), row[1].to_string());
            ia_tenor_map.insert(ia_tenor, row[2].to_string());
            org_tenor_map.insert(org_tenor, row[3].to_string());
            res_tenor_map.insert(res_tenor, row[4].to_string());
        }
    }

    let mut bucket_map: HashMap<Bucket, String> = HashMap::new();
    let mut bucket_master_excel = open_workbook_auto(config_params.bucket_master())
        .expect("Unable to open Bucket Master File.");
    if let Some(Ok(reader)) = bucket_master_excel.worksheet_range(config_params.bucket_sheet()) {
        for row in reader.rows().skip(1) {
            let bucket = Bucket::new(row[0].to_string(), row[1].to_string());
            bucket_map.insert(bucket, row[2].to_string());
        }
    }

    let mut cat_map: HashMap<String, String> = HashMap::new();
    let mut cat_master_excel = open_workbook_auto(config_params.cat_master())
        .expect("Unable to open Category Master File.");
    if let Some(Ok(reader)) = cat_master_excel.worksheet_range(config_params.cat_sheet()) {
        for row in reader.rows().skip(1) {
            cat_map.insert(row[0].to_string(), row[1].to_string());
        }
    }

    let mut lcr_map: HashMap<String, String> = HashMap::new();
    let lcr_reader =
        fs::read_to_string(&config_params.lcr_master()).expect("Failed to read LCR file!");
    for line in lcr_reader.lines() {
        let lcr_fields = line.split("|").collect::<Vec<&str>>();
        lcr_map.insert(lcr_fields[1].to_string(), lcr_fields[2].to_string());
    }

    let mut wd_nwd_map: HashMap<String, String> = HashMap::new();
    let mut wd_nwd_master_excel = open_workbook_auto(config_params.wd_nwd_master())
        .expect("Unable to open WD/NWD Master File.");
    if let Some(Ok(reader)) = wd_nwd_master_excel.worksheet_range(config_params.wd_nwd_sheet()) {
        for row in reader.rows().skip(1) {
            wd_nwd_map.insert(row[0].to_string(), row[1].to_string());
        }
    }
    let mut accounts: Vec<Account> = Vec::new();
    let mut acc_enc = 0;
    let mut acc_succ = 0;
    let mut ip_amt = 0.0;
    let mut op_amt = 0.0;
    let mut output_rows = 0;

    let files_config = config::get_files(config_params.config_file_path());
    for file in files_config.files {
        let mut exrt_map: HashMap<String, f64> = HashMap::new();
        let exrt_file_path = get_file_path(file.exrt_file_path, *config_params.as_on_date());
        let exrt_file = read_file(&exrt_file_path);
        for (line_num, lines) in exrt_file.lines().enumerate() {
            let line = extract_lines(line_num, lines, &exrt_file_path);
            let fields: Vec<&str> = line.split('|').collect();
            let key: String = fields[0].to_string() + &"|".to_string() + &fields[1].to_string();
            let val: f64 = fields[2]
                .to_string()
                .parse::<f64>()
                .expect("could not read exchange rate");
            exrt_map.insert(key, val);
        }

        let keys = AccFieldNames::new_from_path(&file.required_fields_file_path);
        let input_file_path = get_file_path(file.input_file_path, *config_params.as_on_date());
        let mut account_reader =
            reader::Reader::new_at_path(&file.metadata_file_path, &input_file_path);
        let mut input_reader =
            reader::Reader::new_at_path(&file.metadata_file_path, &input_file_path);
        let rules = AggRules::new_from_path(&file.acc_skip_rules_path, &account_reader);
        let mut output_map: HashMap<LLGKey, Val> = HashMap::new();

        for account in account_reader.iter() {
            let ccy = match get_field_value(&account, &input_reader, keys.ccy.to_string()) {
                Ok(val) => val,
                Err(_err) => panic!("{}", _err),
            };

            ip_amt += match get_field_value(&account, &input_reader, keys.bal_lcy.to_string()) {
                Ok(val) => to_f64(val),
                Err(_err) => panic!("{}", _err),
            };
            let mut exrt = 1.0;
            if !file.is_consolidated {
                let key = ccy.to_string() + &"|".to_string() + &config_params.consol_ccy();
                exrt = *exrt_map.get(&key).unwrap_or(&1.0);
            }
            if file.is_exclusion_rules_required && skip_account(&account, &rules) {
                continue;
            }
            acc_enc += 1;
            let acc_data: AccData = grp_key::fetch_acc_data(
                &mut alco_map,
                &mut org_tenor_map,
                &mut res_tenor_map,
                &mut ia_tenor_map,
                &mut bucket_map,
                &mut cat_map,
                &mut lcr_map,
                &mut wd_nwd_map,
                &exrt,
                &file.source,
                account,
                &mut input_reader,
                &keys,
                &config_params,
            );
            output_map
                .entry(acc_data.grp_key.clone())
                .and_modify(|data| data.append_data(acc_data.data.clone()))
                .or_insert(acc_data.data);
            acc_succ += 1;
        }
        for (key, data) in output_map.drain() {
            op_amt += data.amt;
            write!(output_file, "{}|{}", key, data).expect("Unable to write summary file.");
            output_rows += 1;
            let mut opstr: String = String::new();
            opstr.push_str(&key.to_string());
            opstr.push('|');
            opstr.push_str(&data.to_string());
            let fields: Vec<&str> = opstr.split("|").collect();
            accounts.push(create_cf_acc(&fields));
        }
        for acc in accounts.iter() {
            cf_writer.write(acc.clone());
        }
    }
    //Adding footer to the output file.
    let timestamp = Local::now().naive_local().to_string();
    let as_on_date = *config_params.as_on_date();
    let footer = format!(
        "FTR|{}|{}|{}\n",
        as_on_date.format("%d-%m-%Y"),
        timestamp,
        output_rows
    );
    write!(output_file, "{}", footer).expect("Unable to write footer to summary file.");
    let health_report = HealthReport::new(acc_enc, acc_succ, acc_enc - acc_succ, ip_amt, op_amt, 0);
    health_report.gen_health_rpt(&config_params.output_file_path());
    let total_duration = print_return_time_since!(start_time);
    log_info!(logger, "Total time for aggregation: {:?}", total_duration);
}

pub fn skip_account(account: &AccountWithCFs, rules: &AggRules) -> bool {
    let skip_field = match rules.llg_for_acc(account) {
        Some(_) => true,
        None => false,
    };
    skip_field
}
