use sdb_io::new_buf_rdr;
use std::collections::HashMap;
use std::io::BufRead;

pub fn read_exchange_rate(exchange_rate_file: &str) -> HashMap<String, f64> {
    //Get exchange rates
    let rdr = match new_buf_rdr(exchange_rate_file) {
        Ok(rdr) => rdr,
        Err(err) => panic!(
            "Cannot read file at path: '{}', Error: '{}'",
            exchange_rate_file, err
        ),
    };
    let mut exchange_rate_map: HashMap<String, f64> = HashMap::new();
    for line in rdr.lines() {
        let mut line_components: Vec<String> = Vec::new();
        for component in line.expect("Could Not Read Line").split('|') {
            line_components.push(component.to_string());
        }
        let rate: f64 = line_components[2].parse().unwrap_or(0.0);
        let mut key: String = String::new();
        key.push_str(&line_components[0]);
        key.push_str("|");
        key.push_str(&line_components[1]);
        exchange_rate_map.insert(key, rate);
    }
    exchange_rate_map
}

pub fn get_exch_rate(
    currency: String,
    base_currency: &str,
    exch_rt_map: &HashMap<String, f64>,
) -> f64 {
    let key = currency + "|" + base_currency;
    match exch_rt_map.get(&key) {
        Some(val) => *val,
        None => 1.0,
    }
}
