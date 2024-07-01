use chrono::NaiveDate;
use rbdate::DateParser;
use rbdate::{date_from_timestamp, timestamp};
use sdb_io;
use statics::DEFAULT_INT;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io::BufRead;
use std::path::Path;

pub fn read_adj_rates(file_path: &str) -> HashMap<Adj_key, f64> {
    let mut adj_rates: HashMap<Adj_key, f64> = HashMap::new();

    if Path::new(&file_path).exists() {
        let rdr = match sdb_io::new_buf_rdr(file_path) {
            Ok(r) => r,
            Err(e) => panic!(format!(
                "Cannot read file at path: '{}', Error: '{}'",
                file_path, e
            )),
        };

        for line in rdr.lines() {
            let mut line_components: Vec<String> = Vec::new();

            for component in line.unwrap().split('|') {
                line_components.push(component.to_string());
            }

            let st_dt = if let Ok(dt) = NaiveDate::parse_from_str(&line_components[0], "%d-%m-%Y") {
                timestamp(dt)
            } else {
                DEFAULT_INT
            };
            let key = Adj_key::new(st_dt, line_components[1].parse::<i32>().unwrap());
            adj_rates.insert(key, line_components[2].parse::<f64>().unwrap());
        }
    }

    return adj_rates;
}

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct Adj_key {
    pub start_date: i64,
    pub adj_id: i32,
}

impl Adj_key {
    pub fn new(start_date: i64, adj_id: i32) -> Adj_key {
        Adj_key { start_date, adj_id }
    }
}

impl Display for Adj_key {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}|{}", self.start_date, self.adj_id)
    }
}
