use self::derive_fields::*;
use self::io::*;
use self::reconcilation::*;
use self::structs::{alm_master::*, cust_master::*, input_account::InputAccount};
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

pub fn process(config_param: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
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

    let mut cust_master_reader = read_file(config_param.cust_master());
    let mut cust_master: CustMasterMap = CustMasterMap::new();
    for (line_num, lines) in cust_master_reader.deserialize().enumerate() {
        let cust_master_input: CustMasterInput =
            extract_lines(line_num, lines, config_param.cust_master(), log);
        get_cust_master_data(cust_master_input, &mut cust_master);
    }

    let mut recon = ReconMap::new();
    let mut concats = String::new();
    let mut input_reader = read_file(config_param.input_file_path());
    let mut tot_amt = 0.0;
    for (line_num, lines) in input_reader.deserialize().enumerate().skip(1) {
        let mut input_account: InputAccount =
            extract_lines(line_num, lines, config_param.input_file_path(), log);
        tot_rec += 1;
        let amt = input_account.bal_os.parse().unwrap_or(DEFAULT_FLOAT);
        tot_amt += amt;

        op_line.push_str(&get_op_line(
            &mut input_account,
            &mut cust_master,
            &mut alm_master,
            &mut concats,
            config_param.src_file_name(),
        ));
        let recon_key = ReconKey::new(
            input_account.ccy,
            String::from(config_param.gl_type()),
            input_account.gl_cd,
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
        recon.print(*config_param.as_on_date(), config_param.src_file_name()),
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
    debug!(diag_log, "Writing LCBG, Total Duration: {:?}.", duration);
}
