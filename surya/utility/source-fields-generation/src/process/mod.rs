use config::*;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::types::Type;
use sdb_io::buf_file_wrtr;
use sdb_io::open_file_read;
use slog::Logger;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;

use macros;

#[derive(Deserialize)]
struct RequiredFields {
    fields: Vec<String>,
}

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let files_config = get_files(config_params.config_file_path());
    let mut buffer_writer = buf_file_wrtr(config_params.output_file_path(), None)
        .expect("Unable to create writer for output file.");
    let mut tot_acc_encntrd = 0;
    let mut acc_pro_suc = 0;
    let tot_amt = 0.0;
    for file in files_config.files {
        if file.input_file_path.contains(".cf") {
            let mut reader_for_records =
                reader::Reader::new_at_path(&file.metadata_file_path, &file.input_file_path);
            let reader_for_calling_method =
                reader::Reader::new_at_path(&file.metadata_file_path, &file.input_file_path);

            let mut required_fields_file = open_file_read(&file.required_fields_file_path)
                .expect("Cannot open the required fields file.");
            let mut required_fields_buffer = String::new();
            required_fields_file
                .read_to_string(&mut required_fields_buffer)
                .expect("Cannot read the required fields file.");
            let rf: RequiredFields = serde_json::from_str(&required_fields_buffer[..])
                .expect("Unable to parse the required fields file.");

            let mut op = String::new();
            let record_reader = reader_for_records.iter();
            let mut multiple_records: HashSet<String> = HashSet::new();
            for record in record_reader {
                tot_acc_encntrd += 1;
                acc_pro_suc += 1;
                op.clear();
                for field_info in &rf.fields {
                    let field_name = field_info.replace("_#HCY#", "").replace("_#CCY#", "");

                    let mut val = "".to_string();
                    match reader_for_calling_method
                        .get_field_type(&field_name)
                        .unwrap_or(Type::String)
                    {
                        Type::I32 => {
                            val = match record.get_i32_for_key(&field_info) {
                                Ok(value) => value.to_string(),
                                Err(_error) => continue,
                            };
                        }

                        Type::I64 => {
                            val = match record.get_i64_for_key(&field_info) {
                                Ok(value) => value.to_string(),
                                Err(_error) => continue,
                            };
                        }

                        Type::F32 => {
                            val = match record.get_f32_for_key(&field_info) {
                                Ok(value) => value.to_string(),
                                Err(_error) => continue,
                            };
                        }

                        Type::F64 => {
                            val = match record.get_f64_for_key(&field_info) {
                                Ok(value) => value.to_string(),
                                Err(_error) => continue,
                            };
                        }

                        Type::String => {
                            val = match record.get_string_for_key(&field_info) {
                                Ok(value) => value.to_string(),
                                Err(_error) => continue,
                            };
                        }
                        Type::Cashflows => {}
                    }
                    let mut mul_key = String::new();
                    mul_key.push_str(&file.source_file_name);
                    mul_key.push('|');
                    mul_key.push_str(&field_name);
                    mul_key.push('|');
                    mul_key.push_str(&val.to_string());
                    if !multiple_records.contains(&mul_key) {
                        multiple_records.insert(mul_key.to_owned());
                        op.push_str(&mul_key);
                        op.push_str("|Y|admin|");
                        op.push_str(&config_params.as_on_date().format("%d-%m-%Y").to_string());
                        op.push_str("|admin|");
                        op.push_str(&config_params.as_on_date().format("%d-%m-%Y").to_string());
                        op.push('\n');
                    }
                }
                if op.len() == 0 {
                    continue;
                }
                op.pop();
                op.push('\n');
                let record_bytes: &[u8] = op.as_bytes();
                buffer_writer
                    .write(record_bytes)
                    .expect("Unable to write byte array.");
            }
        } else if file.input_file_path.contains(".txt") {
            let account_reader =
                reader::Reader::new_at_path(&file.metadata_file_path, &file.input_file_path);

            let input = File::open(&file.input_file_path).expect("Unable To Open Input txt File");
            let input_file = BufReader::new(input);
            let mut required_fields_file = open_file_read(&file.required_fields_file_path)
                .expect("Cannot open the required fields file.");
            let mut required_fields_buffer = String::new();
            required_fields_file
                .read_to_string(&mut required_fields_buffer)
                .expect("Cannot read the required fields file.");
            let rf: RequiredFields = serde_json::from_str(&required_fields_buffer[..])
                .expect("Unable to parse the required fields file.");

            let mut op = String::new();
            let mut multiple_records: HashSet<String> = HashSet::new();

            for (line_num, lines) in input_file.lines().enumerate() {
                let line = match lines {
                    Ok(line) => line,
                    Err(error) => panic!(
                        "Unable to read file `{}` at line number: `{}` : {}",
                        file.input_file_path,
                        line_num + 1,
                        error
                    ),
                };
                let delimiter = match file.delimiter {
                    Some(val) => {
                        log_info!(logger, "Delimiter used for the input file: {}", val);
                        val
                    }
                    None => {
                        log_info!(logger, "Delimiter used for the input file: '|'",);
                        log_debug!(
                            logger,
                            "Unable to find delimiter for Input file, default delimiter used: '|'"
                        );
                        '|'
                    }
                };

                let fields: Vec<&str> = line.split(delimiter).collect();

                tot_acc_encntrd += 1;
                acc_pro_suc += 1;
                op.clear();
                for field_info in &rf.fields {
                    let field_name = field_info.replace("_#HCY#", "").replace("_#CCY#", "");
                    let field_pos = match account_reader.get_field_pos(&field_info.to_string()) {
                        Some(val) => val,
                        None => {
                            panic!("Cannot get the field position from input file for the key.")
                        }
                    };
                    let val = fields[field_pos - 1].to_string();
                    let mut mul_key = String::new();
                    mul_key.push_str(&file.source_file_name);
                    mul_key.push('|');
                    mul_key.push_str(&field_name);
                    mul_key.push('|');
                    mul_key.push_str(&val.to_string());
                    if !multiple_records.contains(&mul_key) {
                        multiple_records.insert(mul_key.to_owned());
                        op.push_str(&mul_key);
                        op.push_str("|Y|admin|");
                        op.push_str(&config_params.as_on_date().format("%d-%m-%Y").to_string());
                        op.push_str("|admin|");
                        op.push_str(&config_params.as_on_date().format("%d-%m-%Y").to_string());
                        op.push('\n');
                    }
                }
                if op.len() == 0 {
                    continue;
                }
                op.pop();
                op.push('\n');
                let record_bytes: &[u8] = op.as_bytes();
                buffer_writer
                    .write(record_bytes)
                    .expect("Unable to write byte array.");
            }
        } else {
            panic!(
                "Unsupprted input file extension for file {}",
                &file.input_file_path
            )
        }
        buffer_writer.flush().expect("Unable to flush the writer.");
    }
    println!("Total no of records in input: {}", tot_acc_encntrd);
    let health_report = HealthReport::new(
        tot_acc_encntrd,
        acc_pro_suc,
        tot_acc_encntrd - acc_pro_suc,
        tot_amt,
        tot_amt,
        0,
    );
    health_report.gen_health_rpt(&config_params.output_file_path());
}
