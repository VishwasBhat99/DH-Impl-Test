use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_agg_rules::agg_rules::AggRules as AgRule;
use sdb_agg_rules_txt::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::account_with_cfs::get_field_value;
use slog::Logger;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::io::{BufRead, BufReader};
use std::time::SystemTime;

mod get_llg;
mod implementation;
mod writers;

#[derive(Debug, PartialEq, Eq, Hash)]
struct LLGMap {
    llg: String,
    filename: String,
}

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    // Prepare data we will require for processing.
    let start_time = SystemTime::now();
    let mut acc_encountered = 0;
    let mut acc_processed = 0;
    let mut cashflows_count = 0;
    let mut writers_pool: HashMap<String, BufWriter<File>> = HashMap::new();
    //Read input file and metadata mapping.
    let mut account_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );

    let llg_op_map_reader = fs::read_to_string(&config_params.llg_op_mapping_file())
        .expect("Failed to read llg-output map file!");
    for line in llg_op_map_reader.lines() {
        let fields: Vec<&str> = line.split('|').collect();
        let new_writer =
            writers::get_new_writer(fields[1].to_string(), config_params.output_file_path());
        writers_pool.insert(fields[0].to_string(), new_writer);
    }
    let mut merged_writer =
        writers::get_new_writer("merged".to_string(), config_params.output_file_path());
    if config_params.input_file_path().ends_with(".cf") {
        let input_reader = reader::Reader::new_at_path(
            config_params.account_metadata_file_path(),
            config_params.input_file_path(),
        );
        // let mut field_names = Vec::new();
        let metadata_reader = fs::read_to_string(&config_params.account_metadata_file_path())
            .expect("Failed to read metadata file!");
        //Fetch the names from the metadata file.
        let cf_metadata_json: serde_json::Value = serde_json::from_str(&metadata_reader)
            .expect("Metadata JSON does not have correct format.");
        let fields = cf_metadata_json
            .get("fields")
            .expect("Cannot get metadata fields.");
        let val_array = fields
            .as_array()
            .expect("Cannot get metadata fields as array. ");
        let reader_for_calling = reader::Reader::new_at_path(
            config_params.account_metadata_file_path(),
            config_params.input_file_path(),
        );
        let rules = AgRule::new_from_path(config_params.rules_file_path(), &input_reader);
        let _as_on_date = config_params.as_on_date();
        for (_count, mut account) in account_reader.iter().enumerate() {
            acc_encountered += 1;
            let llg = log_measurements!(
                diag_logger,
                ["Type: GetLLG, Identifier".to_string()],
                implementation::llg_for_account(&account, &rules, config_params)
            );
            let mut output_line = String::new();
            for field_name in val_array {
                let key_name = field_name
                    .get("name")
                    .expect("Cannot get metadata fields")
                    .as_str()
                    .expect("Cannot get metadata names as string");
                if key_name == "cashflows" {
                    let cashflows = match account.remove_cfs_for_key(&key_name.to_string()) {
                        Ok(value) => value,
                        Err(_error) => continue,
                    };
                    for cashflow in cashflows {
                        cashflows_count += 1;
                        let cf_date = naivedate_from_timestamp(cashflow.date);
                        output_line = format!(
                            "{}|{}|{}|{}",
                            output_line,
                            &cashflow.interest_amount.to_string(),
                            &cashflow.principal_amount.to_string(),
                            &cf_date.format("%d-%m-%Y").to_string()
                        );
                    }
                } else {
                    let mut value =
                        get_field_value(&account, &reader_for_calling, key_name.to_string())
                            .expect("cannot parse field name in cf file");
                    if key_name.to_lowercase().contains("date") {
                        let timestamp_val = value.parse::<i64>().unwrap_or(0);
                        value = naivedate_from_timestamp(timestamp_val)
                            .format("%d-%m-%Y")
                            .to_string()
                    }
                    output_line = format!("{}|{}", output_line, value);
                }
            }
            output_line.pop();
            output_line.remove(0);
            // writer
            if !config_params.is_split_req() {
                output_line.push('|');
                output_line.push_str(&llg.to_string());
                output_line.push('\n');
                writers::write_data(&mut merged_writer, output_line.to_string(), logger);
            } else {
                output_line.push('\n');
                let writer = match writers_pool.get_mut(&llg.to_string()) {
                    Some(writer) => writer,
                    None => {
                        //If the source id could not be found the output is written to a default file: "NA.txt".
                        let new_writer = writers::get_new_writer(
                            "NA".to_string(),
                            config_params.output_file_path(),
                        );
                        writers_pool.insert(llg.to_string(), new_writer);
                        // cannot return new writer as ownership of that writer is assigned to writers_pool in prev step
                        writers_pool.get_mut(&llg.to_string()).unwrap()
                    }
                };
                writers::write_data(writer, output_line, logger);
            }
            acc_processed += 1;
        }
    } else if config_params.input_file_path().ends_with("txt") {
        let rules = AggRules::new_from_path(config_params.rules_file_path(), &account_reader);
        let input =
            File::open(config_params.input_file_path()).expect("Unable To Open TL Ids File");
        let input_file = BufReader::new(input);

        for (line_num, lines) in input_file.lines().enumerate() {
            let mut line = match lines {
                Ok(line) => line,
                Err(error) => panic!(
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_params.input_file_path(),
                    line_num + 1,
                    error
                ),
            };

            let llg = get_llg::llg_for_account(&line, &rules, &account_reader, config_params);

            if !config_params.is_split_req() {
                line.push('|');
                line.push_str(&llg.to_string());
                line.push('\n');
                writers::write_data(&mut merged_writer, line.to_string(), logger);
            } else {
                line.push('\n');
                let writer = match writers_pool.get_mut(&llg.to_string()) {
                    Some(writer) => writer,
                    None => {
                        //If the source id could not be found the output is written to a default file: "NA.txt".
                        let new_writer = writers::get_new_writer(
                            "NA".to_string(),
                            config_params.output_file_path(),
                        );
                        writers_pool.insert(llg.to_string(), new_writer);
                        // cannot return new writer as ownership of that writer is assigned to writers_pool in prev step
                        writers_pool.get_mut(&llg.to_string()).unwrap()
                    }
                };
                writers::write_data(writer, line, logger);
            }
        }
    } else {
        panic!("Cannot discern input file format. Expected format is .txt or .cf");
    }
    let health_stat = HealthReport::new(
        acc_encountered,
        acc_processed,
        acc_encountered - acc_processed,
        0.0,
        0.0,
        cashflows_count,
    );
    health_stat.gen_health_rpt(config_params.output_file_path());
    let total_duration = print_return_time_since!(start_time);
    log_info!(logger, "Total time for aggregation: {:?}", total_duration);
}

pub fn naivedate_from_timestamp(t: i64) -> rbdate::NaiveDate {
    if t == 0 {
        rbdate::NaiveDate::from_ymd(1900, 1, 1)
    } else {
        let naive_date_time = rbdate::NaiveDateTime::from_timestamp(t, 0);
        naive_date_time.date()
    }
}
