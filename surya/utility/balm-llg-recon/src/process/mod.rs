use configuration_parameters::ConfigurationParameters;
use currency;
use get_derived_fields::get_balm_llg;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::types::Type;
use sdb_io::buf_file_wrtr;
use sdb_io::open_file_read;
use slog::Logger;
use std::io::Read;
use std::io::Write;

#[derive(Deserialize)]
struct RequiredFields {
    fields: Vec<FieldInfo>,
}
#[derive(Deserialize)]
struct FieldInfo {
    field_name: String,
    output_file_type: String,
}

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let mut reader_for_records = reader::Reader::new_at_path(
        config_params.metadata_file_path(),
        config_params.input_file_path(),
    );
    let reader_for_calling_method = reader::Reader::new_at_path(
        config_params.metadata_file_path(),
        config_params.input_file_path(),
    );
    let llg_ids: Vec<String> = config_params
        .llg_ids()
        .split(",")
        .map(|s| s.to_string())
        .collect();
    // Read currency exchange rate file
    let currency_converter = currency::create_currency_converter(
        config_params.base_currency(),
        config_params.exchange_rate_file(),
    );

    let mut buffer_writer = buf_file_wrtr(config_params.output_file_path(), None)
        .expect("Unable to create writer for output file.");

    let mut required_fields_file = open_file_read(config_params.required_fields_file_path())
        .expect("Cannot open the required fields file.");
    let mut required_fields_buffer = String::new();
    required_fields_file
        .read_to_string(&mut required_fields_buffer)
        .expect("Cannot read the required fields file.");
    let rf: RequiredFields = serde_json::from_str(&required_fields_buffer[..])
        .expect("Unable to parse the required fields file.");

    let mut op = String::new();
    let mut acc_encountered = 0;
    let balm_rules =
        AggRules::new_from_path(config_params.balm_rule_file_path(), &reader_for_records);
    let record_reader = reader_for_records.iter();
    let mut llg = 0;
    for record in record_reader {
        acc_encountered += 1;
        // read account currency id
        let acc_ccy: String = record
            .get_string_for_key(config_params.acc_currency())
            .unwrap_or(&config_params.base_currency().to_string())
            .to_string();
        op.clear();
        for field_info in &rf.fields {
            let field_name = field_info
                .field_name
                .replace("_#HCY#", "")
                .replace("_#CCY#", "");
            match reader_for_calling_method
                .get_field_type(&field_name)
                .unwrap_or(Type::String)
            {
                Type::I32 => {
                    let val = match record.get_i32_for_key(&field_info.field_name) {
                        Ok(value) => get_op_data(&field_info.output_file_type, &value.to_string()),
                        Err(_error) => get_default_data(&field_info.output_file_type),
                    };
                    op.push_str(&val.to_string());
                    op.push_str(config_params.field_delimiter());
                }

                Type::I64 => {
                    let val = match record.get_i64_for_key(&field_info.field_name) {
                        Ok(value) => get_op_data(&field_info.output_file_type, &value.to_string()),
                        Err(_error) => get_default_data(&field_info.output_file_type),
                    };
                    op.push_str(&val.to_string());
                    op.push_str(config_params.field_delimiter());
                }

                Type::F32 => {
                    let val = match record.get_f32_for_key(&field_info.field_name) {
                        Ok(value) => get_op_data(&field_info.output_file_type, &value.to_string()),
                        Err(_error) => get_default_data(&field_info.output_file_type),
                    };
                    op.push_str(&val.to_string());
                    op.push_str(config_params.field_delimiter());
                }

                Type::F64 => {
                    let mut val;
                    if field_info.field_name.contains("_#HCY#") {
                        let act_field_name = field_info
                            .field_name
                            .replace("_#HCY#", "")
                            .trim()
                            .to_string();
                        val = match record.get_f64_for_key(&act_field_name) {
                            Ok(value) => {
                                get_op_data(&field_info.output_file_type, &value.to_string())
                            }
                            Err(_error) => get_default_data(&field_info.output_file_type),
                        };
                        let amt: f64 = val.parse().unwrap_or(0.0);
                        val = currency_converter
                            .convert_to_base(amt, &acc_ccy, &logger)
                            .to_string();
                    } else if field_info.field_name.contains("_#CCY#") {
                        let act_field_name = field_info
                            .field_name
                            .replace("_#CCY#", "")
                            .trim()
                            .to_string();
                        val = match record.get_f64_for_key(&act_field_name) {
                            Ok(value) => {
                                get_op_data(&field_info.output_file_type, &value.to_string())
                            }
                            Err(_error) => get_default_data(&field_info.output_file_type),
                        };
                        let amt: f64 = val.parse().unwrap_or(0.0);
                        val = currency_converter
                            .convert_from_base(amt, &acc_ccy, &logger)
                            .to_string();
                    } else {
                        val = match record.get_f64_for_key(&field_info.field_name) {
                            Ok(value) => {
                                get_op_data(&field_info.output_file_type, &value.to_string())
                            }
                            Err(_error) => get_default_data(&field_info.output_file_type),
                        };
                    }
                    op.push_str(&val.to_string());
                    op.push_str(config_params.field_delimiter());
                }

                Type::String => {
                    if field_info.field_name.contains("BALM_LLG") {
                        llg = get_balm_llg(config_params, &record, &balm_rules, logger);
                        op.push_str(&llg.to_string());
                        op.push_str(config_params.field_delimiter());
                    } else {
                        let val = match record.get_string_for_key(&field_info.field_name) {
                            Ok(value) => {
                                get_op_data(&field_info.output_file_type, &value.to_string())
                            }
                            Err(_error) => get_default_data(&field_info.output_file_type),
                        };
                        op.push_str(&val.to_string());
                        op.push_str(config_params.field_delimiter());
                    }
                }
                _ => {
                    panic!("Cashflow data decryption not supported!!");
                }
            }
        }
        op.pop();
        op.push('\n');
        match config_params.output_option() {
            "ALL" => {
                let record_bytes: &[u8] = op.as_bytes();
                buffer_writer
                    .write(record_bytes)
                    .expect("Unable to write byte array.");
            }
            _ => {
                if llg_ids.contains(&llg.to_string()) {
                    let record_bytes: &[u8] = op.as_bytes();
                    buffer_writer
                        .write(record_bytes)
                        .expect("Unable to write byte array.");
                }
            }
        }
    }
    buffer_writer.flush().expect("Unable to flush the writer.");
    println!("Total no of records in input: {}", acc_encountered);
}

pub fn naivedate_from_timestamp(t: i64) -> rbdate::NaiveDate {
    if t == 0 {
        rbdate::NaiveDate::from_ymd(1900, 1, 1)
    } else {
        let naive_date_time = rbdate::NaiveDateTime::from_timestamp(t, 0);
        naive_date_time.date()
    }
}

pub fn get_default_data(data_type: &str) -> String {
    match &data_type[..] {
        "Integer" => 0.to_string(),
        "Float" => 0.0.to_string(),
        "Date" => "01-01-1900".to_string(),
        "String" => "".to_string(),
        _ => panic!("Invalid data type encountered: {}", data_type),
    }
}

pub fn get_op_data(data_type: &str, value: &str) -> String {
    match &data_type[..] {
        "Integer" => value.parse::<i64>().unwrap_or(0).to_string(),
        "Float" => value.parse::<f64>().unwrap_or(0.0).to_string(),
        "Date" => {
            let timestamp_val = value.parse::<i64>().unwrap_or(0);
            naivedate_from_timestamp(timestamp_val)
                .format("%d-%m-%Y")
                .to_string()
        }
        "String" => value.to_string(),
        _ => panic!("Invalid data type encountered: {}", data_type),
    }
}
