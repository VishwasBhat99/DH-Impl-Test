use self::exrt::*;
use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use crate::process::config::{AmountFields, WtFields};
use math::round;
use sdb_agg_rules::agg_rules::{AggRules, get_all_llgs};
use sdb_agg_rules_txt::agg_rules::AggRules as AggRules_txt;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::account_with_cfs::get_field_value;
use sdb_io::buf_file_wrtr;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

use std::time::SystemTime;
mod config;
mod exrt;
mod implementation;

#[derive(Debug, Clone)]
pub struct ValStruct {
    pub sum: f64,
    pub wt_sum: f64,
}
#[derive(Debug)]
pub struct WtdMap {
    pub field: Vec<String>,
    pub mapping_value: Vec<String>,
}
#[derive(Debug, Clone)]
pub struct FinalFields {
    pub weighted_sum: Vec<ValStruct>,
    pub amount_sum: Vec<f64>,
}

pub fn process(config_params: ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let start_time = SystemTime::now();
    //Create Global Data Structures for referencing and writing final output.
    let mut global_aggr_key_vec = Vec::new();
    let mut global_aggr_values_vec = Vec::new();
    let mut global_weight_map_vec = Vec::new();
    let mut global_op_fields_vec: Vec<AmountFields> = Vec::new();
    let mut grouping_hashmap: HashMap<Vec<String>, FinalFields> = HashMap::new();
    let mut file_count = 0;
    let mut derived_llg_map: HashMap<String, bool> = HashMap::new();
    let mut abs_llg_vec: Vec<String> = Vec::new();
    let mut negative_llg_vec: Vec<String> = Vec::new();
    let mut zero_flag:bool=false;
    let mut llg_vec:Vec<i32>=Vec::new();
    let mut default_ccy:String="".to_string();
    let mut currency_field="".to_string();
    let mut default_country="".to_string();

    //Read exrt file and store data in hashmap.
    let mut ex_rt_map: HashMap<ExrtKey, String> = HashMap::new();
    let ex_rt_file = match new_buf_rdr(config_params.exchange_rate_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}`Error : {}.",
            config_params.exchange_rate_file(),
            error
        ),
    };
    for (line_num, lines) in ex_rt_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.exchange_rate_file(),
                line_num + 1,
                error
            ),
        };
        let derived_fields: Vec<&str> = line.split('|').collect();
        let ex_rt_key = ExrtKey::new(derived_fields[0].to_string(), derived_fields[1].to_string());
        ex_rt_map.insert(ex_rt_key, derived_fields[2].to_string());
    }
    let default_exrt_val = "1".to_string();
    let decimal_places = config_params.decimal_places();
    //Get and process the config fields.
    let files_config = config::get_files(config_params.config_file_path());
    for config_fields in files_config.files {
        file_count += 1;
        abs_llg_vec=match config_fields.abs_llgs{
            Some(val)=>val,
            None =>  vec!["0".to_string()]
        };
        negative_llg_vec=match config_fields.negative_llgs{
            Some(val)=>val,
            None =>  vec!["0".to_string()]
        };
        currency_field=config_fields.currency_field.to_string();
        zero_flag=match config_fields.display_zero_assigned_value {
            Some(val)=>val,
            None =>  false
        };
        default_ccy=match  config_fields.default_ccy{
            Some(val)=>val,
            None =>  "INR".to_string()
        };
        default_country=match config_fields.default_country{
            Some(val)=>val,
            None =>  "NA".to_string()
        };
        let mut account_reader = reader::Reader::new_at_path(
            &config_fields.metadata_file_path,
            &config_fields.input_file_path,
        );
        let mut aggr_key_vec: Vec<String> = Vec::new();
        let mut aggr_values_vec: Vec<AmountFields> = Vec::new();
        let mut weight_vec: Vec<WtdMap> = Vec::new();
        let mut weight_map_vec: Vec<String> = Vec::new();
        let mut wtd_avg_fields: Vec<WtFields> = Vec::new();
        if !config_fields.wt_avg_fields.is_empty() {
            wtd_avg_fields = config_fields.wt_avg_fields;
        }
        if !(wtd_avg_fields.is_empty()) {
            for field in wtd_avg_fields.iter() {
                weight_map_vec.push(field.field_name.to_owned());
                let amount_vec = field.wt_values.amt.to_owned();
                let multiplier_vec = field.wt_values.multiplier.to_owned();
                if amount_vec.len() != multiplier_vec.len() {
                    panic!("Number of fields in `amt` and `multiplier` do not match for the field name:{}",field.field_name);
                } else {
                    weight_vec.push(WtdMap {
                        field: amount_vec,
                        mapping_value: multiplier_vec,
                    });
                }
            }
        }
        drop(wtd_avg_fields);
        let op_fields_vec: Vec<AmountFields> = config_fields.op_fields;
        if file_count == 1 {
            //Store the first file configurations in global DS.
            global_aggr_key_vec = config_fields.aggr_keys;
            global_aggr_values_vec = config_fields.aggr_values;

            global_weight_map_vec = weight_map_vec.clone();
            global_op_fields_vec = op_fields_vec.clone();
            aggr_key_vec = global_aggr_key_vec.to_owned();
            aggr_values_vec = global_aggr_values_vec.to_owned();
        } else {
            //Validate the current configuration with the first file's configuration.
            aggr_key_vec = config_fields.aggr_keys;
            aggr_values_vec = config_fields.aggr_values;
            if aggr_key_vec.len() != global_aggr_key_vec.len()
                || aggr_values_vec.len() != global_aggr_values_vec.len()
                || weight_map_vec.len() != global_weight_map_vec.len()
                || op_fields_vec.len() != global_op_fields_vec.len()
            {
                log_error!(logger,"The count of the number of fields in aggr_keys,aggr_values,wt_avg_fields or op_fields in file at position:{} does not match the first file's count.",file_count);
                continue;
            } else {
                //Validate the current output fields configuration with first file's output configuration.

                for op_field in op_fields_vec.iter() {
                    if !(aggr_key_vec.contains(&op_field.field_name)
                        || aggr_values_vec.contains(op_field)
                        || weight_map_vec.contains(&op_field.field_name))
                    {
                        log_error!(logger,"The output field:{} is not found in aggr_keys,aggr_values or wt_avg_fields for file at position:{} .",&op_field.field_name.to_string(),file_count);
                        continue;
                    }
                }
            }
        }

        let base_currency = config_fields.base_currency;
        let rules_file_path=match config_fields.rule_file_path{
            None=>"NA".to_string(),
            Some(val)=>val.to_string()
        };
        let default_llg_code= match config_fields.default_llg_code{
            None=>"8888".to_string(),
            Some(val)=>val.to_string()
        };
        let rules = AggRules::new_from_path(&rules_file_path, &account_reader);
        //Start processing the input file.
        if config_fields.input_file_path.to_owned().ends_with(".cf") {
            let input_reader = reader::Reader::new_at_path(
                &config_fields.metadata_file_path,
                &config_fields.input_file_path.to_owned(),
            );
            llg_vec=get_all_llgs(&rules_file_path).expect("Could not find values");
            for account in account_reader.iter() {
                let mut key_vec: Vec<String> = Vec::new();
                let mut val_vec: Vec<f64> = Vec::new();
                let mut wtd_vec: Vec<ValStruct> = Vec::new();
                let foreign_currency = get_field_value(
                    &account,
                    &input_reader,
                    config_fields.currency_field.to_owned(),
                )
                .unwrap_or(base_currency.to_owned());
                for key in aggr_key_vec.iter() {
                    if config_fields.is_rules_applied.is_some() {
                        let llg = implementation::llg_for_account(
                            &account,
                            &rules,
                            default_llg_code.to_owned(),
                            logger,
                        );
                        llg_vec.push(llg);
                      //vec of llg
                        if key == "LLG" {
                            key_vec.push(llg.to_string());
                            derived_llg_map.insert(llg.to_string(), true);
                            continue;
                        }
                        if key == "ASONDATE" {
                            key_vec.push(config_params.as_on_date().format("%d-%m-%Y").to_string());
                            continue;
                        }
                    }
                    let get_key_field = get_field_value(&account, &input_reader, key.to_string())
                        .expect("Cannot get the field value for a key");
                    key_vec.push(get_key_field);
                }
                for val in aggr_values_vec.iter() {
                    let mut get_value_field = if val.field_name.contains("_#CONSOL#") {
                        let act_field_name =
                            val.field_name.replace("_#CONSOL#", "").trim().to_string();
                        let temp_val_field =
                            get_field_value(&account, &input_reader, act_field_name)
                                .expect("Cannot get field value for a value.")
                                .parse::<f64>()
                                .unwrap_or(0.0);
                        let get_exrt_key =
                            ExrtKey::new(foreign_currency.to_owned(), base_currency.to_owned());
                        let ex_rt = ex_rt_map
                            .get(&get_exrt_key)
                            .unwrap_or(&default_exrt_val)
                            .to_string();
                        temp_val_field * ex_rt.parse::<f64>().unwrap_or(0.0)
                    } else {
                        get_field_value(&account, &input_reader, val.field_name.to_string())
                            .expect("Cannot get field value for a value.")
                            .parse::<f64>()
                            .unwrap_or(0.0)
                    };
                    if val.operator.contains(&"neg".to_string()) {
                        get_value_field *= -1.0;
                    }
                    if val.operator.contains(&"abs".to_string()) {
                        get_value_field = get_value_field.abs();
                    }
                    val_vec.push(get_value_field);
                }
                for weight in weight_vec.iter() {
                    let amt_vec = &weight.field;
                    let multiplier_vec = &weight.mapping_value;
                    let mut weighted_sum = 0.0;
                    let mut aggr_sum = 0.0;
                    for i in 0..amt_vec.len() {
                        let wt =
                            get_field_value(&account, &input_reader, multiplier_vec[i].to_owned())
                                .expect("Cannot get field value for a value.")
                                .parse::<f64>()
                                .unwrap_or(0.0);
                        //Logic for consolidated amount included
                        let amt = if amt_vec[i].contains("_#CONSOL#") {
                            let act_field_name =
                                amt_vec[i].replace("_#CONSOL#", "").trim().to_string();
                            let temp_amt_field =
                                get_field_value(&account, &input_reader, act_field_name)
                                    .expect("Cannot get field value for a value.")
                                    .parse::<f64>()
                                    .unwrap_or(0.0);
                            let default_exrt_val = "1".to_string();
                            let get_exrt_key =
                                ExrtKey::new(foreign_currency.to_owned(), base_currency.to_owned());
                            let ex_rt = ex_rt_map
                                .get(&get_exrt_key)
                                .unwrap_or(&default_exrt_val)
                                .to_string();
                            temp_amt_field * ex_rt.parse::<f64>().unwrap_or(0.0)
                        } else {
                            get_field_value(&account, &input_reader, amt_vec[i].to_string())
                                .expect("Cannot get field value for a value.")
                                .parse::<f64>()
                                .unwrap_or(0.0)
                        };
                        aggr_sum += amt;
                        weighted_sum += wt * amt;
                    }
                    wtd_vec.push(ValStruct {
                        sum: aggr_sum,
                        wt_sum: weighted_sum,
                    });
                }
                let map_value = FinalFields {
                    weighted_sum: wtd_vec,
                    amount_sum: val_vec,
                };
                grouping_hashmap
                    .entry(key_vec)
                    .and_modify(|data| append_data(data, map_value.to_owned()))
                    .or_insert(map_value);
            }
        } else if config_fields.input_file_path.to_owned().ends_with(".txt") {
            //Handle .txt field data.
            let input =
                File::open(&config_fields.input_file_path).expect("Unable To Open Input txt File");
            let input_file = BufReader::new(input);

            let ccy_field_pos =
                match account_reader.get_field_pos(&config_fields.currency_field.to_string()) {
                    Some(val) => val,
                    None => panic!(
                        "Cannot get the field position for currency field from metadata file."
                    ),
                };
                llg_vec=get_all_llgs(&rules_file_path).expect("Could not find values");
            let rules =
                AggRules_txt::new_from_path(&rules_file_path, &account_reader);
            for (line_num, lines) in input_file.lines().enumerate() {
                let line = match lines {
                    Ok(line) => line,
                    Err(error) => panic!(
                        "Unable to read file `{}` at line number: `{}` : {}",
                        config_fields.input_file_path,
                        line_num + 1,
                        error
                    ),
                };
                let delimiter = match config_fields.delimiter {
                    Some(val) => {
                        log_info!(
                            logger,
                            "Delimiter used for the input file: {}",
                            val
                        );
                        val
                    }
                    None => {
                        log_info!(
                            logger,
                            "Delimiter used for the input file: '|'",
                        );
                        log_debug!(
                            logger,
                            "Unable to find delimiter for Input file, default delimiter used: '|'"
                        );
                        '|'
                    }
                };
                let fields: Vec<&str> = line.split(delimiter).collect();
                let mut key_vec: Vec<String> = Vec::new();
                let mut val_vec: Vec<f64> = Vec::new();
                let mut wtd_vec: Vec<ValStruct> = Vec::new();
                for key in aggr_key_vec.iter() {
                    if config_fields.is_rules_applied.is_some() {
                        let llg = match rules.llg_for_acc(&line, &account_reader) {
                            Some(c) => c.llg,
                            None => default_llg_code.parse().unwrap_or(8888),
                        };
                        if key == "LLG" {
                            key_vec.push(llg.to_string());
                            derived_llg_map.insert(llg.to_string(), true);
                            continue;
                        }
                        if key == "ASONDATE" {
                            key_vec.push(config_params.as_on_date().format("%d-%m-%Y").to_string());
                            continue;
                        }
                    }
                    let field_pos = match account_reader.get_field_pos(&key.to_string()) {
                        Some(val) => val,
                        None => {
                            panic!("Cannot get the field position from input file for the key.")
                        }
                    };
                    let get_key_field = fields[field_pos - 1].to_string();
                    key_vec.push(get_key_field);
                }
                let foreign_currency = fields[ccy_field_pos - 1].to_string();
                let get_exrt_key =
                    ExrtKey::new(foreign_currency.to_owned(), base_currency.to_owned());
                let ex_rt = ex_rt_map
                    .get(&get_exrt_key)
                    .unwrap_or(&default_exrt_val)
                    .to_string();

                for val in aggr_values_vec.iter() {
                    //Logic for consolidated amount included
                    let mut get_value_field = if val.field_name.contains("_#CONSOL#") {
                        let act_field_name =
                            val.field_name.replace("_#CONSOL#", "").trim().to_string();
                        let field_pos =
                            match account_reader.get_field_pos(&act_field_name.to_string()) {
                                Some(val) => val,
                                None => panic!(
                                    "Cannot get the field position from input file for the key."
                                ),
                            };
                        let temp_amt_field = fields[field_pos - 1]
                            .to_string()
                            .parse::<f64>()
                            .unwrap_or(0.0);
                        temp_amt_field * ex_rt.parse::<f64>().unwrap_or(0.0)
                    } else {
                        let field_pos =
                            match account_reader.get_field_pos(&val.field_name.to_string()) {
                                Some(val) => val,
                                None => panic!(
                                    "Cannot get the field position from input file for the key."
                                ),
                            };
                        fields[field_pos - 1]
                            .to_string()
                            .parse::<f64>()
                            .unwrap_or(0.0)
                    };
                    if val.operator.contains(&"neg".to_string()) {
                        get_value_field *= -1.0;
                    }
                    if val.operator.contains(&"abs".to_string()) {
                        get_value_field = get_value_field.abs();
                    }
                    val_vec.push(get_value_field);
                }
                for weight in weight_vec.iter() {
                    let amt_vec = &weight.field;
                    let mut weighted_sum = 0.0;
                    let mut aggr_sum = 0.0;
                    let multiplier_vec = &weight.mapping_value;
                    for i in 0..amt_vec.len() {
                        let wt_field_pos = match account_reader.get_field_pos(&multiplier_vec[i]) {
                            Some(val) => val,
                            None => {
                                panic!("Cannot get the field position from input file for the key.")
                            }
                        };
                        let get_wt_key_field =
                            fields[wt_field_pos - 1].parse::<f64>().unwrap_or(0.0);
                        //Logic for consolidated amount included
                        let get_amt_key_field = if amt_vec[i].contains("_#CONSOL#") {
                            let act_field_name =
                                amt_vec[i].replace("_#CONSOL#", "").trim().to_string();
                            let field_pos =
                                match account_reader.get_field_pos(&act_field_name.to_string()) {
                                    Some(val) => val,
                                    None => panic!(
                                        "Cannot get the field position from input file for the key."
                                    ),
                                };
                            let temp_amt_field = fields[field_pos - 1]
                                .to_string()
                                .parse::<f64>()
                                .unwrap_or(0.0);
                            temp_amt_field * ex_rt.parse::<f64>().unwrap_or(0.0)
                        } else {
                            let amt_field_pos = match account_reader.get_field_pos(&amt_vec[i]) {
                                Some(val) => val,
                                None => panic!(
                                    "Cannot get the field position from input file for the key."
                                ),
                            };
                            fields[amt_field_pos - 1].parse::<f64>().unwrap_or(0.0)
                        };

                        weighted_sum += get_wt_key_field * get_amt_key_field;
                        aggr_sum += get_amt_key_field;
                    }
                    wtd_vec.push(ValStruct {
                        sum: aggr_sum,
                        wt_sum: weighted_sum,
                    })
                }
                let map_value = FinalFields {
                    weighted_sum: wtd_vec,
                    amount_sum: val_vec,
                };
                grouping_hashmap
                    .entry(key_vec)
                    .and_modify(|data| append_data(data, map_value.to_owned()))
                    .or_insert(map_value);
            }
        } else {
            panic!("Cannot discern input file format. Expected .txt or .cf file.");
        }
    }

    llg_vec.retain(|&x| !derived_llg_map.contains_key(&x.to_string()));
    let output_file = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => {
            panic!(
                "Could not create output file: `{}` on location `{}`: {}.",
                config_params.output_file_path(),
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    

    //Writing to output in order provided in config file.
    let mut writer = BufWriter::new(output_file);

    for (key, value) in grouping_hashmap.iter() {
        let mut op_line = String::new();
        let mut abs_flag = false;
        let mut negative_flag = false;
        for op_field in global_op_fields_vec.iter() {
            if global_aggr_key_vec.contains(&op_field.field_name) {
                let key_index = global_aggr_key_vec
                    .iter()
                    .position(|r| r == op_field.field_name.as_str())
                    .unwrap();
                let op_line_field = key[key_index].to_owned();
                if op_field.field_name.to_uppercase() == "LLG" {
                    if derived_llg_map.contains_key(&op_line_field)
                        && negative_llg_vec.contains(&op_line_field)
                    {
                        negative_flag = true;
                    } else if derived_llg_map.contains_key(&op_line_field)
                        && abs_llg_vec.contains(&op_line_field)
                    {
                        abs_flag = true;
                    }
                }
                let op_format = format!("{}|", op_line_field);
                op_line.push_str(&op_format);
            } else if global_aggr_values_vec.contains(op_field) {
                let key_index = global_aggr_values_vec
                    .iter()
                    .position(|r| r == op_field)
                    .unwrap();
                let op_line_field = value.amount_sum[key_index].to_owned();
                let mut formatted_op_line_field = round::ceil(op_line_field, *decimal_places);
                if negative_flag {
                    formatted_op_line_field *= -1.0;
                } else if abs_flag {
                    formatted_op_line_field = formatted_op_line_field.abs();
                }
                let op_format = format!("{}|", formatted_op_line_field);
                op_line.push_str(&op_format);
            } else if global_weight_map_vec.contains(&op_field.field_name) {
                let key_index = global_weight_map_vec
                    .iter()
                    .position(|r| r == op_field.field_name.as_str())
                    .unwrap();
                let wtd_values = value.weighted_sum[key_index].to_owned();
                let mut final_weighted_sum = wtd_values.wt_sum / wtd_values.sum;
                if wtd_values.sum == 0.0 {
                    final_weighted_sum = 0.0
                }
                let mut formatted_final_weighted_sum =
                    round::ceil(final_weighted_sum, *decimal_places);
                if negative_flag {
                    formatted_final_weighted_sum *= -1.0;
                } else if abs_flag {
                    formatted_final_weighted_sum = formatted_final_weighted_sum.abs();
                }
                let op_format = format!("{}|", formatted_final_weighted_sum);
                op_line.push_str(&op_format);
            }
        }
    
        op_line.pop();
        op_line.push('\n');
        writer
            .write_all(op_line.as_bytes())
            .expect("Could not write to the output file.");
    }
    let mut op_line = String::new();
    if zero_flag==true {
        let tot_len=llg_vec.len();
        let mut vec_index=0;
        for op_field in global_op_fields_vec.iter() {
          if op_field.field_name.to_uppercase() == "LLG" && vec_index < tot_len{
               let llg_id=llg_vec.get(vec_index).unwrap_or(&0);
               let op_format = format!("{}|",llg_id );
               op_line.push_str(&op_format);
               vec_index +=1;
           }
           else if op_field.field_name.trim().to_lowercase() == currency_field.trim().to_lowercase(){
            let op_format = format!("{}|",default_ccy );
            op_line.push_str(&op_format);
           }
           else if op_field.field_name.trim() == "Country" {
            let op_format = format!("{}|",default_country );
            op_line.push_str(&op_format);
           }
           else{
            let op_format = format!("{}|",0 );
            op_line.push_str(&op_format);
           }
        }
        op_line.pop();
        op_line.push('\n');
        writer
            .write_all(op_line.as_bytes())
            .expect("Could not write to the output file.");
   }
   
    let end_timer = SystemTime::now();
    let process_duration = end_timer
        .duration_since(start_time)
        .expect("Could not calculate total duration.");
    info!(logger, "Time for processing {:?}", process_duration);
    println!("Time for processing: {:?}", process_duration);
}
pub fn append_data(org_data: &mut FinalFields, new_data: FinalFields) {
    for i in 0..org_data.amount_sum.len() {
        org_data.amount_sum[i] += new_data.amount_sum[i];
    }
    for i in 0..org_data.weighted_sum.len() {
        org_data.weighted_sum[i].wt_sum += new_data.weighted_sum[i].wt_sum;
        org_data.weighted_sum[i].sum += new_data.weighted_sum[i].sum;
    }
}
