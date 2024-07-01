mod account_as_cashflows;
mod account_writer;
mod cashflow_appender;
mod structs;

use self::cashflow_appender::append_data;
use self::structs::LCBGMasterFields;
use calamine::{open_workbook, Reader, Xlsx};
use cashflow_generator::account_writer::AccountWriter;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
// use sdb_dyn_proto_rdr::reader;
use slog::Logger;
use std::collections::HashMap;

pub fn generate(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut writer = AccountWriter::new(&config_params.output_file_path(), log);

    let mut tot_rec = 0;
    let mut skp_rec = 0;
    let mut tot_amt = 0.0;
    let mut counter = 0;
    let mut ref_file: Xlsx<_> = open_workbook(config_params.ref_file_path()).unwrap();
    let mut ref_map: HashMap<String, LCBGMasterFields> = HashMap::new();
    if let Some(Ok(reader)) = ref_file.worksheet_range(config_params.master_sheet_name()) {
        for row in reader.rows() {
            if counter == 0 {
                counter += 1;
                continue;
            }
            let master_fields = LCBGMasterFields {
                pay_on_demand: row[1].to_string(),
                backed_by_td: row[2].to_string(),
                trade_non_trade: row[3].to_string(),
                td_exp_dt: row[4].to_string(),
            };
            let reference = row[0].to_string();
            ref_map.insert(reference, master_fields);
        }
    }
    counter = 0;
    let mut input_file: Xlsx<_> = open_workbook(config_params.input_file_path()).unwrap();
    if let Some(Ok(reader)) = input_file.worksheet_range(config_params.input_sheet_name()) {
        tot_rec += 1;
        for row in reader.rows() {
            if counter == 0 || counter == 1 {
                counter += 1;
                continue;
            }
            if !ref_map.contains_key(&row[2].to_string()) {
                skp_rec += 1;
                continue;
            }

            let master_values = ref_map.get(&row[2].to_string()).expect(&format!(
                "Could not get master values for reference : {}",
                row[2]
            ));
            tot_amt += row[10].to_string().parse::<f64>().unwrap_or(0.0);
            let account_data = append_data(
                row,
                &master_values,
                config_params.as_on_date(),
                config_params,
            );
            writer.write(account_data);
        }
    }

    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    log_info!(log, "{}", health_report.display());
    health_report.gen_health_rpt(&config_params.output_file_path());
}
