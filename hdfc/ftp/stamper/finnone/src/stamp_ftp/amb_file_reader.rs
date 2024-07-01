use sdb_io;
use statics::*;
use std::collections::HashMap;
use std::io::BufRead;
use std::path::Path;

#[derive(Default)]
pub struct AverageBalance {
    pub avg_bal: f64,
    pub accr_int: f64,
}

impl AverageBalance {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn missing_avg_bal(amt: f64, rate: f64, time: f64) -> Self {
        Self {
            avg_bal: amt,
            accr_int: amt * rate * time / 100.0,
        }
    }
}

pub fn read_avg_bal(amb_file_path: &str) -> HashMap<String, AverageBalance> {
    let mut avg_bal: HashMap<String, AverageBalance> = HashMap::new();

    if Path::new(&amb_file_path).exists() {
        let rdr = match sdb_io::new_buf_rdr(amb_file_path) {
            Ok(r) => r,
            Err(e) => panic!(
                "Cannot read file at path: '{}', Error: '{}'",
                amb_file_path, e
            ),
        };

        for (line_num, line) in rdr.lines().enumerate() {
            let mut line_components: Vec<String> = Vec::new();

            if line_num != 0 {
                for component in line.unwrap().split('|') {
                    line_components.push(component.to_string());
                }

                if line_components.len() >= 4 {
                    avg_bal.insert(
                        line_components[0].to_string(),
                        AverageBalance {
                            avg_bal: line_components[3].parse::<f64>().unwrap_or(DEFAULT_FLOAT),
                            accr_int: line_components[6].parse::<f64>().unwrap_or(DEFAULT_FLOAT),
                        },
                    );
                }
            }
        }
    }
    return avg_bal;
}
