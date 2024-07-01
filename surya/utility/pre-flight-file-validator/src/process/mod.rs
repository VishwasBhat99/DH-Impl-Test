use calamine::*;
use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::time::SystemTime;

use self::structs::*;
use crate::process::config::DataCheck;
mod config;
mod structs;

pub fn validate_file(
    config_param: ConfigurationParameters,
    logger: &Logger,
    _diag_logger: &Logger,
) {
    let start_process_timer = SystemTime::now();
    let as_on_date = NaiveDate::parse_from_str(&config_param.as_on_date(), "%d-%m-%Y")
        .expect("Cannot parse as_on_date to a valid NaiveDate type.");
    let date_folder = as_on_date.format("%d%m%Y").to_string();
    let format1 = as_on_date.format("%Y%m%d").to_string();
    let format2 = as_on_date.format("%m%Y").to_string();
    let mut tot_acc = 0;
    let mut tot_fail = 0;
    let mut panic_flags = PanicFlags::new();
    //Check if the input file exists.
    let file_config = config::get_files(config_param.config_file_path());
    //Remove the health-check file.
    match fs::remove_file(&file_config.health_check_report_path) {
        Ok(_) => {}
        Err(e) => {
            log_error!(
                logger,
                "Could not remove the health-check-report:{}. Error:{}",
                file_config.health_check_report_path,
                e
            )
        }
    };
    let input_file_path = &file_config.input_file_path;
    let input_file_name = input_file_path
        .replace("{ddmmyyyy}", &date_folder)
        .replace("{yyyymmdd}", &format1)
        .replace("{mmyyyy}", &format2);
    let (_, input_file_extension) = input_file_name
        .rsplit_once('.')
        .expect("Unable to get the file extension");

    let field_separator = file_config.field_separator.unwrap_or("|".to_string());
    let sheet_name = file_config
        .excel_sheet_name
        .clone()
        .unwrap_or("Sheet1".to_string());
    let mut duplication_check_keys = file_config.duplication_check_keys.unwrap_or(Vec::new());
    let header_count = file_config.header_count;
    let footer_count = file_config.footer_count;
    let expected_column_count = file_config.expected_column_count.unwrap_or(0);
    let amount_col_no = file_config.amount_col_no.unwrap_or(0);
    let key_col_no = file_config.key_col_no.unwrap_or(Vec::new());
    let data_check_details = file_config.data_check.unwrap_or(DataCheck::def());
    let values_in_details = data_check_details.values_in.unwrap_or(Vec::new());
    let values_not_in_details = data_check_details.values_not_in.unwrap_or(Vec::new());
    let data_type_details = data_check_details.data_type.unwrap_or(Vec::new());
    let exclude_chars_vec = file_config.exclude_char_in_footer.unwrap_or(Vec::new());
    let mut tot_amt = 0.0;

    let mut date_mapper: HashMap<u8, String> = HashMap::new();
    for each_date_field in file_config.date_fields_formats.unwrap_or(Vec::new()).iter() {
        date_mapper.insert(
            each_date_field
                .field_column
                .parse::<u8>()
                .expect("Error Parsing Date Field Index"),
            each_date_field.date_format.to_string(),
        );
    }

    let file_size;
    //Define all the data-structures used in the processing.
    let mut dup_keys: HashMap<String, i64> = HashMap::new();

    let mut column_count: Vec<ColumnCount> = Vec::new();
    let mut data_type: HashMap<String, TypeCheck> = HashMap::new();
    let mut values_in: HashMap<String, Vec<ValueStr>> = HashMap::new();
    let mut values_not_in: HashMap<String, Vec<ValueStr>> = HashMap::new();

    match input_file_extension.to_uppercase().as_str() {
        "TXT" | "CSV" => {
            //Get the input file.
            let input_file = match new_buf_rdr(&input_file_name) {
                Ok(file) => BufReader::new(file),
                Err(error) => panic!("Could not find the file `{}`, {}", input_file_name, error),
            };
            //Check file size.
            let file = File::open(&input_file_name).expect("Cannot open file.");
            file_size = file
                .metadata()
                .expect("Could not fetch input file metadata.")
                .len() as usize;
            let ip_row = fs::read_to_string(&input_file_name).expect("Failed to read input file");
            let last_row_num = ip_row.lines().count().to_owned() - 1;
            let mut iterator = input_file
                .lines()
                .enumerate()
                .skip(header_count.unwrap_or(0) as usize)
                .peekable();
            //Read the Text File.
            while let Some((line_num, lines)) = iterator.next() {
                let line = match lines {
                    Ok(line) => line,
                    Err(error) => panic!(
                        "Unable to read the file `{}` at line number: `{}` : {}",
                        &input_file_name,
                        line_num + 1,
                        error
                    ),
                };
                let fields: Vec<&str> = line.split(&field_separator).collect();
                tot_acc += 1;
                if line_num < last_row_num
                    && line_num + 1 > header_count.unwrap_or(0).to_string().parse().unwrap_or(0)
                    && fields.len() as i64 != expected_column_count
                    && expected_column_count != 0
                {
                    column_count.push(ColumnCount {
                        row_no: line_num + 1,
                        col_count: fields.len(),
                    });
                    log_info!(
                        _diag_logger,
                        "ERR:COLUMN COUNT\n Expected:{}, Actual:{} in Line-No: {}",
                        expected_column_count,
                        fields.len(),
                        line_num + 1
                    );
                    continue;
                }

                //Check footer if it is an extracted file.
                if iterator.peek().is_none() {
                    if footer_count.unwrap_or(0) > 0 {
                        let slice_count = &line[exclude_chars_vec[0] as usize
                            ..(line.len() - exclude_chars_vec[1] as usize)]
                            .trim()
                            .replace(',', "");
                        let actual_line_count = line_num as i64;
                        if actual_line_count
                            != slice_count
                                .parse::<i64>()
                                .expect("Could not get row count from footer.")
                        {
                            //Row-Count Error.
                            panic_flags.row_count = true;
                            log_info!(
                                _diag_logger,
                                "ERR:ROW COUNT\n Expected:{}, Actual:{}\n",
                                slice_count,
                                actual_line_count
                            )
                        }
                        continue;
                    } else {
                        //Check if the last row matches the first row in an extracted file with no footer.
                        if fields.len() as i64 != expected_column_count
                            && expected_column_count != 0
                        {
                            column_count.push(ColumnCount {
                                row_no: line_num + 1,
                                col_count: fields.len(),
                            });
                            continue;
                        }
                    }
                }

                if amount_col_no != 0 {
                    tot_amt += fields[(amount_col_no - 1) as usize]
                        .parse::<f64>()
                        .unwrap_or(0.0);
                }

                let mut key_col_value = String::new();
                //Get the key column number:
                for key in key_col_no.iter() {
                    let key_val = format!("{}_", fields[(key - 1) as usize]);
                    key_col_value.push_str(&key_val);
                }
                key_col_value.pop();

                //Duplication key check.
                let mut key = String::new();
                if !duplication_check_keys.is_empty() {
                    for col_no in duplication_check_keys.iter_mut() {
                        let key_val = format!("{}_", fields[(*col_no - 1) as usize]);
                        key.push_str(&key_val);
                    }
                }
                if !duplication_check_keys.is_empty() {
                    dup_keys
                        .entry(key)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
                //Data-Type-Check:Integer or Float
                for col_details in data_type_details.iter() {
                    for index in col_details.col_no.iter() {
                        let value = fields[(index - 1) as usize];
                        match col_details.col_values[0].to_lowercase().as_str() {
                            "integer" => {
                                if value.parse::<i64>().is_err() {
                                    //Add to vector of data_types.
                                    data_type
                                        .entry(key_col_value.clone())
                                        .and_modify(|record| record.append_integer(*index))
                                        .or_insert(TypeCheck {
                                            integer: vec![*index],
                                            float: Vec::new(),
                                        });
                                }
                            }
                            "decimal" => {
                                if value.parse::<f64>().is_err() {
                                    data_type
                                        .entry(key_col_value.clone())
                                        .and_modify(|record| record.append_float(*index))
                                        .or_insert(TypeCheck {
                                            integer: Vec::new(),
                                            float: vec![*index],
                                        });
                                }
                            }
                            _ => {
                                panic!("Invalid DataType entered. Expected: 'Integer/Decimal'")
                            }
                        }
                    }
                }

                //Data-Check values-in:
                for col_details in values_in_details.iter() {
                    for index in col_details.col_no.iter() {
                        let value = fields[(index - 1) as usize];
                        if !col_details.col_values.contains(&value.to_string()) {
                            //Add to values in check vector.
                            values_in
                                .entry(key_col_value.clone())
                                .and_modify(|record| {
                                    record.push(ValueStr {
                                        col_no: *index,
                                        value: value.to_string(),
                                    })
                                })
                                .or_insert(vec![ValueStr {
                                    col_no: *index,
                                    value: value.to_string(),
                                }]);
                        }
                    }
                }

                //Data-Check values-not-in:
                for col_details in values_not_in_details.iter() {
                    for index in col_details.col_no.iter() {
                        let value = fields[(index - 1) as usize];
                        if col_details.col_values.contains(&value.to_string()) {
                            //Add to values in check vector.
                            values_not_in
                                .entry(key_col_value.clone())
                                .and_modify(|record| {
                                    record.push(ValueStr {
                                        col_no: *index,
                                        value: value.to_string(),
                                    })
                                })
                                .or_insert(vec![ValueStr {
                                    col_no: *index,
                                    value: value.to_string(),
                                }]);
                        }
                    }
                }

                //Date Field Checks
                for (field, format) in date_mapper.iter() {
                    let date_parser = rbdate::DateParser::new(format.to_string(), false);
                    match date_parser.parse_opt(fields[*field as usize - 1]).is_some() {
                        true => date_parser.parse_opt(fields[*field as usize - 1]),
                        false => {
                            panic_flags.date_check = true;
                            log_info!(
                                _diag_logger,
                                "ERR: Either Date Field Read: {} is Invalid OR Not in Expected Format: {} (Row-Num: {}, Column-Num: {})\n",
                                fields[*field as usize-1],
                                format,
                                line_num+1,
                                field,
                                );
                            continue;
                        }
                    };
                }
            }
        }
        "XLSX" | "XLS" => {
            //Check file size.
            let file = File::open(&input_file_name).expect("Cannot open excel file.");
            file_size = file.metadata().unwrap().len() as usize;
            let mut input_reader =
                open_workbook_auto(&input_file_name).expect("Unable to open Input xlsx File.");
            if !input_reader.sheet_names().contains(&sheet_name) {
                panic!(
                    "Sheet name: `{}` not present in Input-File: `{}`",
                    sheet_name, input_file_name
                );
            }
            if let Some(Ok(reader)) = input_reader.worksheet_range(&sheet_name) {
                let mut row_value: &[DataType];
                //Define all the data-structures used in the processing.
                let mut row_count = file_config.header_count.unwrap_or(0);
                for (_, row) in reader
                    .rows()
                    .enumerate()
                    .skip((file_config.header_count.unwrap_or(0)) as usize)
                {
                    row_value = row;
                    row_count += 1;
                    tot_acc += 1;
                    //Column count check
                    let col_count = file_config.expected_column_count.unwrap_or(0);
                    if row_value.len() as i64 != col_count {
                        if col_count == 0 {
                            continue;
                        }
                        panic_flags.column_count = true;
                        column_count.push(ColumnCount {
                            row_no: row_count as usize,
                            col_count: row_value.len(),
                        });
                        tot_fail += 1;
                        log_info!(
                            _diag_logger,
                            "ERR:COLUMN COUNT\nExpected:{}, Actual:{} in Line-No: {}",
                            expected_column_count,
                            row_value.len(),
                            row_count
                        );
                        continue;
                    }
                    let mut key_col_value = String::new();
                    //Get the key column number:
                    for key in key_col_no.iter() {
                        let key_val = format!("{}_", row[(key - 1) as usize]);
                        key_col_value.push_str(&key_val);
                    }
                    key_col_value.pop();

                    //Duplication key check.
                    let mut key = String::new();
                    if !duplication_check_keys.is_empty() {
                        for col_no in duplication_check_keys.iter_mut() {
                            let key_val = format!("{}_", row[(*col_no - 1) as usize]);
                            key.push_str(&key_val);
                        }
                    }
                    if !duplication_check_keys.is_empty() {
                        dup_keys
                            .entry(key)
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }
                    //Data-Type-Check:Integer or Float
                    for col_details in data_type_details.iter() {
                        for index in col_details.col_no.iter() {
                            let value = row[(index - 1) as usize].to_string();
                            match col_details.col_values[0].to_lowercase().as_str() {
                                "integer" => {
                                    if value.parse::<i64>().is_err() {
                                        //Add to vector of data_types.
                                        data_type
                                            .entry(key_col_value.clone())
                                            .and_modify(|record| record.append_integer(*index))
                                            .or_insert(TypeCheck {
                                                integer: vec![*index],
                                                float: Vec::new(),
                                            });
                                    }
                                }
                                "decimal" => {
                                    if value.parse::<f64>().is_err() {
                                        data_type
                                            .entry(key_col_value.clone())
                                            .and_modify(|record| record.append_float(*index))
                                            .or_insert(TypeCheck {
                                                integer: Vec::new(),
                                                float: vec![*index],
                                            });
                                    }
                                }
                                _ => {
                                    panic!("Invalid Data-Type entered. Expected: 'Integer/Float'")
                                }
                            }
                        }
                    }

                    //Data-Check values-in:
                    for col_details in values_in_details.iter() {
                        for index in col_details.col_no.iter() {
                            let value = row[(index - 1) as usize].to_string();
                            if !col_details.col_values.contains(&value.to_string()) {
                                //Add to values in check vector.
                                values_in
                                    .entry(key_col_value.clone())
                                    .and_modify(|record| {
                                        record.push(ValueStr {
                                            col_no: *index,
                                            value: value.to_string(),
                                        })
                                    })
                                    .or_insert(vec![ValueStr {
                                        col_no: *index,
                                        value: value.to_string(),
                                    }]);
                            }
                        }
                    }

                    //Data-Check values-not-in:
                    for col_details in values_not_in_details.iter() {
                        for index in col_details.col_no.iter() {
                            let value = row[(index - 1) as usize].to_string();
                            if col_details.col_values.contains(&value.to_string()) {
                                //Add to values in check vector.
                                values_not_in
                                    .entry(key_col_value.clone())
                                    .and_modify(|record| {
                                        record.push(ValueStr {
                                            col_no: *index,
                                            value: value.to_string(),
                                        })
                                    })
                                    .or_insert(vec![ValueStr {
                                        col_no: *index,
                                        value: value.to_string(),
                                    }]);
                            }
                        }
                    }
                }
            }
        }
        _ => {
            panic!("Invalid file extension:{}.", input_file_extension);
        }
    }

    //Write error messages to diaglog file.
    //Column count error:
    let mut column_count_error_message = String::new();
    if !column_count.is_empty() {
        panic_flags.column_count = true;
        for row_detail in column_count.iter() {
            if row_detail.row_no as i64 <= header_count.unwrap_or(0)
                || row_detail.row_no as i64 >= footer_count.unwrap_or(0)
            {
                continue;
            }
            column_count_error_message.push_str(
                format!(
                    "Row number {} has {} columns.\n",
                    row_detail.row_no, row_detail.col_count
                )
                .as_str(),
            );
        }
        log_info!(_diag_logger, "{}", column_count_error_message);
    }

    //Duplicate column error.
    if !dup_keys.is_empty() {
        let mut duplicate_data_error_message = String::new();
        for (key, count) in dup_keys.iter() {
            if count > &1 {
                duplicate_data_error_message.push_str(
                    format!("Key {} has duplicate entries {} times. \n", key, count).as_str(),
                );

                panic_flags.duplicate_key = true;
            }
        }
        if duplicate_data_error_message.contains("duplicate") {
            log_info!(_diag_logger, "{}", duplicate_data_error_message);
        }
    }

    //Values IN Error:
    if !values_in.is_empty() {
        panic_flags.values_in = true;
        let mut values_in_error_message = String::new();
        for (key_col, val_in_vec) in values_in.iter() {
            values_in_error_message
                .push_str(format!("Key: `{}` has values not in expected set:", key_col).as_str());
            for val in val_in_vec.iter() {
                values_in_error_message.push_str(
                    format!("column `{}` has value `{}`.", val.col_no, val.value).as_str(),
                );
            }
            values_in_error_message.push('\n');
        }
        log_info!(_diag_logger, "{}", values_in_error_message);
    }

    //Values NOT IN Error:
    if !values_not_in.is_empty() {
        panic_flags.values_not_in = true;
        let mut values_not_in_error_message = String::new();
        for (key_col, val_not_in_vec) in values_not_in.iter() {
            values_not_in_error_message.push_str(
                format!("Key: `{}` has values that must not be present:", key_col).as_str(),
            );
            for val in val_not_in_vec.iter() {
                values_not_in_error_message.push_str(
                    format!("column `{}` has value `{}`.", val.col_no, val.value).as_str(),
                );
            }
            values_not_in_error_message.push('\n');
        }
        log_info!(_diag_logger, "{}", values_not_in_error_message);
    }
    //Data-type Error:
    if !data_type.is_empty() {
        panic_flags.data_type = true;
        let mut data_type_error_message = String::new();
        for (key_col, data) in data_type.iter() {
            if !data.integer.is_empty() {
                data_type_error_message.push_str(
                    format!(
                        "Key: `{}` has errors in the data-type: `Integer` on columns:{:?}",
                        key_col, data.integer
                    )
                    .as_str(),
                );
            }
            if !data.float.is_empty() {
                data_type_error_message.push_str(
                    format!(
                        "Key: `{}` has errors in the data-type: `Decimal` in columns:{:?}",
                        key_col, data.float
                    )
                    .as_str(),
                );
            }
            data_type_error_message.push('\n');
        }
        log_info!(_diag_logger, "{}", data_type_error_message);
    }
    let end_process_timer = SystemTime::now();
    let total_duration = end_process_timer
        .duration_since(start_process_timer)
        .expect("Could not calculate total duration for pre-validation.");
    info!(
        logger,
        "Total Time Taken for validation: {:?}", total_duration
    );
    println!("Total Time Taken for validation: {:?}", total_duration);
    let mut error_flags = String::new();
    if panic_flags.duplicate_key {
        let msg = "Duplicate Keys".to_string();
        error_flags.push_str(&msg);
    }
    if panic_flags.row_count {
        let msg = "Row Count".to_string();
        error_flags.push_str(&msg);
    }
    if panic_flags.column_count {
        let msg: String = "Column Count".to_string();
        error_flags.push_str(&msg);
    }
    if panic_flags.values_in {
        let msg = "Values-IN".to_string();
        error_flags.push_str(&msg);
    }
    if panic_flags.values_not_in {
        let msg = "Values Not IN".to_string();
        error_flags.push_str(&msg);
    }
    if panic_flags.data_type {
        let msg: String = "Data Type".to_string();
        error_flags.push_str(&msg);
    }
    if panic_flags.date_check {
        let msg: String = "Date Check".to_string();
        error_flags.push_str(&msg);
    }
    //error_flags.pop();
    if !error_flags.is_empty() {
        panic!(
            "Issues found in the input file for {} Checks. Please check the DiagLog file: {} for details.",
            error_flags,
            config_param.diagnostics_file_path()
        );
    } else {
        println!(
            "Successfully processed the file: {}.",
            file_config.input_file_path
        );
    }

    let health_report = HealthReport::new(
        tot_acc,
        tot_acc - tot_fail,
        tot_fail,
        tot_amt,
        tot_amt,
        0,
        0.0,
        file_size,
    );
    log_info!(
        logger,
        "File:{}\n{}",
        file_config.health_check_report_path,
        health_report.display()
    );
    health_report.gen_health_rpt(&file_config.health_check_report_path);
}
