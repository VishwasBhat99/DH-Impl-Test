use self::derive_fields::*;
use self::io::*;
use self::reconcilation::*;
use self::structs::{alm_master::*, gl_map::*, input_account::InputAccount};
use calamine::{open_workbook, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use slog::Logger;
use statics::*;
use std::collections::HashMap;
use std::default::Default;
use std::time::SystemTime;

mod derive_fields;
mod io;
mod reconcilation;
mod structs;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let st_tm_read = SystemTime::now();
    let mut op_line: String = String::new();
    let mut tot_rec = DEFAULT_INT;
    let mut skp_rec = DEFAULT_INT;

    let mut alm_master: HashMap<AlmMasterKey, AlmMaster> = HashMap::new();
    let mut alm_master_excel: Xlsx<_> =
        open_workbook(config_param.alm_master()).expect("Unable to open Alm Master File.");
    if let Some(Ok(reader)) = alm_master_excel.worksheet_range(config_param.alm_master_sheet_name())
    {
        for row in reader.rows() {
            get_alm_master_data(row, &mut alm_master);
        }
    }
    let mut gl_map_reader = read_file(config_param.gl_mapping_file());

    let mut gl_map: GLMapMap = GLMapMap::new();
    for (line_num, lines) in gl_map_reader.deserialize().enumerate() {
        let gl_map_input: GLMapInput =
            extract_lines(line_num, lines, config_param.gl_mapping_file(), log);
        get_gl_map_data(gl_map_input, &mut gl_map);
    }
    let mut recon = ReconMap::new();
    let mut concats = String::new();
    let mut input_reader = read_file(config_param.input_file_path());
    let mut tot_amt = 0.0;
    for (line_num, lines) in input_reader.deserialize().enumerate().skip(1) {
        let mut input_account: InputAccount =
            extract_lines(line_num, lines, config_param.input_file_path(), log);
        tot_rec += 1;
        if input_account.portfolio_num.parse::<f64>().is_err() {
            skp_rec += 1;
            continue;
        }
        let amt = if input_account.cf_type.trim_matches('"').to_uppercase() == "PRINCIPAL" {
            input_account.cf_amt.parse().unwrap_or(DEFAULT_FLOAT)
        } else {
            DEFAULT_FLOAT
        };
        tot_amt += amt;

        op_line.push_str(&get_op_line(
            &mut input_account,
            &mut alm_master,
            &mut concats,
            &mut gl_map,
            "SecCloseStock",
        ));
        let recon_key = ReconKey::new(
            input_account.curr,
            "SecCloseStock".to_string(),
            input_account.trsy_gl_cd,
        );
        recon
            .store
            .entry(recon_key)
            .and_modify(|val| *val += amt)
            .or_insert(amt);
    }

    let ed_tm_read = SystemTime::now();
    let duration = ed_tm_read
        .duration_since(st_tm_read)
        .expect("Could not calculate total read process duration.");
    debug!(diag_log, "Read Process Total Duration: {:?}.", duration);

    let st_tm_writer = SystemTime::now();
    let mut op_writer = get_writer(config_param.output_file_path());
    output_writer(&mut op_writer, op_line, config_param.output_file_path());

    let mut recon_writer = get_writer(config_param.rec_output_file_path());
    output_writer(
        &mut recon_writer,
        recon.print(*config_param.as_on_date(), "Close Stock"),
        config_param.rec_output_file_path(),
    );

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
    debug!(
        diag_log,
        "Writing Close Stock, Total Duration: {:?}.", duration
    );
}
