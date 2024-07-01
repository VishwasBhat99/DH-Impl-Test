use crate::configuration_parameters::ConfigurationParameters;
use crate::io::*;
use crate::statics::*;
use health_report::HealthReport;
use rbdate::NaiveDate;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use std::collections::HashMap;
use std::io::*;
#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct CustBasel {
    pub cust_id: String,
    pub basel_id: String,
}
//process the whole data
pub fn process(config_params: ConfigurationParameters) {
    let total_positive_bal = DEFAULT_FLOAT;
    let total_negative_bal = DEFAULT_FLOAT;
    let mut ttl_acc_encntrd: i64 = DEFAULT_INT;
    let mut skp_acc: i64 = DEFAULT_INT;
    // read the master file
    let master_file = match new_buf_rdr(config_params.master_file_path()) {
        Ok(file) => file,
        Err(error) => panic!("Unable to open input file : {:?}", error),
    };
    let mut master_map: HashMap<String, String> = HashMap::new();
    for line in BufReader::new(master_file).lines().skip(0) {
        let record = match line {
            Ok(ln) => ln,
            Err(error) => {
                panic!("Error while reading input file : {:?}", error);
            }
        };
        let fields: Vec<&str> = record.split("|").collect();
        master_map.insert(fields[0].to_string(), fields[1].to_string());
    }
    //master file reading is completed --!

    let input_file = match new_buf_rdr(config_params.ret_cust_path()) {
        Ok(file) => file,
        Err(error) => panic!("Unable to open input file : {:?}", error),
    };

    let mut output_map: HashMap<String, Vec<i64>> = HashMap::new();
    // read rows and store in output map
    for line in BufReader::new(input_file).lines().skip(0) {
        let record = match line {
            Ok(ln) => ln,
            Err(error) => {
                panic!("Error while reading input file : {:?}", error);
            }
        };
        let fields: Vec<&str> = record.split("|").collect();
        ttl_acc_encntrd += 1;
        let key = fields[2].to_string();
        let def_val = vec![0; 8];
        let mut curr_row = def_val;
        if output_map.contains_key(&key) {
            curr_row = output_map.get_mut(&key).unwrap().to_vec();
        }
        let mut curr_row_index = 0;
        for index in 70..86 {
            if index > 73 && index < 81 {
                continue;
            }

            if fields[index].to_string() != "0".to_string() {
                curr_row[curr_row_index] += 1;
            }
            if index != 81 {
                curr_row_index += 1;
            }
        }
        output_map.insert(key, curr_row);
    }
    let mut op_writer = get_writer(&config_params.dis_smry_path());
    for (key, value) in output_map.iter() {
        let stamped_key = master_map.get(key).unwrap();
        let joined: String = value
            .iter()
            .map(|&id| "|".to_owned() + &id.to_string())
            .collect();
        let as_on_date_formatted = config_params.as_on_date().format("%d-%m-%Y");
        writeln!(
            op_writer,
            "{}|{}{}",
            as_on_date_formatted, &stamped_key, joined
        )
        .expect("Error in writing output")
    }

    let health_report = HealthReport::new(
        ttl_acc_encntrd,
        ttl_acc_encntrd - skp_acc,
        skp_acc,
        total_negative_bal + total_positive_bal,
        total_negative_bal + total_positive_bal,
        DEFAULT_INT,
    );
    health_report.gen_health_rpt(&config_params.dis_smry_path());
}
