mod account_field_names;
pub(crate) mod config;
mod currency;
mod structs;
use super::configuration_parameters::ConfigurationParameters;
use super::get_derived_fields::get_llg;
use crate::macros;
use account_field_names::AccFieldNames;
use health_report::HealthReport;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::compound_types::Cashflow;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::account_with_cfs::get_field_value;
use sdb_dyn_proto_rdr::reader::types::*;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;
use structs::*;

pub fn aggregate(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let start_time = SystemTime::now();
    //Initialize req Data
    let mut acc_encountered = 0;
    let mut cf_encountered = 0;
    let mut acc_success = 0;
    let mut ip_amt = 0.0;
    let mut op_amt = 0.0;
    let mut buffer_writer = buf_file_wrtr(config_params.output_file_path(), None)
        .expect("Unable to create writer for output file.");
    let mut aggr_op: HashMap<AggregatorKeys, AggregatorValues> = HashMap::new();
    let files_config = config::get_files(config_params.config_file_path());
    for file in files_config.files {
        //Get Required Data
        let mut file_reader =
            reader::Reader::new_at_path(file.metadata_file_path(), file.input_file_path());
        let method_reader =
            reader::Reader::new_at_path(file.metadata_file_path(), file.input_file_path());
        let req_fields = AccFieldNames::new_from_path(file.req_fields_file_path());
        let rules = AggRules::new_from_path(file.rules_file_path(), &file_reader);
        let currency_converter = currency::create_currency_converter(
            config_params.base_currency(),
            config_params.currency_conversion_file_path(),
        );

        //Get Output Data
        for mut record in file_reader.iter() {
            cf_encountered += 1;
            let _acc_num = record
                .get_string_for_key(&req_fields.acc_num)
                .unwrap_or(&"NA".to_string())
                .to_string();
            acc_encountered += 1;
            let llg = get_llg(&file, &record, &rules, logger);
            let ccy = record
                .get_string_for_key(&req_fields.ccy)
                .unwrap_or(&"NA".to_string())
                .to_string();
            let mut cf_amt = 0.0;
            match method_reader
                .get_field_type(&req_fields.amount[0])
                .expect(&format!(
                    "could not get value for amount field for the source: {}",
                    file.source_name()
                )) {
                Type::Cashflows => {
                    let default_cf_vec: Vec<Cashflow> = Vec::new();
                    let cashflow_vec = record
                        .remove_cfs_for_key(&req_fields.amount[0])
                        .unwrap_or(default_cf_vec);
                    for cf in cashflow_vec {
                        cf_amt += cf.principal_amount;
                    }
                }
                _ => {}
            };
            for (pos, _gl_cd) in req_fields.gl_cd.iter().enumerate() {
                let mut gl_cd = match get_field_value(
                    &record,
                    &method_reader,
                    req_fields.gl_cd[pos].to_string(),
                ) {
                    Ok(value) => value.parse().unwrap_or(file.default_gl_code().to_string()),
                    Err(_error) => panic!("{}", _error),
                };
                if gl_cd.is_empty() {
                    gl_cd = file.default_gl_code().to_string();
                }
                let amt = match method_reader
                    .get_field_type(&req_fields.amount[pos])
                    .expect(&format!(
                        "could not get value for amount field for the source: {}",
                        file.source_name()
                    )) {
                    Type::Cashflows => cf_amt,
                    _ => match get_field_value(
                        &record,
                        &method_reader,
                        req_fields.amount[pos].to_string(),
                    ) {
                        Ok(value) => value.parse().unwrap_or(0.0),
                        Err(_error) => panic!("{}", _error),
                    },
                };
                let keys = AggregatorKeys {
                    llg,
                    gl_cd,
                    ccy: ccy.to_string(),
                };
                let values = AggregatorValues { aggr_amt: amt };
                insert_aggrdata(&mut aggr_op, keys, values);
                ip_amt += amt;
            }
            acc_success += 1;
        }
        //Write Output Data
        let formated_as_on_date = format!("{}", config_params.as_on_date().format("%d-%m-%Y"));
        let source_name = if file.source_name().trim().is_empty() {
            Path::new(config_params.output_file_path())
                .file_name()
                .unwrap()
                .to_str()
                .expect("source name not extracted from output file")
        } else {
            file.source_name()
        };
        for (keys, values) in aggr_op.drain() {
            op_amt += values.aggr_amt;
            let conv_data =
                currency_converter.convert(&keys.ccy, &values, file.is_consolidated(), logger);
            if file.is_consolidated() {
                write!(
                    buffer_writer,
                    "{}|{}|{}|{}|{}|{}|{}\n",
                    formated_as_on_date,
                    source_name,
                    keys.llg,
                    keys.gl_cd,
                    keys.ccy,
                    conv_data.aggr_amt,
                    values.aggr_amt
                )
                .expect("Unable to generate aggregated summary file.");
            } else {
                write!(
                    buffer_writer,
                    "{}|{}|{}|{}|{}|{}|{}\n",
                    formated_as_on_date,
                    source_name,
                    keys.llg,
                    keys.gl_cd,
                    keys.ccy,
                    values.aggr_amt,
                    conv_data.aggr_amt,
                )
                .expect("Unable to generate aggregated summary file.");
            }
        }

        buffer_writer.flush().expect("Unable to flush the writer.");
        let total_duration = print_return_time_since!(start_time);
        log_debug!(
            logger,
            "{} has been written within {:?}",
            file.source_name(),
            total_duration
        );
    }

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
