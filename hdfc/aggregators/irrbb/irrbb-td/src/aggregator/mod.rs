use self::passthrough_handler::get_passthrough;
use aggregator::account_field_names::AccField;
use aggregator::required_fields::ReqFields;
use configuration_parameters::ConfigurationParameters;
use currency;
use macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use sdb_dyn_proto_rdr::reader::types::Type;
use sdb_dyn_proto_rdr::reader::Reader;
use sdb_io::open_file_read;
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::time::SystemTime;
use std::io::Read;

mod account_field_names;
mod implementation;
mod llg_key;
mod passthrough_handler;
mod required_fields;
mod writer;

pub fn aggregate_cashflows(
    config_params: ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) {
    // init
    let req_fields = ReqFields::new_from_path(config_params.req_fields_file_path());
    let mut required_fields_file = open_file_read(config_params.req_fields_file_path())
        .expect("Cannot open the required fields file.");

    let mut required_fields_buffer = String::new();
    required_fields_file
        .read_to_string(&mut required_fields_buffer)
        .expect("Cannot read the required fields file.");

    let reader_for_calling_method = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );
    let delimeter: &str = "|";
    let rf: ReqFields = serde_json::from_str(&required_fields_buffer[..])
        .expect("Unable to parse the required fields file.");

    let currency_converter = currency::create_currency_converter(
        config_params.base_ccy(),
        config_params.exchange_rate_file(),
    );

    // Prepare data we will require for processing.
    let start_time = SystemTime::now();
    let keys = AccField::new_from_path(config_params.output_fields_file_path());

    let mut account_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );
    let rules = AggRules::new_from_path(config_params.rules_file_path(), &account_reader);

    let mut line_no = 0;

    // init a pool of writers as it depends on classification id
    let mut writers_pool: HashMap<i32, BufWriter<File>> = HashMap::new();

    for mut account in account_reader.iter() {
        line_no += 1;
        let input_account = get_passthrough(
            &account,
            &keys,
            &req_fields,
            &config_params,
            &reader_for_calling_method,
            &currency_converter,
            &logger,
        );

        let llg = log_measurements!(
            diag_logger,
            [format!("Type: GetLLG, Line no: {:?}", line_no)],
            implementation::llg_for_account(&account, &rules, &config_params, logger)
        );
        if llg.category == config_params.default_llg_code() {
            let default_llg_log = get_default_llg_log(
                &rf,
                &mut account,
                delimeter,
                &reader_for_calling_method,
                input_account.to_string(),
            );
            log_trace!(
                logger,
                "Default LLG account with values: {:?}",
                default_llg_log
            );
        }
        // writer
        let writer = match writers_pool.get_mut(&llg.category) {
            Some(writer) => writer,
            None => {
                let new_writer =
                    writer::get_new_writer(llg.category, config_params.output_file_path());
                writers_pool.insert(llg.category, new_writer);
                // cannot return new writer as ownership of that writer is assigned to writers_pool in prev step
                writers_pool.get_mut(&llg.category).unwrap()
            }
        };
        write!(writer, "{}\n", input_account.trim_matches('|'))
            .expect("Unable to generate summary file.");
    }
    // flush all writers
    for (_, mut writer) in writers_pool.drain() {
        let _ = writer.flush();
    }
    let total_duration = print_return_time_since!(start_time);
    log_info!(logger, "Total time for aggregation: {:?}", total_duration);
}

pub fn get_default_llg_log(
    rf: &ReqFields,
    record: &mut AccountWithCFs,
    delimiter: &str,
    reader_for_calling_method: &Reader,
    input_acc: String,
) -> String {
    let mut string_for_record_prefix = String::new();
    string_for_record_prefix.push_str(&input_acc);
    for field_name in &rf.debug_fields {
        match reader_for_calling_method
            .get_field_type(&field_name)
            .expect("Key type not known.")
        {
            Type::I32 => {
                let val = match record.get_i32_for_key(&field_name) {
                    Ok(value) => value,
                    Err(_error) => continue,
                };
                string_for_record_prefix.push_str(&val.to_string());
                string_for_record_prefix.push_str(&delimiter);
            }

            Type::I64 => {
                let val = match record.get_i64_for_key(&field_name) {
                    Ok(value) => value,
                    Err(_error) => continue,
                };
                string_for_record_prefix.push_str(&val.to_string());
                string_for_record_prefix.push_str(&delimiter);
            }

            Type::F32 => {
                let val = match record.get_f32_for_key(&field_name) {
                    Ok(value) => value,
                    Err(_error) => continue,
                };
                string_for_record_prefix.push_str(&val.to_string());
                string_for_record_prefix.push_str(&delimiter);
            }

            Type::F64 => {
                let val = match record.get_f64_for_key(&field_name) {
                    Ok(value) => value,
                    Err(_error) => continue,
                };
                string_for_record_prefix.push_str(&val.to_string());
                string_for_record_prefix.push_str(&delimiter);
            }

            Type::String => {
                let val = match record.get_string_for_key(&field_name) {
                    Ok(value) => value,
                    Err(_error) => continue,
                };
                string_for_record_prefix.push_str(val);
                string_for_record_prefix.push_str(&delimiter);
            }

            Type::Cashflows => {
                let cashflows = match record.remove_cfs_for_key(&field_name) {
                    Ok(value) => value,
                    Err(_error) => continue,
                };
                for cashflow in cashflows {
                    string_for_record_prefix.push_str(&cashflow.interest_amount.to_string());
                    string_for_record_prefix.push_str(":");
                    string_for_record_prefix.push_str(&cashflow.principal_amount.to_string());
                    string_for_record_prefix.push_str(":");
                    string_for_record_prefix.push_str(
                        &naivedate_from_timestamp(cashflow.date)
                            .format("%d-%m-%Y")
                            .to_string(),
                    );
                    string_for_record_prefix.push('#');
                }
            }
        }
    }
    string_for_record_prefix
}

pub fn naivedate_from_timestamp(t: i64) -> rbdate::NaiveDate {
    let naive_date_time = rbdate::NaiveDateTime::from_timestamp(t, 0);
    naive_date_time.date()
}
