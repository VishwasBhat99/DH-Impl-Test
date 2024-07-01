use self::derive_fields::{derive_moc_fields::*, get_alm_master_data};
use self::io::*;
use self::structs::{alm_master::*, moc_input_account::MocInputAccount};
use calamine::{open_workbook, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use slog::Logger;
use statics::*;
use std::collections::HashMap;
use std::time::SystemTime;

mod derive_fields;
mod io;
mod structs;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let st_tm_read = SystemTime::now();
    let mut op_line: String = String::new();
    let mut tot_rec = DEFAULT_INT;
    let skp_rec = DEFAULT_INT;

    let mut alm_master: HashMap<AlmMasterKey, AlmMaster> = HashMap::new();
    let mut alm_master_excel: Xlsx<_> =
        open_workbook(config_param.alm_master()).expect("Unable to open Alm Master File.");
    if let Some(Ok(reader)) = alm_master_excel.worksheet_range(config_param.alm_master_sheet_name())
    {
        for row in reader.rows() {
            get_alm_master_data(row, &mut alm_master);
        }
    }

    let mut concats = String::new();
    let mut tot_amt = 0.0;
    let mut moc_input_account = MocInputAccount::new();
    let mut moc_excel: Xlsx<_> =
        open_workbook(config_param.moc_input_file_path()).expect("Unable to open Moc Input File.");
    if let Some(Ok(reader)) = moc_excel.worksheet_range(config_param.moc_sheet_name()) {
        for row in reader.rows().skip(1) {
            moc_input_account.insert(&row);
            tot_rec += 1;

            tot_amt += moc_input_account
                .amt
                .trim()
                .parse()
                .unwrap_or(DEFAULT_FLOAT);

            op_line.push_str(&get_moc_op_line(
                &mut moc_input_account,
                &mut alm_master,
                &mut concats,
            ));
        }
    }
    let ed_tm_read = SystemTime::now();
    let duration = ed_tm_read
        .duration_since(st_tm_read)
        .expect("Could not calculate total read process duration.");
    debug!(diag_log, "Read Process Total Duration: {:?}.", duration);

    let st_tm_writer = SystemTime::now();
    let mut op_writer = get_writer(config_param.output_file_path());
    output_writer(&mut op_writer, op_line, config_param.output_file_path());

    let mut concat_writer = get_writer(config_param.concat_file_path());
    output_writer(&mut concat_writer, concats, config_param.concat_file_path());

    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    log_info!(log, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_param.output_file_path());

    let ed_tm_writer = SystemTime::now();
    let duration = ed_tm_writer
        .duration_since(st_tm_writer)
        .expect("Could not calculate total duration for write process.");
    debug!(diag_log, "Writing GL MOC, Total Duration: {:?}.", duration);
}
