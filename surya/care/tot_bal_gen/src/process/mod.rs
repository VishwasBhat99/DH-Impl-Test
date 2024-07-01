use self::writer::{write_aggr_smry, write_file_data};
use calamine::{open_workbook_auto, Reader};
use configuration_parameters::ConfigurationParameters;
use macros;
use slog::Logger;
use std::collections::HashMap;

mod config;
mod reader;
mod writer;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    // Read Files Configuration
    let mut acc_enc = 0;
    let mut acc_succ = 0;
    let mut tot_amt = 0.0;
    let files_config = config::get_files(config_params.input_config());
    let mut aggr_data: HashMap<String, ((f64, i64), (f64, i64))> = HashMap::new();
    let mut exp_base_map: HashMap<String, (f64, f64)> = HashMap::new();
    let mut exp_base_file_excel = open_workbook_auto(config_params.exp_base_file())
        .expect("Unable to open Base exposure reference File.");
    if let Some(Ok(reader)) =
        exp_base_file_excel.worksheet_range(config_params.exp_base_file_sheet_name())
    {
        for row in reader.rows().skip(1) {
            let key = row[2].to_string();
            let amt: f64 = row[7].to_string().parse().unwrap_or(0.0);
            let limit_amt: f64 = row[8].to_string().parse().unwrap_or(0.0);
            exp_base_map.insert(key, (amt, limit_amt));
        }
    }

    log_info!(
        logger,
        "Total no of base exposure accounts read: {}",
        exp_base_map.len()
    );
    log_debug!(logger, "\nExposure Data:\n{:#?}\n", exp_base_map);

    let mut tot_bal = 0.0;
    let mut limit_bal = 0.0;
    for file in files_config.files {
        // Read cashflow file
        let mut file_rdr: sdb_dyn_proto_rdr::reader::Reader =
            reader::read_file(&file.input_file_path, &file.metadata_file_path);
        let mut file_aggr_bal: HashMap<String, ((f64, i64), (f64, i64))> = HashMap::new();
        for account in file_rdr.iter() {
            acc_succ += 1;
            let cust_id: String = match account.get_string_for_key(&file.cust_id) {
                Ok(val) => val.to_string(),
                Err(_) => account
                    .get_i64_for_key(&file.cust_id)
                    .expect("Cannot read cust_id field.")
                    .to_string(),
            };
            let mut balance = match account.get_f64_for_key(&file.balance) {
                Ok(val) => val,
                Err(_) => {
                    let def = "0.0".to_string();
                    let amt = account.get_string_for_key(&file.balance).unwrap_or(&def);
                    amt.parse().unwrap_or(0.0)
                }
            };
            let mut limit_bal = 0.0;
            if file.is_negative {
                balance = balance * -1.0;
            }
            tot_amt += balance;
            if config_params.is_limit_required() {
                limit_bal = match account.get_f64_for_key(&file.limit_balance) {
                    Ok(val) => val,
                    Err(_) => {
                        let def = "0.0".to_string();
                        let limit_amt = account
                            .get_string_for_key(&file.limit_balance)
                            .unwrap_or(&def);
                        limit_amt.parse().unwrap_or(0.0)
                    }
                };
                if file.is_limit_negative {
                    limit_bal = limit_bal * -1.0;
                }
            }
            // for file level, used for recon
            file_aggr_bal
                .entry(cust_id.to_string())
                .and_modify(|((prev_bal, count), (limit_balance, limit_count))| {
                    *prev_bal += balance;
                    *count += 1;
                    *limit_balance += limit_bal;
                    *limit_count += 1;
                })
                .or_insert(((balance, 1), (limit_bal, 1)));
            // for total level, used for classification
            aggr_data
                .entry(cust_id.to_string())
                .and_modify(|((prev_bal, count), (limit_balance, limit_count))| {
                    *prev_bal += balance;
                    *count += 1;
                    *limit_balance += limit_bal;
                    *limit_count += 1;
                })
                .or_insert(((balance, 1), (limit_bal, 1)));
            acc_succ += 1;
        }
        // Write file level output
        let op_path = format!("{}-{}.txt", config_params.output_path(), file.src_name);
        let mut file_tot_bal = 0.0;
        let mut limit_tot_bal = 0.0;
        write_file_data(
            &mut file_aggr_bal,
            &mut file_tot_bal,
            &mut limit_tot_bal,
            &op_path,
            config_params.is_limit_required(),
        );
        log_info!(
            logger,
            "Total Balance for {}: {}, Total Limit Balance: {}",
            file.src_name,
            file_tot_bal,
            limit_tot_bal,
        );
    }
    // Write summary output
    let op_path = format!("{}-summary.txt", config_params.output_path());
    let cf_op_path = format!("{}-summary.cf", config_params.output_path());
    write_aggr_smry(
        &mut aggr_data,
        &mut tot_bal,
        &mut limit_bal,
        exp_base_map,
        &op_path,
        &cf_op_path,
        config_params.is_limit_required(),
    );
    log_info!(
        logger,
        "Total Balance in summary file: {}\nTotal Limit Balance in summary file: {}",
        tot_bal,
        limit_bal
    );
    let health_stat = health_report::HealthReport::new(
        acc_enc,
        acc_succ,
        acc_enc - acc_succ,
        tot_amt,
        tot_amt,
        0,
    );
    health_stat.gen_health_rpt(config_params.output_path())
}
