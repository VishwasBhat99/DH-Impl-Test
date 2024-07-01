use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use crate::process::account::{format_output, Account, MasterData};
use calamine::{open_workbook_auto, Reader};
use health_report::HealthReport;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
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
    let mut tot_amt_ip = 0.0;
    let mut tot_amt_op = 0.0;
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
    let mut input_map: HashMap<String, Account> = HashMap::new();
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
        let in_date = rbdate::NaiveDate::parse_from_str(&input_acc.in_date, "%d-%m-%Y")
            .expect("Error reading IN-Date expected format DD-MM-YYYY)");
        //Add Tenor to Maturity-Date
        input_acc.maturity_date =
            rbdate::incr_dt_by_mon_presrv_eom_checked(in_date, input_acc.tenurs as usize)
                .unwrap_or(*config_params.as_on_date())
                .format("%d-%m-%Y")
                .to_string();
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
        let concat_key = format!(
            "{}_{}_{}",
            input_acc.card_no,
            &in_date.format("%d%m%Y").to_string(),
            input_acc.tenurs
        );
        tot_amt_ip += input_acc.outstanding_bal;
        input_map
            .entry(concat_key.clone())
            .and_modify(|data| data.append_data(input_acc.clone()))
            .or_insert(input_acc);
    }
    for (concat, data) in input_map.iter() {
        tot_amt_op += data.outstanding_bal;
        writeln!(writer, "{}|{}", concat, format_output(data))
            .expect("Output Line can not be written");
    }
    let end_process_timer = SystemTime::now();
    let duration = end_process_timer
        .duration_since(start_process_timer)
        .expect("Could not calculate total duration for the process.");
    debug!(
        diag_log,
        "Total Duration for Reading and Writing Records: {:?}.", duration
    );
    let health_report = HealthReport::new(
        tot_rec,
        succ_rec,
        tot_rec - succ_rec,
        tot_amt_ip,
        tot_amt_op,
        0,
    );
    health_report.gen_health_rpt(config_params.output_file_path());
}
