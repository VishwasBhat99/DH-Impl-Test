use comparison::resource::*;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::NaiveDate;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::account_with_cfs::get_field_value;
use sdb_io::buf_file_wrtr;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::env;
use std::env::current_dir;
use math::round;
use std::io::prelude::*;
mod config;
mod resource;
#[derive(Debug, Deserialize)]
pub struct LstVal {
    lst_comp_val: Vec<String>,
    lst_cf: LstCashflow,
}
#[derive(Debug, Deserialize)]
pub struct LstCashflow {
    lst_prin_amt: String,
    lst_int_amt: String,
    lst_cf_date: String,
}

pub fn compare(config_params: ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let mut acc_enc = 0;
    let mut acc_succ = 0;
    let mut total_cfs = 0;

    let mut op_diff_fields = String::new();
    let mut op_match_fields = String::new();
    let mut only_lst_acc_nos: String = String::new();
    let mut only_cf_acc_nos: String = String::new();

    let config_fields = config::get_config_fields(config_params.config_file_path());
    let mut lst_key_vec: Vec<String> = Vec::new();
    let mut cf_key_vec: Vec<String> = Vec::new();

    for key in &config_fields.keys {
        lst_key_vec.push(key.lst_field_name.to_owned());
        cf_key_vec.push(key.cf_field_name.to_owned());
    }
    let mut lst_val_vec: Vec<String> = Vec::new();
    let mut cf_val_vec: Vec<String> = Vec::new();
    let mut data_type: Vec<String> = Vec::new();

    for field_name in &config_fields.comparison_fields {
        lst_val_vec.push(field_name.lst_field_name.to_owned());
        cf_val_vec.push(field_name.cf_field_name.to_owned());
        data_type.push(field_name.data_type.to_owned());
    }

    let scale = config_fields.decimal_places.as_str().parse::<i8>().unwrap_or(5);

    let cf_fields_in_lst = LstCashflow {
        lst_prin_amt: config_fields.cashflow.lst_principal_field,
        lst_int_amt: config_fields.cashflow.lst_interest_field,
        lst_cf_date: config_fields.cashflow.lst_date_field,
    };

    let lst_file = match new_buf_rdr(config_params.input_lst_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.input_lst_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    let mut input_cf_reader = reader::Reader::new_at_path(
        config_params.cf_metadata_file_path(),
        config_params.input_cf_file_path(),
    );
    let input_cf_reader_call = reader::Reader::new_at_path(
        config_params.cf_metadata_file_path(),
        config_params.input_cf_file_path(),
    );
    let input_lst_reader = reader::Reader::new_at_path(
        config_params.lst_metadata_file_path(),
        config_params.input_cf_file_path(),
    );

    let mut lst_map: HashMap<Vec<String>, LstVal> = HashMap::new();
    for (line_num, lines) in lst_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.input_lst_file_path(),
                line_num + 1,
                error
            ),
        };
        let lst_fields: Vec<&str> = line.split('|').collect();
        let mut lst_key_fields: Vec<String> = Vec::new();
        let mut lst_val_fields: Vec<String> = Vec::new();
        for key in &lst_key_vec {
            let lst_pos = match input_lst_reader.get_field_pos(&key.to_string()) {
                Some(val) => val,
                None => panic!("Cannot get the field position from lst file for the key."),
            };

            let lst_key_field = lst_fields[lst_pos - 1];
            lst_key_fields.push(lst_key_field.to_string());
        }
        for val in &lst_val_vec {
            let lst_pos = match input_lst_reader.get_field_pos(&val.to_string()) {
                Some(val) => val,
                None => panic!("Cannot read the field position from lst file for the value."),
            };
            let lst_val_field = lst_fields[lst_pos - 1];
            lst_val_fields.push(lst_val_field.to_string());
        }
        //LST Cashflow values.
        let mut cf_fields_in_lst_val: LstCashflow = LstCashflow {
            lst_prin_amt: "".to_string(),
            lst_int_amt: "".to_string(),
            lst_cf_date: "".to_string(),
        };
        //Cashflows for maturity sources.
        if &config_fields.cashflow.cf_field_name == "cashflows" {
            let prin_key = &cf_fields_in_lst.lst_prin_amt.clone();
            let lst_prin_pos = match input_lst_reader.get_field_pos(&prin_key.to_string()) {
                Some(val) => val,
                None => panic!("Cannot read key field from lst file for principal field."),
            };
            let lst_prin_field = lst_fields[lst_prin_pos - 1];
            let int_key = &cf_fields_in_lst.lst_int_amt.clone();
            let lst_int_pos = match input_lst_reader.get_field_pos(&int_key.to_string()) {
                Some(val) => val,
                None => panic!("Cannot read key field from lst file for interest field."),
            };
            let lst_int_field = lst_fields[lst_int_pos - 1];
            let cfdate_key = &cf_fields_in_lst.lst_cf_date.clone();
            let lst_cfdate_pos = match input_lst_reader.get_field_pos(&cfdate_key.to_string()) {
                Some(val) => val,
                None => panic!("Cannot read key field from lst file for cashflow date."),
            };
            let lst_cf_field = lst_fields[lst_cfdate_pos - 1];
            cf_fields_in_lst_val = LstCashflow {
                lst_prin_amt: lst_prin_field.to_string(),
                lst_int_amt: lst_int_field.to_string(),
                lst_cf_date: lst_cf_field.to_string(),
            }
        }

        let lst_value = LstVal {
            lst_comp_val: lst_val_fields,
            lst_cf: cf_fields_in_lst_val,
        };

        lst_map.insert(lst_key_fields, lst_value);
    }
    for mut account in input_cf_reader.iter() {
        acc_enc += 1;
        let mut cf_keys: Vec<String> = Vec::new();
        for key in &cf_key_vec {
            let cf_key_val = match get_field_value(&account, &input_cf_reader_call, key.to_string())
            {
                Ok(val) => val,
                Err(_err) => "".to_string(),
            };
            cf_keys.push(cf_key_val);
        }
        match lst_map.get(&cf_keys) {
            Some(lst_value) => {
                log_debug!(logger, "Match found for key(s):{:?} in lst map.", &cf_keys);
                //Compare the fields.
                let mut diff_line = String::new();
                let mut diff_flag = false;
                for (index, value) in cf_val_vec.iter().enumerate() {
                    let mut cf_value = match get_field_value(
                        &account,
                        &input_cf_reader_call,
                        value.to_string(),
                    ) {
                        Ok(val) => val,
                        Err(_err) => {
                            log_error!(logger,"Could not get the value for the field:{} from cf file. Empty string used.",value);
                            "".to_string()
                        }
                    };
                    let val_type = &data_type[index];
                    let mut lst_val_read = lst_value.lst_comp_val[index].clone();
                    match val_type.to_lowercase().as_str() {
                        "date" => {
                            let tm_lst_val = NaiveDate::parse_from_str(&lst_val_read, "%d-%m-%Y")
                                .unwrap_or_else(|_| naivedate_from_timestamp(0));
                            let tm_lst_val = rbdate::timestamp(tm_lst_val);
                            if tm_lst_val == cf_value.parse::<i64>().unwrap_or(0) {
                                diff_line.push_str("NA|NA|");
                            } else {
                                diff_flag = true;
                                let diff_val = format!(
                                    "{}|{}|",
                                    naivedate_from_timestamp(tm_lst_val).format("%d-%m-%Y"),
                                    naivedate_from_timestamp(cf_value.parse::<i64>().unwrap_or(0))
                                        .format("%d-%m-%Y")
                                );
                                diff_line.push_str(&diff_val);
                            }
                        }
                        "Float" => {
                            let cf_val = cf_value.as_str().parse::<f64>().unwrap_or(0.0);
                            let lst_val = lst_val_read.as_str().parse::<f64>().unwrap_or(0.0);
                            let cf_val_round = round::ceil(cf_val, scale);
                            let lst_val_round = round::ceil(lst_val, scale);
                            cf_value = cf_val_round.to_string();
                            lst_val_read = lst_val_round.to_string();

                            if cf_val_round == lst_val_round {
                                diff_line.push_str("NA|NA|");
                            } else {
                                diff_flag = true;
                                let diff_val = format!("{}|{}|", lst_val_round, cf_val_round);
                                diff_line.push_str(&diff_val);
                            }
                        }
                        _ => {
                            if cf_value == lst_val_read {
                                diff_line.push_str("NA|NA|");
                            } else {
                                diff_flag = true;
                                let diff_val = format!("{}|{}|", lst_val_read, cf_value);
                                diff_line.push_str(&diff_val);
                            }
                        }
                    };
                }

                //Compare cashflow value:
                if &config_fields.cashflow.cf_field_name == "cashflows" {
                    total_cfs += 1;
                    let mut cashflows = account
                        .remove_cfs_for_key(&"cashflows".to_string())
                        .expect("Error while removing cashflow from the pool of cashflows.");
                    let lst_cashflow = &lst_value.lst_cf;
                    for cf in cashflows.iter_mut() {
                        let prin_amount = round::ceil(cf.get_principal_amount(), scale);
                        let int_amount = round::ceil(cf.get_interest_amount(), scale);
                        let cf_date = naivedate_from_timestamp(cf.get_date());
                        let lst_cf_date =
                            NaiveDate::parse_from_str(&lst_cashflow.lst_cf_date, "%d-%m-%Y")
                                .unwrap_or_else(|_| naivedate_from_timestamp(0));
                        let lst_prin_amt = round::ceil(lst_cashflow.lst_prin_amt.parse::<f64>().unwrap_or(0.0), scale);
                        let lst_int_amount = round::ceil(lst_cashflow.lst_int_amt.parse::<f64>().unwrap_or(0.0), scale);
                        if lst_cf_date == cf_date {
                            if lst_prin_amt == prin_amount && lst_int_amount == int_amount {
                                //Cashflows match.
                                let diff = "NA|NA|NA|NA|NA|NA|".to_string();
                                if diff_flag {
                                    diff_line.push_str(&diff);
                                }
                            } else {
                                //Cashflows do not match.
                                diff_flag = true;
                                let diff = format!(
                                    "{}|{}|{}|{}|{}|{}|",
                                    lst_cf_date.format("%d-%m-%Y"),
                                    cf_date.format("%d-%m-%Y"),
                                    lst_prin_amt,
                                    prin_amount,
                                    lst_int_amount,
                                    int_amount
                                );
                                diff_line.push_str(&diff);
                            }
                        } else {
                            diff_flag = true;
                            //Cashflow dates do not match. Hence furthur check is not done.
                            let diff = format!(
                                "{}|{}|{}|{}|{}|{}|",
                                lst_cf_date.format("%d-%m-%Y"),
                                cf_date.format("%d-%m-%Y"),
                                lst_prin_amt,
                                prin_amount,
                                lst_int_amount,
                                int_amount
                            );
                            diff_line.push_str(&diff);
                        }
                    }
                }

                if diff_flag {
                    //Write the account to difference file.
                    let cf_key_format = get_key_format(&cf_keys);
                    let mut diff_op_format = format!("{}|{}", &cf_key_format, &diff_line);
                    diff_op_format.pop();
                    diff_op_format.push('\n');
                    op_diff_fields.push_str(&diff_op_format);
                } else {
                    //Write the account to match file.
                    let mut cf_key_format = get_key_format(&cf_keys);
                    cf_key_format.push('\n');
                    op_match_fields.push_str(&cf_key_format);
                }
                acc_succ += 1;
                lst_map.remove(&cf_keys);
            }
            None => {
                log_debug!(logger, "Match not found in lst map for key:{:?}", cf_keys);
                //To only cf accounts file.
                let mut cf_key_format = get_key_format(&cf_keys);
                cf_key_format.push('\n');
                only_cf_acc_nos.push_str(&cf_key_format);
            }
        };
    }
    for (lst_keys, _value) in lst_map {
        //To only lst accounts file.
        let mut lst_key_format = get_key_format(&lst_keys);
        lst_key_format.push('\n');
        only_lst_acc_nos.push_str(&lst_key_format);
    }
    let output_report_path = config_params.base_output_file_path();
    let output_cf_path = get_file_name(
        output_report_path.to_owned(),
        config_params.output_file1_name().to_string(),
    );
    let output_lst_path = get_file_name(
        output_report_path.to_owned(),
        config_params.output_file2_name().to_string(),
    );
    let output_diff_path = get_file_name(
        output_report_path.to_owned(),
        config_params.output_file3_name().to_string(),
    );
    let output_match_path = get_file_name(
        output_report_path.to_owned(),
        config_params.output_file4_name().to_string(),
    );

    let mut output_writer_cf = match buf_file_wrtr(&output_cf_path, None) {
        Ok(wrtr) => wrtr,
        Err(error) => {
            panic!(
                "Could not create file: `{}` at location `{}` : {:?}.",
                output_cf_path,
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    let mut output_writer_lst = match buf_file_wrtr(&output_lst_path, None) {
        Ok(wrtr) => wrtr,
        Err(error) => {
            panic!(
                "Could not create file: `{}` at location `{}` : {:?}.",
                output_lst_path,
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    let mut output_writer_diff = match buf_file_wrtr(&output_diff_path, None) {
        Ok(wrtr) => wrtr,
        Err(error) => {
            panic!(
                "Could not create file: `{}` at location `{}` : {:?}.",
                output_diff_path,
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    let mut output_writer_match = match buf_file_wrtr(&output_match_path, None) {
        Ok(wrtr) => wrtr,
        Err(error) => {
            panic!(
                "Could not create file: `{}` at location `{}` : {:?}.",
                output_match_path,
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    write_data(&mut output_writer_cf, only_cf_acc_nos, logger);
    write_data(&mut output_writer_lst, only_lst_acc_nos, logger);
    write_data(&mut output_writer_diff, op_diff_fields, logger);
    write_data(&mut output_writer_match, op_match_fields, logger);

    let health_stat = HealthReport::new(
        acc_enc,
        acc_succ,
        acc_enc - acc_succ,
        0.0,
        0.0,
        total_cfs as i64,
    );
    health_stat.gen_health_rpt(config_params.base_output_file_path())
}
