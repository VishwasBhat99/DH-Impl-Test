use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use crate::process::account::{format_output, get_due_mat_date, Account, MasterData};
use calamine::{open_workbook_auto, Reader};
use health_report::HealthReport;
use sdb_io::buf_file_wrtr;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::io::BufRead;
use std::io::Write;
use std::time::SystemTime;

mod account;

pub fn process(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_process_timer = SystemTime::now();
    let mut tot_rec = 0;
    let mut succ_rec = 0;
    let mut tot_amt = 0.0;
    let input_file = match new_buf_rdr(config_params.input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` due to : `{}`.",
            config_params.input_file_path(),
            error
        ),
    };

    let mut writer = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!(
            "Unable to create output file: `{}` due to : `{}`",
            config_params.output_file_path(),
            error,
        ),
    };

    let mut bgl_cgl_map: HashMap<String, String> = HashMap::new();
    let bgl_cgl_file = match new_buf_rdr(config_params.bgl_cgl_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found bgl-cgl file: `{}` due to: `{}`.",
            config_params.bgl_cgl_file_path(),
            error
        ),
    };

    for (line_num, lines) in bgl_cgl_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.bgl_cgl_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields = line.split('|').collect::<Vec<&str>>();
        log_info!(
            log,
            "bgl_cgl_map: mapped key: {} to val: {}",
            fields[0],
            fields[1]
        );
        //Key: BGL and Value: CGL
        bgl_cgl_map.insert(fields[0].to_string(), fields[1].to_string());
    }

    let mut master_map: HashMap<String, MasterData> = HashMap::new();
    let mut master_excel = open_workbook_auto(config_params.master_file_path())
        .expect("Unable to open Mapping Master File.");
    info!(
        log,
        "Sheets found in Master-Excel: `{:?}` and Sheet to be read: `{}`",
        master_excel.sheet_names(),
        config_params.sheet_name()
    );
    if let Some(Ok(reader)) = master_excel.worksheet_range(config_params.sheet_name()) {
        for row in reader.rows().skip(0) {
            let gl_acc_no = row[0].to_string();
            let grp = row[3].to_string();
            let llg = row[4].to_string();
            log_info!(
                log,
                "master_map: mapped key: {} to val: [{}, {}]",
                gl_acc_no,
                grp,
                llg
            );
            master_map.insert(gl_acc_no, MasterData::new(grp, llg));
        }
    }
    for (line_num, lines) in input_file.lines().enumerate().skip(1) {
        tot_rec += 1;
        let input_line = match lines {
            Ok(input_line) => input_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.input_file_path(),
                line_num + 1,
                error
            ),
        };
        let input_fields = input_line.split('|').collect::<Vec<&str>>();
        succ_rec += 1;
        let mut input_acc = Account::new(input_fields, config_params);
        //Derive Due-Date and Maturity-Date
        get_due_mat_date(&mut input_acc, config_params);
        //Derive LLG and Group
        input_acc.llg = master_map
            .get(&input_acc.pan_no)
            .unwrap_or(&MasterData::def())
            .llg
            .to_string();
        input_acc.group = master_map
            .get(&input_acc.pan_no)
            .unwrap_or(&MasterData::def())
            .grp
            .to_string();
        tot_amt += input_acc.outstanding_bal;
        writeln!(writer, "{}", format_output(&input_acc)).expect("Output Line can not be written");
    }
    let end_process_timer = SystemTime::now();
    let duration = end_process_timer
        .duration_since(start_process_timer)
        .expect("Could not calculate total duration for the process.");
    debug!(
        diag_log,
        "Total Duration for Reading and Writing Records: {:?}.", duration
    );
    let health_report =
        HealthReport::new(tot_rec, succ_rec, tot_rec - succ_rec, tot_amt, tot_amt, 0);
    health_report.gen_health_rpt(config_params.output_file_path());
}
