use chrono::NaiveDate;
use rbdate::timestamp;
use sdb_io;
use stamp_ftp::aggr_key::Customer;
use statics::DEFAULT_INT;
use std::collections::HashMap;
use std::io::BufRead;
use std::path::Path;

pub fn read_file(aggr_file_path: &str) -> HashMap<Customer, f64> {
    let mut cust_aggr_bal: HashMap<Customer, f64> = HashMap::new();

    if Path::new(&aggr_file_path).exists() {
        let rdr = match sdb_io::new_buf_rdr(aggr_file_path) {
            Ok(r) => r,
            Err(e) => panic!(format!(
                "Cannot read file at path: '{}', Error: '{}'",
                aggr_file_path, e
            )),
        };

        for line in rdr.lines() {
            let mut line_components: Vec<String> = Vec::new();

            for component in line.unwrap().split('|') {
                line_components.push(component.to_string());
            }

            let st_dt = if let Ok(dt) = NaiveDate::parse_from_str(&line_components[1], "%d-%m-%Y") {
                timestamp(dt)
            } else {
                DEFAULT_INT
            };

            let mat_dt = if let Ok(dt) = NaiveDate::parse_from_str(&line_components[2], "%d-%m-%Y")
            {
                timestamp(dt)
            } else {
                DEFAULT_INT
            };

            let cust_key = Customer::new(
                line_components[0].parse::<i64>().unwrap(),
                st_dt,
                mat_dt,
                line_components[3].to_string(),
            );

            cust_aggr_bal.insert(cust_key, line_components[4].parse::<f64>().unwrap());
        }
    }

    return cust_aggr_bal;
}
