use sdb_io;
use std::collections::HashMap;
use std::io::BufRead;
use std::path::Path;

pub fn read_ftp_rates(ftp_rates_file_path: &str) -> HashMap<String, Vec<f64>> {
    let mut ftp_rates: HashMap<String, Vec<f64>> = HashMap::new();

    if Path::new(&ftp_rates_file_path).exists() {
        let rdr = match sdb_io::new_buf_rdr(ftp_rates_file_path) {
            Ok(r) => r,
            Err(e) => panic!(
                "Cannot read file at path: '{}', Error: '{}'",
                ftp_rates_file_path, e
            ),
        };

        for line in rdr.lines() {
            let mut line_components: Vec<String> = Vec::new();

            for component in line.unwrap().split('|') {
                line_components.push(component.to_string());
            }

            let mut _lst_rates: Vec<f64> = Vec::new();

            for i in 1..10 {
                _lst_rates.push(line_components[i].parse::<f64>().unwrap());
            }

            ftp_rates.insert(line_components[0].to_string(), _lst_rates);
        }
    }

    return ftp_rates;
}
