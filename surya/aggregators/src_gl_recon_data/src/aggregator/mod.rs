mod account_field_names;
mod currency;
mod structs;
use super::configuration_parameters::ConfigurationParameters;
use crate::macros;
use account_field_names::AccFieldNames;
use calamine::{open_workbook_auto, Reader};
use health_report::HealthReport;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::account_with_cfs::get_field_value;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::BufRead;
use std::io::Write;
use structs::*;

pub fn aggregate(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    //Get Required Data

    let mut file_reader = reader::Reader::new_at_path(
        config_params.metadata_file_path(),
        config_params.input_file_path(),
    );
    let method_reader = reader::Reader::new_at_path(
        config_params.metadata_file_path(),
        config_params.input_file_path(),
    );
    //Alm master File reading started
    let mut alm_master_map: HashMap<String, String> = HashMap::new();
    let mut alm_master_excel = open_workbook_auto(config_params.alm_master_file_path())
        .expect("Unable to open Alm Master File.");
    if let Some(Ok(reader)) =
        alm_master_excel.worksheet_range(config_params.alm_master_sheet_name())
    {
        for row in reader.rows().skip(0) {
            let gl_code = row[0].to_string();
            let gl_desc = row[1].to_string();
            alm_master_map.insert(gl_code, gl_desc);
        }
    }
    //Alm master File reading Completed
    //GL MASTER File reading
    let mut gl_master_map: HashMap<String, String> = HashMap::new();
    if config_params.gl_master_file_path() != "NA" {
        let mut gl_master_excel = open_workbook_auto(config_params.gl_master_file_path())
            .expect("Unable to open GL Master File.");
        if let Some(Ok(reader)) =
            gl_master_excel.worksheet_range(config_params.gl_master_sheet_name())
        {
            for row in reader.rows().skip(1) {
                let gl_code = row[0].to_string().trim().to_string();
                let tb_gl_code = row[1].to_string().trim().to_string();
                gl_master_map.insert(gl_code, tb_gl_code);
            }
        }
    }
    //GL Master File reading completed
    //LLG Recon File reading started
    let llg_recon_file = match new_buf_rdr(config_params.llg_recon_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found llg_recon_file: `{}`: {}.",
            config_params.llg_recon_file_path(),
            error
        ),
    };
    let mut llg_recon_map: HashMap<String, String> = HashMap::new();
    for (line_num, lines) in llg_recon_file.lines().enumerate() {
        let llg_recon_line = match lines {
            Ok(input_line) => input_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.llg_recon_file_path(),
                line_num + 1,
                error
            ),
        };
        let llg_recon_fields: Vec<String> =
            llg_recon_line.split('|').map(|s| s.to_string()).collect();
        let gl_code = llg_recon_fields[3].to_string();
        let source_field = llg_recon_fields[1].to_string();
        llg_recon_map.insert(gl_code, source_field);
    }
    //LLG Recon File reading Completed
    let mut buffer_writer = buf_file_wrtr(config_params.output_file_path(), None)
        .expect("Unable to create writer for output file.");
    let req_fields = AccFieldNames::new_from_path(config_params.req_fields_file_path());
    let currency_converter = currency::create_currency_converter(
        config_params.base_currency(),
        config_params.exchange_rate_file_path(),
    );
    //Initialize req Data
    let mut acc_encountered = 0;
    let mut cf_encountered = 0;
    let mut acc_success = 0;
    let mut ip_amt = 0.0;
    let mut op_amt = 0.0;
    let mut aggr_op: HashMap<AggregatorKeys, AggregatorValues> = HashMap::new();
    //Get Output Data
    for record in file_reader.iter() {
        cf_encountered += 1;
        let mut gl_code =
            match get_field_value(&record, &method_reader, req_fields.gl_cd.to_string()) {
                Ok(value) => value
                    .parse()
                    .unwrap_or_else(|_| config_params.default_gl_code().to_string()),
                Err(_error) => panic!("{}", _error),
            };
        if config_params.gl_master_file_path() != "NA" {
            gl_code = gl_master_map.get(&gl_code).unwrap_or(&gl_code).to_string();
        }
        let src_ccy = record
            .get_string_for_key(&req_fields.ccy)
            .unwrap_or(&"NA".to_string())
            .to_string();
        let gl_amt = record.get_f64_for_key(&req_fields.gl_amount).unwrap_or(0.0);
        let src_field_cd = llg_recon_map
            .get(&gl_code)
            .unwrap_or(&"NA".to_string())
            .to_string();
        let gl_desc = alm_master_map
            .get(&gl_code)
            .unwrap_or(&"NA".to_string())
            .to_string();
        acc_encountered += 1;
        let keys = AggregatorKeys {
            src_field_cd,
            gl_type: "".to_string(),
            src_gl_cd: gl_code,
            gl_desc,
            src_ccy,
        };
        let values = AggregatorValues { gl_amt };
        insert_aggrdata(&mut aggr_op, keys, values);
        ip_amt += gl_amt;

        acc_success += 1;
    }
    //Write Output Data
    let formated_as_on_date = format!("{}", config_params.as_on_date().format("%d-%m-%Y"));
    for (keys, values) in aggr_op.drain() {
        op_amt += values.gl_amt;
        let conv_data = currency_converter.convert(
            &keys.src_ccy,
            &values,
            config_params.is_consolidated(),
            logger,
        );
        if config_params.is_consolidated() {
            writeln!(
                buffer_writer,
                "{}|{}|{}|{}|{}|{}|{}|{}",
                formated_as_on_date,
                keys.src_field_cd,
                keys.gl_type,
                keys.src_gl_cd,
                keys.gl_desc,
                keys.src_ccy,
                conv_data.gl_amt,
                values.gl_amt,
            )
            .expect("Unable to generate aggregated summary file.");
        } else {
            writeln!(
                buffer_writer,
                "{}|{}|{}|{}|{}|{}|{}|{}",
                formated_as_on_date,
                keys.src_field_cd,
                keys.gl_type,
                keys.src_gl_cd,
                keys.gl_desc,
                keys.src_ccy,
                values.gl_amt,
                conv_data.gl_amt,
            )
            .expect("Unable to generate aggregated summary file.");
        }
    }
    buffer_writer.flush().expect("Unable to flush the writer.");
    //Write Health Check Report
    let health_report = HealthReport::new(
        acc_encountered,
        acc_success,
        acc_encountered - acc_success,
        ip_amt,
        op_amt,
        cf_encountered,
    );
    log_info!(logger, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file_path());
}
fn insert_aggrdata(
    op: &mut HashMap<AggregatorKeys, AggregatorValues>,
    keys: AggregatorKeys,
    values: AggregatorValues,
) {
    let val = values;
    op.entry(keys)
        .and_modify(|m| m.aggregateamount(val))
        .or_insert(val);
}
