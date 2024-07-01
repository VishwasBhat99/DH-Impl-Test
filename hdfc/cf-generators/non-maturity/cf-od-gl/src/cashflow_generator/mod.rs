use chrono::Datelike;
use sdb_dyn_proto_rdr::reader;
use slog::Logger;
mod account_without_cashflows;
mod account_without_cashflows_writer;
use calamine::{open_workbook_auto, Reader, Sheets};
use cashflow_generator::account_without_cashflows_writer::AccountWithOutCashflowsWriter;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::is_month_end_date;
use statics::*;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, Read};
use std::time::SystemTime;
use std::{fs::File, io::BufReader};

use crate::cashflow_generator::account_without_cashflows::Account;
mod structs;
use cashflow_generator::structs::{AccFieldNames, Currency};
pub fn generate(config_params: &ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let mut total_accounts_encountered: i64 = DEFAULT_INT;
    let total_accounts_with_cashflows: i64 = DEFAULT_INT;
    let total_cfs: usize = 0;
    let mut tot_prin_in_in = DEFAULT_FLOAT;
    let mut tot_prin_in_op = DEFAULT_FLOAT;
    let tot_int_in_op = DEFAULT_FLOAT;

    let start_generate_timer = SystemTime::now();
    let mut writer = create_io_workers(config_params.output_file_path(), log);
    let mut account_reader = reader::Reader::new_at_path(
        config_params.metadata_file_path(),
        config_params.input_cf_file_path(),
    );
    let vpa_line = fs::read_to_string(config_params.vp_npa_file_path())
        .expect("Not able to read the vpa_npa_file");
    let vp_npa_amount = vpa_line.trim().parse().unwrap_or(0.000);
    let mut od_study_map: HashMap<String, Vec<f64>> = HashMap::new();
    let mut od_study_master: Sheets<BufReader<File>> =
        open_workbook_auto(config_params.od_study_master_file_path())
            .expect("Error while opening `od_study_master` file.");
    if let Some(Ok(reader)) =
        od_study_master.worksheet_range(&config_params.od_study_master_sheet_name())
    {
        for row in reader.rows() {
            od_study_map.insert(
                row[0].to_string(),
                vec![
                    row[1].to_string().parse().unwrap_or(0.0000),
                    row[2].to_string().parse().unwrap_or(0.0000),
                    row[3].to_string().parse().unwrap_or(0.0000),
                ],
            );
        }
    }
    let default_distribution = vec![0.0000, 0.0000, 0.0000];
    let mut distribution_vec = &default_distribution;
    let mut total_amount = 0.00;
    let currency_map: HashMap<Currency, f64> =
        Currency::new(config_params.exchange_rate_file_path());
    for account in account_reader.iter() {
        total_accounts_encountered += 1;
        let keys = AccFieldNames::new_from_path(config_params.req_fields_file_path());
        let from_ccy = account
            .get_string_for_key(&keys.ccy)
            .expect("Could not read currency");
        let to_ccy: String = config_params.base_currency().to_string();
        let alm_line: String = account
            .get_string_for_key(&keys.alm_line)
            .unwrap_or(&"NONE".to_string())
            .to_string();
        let is_acct_gl: String = account
            .get_string_for_key(&keys.is_acct_gl)
            .unwrap_or(&"NONE".to_string())
            .to_string();
        let bal_total: f64 = account.get_f64_for_key(&keys.bal_total).unwrap_or(0.0);
        let ex_rt = currency_map
            .get(&Currency::get_key(&from_ccy, &to_ccy))
            .unwrap_or(&1.0);

        let actual_exrt: f64 = if *config_params.is_consolidated() {
            1.0 / ex_rt
        } else {
            *ex_rt
        };
        tot_prin_in_in += bal_total;
        if is_acct_gl.trim().to_uppercase() == "N".to_string()
            && config_params
                .alm_line()
                .contains(&alm_line.trim().to_string())
        {
            total_amount += bal_total * actual_exrt;
        }
    }
    tot_prin_in_op += total_amount - vp_npa_amount;
    let dist_amount = total_amount - vp_npa_amount;
    let mut out_acc = Account::new();
    if is_month_end_date(*config_params.as_on_date()) {
        distribution_vec = od_study_map.get("EOM").unwrap_or(&default_distribution);
    } else {
        distribution_vec = od_study_map
            .get(&config_params.as_on_date().day().to_string())
            .unwrap_or(&default_distribution);
    }
    out_acc.as_on_date = rbdate::timestamp(*config_params.as_on_date());
    out_acc.b1 = dist_amount * distribution_vec[0] / 100.00;
    out_acc.b2 = dist_amount * distribution_vec[1] / 100.00;
    out_acc.b3 = dist_amount * distribution_vec[2] / 100.00;
    writer.write(out_acc);
    writer.close();

    let end_generate_timer = SystemTime::now();
    let total_duration = end_generate_timer
        .duration_since(start_generate_timer)
        .expect("Could not calculate total duration for generate timer.");
    let report_string = format!(
        "Accounts Encountered: {}\n\
         Accounts With Cashflows: {}\n\
         Total Cashflows: {}\n\
         Total Duration: {:.2?}\n\
         Total outstanding amount in input: {:.4} \n\
         Total outstanding amount in output: {:.4}\n\
         Total interest amount in output: {:.4}",
        total_accounts_encountered,
        total_accounts_with_cashflows,
        total_cfs,
        total_duration,
        tot_prin_in_in,
        tot_prin_in_op,
        tot_int_in_op
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);

    let health_stat = HealthReport::new(
        total_accounts_encountered,
        total_accounts_with_cashflows,
        0,
        tot_prin_in_in,
        tot_prin_in_op,
        total_cfs as i64,
    );
    health_stat.gen_health_rpt(config_params.output_file_path())
}

fn create_io_workers(output_path: &str, log: &Logger) -> AccountWithOutCashflowsWriter {
    let output_path_str = format!("{}", output_path);
    let writer = AccountWithOutCashflowsWriter::new(&output_path_str, log);

    writer
}
