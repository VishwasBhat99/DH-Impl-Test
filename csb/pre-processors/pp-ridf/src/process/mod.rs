use self::derive_fields::*;
use self::io::*;
use self::reconcilation::*;
use self::structs::{alm_master::*, ridf::*};
use calamine::{open_workbook_auto, Reader};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use slog::Logger;
use std::collections::HashMap;
mod derive_fields;
mod io;
mod reconcilation;
mod structs;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let mut tot_rec = 0;
    let skp_rec = 0;

    let mut alm_master: AlmMasterMap = AlmMasterMap::new();
    let mut alm_master_excel =
        open_workbook_auto(config_params.alm_master()).expect("Unable to open Alm Master File.");
    if let Some(Ok(reader)) =
        alm_master_excel.worksheet_range(config_params.alm_master_sheet_name())
    {
        for row in reader.rows() {
            get_alm_master_data(row, &mut alm_master);
        }
    }

    let mut op_line_td: String = String::new();
    let mut ridf_reader = read_file(config_params.input_file_path());
    let mut ridf_data: RIDFData = RIDFData::new();
    let mut recon = ReconMap::new();
    let mut tot_amt = 0.0;
    for (line_num, lines) in ridf_reader.deserialize().enumerate().skip(1) {
        let mut ridf_input: RIDFInput =
            extract_lines(line_num, lines, config_params.input_file_path(), logger);
        get_cust_master_data(&mut ridf_input, &mut ridf_data);
        op_line_td.push_str(&get_td_op_line(&mut ridf_data, &mut alm_master));

        tot_rec += 1;
        let amt = ridf_input.net_val.parse().unwrap_or(0.0);
        tot_amt += amt;

        let recon_key = ReconKey::new(
            String::from("INR"),
            String::from("RIDF"),
            ridf_input.gl_code,
        );
        recon
            .store
            .entry(recon_key)
            .and_modify(|val| *val += amt)
            .or_insert(amt);
    }
    let mut out_file = get_writer(config_params.output_file_path());
    output_writer(&mut out_file, op_line_td, config_params.output_file_path());
    let mut recon_writer = get_writer(config_params.rec_output_file_path());
    output_writer(
        &mut recon_writer,
        recon.print(*config_params.as_on_date(), "RIDF"),
        config_params.rec_output_file_path(),
    );

    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    log_info!(logger, "{}", health_report.display());
    println!("{}", health_report.display());
}
