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
    let mut is_cf_passed = false;
    if config_params.req_header() {
        if config_params.as_on_mandatory() {
            op.push_str("as_on_date");
            op.push_str("|")
        }
        for name in &rf.fields {
            if name.field_name == "cashflows" {
                is_cf_passed = true;
                op.push_str("int_amt|prin_amt|cf_date");
                continue;
            }
            op.push_str(&name.field_name);
            op.push_str("|")
        }
        op.push('\n');
        let record_bytes: &[u8] = op.as_bytes();
        buffer_writer
            .write(record_bytes)
            .expect("Unable to write byte array.");
    }
    op.clear();
    let mut cf_op = String::new();
    let mut acc_encountered = 0;
    let mut llg_position = 0;
    let balm_rules =
        AggRules::new_from_path(config_params.balm_rule_file_path(), &reader_for_records);
    let record_reader = reader_for_records.iter();
    for mut record in record_reader {
        acc_encountered += 1;
        // read account currency id
        let acc_ccy: String = record
            .get_string_for_key(config_params.acc_currency())
            .unwrap_or(&config_params.base_currency().to_string())
            .to_string();
        op.clear();
        if config_params.as_on_mandatory() {
            op.push_str(&config_params.as_on_date().format("%d-%m-%Y").to_string());
            op.push_str(config_params.field_delimiter());
        }
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
                    if field_info.field_name.to_uppercase().contains("BALM_LLG") {
                        let llg = get_balm_llg(config_params, &record, &balm_rules, logger);
                        op.push_str(&llg.to_string());
                        let op_fields: Vec<&str> =
                            op.split(config_params.field_delimiter()).collect();
                        llg_position = op_fields.len() - 1;
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

                Type::Cashflows => {
                    let cashflows = match record.remove_cfs_for_key(&field_info.field_name) {
                        Ok(value) => value,
                        Err(_error) => continue,
                    };
                    for cashflow in cashflows {
                        let cf_date = naivedate_from_timestamp(cashflow.date);
                        if config_params.req_overdue() {
                            if cf_date <= *config_params.as_on_date() {
                                let mut op_fields: Vec<&str> =
                                    op.split(&config_params.field_delimiter()).collect();
                                let llg = config_params.default_overdue_llg_code().to_string();
                                op_fields.remove(llg_position);
                                op_fields.insert(llg_position, &llg);
                                let output = op_fields.join(&config_params.field_delimiter());
                                cf_op.clear();
                                cf_op.push_str(&output);
                            } else {
                                cf_op.clear();
                                cf_op.push_str(&op);
                            }
                        } else {
                            cf_op.clear();
                            cf_op.push_str(&op);
                        }
                        cf_op.push_str(&cashflow.interest_amount.to_string());
                        cf_op.push_str("|");
                        cf_op.push_str(&cashflow.principal_amount.to_string());
                        cf_op.push_str("|");
                        cf_op.push_str(&cf_date.format("%d-%m-%Y").to_string());
                        cf_op.push('\n');
                        let record_bytes: &[u8] = cf_op.as_bytes();
                        buffer_writer
                            .write(record_bytes)
                            .expect("Unable to write byte array.");
                    }
                }
            }
        }
        op.pop();
        op.push('\n');
        if !is_cf_passed {
            let record_bytes: &[u8] = op.as_bytes();
            buffer_writer
                .write(record_bytes)
                .expect("Unable to write byte array.");
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
