use self::account::*;
use self::io::*;
use self::agg_key::AGGKey;
use aggregator::account_field_names::AccFieldNames;
use calamine::{open_workbook_auto, Reader};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_dyn_proto_rdr::reader;
// use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::Write;
use std::time::SystemTime;
mod account;
mod account_field_names;
pub mod config;
mod io;
mod agg_key;

pub fn aggregate_cashflows(
    config_params: ConfigurationParameters,
    logger: &Logger,
    _diag_logger: &Logger,
) {
    let start_time = SystemTime::now();
    let mut output_file = get_writer(config_params.output_file_path());
    let mut acc_enc = 0;
    let mut acc_succ = 0;
    let mut avg_bal = 0.0;
    let mut eop_bal = 0.0;

    let mut aggr_map: HashMap<AGGKey, AggrVal> = HashMap::new();
    let files_config = config::get_files(config_params.config_file_path());
    for file in files_config.files {
        //Read The master file 1
        let mut master_map_1: HashMap<String, String> = HashMap::new();
        let master_file_path_1 =
            get_file_path(file.master_file_path_1, *config_params.as_on_date());
        let mut master_excel_1 = open_workbook_auto(master_file_path_1)
            .expect("Unable to open PSL/Non PSL Master File.");
        if let Some(Ok(reader)) = master_excel_1.worksheet_range(&file.sheet_name_1) {
            for row in reader.rows().skip(1) {
                master_map_1.insert(
                    row[0].to_string(),
                    row[file.div_master_pos.parse().unwrap_or(0)-1].to_string(),
                );
            }
        }
        //Read The master file 2
        let mut master_map_2: HashMap<String, String> = HashMap::new();
        let master_file_path_2 =
            get_file_path(file.master_file_path_2, *config_params.as_on_date());
        let mut master_excel_2 = open_workbook_auto(master_file_path_2)
            .expect("Unable to open PSL/Non PSL Master File.");
        if let Some(Ok(reader)) = master_excel_2.worksheet_range(&file.sheet_name_2) {
            for row in reader.rows().skip(1) {
                master_map_2.insert(
                    row[0].to_string(),
                    row[3].to_string(),
                );
            }
        }
        //Read the amb reference file
        let mut amb_map: HashMap<String, f64> = HashMap::new();
        let amb_file_path = get_file_path(file.amb_file_path, *config_params.as_on_date());
        let amb_file = read_file(&amb_file_path);
        for (line_num, lines) in amb_file.lines().enumerate() {
            let line = extract_lines(line_num, lines, &amb_file_path);
            let fields: Vec<&str> = line.split('|').collect();
            let key: String = fields[0].to_string();
            let val: f64 = fields[file.amb_pos.parse().unwrap_or(0)-1].to_string().parse().unwrap_or(0.0);
            amb_map.insert(key, val);
        }
        let keys = AccFieldNames::new_from_path(&file.required_fields_file_path);
        let input_file_path = get_file_path(file.input_file_path, *config_params.as_on_date());
        let mut account_reader =
            reader::Reader::new_at_path(&file.metadata_file_path, &input_file_path);
        for mut account in account_reader.iter() {
            let division_code = master_map_1
                .get(
                    &account
                        .get_string_for_key(&keys.division_code)
                        .unwrap_or(&"NA".to_string())
                        .to_string(),
                )
                .unwrap_or(&"NA".to_string())
                .to_string();
            let psl_cat = master_map_2
                .get(
                    &account
                        .get_string_for_key(&keys.psl_catagory)
                        .unwrap_or(&"NA".to_string())
                        .to_string(),
                )
                .unwrap_or(&"NA".to_string())
                .to_string();
            // get the Average Balance from amb file
            let avg_bal = *amb_map
                .get(
                    &account
                        .get_string_for_key(&keys.acc_no)
                        .unwrap_or(&"NA".to_string())
                        .to_string(),
                )
                .unwrap_or(&0.0);
            // get the EOP Balance from base input file 
            let cashflows = &account
                            .remove_cfs_for_key(&keys.eop_bal)
                            .expect("Error while removing cashflow from the pool of cashflows.");
            let mut prin_amount = 0.0;
            for cf in cashflows.iter() {
                        prin_amount += cf.get_principal_amount();
            }
            let eop_bal=prin_amount;
            acc_enc += 1;
            let acc_key = AGGKey::new(
                config_params.as_on_date().format("%d-%m-%Y").to_string(),
                file.source.clone(),
                psl_cat,
                division_code,
            );
            let acc_data = AggrVal { avg_bal,eop_bal };
            aggr_map
                .entry(acc_key)
                .and_modify(|data| data.append_data(acc_data.clone()))
                .or_insert(acc_data);
            acc_succ += 1;
        }
    }
    for (key, data) in aggr_map.drain() {
        avg_bal += data.avg_bal;
        eop_bal += data.eop_bal;
        write!(output_file, "{}|{}", key, data).expect("Unable to write summary file.");
    }
    let health_report =
        HealthReport::new(acc_enc, acc_succ, acc_enc - acc_succ, avg_bal, eop_bal, 0);
    health_report.gen_health_rpt(config_params.output_file_path());
    let total_duration = print_return_time_since!(start_time);
    log_info!(logger, "Total time for aggregation: {:?}", total_duration);
}

