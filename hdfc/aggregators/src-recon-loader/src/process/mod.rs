use configuration_parameters::ConfigurationParameters;
use macros;
use sdb_io::buf_file_wrtr;
use slog::Logger;
mod io;
mod account_field_names;
mod read_metadata;
use calamine::{open_workbook, Reader, Xlsx};
use health_report::HealthReport;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;
mod exchange_rate;
use std::time::SystemTime;
use self::account_field_names::InputFields;
use self::read_metadata::{read_input_metadata, MetaDataFields};

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let start_read_timer = SystemTime::now();
    let mut tot_acc_encntrd = 0;
    let mut acc_pro_suc = 0;
    let mut master_llg_updated: Xlsx<_> =
        open_workbook(&config_params.master_llg_updated()).expect("Unable To Open `Master File`.");
    let mut ora_gl_cd_to_fm: HashMap<String, String> = HashMap::new();
    let input_keys = InputFields::new_from_path(&config_params.req_fields_file_path());
    if let Some(Ok(reader)) =
        master_llg_updated.worksheet_range(config_params.alm_master_sheet_name())
    {
        for record in reader.rows().skip(1) {
            ora_gl_cd_to_fm.insert(record[input_keys.master_gl_code_col-1].to_string(), record[input_keys.master_gl_desc_col-1].to_string());
        }
    }

    let input_file = File::open(&config_params.input_file()).expect("Could Not Read File");
    let input_reader = BufReader::new(input_file);
    let exchange_rate_file = exchange_rate::read_exchange_rate(config_params.exchange_rate_file());
    let end_read_timer = SystemTime::now();
    let duration = end_read_timer
        .duration_since(start_read_timer)
        .expect("Could not calculate total duration read timer.");
    debug!(
        diag_logger,
        "Reading Reference Files, Total Duration: {:?}.", duration
    );
    let mut input_map: HashMap<String, f64> = HashMap::new();
    let mut output_str = String::new();
    let start_derive_timer = SystemTime::now();
    let mut ip_map: HashMap<String, MetaDataFields> = HashMap::new();

    read_input_metadata(
        config_params,
        logger,
        diag_logger,
        &mut ip_map,
    );

    for (_index, record) in input_reader.lines().enumerate() {
        let record = record.expect("Could Not Read Line").to_string();
        tot_acc_encntrd += 1;
        let input_fields: Vec<&str> = record.split('|').collect();
        let src = ip_map.get(&input_keys.source).unwrap().position -1;
        let ip_gl_code = ip_map.get(&input_keys.gl_code).unwrap().position -1;
        let ccy = ip_map.get(&input_keys.currency).unwrap().position -1;
        let amt = ip_map.get(&input_keys.lcy_amount).unwrap().position -1;
        let llg_id = ip_map.get(&input_keys.llg_id).unwrap().position -1;

        let key = format!(
            "{}|{}|{}|{}",
            config_params.as_on_date().format("%d-%m-%Y").to_string(),
            input_fields[src as usize],
            input_fields[ip_gl_code as usize],
            input_fields[ccy as usize]
        );
        input_map
            .entry(key)
            .and_modify(|val| *val += input_fields[amt as usize].to_string().parse::<f64>().unwrap_or(0.0))
            .or_insert(input_fields[amt as usize].to_string().parse::<f64>().unwrap_or(0.0));
        acc_pro_suc += 1;
        let rec_exchange_rate = exchange_rate::get_exch_rate(
            input_fields[ccy as usize].to_string(),
            config_params.base_currency(),
            &exchange_rate_file,
        );
        let mut rec_llg_val = "";
        if ora_gl_cd_to_fm.contains_key(input_fields[ip_gl_code as usize]) {
            rec_llg_val = ora_gl_cd_to_fm.get(input_fields[ip_gl_code as usize]).unwrap();
        }
        log_debug!(
            logger,
            "AsOnDate: `{}`, SrcFileCd: `{}`, GLType: `{}`, SrcGLCd: `{:?}`, GLDesc: `{}` , SrcCCY: `{}` , SrcGLAmtLCY: `{}` , SrcGLAmtCCY: `{}` ",
            config_params.as_on_date().format("%d-%m-%Y").to_string(),
            input_fields[src as usize],
            input_fields[llg_id as usize],
            input_fields[ip_gl_code as usize],
            rec_llg_val,
            input_fields[ccy as usize],
            input_fields[amt as usize],
            input_fields[amt as usize].to_string().parse::<f64>().unwrap_or(0.0)*rec_exchange_rate,
        );
    }
    for (key, val) in input_map {
        let key_fields: Vec<&str> = key.split('|').collect();
        let exchange_rate = exchange_rate::get_exch_rate(
            key_fields[3].to_string(),
            config_params.base_currency(),
            &exchange_rate_file,
        );
        let mut llg_val = "";
        if ora_gl_cd_to_fm.contains_key(key_fields[2]) {
            llg_val = ora_gl_cd_to_fm.get(key_fields[2]).unwrap();
        }

        output_str.push_str(&format!(
            "{}|{}|{}|{}|{}|{}|{}|{}\n",
            config_params.as_on_date().format("%d-%m-%Y").to_string(),
            key_fields[1],
            "",
            key_fields[2],
            llg_val,
            key_fields[3],
            val,
            val * exchange_rate,
        ));
    }

    let end_derive_timer = SystemTime::now();
    let duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total derive process duration.");
    debug!(
        diag_logger,
        "Derive Process Total Duration: {:?}.", duration
    );

    let start_write_timer = SystemTime::now();

    let mut op_writer = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create Output file: `{}` on location `{}` : {}",
            config_params.output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match op_writer.write_all(output_str.as_bytes()) {
        Ok(_) => info!(logger, "Successfully written outputfile."),
        Err(error) => panic!(
            "Unable to write processed lines to file `{}`: {}.",
            config_params.output_file_path(),
            error
        ),
    }

    let end_write_timer = SystemTime::now();
    let duration = end_write_timer
        .duration_since(start_write_timer)
        .expect("Could not calculate total duration for writing src-recon-loader.");
    debug!(
        diag_logger,
        "Writing src-recon-loader output file. {:?}.", duration
    );

    let health_report = HealthReport::new(
        tot_acc_encntrd,
        acc_pro_suc,
        tot_acc_encntrd - acc_pro_suc,
        0.0,
        0.0,
        0,
    );
    health_report.gen_health_rpt(&config_params.output_file_path());
}
