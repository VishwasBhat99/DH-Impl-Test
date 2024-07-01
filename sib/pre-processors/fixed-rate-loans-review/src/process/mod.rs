use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use crate::process::input_account::*;
use crate::process::output_account::format_output;
use bucket::*;
use health_report::HealthReport;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::fs;
use std::io::BufWriter;
mod input_account;
use std::io::prelude::*;
mod bucket;
mod output_account;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let mut acc_enc = 0;
    let mut acc_proc = 0;

    let output_file = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{} Cannot read output file path", error),
    };
    let mut writer = BufWriter::new(output_file);
    //Reading Bucket Master File
    let mut bucket_map: HashMap<Bucket, i64> = HashMap::new();
    let bucket_master_reader = fs::read_to_string(config_params.bucket_master_file_path())
        .expect("Could Not Read Bucket master file");
    for line in bucket_master_reader.lines() {
        let bucket_master_vec: Vec<&str> = line.split('|').collect::<Vec<&str>>();
        let bucket_key: Bucket = Bucket::new(
            bucket_master_vec[0].to_string(),
            bucket_master_vec[1].to_string(),
        );
        bucket_map.insert(bucket_key, bucket_master_vec[2].parse::<i64>().unwrap_or(1));
    }
    //Reading Mapping Master File
    let mut mapping_master: HashMap<String, Vec<String>> = HashMap::new();
    let mapping_master_reader = fs::read_to_string(config_params.mapping_master_file_path())
        .expect("Could Not Read Mapping master file");
    for line in mapping_master_reader.lines() {
        let fields: Vec<&str> = line.split('|').collect::<Vec<&str>>();
        mapping_master
            .entry(fields[1].to_string().to_uppercase())
            .and_modify(|data| data.push(fields[0].to_uppercase().to_string()))
            .or_insert(vec![fields[0].to_string().to_uppercase()]);
    }
    //Reading Category Master File
    let mut category_map: HashMap<String, HashMap<i64, CategoryValue>> = HashMap::new();
    let mut loans_map: HashMap<String, HashMap<i64, CategoryValue>> = HashMap::new();

    let category_file = match new_buf_rdr(config_params.category_master_file_path()) {
        Ok(file) => file,
        Err(_error) => panic!(
            "File not found : `{}` Error:`{}`",
            config_params.category_master_file_path(),
            _error,
        ),
    };

    for (line_num, lines) in category_file.lines().enumerate() {
        let master_line = match lines {
            Ok(master_line) => master_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.category_master_file_path(),
                line_num + 1,
                error
            ),
        };
        let category_fields: Vec<&str> = master_line.split('|').collect();
        let sub_cat_desc = category_fields[3].to_string().to_uppercase();
        let bucket_id = category_fields[4].parse::<i64>().unwrap_or(0);
        let cat_val = CategoryValue::new(&category_fields);
        let mut new_entry = HashMap::new();
        new_entry.insert(bucket_id.to_owned(), cat_val.to_owned());
        //Sub_cat_desc in mapping master have to be derived from the other sub_cat_desc.
        if mapping_master.contains_key(&sub_cat_desc) {
            loans_map
                .entry(sub_cat_desc.to_owned())
                .and_modify(|data| {
                    data.insert(bucket_id, cat_val.to_owned());
                })
                .or_insert(new_entry.to_owned());
        } else {
            category_map
                .entry(sub_cat_desc)
                .and_modify(|data| {
                    data.insert(bucket_id, cat_val);
                })
                .or_insert(new_entry);
        }
    }
    //Reading Input File.
    let input_file = match new_buf_rdr(config_params.input_file_path()) {
        Ok(file) => file,
        Err(_error) => panic!(
            "File not found : `{}` Error:`{}`",
            config_params.input_file_path(),
            _error,
        ),
    };
    for (line_num, lines) in input_file.lines().enumerate() {
        acc_enc += 1;
        let input_line = match lines {
            Ok(input_line) => input_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.input_file_path(),
                line_num + 1,
                error
            ),
        };
        let input_fields: Vec<&str> = input_line.split('|').collect();
        let input_bucket_id =
            get_bucket_id(input_fields[14].parse::<i64>().unwrap_or(0), &bucket_map);

        let mut sub_cat_desc = input_fields[15].to_string().to_uppercase();
        if sub_cat_desc == "OTHERS" {
            sub_cat_desc = input_fields[16].to_string().to_uppercase();
        }
        match category_map.get_mut(&sub_cat_desc) {
            Some(cat_bucket_map) => {
                match cat_bucket_map.get_mut(&input_bucket_id) {
                    Some(cat_val) => {
                        cat_val.os_bal_ccy += input_fields[5].parse::<f64>().unwrap_or(0.0).abs();
                        cat_val.disb_bal_ccy +=
                            input_fields[12].parse::<f64>().unwrap_or(0.0).abs();
                        cat_val.lim_bal_ccy += input_fields[7].parse::<f64>().unwrap_or(0.0).abs();
                        cat_val.lim_ccy_int += input_fields[17].parse::<f64>().unwrap_or(0.0).abs();
                        acc_proc += 1;
                    }
                    None => {
                        //Skip the record.
                        log_debug!(
                            logger,
                            "BucketID:{} not found for sub-category:{}. Line no:{}",
                            input_bucket_id,
                            sub_cat_desc,
                            line_num + 1
                        );
                    }
                }
            }
            None => {
                //Skip the input record.
                log_debug!(
                    logger,
                    "Record at line number:{} skipped. sub_cat_desc:{} not found.",
                    line_num + 1,
                    sub_cat_desc
                );
            }
        }
        //Classify the amount based on the fixed/floating loans.
        for (loans_key, value) in mapping_master.iter_mut() {
            if value.contains(&sub_cat_desc) {
                match loans_map.get_mut(loans_key) {
                    Some(loan_bucket_map) => {
                        match loan_bucket_map.get_mut(&input_bucket_id) {
                            Some(loan_val) => {
                                loan_val.os_bal_ccy +=
                                    input_fields[5].parse::<f64>().unwrap_or(0.0).abs();
                                loan_val.disb_bal_ccy +=
                                    input_fields[12].parse::<f64>().unwrap_or(0.0).abs();
                                loan_val.lim_bal_ccy +=
                                    input_fields[7].parse::<f64>().unwrap_or(0.0).abs();
                                loan_val.lim_ccy_int +=
                                    input_fields[17].parse::<f64>().unwrap_or(0.0).abs();
                                acc_proc += 1;
                            }
                            None => {
                                //Skip the record.
                                log_debug!(
                                    logger,
                                    "BucketID:{} not found for sub-category:{}. Line no:{}",
                                    input_bucket_id,
                                    sub_cat_desc,
                                    line_num + 1
                                );
                            }
                        }
                    }
                    None => {
                        //Skip the input record.
                        log_debug!(
                            logger,
                            "Record at line number:{} skipped. sub_cat_desc:{} not found.",
                            line_num + 1,
                            sub_cat_desc
                        );
                    }
                }
            }
        }
    }
    for (sub_cat_desc, val) in category_map.iter() {
        let mut sub_cat_id_hm: HashMap<String, AmtSum> = HashMap::new();
        for (_cat_bucket_id, cat_value) in val.iter() {
            let amt_sum = AmtSum {
                sum_limbalccy: cat_value.lim_bal_ccy,
                sum_limccyint: cat_value.lim_ccy_int,
            };
            sub_cat_id_hm
                .entry(cat_value.sub_cat_id.to_owned())
                .and_modify(|data| data.add_sum(amt_sum.to_owned()))
                .or_insert(amt_sum);
        }
        for (cat_bucket_id, cat_value) in val.iter() {
            let op_str = format_output(
                sub_cat_desc,
                cat_bucket_id,
                &sub_cat_id_hm,
                cat_value,
                *config_params.as_on_date(),
                config_params.currency(),
            );
            match writer.write_all(op_str.as_bytes()) {
                Ok(val) => val,
                Err(error) => {
                    panic!("Error writing output data: {:?}", error);
                }
            }
        }
    }

    for (sub_cat_desc, val) in loans_map.iter() {
        let mut sub_cat_id_hm: HashMap<String, AmtSum> = HashMap::new();
        for (_cat_bucket_id, cat_value) in val.iter() {
            let amt_sum = AmtSum {
                sum_limbalccy: cat_value.lim_bal_ccy,
                sum_limccyint: cat_value.lim_ccy_int,
            };
            sub_cat_id_hm
                .entry(cat_value.sub_cat_id.to_owned())
                .and_modify(|data| data.add_sum(amt_sum.to_owned()))
                .or_insert(amt_sum);
        }
        for (cat_bucket_id, cat_value) in val.iter() {
            let op_str = format_output(
                sub_cat_desc,
                cat_bucket_id,
                &sub_cat_id_hm,
                cat_value,
                *config_params.as_on_date(),
                config_params.currency(),
            );
            match writer.write_all(op_str.as_bytes()) {
                Ok(val) => val,
                Err(error) => {
                    panic!("Error writing header data: {:?}", error);
                }
            }
        }
    }

    let health_report = HealthReport::new(acc_enc, acc_proc, acc_enc - acc_proc, 0.0, 0.0, 0);
    health_report.gen_health_rpt(config_params.output_file_path());
}
