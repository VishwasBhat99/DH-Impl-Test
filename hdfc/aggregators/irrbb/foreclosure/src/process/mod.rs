use self::io::*;
use configuration_parameters::ConfigurationParameters;
use required_fields::ReqFields;
use sdb_agg_rules_txt::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use slog::Logger;
mod get_llg;
use std::io::Write;
mod io;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn process_name(
    config_params: &ConfigurationParameters,
    _logger: &Logger,
    _diag_logger: &Logger,
) {
    let account_metadata_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file(),
        config_params.input_file_path(),
    );

    let rules = AggRules::new_from_path(config_params.rules_file_path(), &account_metadata_reader);

    let mut op_writer = get_writer(config_params.output_file_path());
    let req_fields = ReqFields::new_from_path(config_params.req_file_path());

    let td_ids = File::open(config_params.tl_ids_file_path()).expect("Unable To Open TL Ids File");

    let mut bucket_id = 0;
    let reader = BufReader::new(td_ids);

    for (_, line) in reader.lines().enumerate() {
        let line = line.expect("Could Not Read Line");
        let input =
            File::open(config_params.input_file_path()).expect("Unable To Open TL Ids File");
        let input_file = BufReader::new(input);
        for records in input_file.lines().skip(1) {
            let record = records.expect("Unable to read line");
            let row: Vec<&str> = record.split("~#~").collect();
            if row.len() == 7 {
                let record = row[req_fields.scheme_id].to_string()
                    + "|"
                    + &row[req_fields.from_tenure].to_string()
                    + "|"
                    + &row[req_fields.mc_status].to_string()
                    + "|"
                    + &row[req_fields.rate_pct].to_string()
                    + "|"
                    + &row[req_fields.to_tenure].to_string()
                    + "|"
                    + &row[req_fields.tenure_in].to_string()
                    + "|"
                    + &row[req_fields.stream_desc].to_string();
                let llg = get_llg::llg_for_account(
                    &record,
                    &rules,
                    &account_metadata_reader,
                    config_params,
                );

                bucket_id += 1;
                let tenure = if row[req_fields.tenure_in].to_string() == "D" {
                    "Day"
                } else if row[req_fields.tenure_in].to_string() == "M" {
                    "Month"
                } else {
                    "Year"
                };
                write!(
                    op_writer,
                    "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
                    line,
                    config_params.as_on_date().format("%d-%m-%Y"),
                    llg,
                    bucket_id,
                    row[req_fields.from_tenure].to_string()
                        + &row[req_fields.tenure_in].to_string()
                        + &'-'.to_string()
                        + &row[req_fields.to_tenure].to_string()
                        + &row[req_fields.tenure_in].to_string(),
                    row[req_fields.from_tenure],
                    row[req_fields.to_tenure],
                    tenure.to_owned(),
                    tenure.to_owned(),
                    row[req_fields.rate_pct]
                )
                .expect("Error while writing output.");
            }
        }
    }
}
