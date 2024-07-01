use super::io::*;
use super::statics::*;
use crate::configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use sdb_io::*;
use std::collections::HashMap;
use std::io::Write;
use std::io::{prelude::*, BufReader};
pub fn process(config_params: &ConfigurationParameters) {
    println!("Processing Started,Please Wait .....");
    let total_positive_bal = DEFAULT_FLOAT;
    let total_negative_bal = DEFAULT_FLOAT;
    let mut ttl_acc_encntrd: i64 = DEFAULT_INT;
    let mut skp_acc: i64 = DEFAULT_INT;
    let mut master_map: HashMap<String, String> = HashMap::new();
    let master_file = match new_buf_rdr(config_params.master_file()) {
        Ok(file) => file,
        Err(error) => panic!("Unable to open master file : {:?}", error),
    };
    for line in BufReader::new(master_file).lines().skip(0) {
        let record = match line {
            Ok(ln) => ln,
            Err(error) => {
                panic!("Error while reading input file : {:?}", error);
            }
        };
        let fields: Vec<&str> = record.split("|").collect();
        if fields.len() == 0 {
            continue;
        }
        master_map.insert(fields[0].to_string(), fields[1].to_string());
    }
    let input_file = match new_buf_rdr(config_params.input_file()) {
        Ok(file) => file,
        Err(error) => panic!("Unable to open input file : {:?}", error),
    };
    let mut output_map: HashMap<String, Vec<String>> = HashMap::new();
    let ca_ops_stable = 33;
    let ca_ops_l_stable = 34;
    let ca_ops_inrate = 63;
    let no_ca_wd_acc = 76;
    let no_ca_nwd_acc = 81;
    for line in BufReader::new(input_file).lines().skip(0) {
        let record = match line {
            Ok(ln) => ln,
            Err(error) => {
                panic!("Error while reading input file : {:?}", error);
            }
        };
        let fields: Vec<&str> = record.split("|").collect();
        ttl_acc_encntrd += 1;
        if fields.len() == 0 {
            skp_acc += 1;
        }
        let basel_id = fields[2].to_string();
        let mut output_val = vec!["0".to_string(); 57];
        if basel_id != "6".to_string()
            && (fields[ca_ops_stable].to_string() != "0".to_string()
                || fields[ca_ops_l_stable].to_string() != "0".to_string()
                || fields[no_ca_wd_acc].to_string() != "0".to_string()
                || fields[no_ca_nwd_acc].to_string() != "0".to_string()
                || fields[ca_ops_inrate].to_string() != "0".to_string())
        {
            //case 1:when basel_id is 6
            let basel_id_6 = "6".to_string();
            let mut val_with_6 = vec!["0".to_string(); 57];
            val_with_6[0] = fields[5].to_string();
            val_with_6[1] = fields[ca_ops_stable].to_string();
            val_with_6[2] = fields[ca_ops_l_stable].to_string();
            val_with_6[45] = fields[no_ca_wd_acc].to_string();
            val_with_6[50] = fields[no_ca_nwd_acc].to_string();
            val_with_6[38] =
                (fields[76].parse().unwrap_or(0.0) + fields[81].parse().unwrap_or(0.0)).to_string();
            if output_map.contains_key(&basel_id_6) {
                let prev_val = output_map.get_mut(&basel_id_6).unwrap();
                for index in 1..prev_val.len() {
                    prev_val[index] = (prev_val[index].parse().unwrap_or(0.0)
                        + val_with_6[index].parse().unwrap_or(0.0))
                    .to_string();
                }
            } else {
                output_map.insert(basel_id_6, val_with_6);
            }

            //case 2:when basel_id is not 6
            let basel_id = fields[2].to_string();
            let mut output_index = 1;
            for index in ca_ops_stable..70 {
                output_val[output_index] = fields[index].to_string();
                output_index += 1;
            }
            output_index += 1;
            output_val[output_index] =
                (fields[77].parse().unwrap_or(0.0) + fields[82].parse().unwrap_or(0.0)).to_string();
            output_index += 1;
            for index in 71..fields.len() {
                output_val[output_index] = fields[index].to_string();
                output_index += 1;
            }
            output_val[0] = fields[5].to_string();
            output_val[1] = "0".to_string();
            output_val[2] = "0".to_string();
            output_val[45] = "0".to_string();
            output_val[50] = "0".to_string();
            if output_map.contains_key(&basel_id) {
                let prev_val = output_map.get_mut(&basel_id).unwrap();
                for index in 1..prev_val.len() {
                    prev_val[index] = (prev_val[index].parse().unwrap_or(0.0)
                        + output_val[index].parse().unwrap_or(0.0))
                    .to_string();
                }
            } else {
                output_map.insert(basel_id, output_val);
            }
        }
        // when the basel id is not 6 or value is not present above 4 columns
        else {
            let basel_id = fields[2].to_string();
            output_val[0] = fields[5].to_string();
            let mut output_index = 1;
            for index in ca_ops_stable..70 {
                output_val[output_index] = fields[index].to_string();
                output_index += 1;
            }
            output_val[output_index] =
                (fields[76].parse().unwrap_or(0.0) + fields[81].parse().unwrap_or(0.0)).to_string();
            output_index += 1;
            output_val[output_index] =
                (fields[77].parse().unwrap_or(0.0) + fields[82].parse().unwrap_or(0.0)).to_string();
            output_index += 1;
            for index in 71..fields.len() {
                output_val[output_index] = fields[index].to_string();
                output_index += 1;
            }
            output_val[0] = fields[5].to_string();

            if output_map.contains_key(&basel_id) {
                let prev_val = output_map.get_mut(&basel_id).unwrap();
                for index in 1..prev_val.len() {
                    prev_val[index] = (prev_val[index].parse().unwrap_or(0.0)
                        + output_val[index].parse().unwrap_or(0.0))
                    .to_string();
                }
            } else {
                output_map.insert(basel_id, output_val);
            }
        }
    }
    let mut output_file = get_writer(config_params.output_file_path());
    // write the output map in output file
    for (key, value) in output_map.iter_mut() {
        let as_on_date = config_params.as_on_date().format("%d-%m-%y").to_string();
        let country = config_params.country_name();
        let currency = value.remove(0);
        let cust_categary = (master_map.get(key).unwrap_or(&"99".to_string())).to_string();
        writeln!(
            output_file,
            "{}|{}|{}|{}|{}",
            as_on_date,
            country,
            currency,
            cust_categary,
            value.join("|")
        )
        .expect("output row can not be written");
    }
    let health_report = HealthReport::new(
        ttl_acc_encntrd,
        ttl_acc_encntrd - skp_acc,
        skp_acc,
        total_negative_bal + total_positive_bal,
        total_negative_bal + total_positive_bal,
        DEFAULT_INT,
    );
    health_report.gen_health_rpt(&config_params.output_file_path());
}
