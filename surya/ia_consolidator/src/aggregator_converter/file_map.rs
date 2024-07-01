use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
#[derive(Debug, Clone, PartialEq)]
pub struct BucketValue {
    pub principal_vec: Vec<String>,
    pub rate_vec: Vec<String>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct SummaryValue {
    pub principal_smry: f64,
    pub rate_smry: f64,
}

pub fn get_map(map: &mut HashMap<Vec<String>, BucketValue>, principal: File, rate: File) {
    let mut amt_map: HashMap<Vec<String>, Vec<String>> = HashMap::new();
    let header_size = 9;
    for lines in BufReader::new(principal).lines() {
        let line = lines.expect("Cannot get principal file fields");
        if line.is_empty() {
            continue;
        }
        let mut key: Vec<String> = line.split('|').map(String::from).collect();
        let val = key.split_off(9);
        amt_map.insert(key, val);
    }
    for lines in BufReader::new(rate).lines() {
        let line = lines.expect("Cannot get rate file fields");
        if line.is_empty() {
            continue;
        }
        let mut key: Vec<String> = line.split('|').map(String::from).collect();
        let rate_val = key.split_off(header_size);
        let amt_val = amt_map.get(&key).expect("Cannot get amount value");
        let val: BucketValue = BucketValue {
            principal_vec: amt_val.clone(),
            rate_vec: rate_val.clone(),
        };
        map.insert(key.clone(), val.clone());
    }
}

pub fn get_smry_map(map: &mut HashMap<Vec<String>, SummaryValue>, smry: File) {
    let header_size = 9;
    for lines in BufReader::new(smry).lines() {
        let line = lines.expect("Cannot get smry file fields");
        if line.is_empty() {
            continue;
        }
        let mut key: Vec<String> = line.split('|').map(String::from).collect();
        let smry_val = key.split_off(header_size);
        let val: SummaryValue = SummaryValue {
            principal_smry: smry_val[0].parse::<f64>().unwrap_or(0.0),
            rate_smry: smry_val[1].parse::<f64>().unwrap_or(0.0),
        };
        map.insert(key.clone(), val.clone());
    }
}
