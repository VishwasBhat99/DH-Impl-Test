use self::account::AccData;
use self::account::*;
use self::io::*;
use self::llg_key::LLGKey;
use crate::aggregator::rate_bucket::*;
use aggregator::account_field_names::AccFieldNames;
use aggregator::tenor::*;
use calamine::{open_workbook_auto, Reader};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::Write;
use std::time::SystemTime;

mod account;
mod account_field_names;
pub mod config;
mod grp_key;
mod io;
mod llg_key;
mod rate_bucket;
mod tenor;

pub fn aggregate_cashflows(
    config_params: ConfigurationParameters,
    logger: &Logger,
    _diag_logger: &Logger,
) {
    let start_time = SystemTime::now();
    let mut output_file = get_writer(config_params.output_file_path());

    let mut tenor_map: HashMap<Tenor, String> = HashMap::new();
    let mut tenor_master_excel = open_workbook_auto(config_params.tenor_master())
        .expect("Unable to open Tenor Master File.");
    if let Some(Ok(reader)) = tenor_master_excel.worksheet_range(config_params.tenor_sheet()) {
        for row in reader.rows() {
            let tenor = Tenor::new(row[0].to_string(), row[1].to_string());
            tenor_map.insert(tenor, row[2].to_string());
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

    let mut psl_map: HashMap<String, String> = HashMap::new();
    let mut psl_master_excel = open_workbook_auto(config_params.psl_master())
        .expect("Unable to open PSL/Non PSL Master File.");
    if let Some(Ok(reader)) = psl_master_excel.worksheet_range(config_params.psl_sheet()) {
        for row in reader.rows().skip(1) {
            psl_map.insert(row[0].to_string(), row[1].to_string());
        }
    }
    let mut rate_bucket_map: &mut HashMap<rate_bucket::Bucket, std::string::String> =
        &mut HashMap::new();
    let mut rate_bucket_master_excel = open_workbook_auto(config_params.rate_bucket_master())
        .expect("Unable to open Rate Bucket Master File.");
    if let Some(Ok(reader)) =
        rate_bucket_master_excel.worksheet_range(config_params.rate_bucket_sheet())
    {
        for row in reader.rows().skip(1) {
            if config_params.is_perf_diagnostics_enabled() {
                info!(logger, "Reading row: {:?} ", row);
            }
            let rate_bucket = Bucket::new(row[0].to_string(), row[1].to_string());
            rate_bucket_map.insert(rate_bucket, row[2].to_string());
        }
    }

    info!(
        _diag_logger,
        "Read and Stored {} Records from Rate-Bucket File.",
        rate_bucket_map.len()
    );

    if config_params.is_perf_diagnostics_enabled() {
        for (k, v) in rate_bucket_map.iter() {
            info!(_diag_logger, "Range: {:?} and Bucket: {}", k, v);
        }
    }

    let mut acc_enc = 0;
    let mut acc_succ = 0;
    let mut ip_amt = 0.0;
    let mut op_amt = 0.0;
    let mut aggr_map: HashMap<LLGKey, AggrVal> = HashMap::new();

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
        let rules = AggRules::new_from_path(&file.acc_skip_rules_path, &account_reader);

        for mut account in account_reader.iter() {
            let ccy = account
                .get_string_for_key(&keys.ccy)
                .unwrap_or(&String::from("INR"))
                .to_string();
            let mut cf_amt = 0.0;
            let is_cf_passed = !keys.cashflows.is_empty();
            if is_cf_passed {
                let cashflows = match account.remove_cfs_for_key(&keys.cashflows) {
                    Ok(value) => value,
                    Err(err) => {
                        log_info!(
                            logger,
                            "Account: {} \n Error while removing cashflow from the pool of cashflows.{:#?}",
                            account
                                .get_string_for_key(&keys.prod_code)
                                .unwrap_or(&"".to_string()),
                            err
                        );
                        Vec::new()
                    }
                };
                for cf in cashflows {
                    cf_amt += cf.get_principal_amount();
                }
            }
            if is_cf_passed {
                ip_amt += cf_amt;
            } else {
                ip_amt += account.get_f64_for_key(&keys.amt).unwrap_or(0.0);
            }

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
                &mut alco_map,
                &mut psl_map,
                rate_bucket_map,
                file.is_neg,
                logger,
                is_cf_passed,
                cf_amt,
            );

            aggr_map
                .entry(acc_data.grp_key.clone())
                .and_modify(|data| data.append_data(acc_data.aggr_data.clone()))
                .or_insert(acc_data.aggr_data);
            acc_succ += 1;
        }
    }
    let header = "AS ON DATE|SOURCE|CCY|PRODUCT CODE|SCHEME ID|MIS1|MIS2|MIS3|RAW BM|FINAL BM|CONCAT|NPA FLAG|DIVISION|ALM LINE|IA LINE|ORIGINAL TENOR|ALCO MAPPING|PSL/NON PSL|RATE BUCKET|AMOUNT|YIELD RATE\n";
    write!(output_file, "{}", header).expect("Unable to write summary file.");
    for (key, data) in aggr_map.drain() {
        op_amt += data.tot_amt;
        write!(output_file, "{}|{}", key, data).expect("Unable to write summary file.");
    }
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
