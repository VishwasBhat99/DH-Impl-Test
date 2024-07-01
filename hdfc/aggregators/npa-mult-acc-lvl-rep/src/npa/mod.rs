use self::account::AccData;
use self::account::*;
use self::io::*;
use self::llg_key::LLGKey;
use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use crate::npa::account_field_names::AccFieldNames;
use crate::npa::tenor::*;
use calamine::{open_workbook_auto, Reader};
use chrono::Local;
use health_report::HealthReport;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::io::Write;
use std::time::SystemTime;
mod account;
mod account_field_names;
pub mod config;
mod grp_key;
mod io;
mod llg_key;
mod tenor;

pub fn npa_calc(config_params: ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let start_time = SystemTime::now();
    let mut output_file = get_writer(config_params.output_file_path());

    let npa_prev_reader =
        fs::read_to_string(&config_params.npa_prev_master()).expect("Failed to read input file!");
    let mut npa_prev_map: HashMap<String, Vec<&str>> = HashMap::new();
    for line in npa_prev_reader.lines() {
        let npa_prev_fields = line.split(",").collect::<Vec<&str>>();
        npa_prev_map.insert(npa_prev_fields[1].to_string(), npa_prev_fields);
    }

    let npa_curr_reader =
        fs::read_to_string(&config_params.npa_curr_master()).expect("Failed to read input file!");
    let mut npa_curr_map: HashMap<String, Vec<&str>> = HashMap::new();
    for line in npa_curr_reader.lines() {
        let npa_curr_fields = line.split(",").collect::<Vec<&str>>();
        npa_curr_map.insert(npa_curr_fields[1].to_string(), npa_curr_fields);
    }

    let mut npa_flag_map: HashMap<String, String> = HashMap::new();
    for key in npa_curr_map.keys() {
        if npa_prev_map.contains_key(key) {
            npa_flag_map.insert(key.to_string(), "NO".to_string());
        } else {
            npa_flag_map.insert(key.to_string(), "YES".to_string());
        }
    }

    let mut alco_map: HashMap<String, String> = HashMap::new();
    let mut alco_master_excel =
        open_workbook_auto(config_params.alco_master()).expect("Unable to open ALCO Master File.");
    if let Some(Ok(reader)) = alco_master_excel.worksheet_range(config_params.alco_sheet()) {
        for row in reader.rows().skip(1) {
            alco_map.insert(row[0].to_string(), row[1].to_string());
        }
    }

    let mut tenor_map: HashMap<Tenor, String> = HashMap::new();
    let mut tenor_master_excel = open_workbook_auto(config_params.tenor_master())
        .expect("Unable to open Tenor Master File.");
    if let Some(Ok(reader)) = tenor_master_excel.worksheet_range(config_params.tenor_sheet()) {
        for row in reader.rows() {
            let tenor = Tenor::new(row[0].to_string(), row[1].to_string());
            tenor_map.insert(tenor, row[2].to_string());
        }
    }

    let mut psl_map: HashMap<String, String> = HashMap::new();
    let mut psl_master_excel = open_workbook_auto(config_params.psl_master())
        .expect("Unable to open PSL/Non PSL Master File.");
    if let Some(Ok(reader)) = psl_master_excel.worksheet_range(config_params.psl_sheet()) {
        for row in reader.rows().skip(1) {
            psl_map.insert(row[0].to_string(), row[1].to_string());
        }
    }

    let mut acc_enc = 0;
    let mut acc_succ = 0;
    let mut ip_amt = 0.0;
    let mut op_amt = 0.0;
    let mut rec_count = 0;
    let header="As On Date|Source system|Account Number|CCY|Product Code|Scheme ID|Incremental Flag|MIS1|MIS2|MIS3|Raw BM|Final BM|Concat|NPA Flag|Division|ALM Line|IA Line|Org Tenor|ALM Grouping|PSL/Non PSL|Amount as per NPA File|Amount as per source|Yield|Unservised interest suspense|Unservised other charges / incomes|Date of NPA\n";
    write!(output_file, "{}", header).expect("Unable to write header to summary file.");
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
        let mut npa_map: HashMap<LLGKey, NpaVal> = HashMap::new();
        let mut npa_amt_map: HashMap<String, f64> = HashMap::new();
        let mut npa_unser_int_sus_map: HashMap<String, String> = HashMap::new();
        let mut npa_unser_oth_inc_map: HashMap<String, String> = HashMap::new();
        let mut npa_date_map: HashMap<String, String> = HashMap::new();
        let keys = AccFieldNames::new_from_path(&file.required_fields_file_path);
        let input_file_path = get_file_path(file.input_file_path, *config_params.as_on_date());
        let mut account_reader =
            reader::Reader::new_at_path(&file.metadata_file_path, &input_file_path);
        let rules = AggRules::new_from_path(&file.acc_skip_rules_path, &account_reader);

        for account in account_reader.iter() {
            let acc_no = account
                .get_string_for_key(&keys.acc_no)
                .unwrap_or(&String::from("NA"))
                .to_string();
            if !npa_curr_map.contains_key(&acc_no) {
                continue;
            }
            let ccy = account
                .get_string_for_key(&keys.ccy)
                .unwrap_or(&String::from(config_params.consol_ccy()))
                .to_string();

            if npa_curr_map.contains_key(&acc_no) {
                npa_amt_map.insert(
                    acc_no.to_string(),
                    npa_curr_map
                        .get(&acc_no)
                        .expect("error getting npa_amt from npa_curr_map")[2]
                        .to_string()
                        .parse::<f64>()
                        .unwrap_or(0.0),
                );
                npa_unser_int_sus_map.insert(
                    acc_no.to_string(),
                    npa_curr_map
                        .get(&acc_no)
                        .expect("error getting npa_unser_ints_sus from npa_curr_map")[3]
                        .to_string(),
                );
                npa_unser_oth_inc_map.insert(
                    acc_no.to_string(),
                    npa_curr_map
                        .get(&acc_no)
                        .expect("error getting npa_unser_oth_inc from npa_curr_map")[4]
                        .to_string(),
                );
                npa_date_map.insert(
                    acc_no.to_string(),
                    npa_curr_map
                        .get(&acc_no)
                        .expect("error getting npa_date from npa_curr_map")[5]
                        .to_string(),
                );
            }

            ip_amt += account.get_f64_for_key(&keys.amt_as_per_src).unwrap_or(0.0);
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
                &tenor_map,
                &exrt,
                &file.source,
                account,
                &keys,
                &config_params,
                &mut npa_flag_map,
                &mut alco_map,
                &mut psl_map,
                &mut npa_amt_map,
                &mut npa_unser_int_sus_map,
                &mut npa_unser_oth_inc_map,
                &mut npa_date_map,
                file.is_neg,
            );
            npa_map
                .entry(acc_data.grp_key.clone())
                .and_modify(|data| data.append_data(acc_data.npa_data.clone()))
                .or_insert(acc_data.npa_data);
            acc_succ += 1;
        }
        for (key, data) in npa_map.drain() {
            op_amt += data.amt_as_per_src;
            write!(output_file, "{}|{}", key, data).expect("Unable to write summary file.");
            rec_count += 1;
        }
    }

    let timestamp = Local::now().naive_local().to_string();
    let footer = format!(
        "FTR|{}|{}|{}\n",
        config_params.as_on_date().format("%d-%m-%Y"),
        timestamp,
        rec_count
    );
    write!(output_file, "{}", footer).expect("Unable write footer to summary file.");

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
