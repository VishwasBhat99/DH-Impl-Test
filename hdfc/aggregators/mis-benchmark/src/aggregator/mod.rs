use self::account::AccData;
use self::llg_key::LLGKey;
use self::util::add_to_prev_data;
use self::util::get_days;
use aggregator::account_field_names::AccFieldNames;
use aggregator::tenor::*;
use calamine::{open_workbook, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_dyn_proto_rdr::reader;
use sdb_io::open_file_read;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use std::collections::HashMap;
use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Write;
use std::time::SystemTime;

mod account;
mod account_field_names;
pub mod config;
mod grp_key;
mod llg_key;
mod tenor;
mod util;

pub fn aggregate_cashflows(
    config_params: ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) {
    let start_time = SystemTime::now();
    let mut output_file = match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&config_params.output_file_path())
    {
        Ok(create) => create,
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

    let def_val = "NA".to_string();

    let mut tenor_map: HashMap<Tenor, String> = HashMap::new();
    let mut tenor_file: Xlsx<_> =
        open_workbook(&config_params.org_tenor_file_path()).expect("Unable to open `Tenor File`.");
    let tenor_sheet_name = tenor_file
        .sheet_names()
        .first()
        .expect("excel is empty")
        .to_owned();
    if let Some(Ok(reader)) = tenor_file.worksheet_range(tenor_sheet_name.as_str()) {
        for tenor_fields in reader.rows() {
            let key: Tenor = Tenor::new(tenor_fields[0].to_string(), tenor_fields[1].to_string());
            let val: String = tenor_fields[2].to_string();
            tenor_map.insert(key, val);
        }
    }

    let mut acc_enc = 0;
    let mut acc_succ = 0;
    let mut ip_amt = 0.0;
    let mut op_amt = 0.0;
    let mut aggr_data: HashMap<LLGKey, Vec<f64>> = HashMap::new();

    let files_config = config::get_files(config_params.config_file_path());
    for file in files_config.files {
        let reader_for_calling_method =
            reader::Reader::new_at_path(&file.metadata_file_path, &file.input_file_path);
        let mut required_fields_file = open_file_read(&file.required_fields_file_path)
            .expect("Cannot open the required fields file.");

        let mut required_fields_buffer = String::new();
        required_fields_file
            .read_to_string(&mut required_fields_buffer)
            .expect("Cannot read the required fields file.");

        let mut exrt_map: HashMap<String, f64> = HashMap::new();
        let exrt_file = match new_buf_rdr(&file.exrt_file_path) {
            Ok(file) => file,
            Err(error) => panic!(
                "Could not found exrt file: `{}`  : {}.",
                file.exrt_file_path, error
            ),
        };
        for (line_num, lines) in exrt_file.lines().enumerate() {
            let line = match lines {
                Ok(line) => line,
                Err(error) => panic!(
                    "Unable to read file `{}` at line number: `{}` : {}",
                    file.exrt_file_path,
                    line_num + 1,
                    error
                ),
            };
            let fields: Vec<&str> = line.split('|').collect();
            let key: String = fields[0].to_string() + &"|".to_string() + &fields[1].to_string();
            let val: f64 = fields[2]
                .to_string()
                .parse::<f64>()
                .expect("could not read exchange rate");
            exrt_map.insert(key, val);
        }
        let keys = AccFieldNames::new_from_path(&file.required_fields_file_path);

        let mut account_reader =
            reader::Reader::new_at_path(&file.metadata_file_path, &file.input_file_path);

        let bkt_info = config_params.bkt_scheme();
        let bkts: Vec<&str> = bkt_info.split(',').collect();
        let mut num_day_bkts = Vec::new();
        for val in bkts {
            let num_days = get_days(val, config_params.as_on_date());
            num_day_bkts.push(num_days);
        }
        let rules = AggRules::new_from_path(&file.acc_skip_rules_path, &account_reader);
        for account in account_reader.iter() {
            let ccy = account.get_string_for_key(&keys.ccy).unwrap_or(&def_val);
            let tot_amt = account.get_f64_for_key(&keys.tot_amt).unwrap_or(0.0);
            let mut exrt = 1.0;
            if !file.is_consolidated {
                let key = ccy.to_string() + &"|".to_string() + &config_params.consol_ccy();
                exrt = *exrt_map.get(&key).unwrap_or(&1.0);
            }
            if file.is_exclusion_rules_required && skip_account(&account, &rules){
                continue;
            }
            acc_enc += 1;
            let acc_data: AccData = grp_key::fetch_acc_data(
                &reader_for_calling_method,
                &file,
                &tenor_map,
                &exrt,
                file.source.to_string(),
                account,
                &keys,
                &num_day_bkts,
                &config_params,
                logger,
                &file.is_maturity,
                &tot_amt,
                &file.is_negative,
            );
            for val in &acc_data.acc_data {
                ip_amt += val;
            }
            if aggr_data.contains_key(&acc_data.grp_key) {
                let prev_data: Vec<f64> = aggr_data
                    .remove_entry(&acc_data.grp_key)
                    .expect("Unexpected unwrap error.")
                    .1;
                let accnt_data: Vec<f64> = acc_data.acc_data;
                let new_data: Vec<f64> = add_to_prev_data(prev_data, &accnt_data);
                aggr_data.insert(acc_data.grp_key, new_data);
            } else {
                aggr_data.insert(acc_data.grp_key, acc_data.acc_data);
            }
            acc_succ += 1;
        }
    }
    for (key, data) in aggr_data.drain() {
        write!(output_file, "{}", key).expect("Unable to write key to summary file.");
        let mut data_op = String::new();
        let mut tot_amt: f64 = 0.0;
        for val in data {
            op_amt += val;
            tot_amt += val;
            data_op.push_str(&val.to_string());
            data_op.push_str("|");
        }
        data_op.push_str(&tot_amt.to_string());
        data_op.push_str("\n");
        write!(output_file, "{}", data_op).expect("Unable to write data to summary file.");
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
