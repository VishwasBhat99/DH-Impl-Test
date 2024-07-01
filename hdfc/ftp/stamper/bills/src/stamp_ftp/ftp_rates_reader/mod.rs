use sdb_io;
use std::collections::HashMap;
use std::io::BufRead;
use std::path::Path;

pub fn read_ftp_rates(
    ftp_rates_file_path: &str,
) -> (HashMap<String, Vec<f64>>, HashMap<i32, String>) {
    let mut ftp_rates: HashMap<String, Vec<f64>> = HashMap::new();
    let mut lock_adjs: HashMap<i32, String> = HashMap::new();

    if Path::new(&ftp_rates_file_path).exists() {
        let rdr = match sdb_io::new_buf_rdr(ftp_rates_file_path) {
            Ok(r) => r,
            Err(e) => panic!(format!(
                "Cannot read file at path: '{}', Error: '{}'",
                ftp_rates_file_path, e
            )),
        };

        for line in rdr.lines() {
            let mut line_components: Vec<String> = Vec::new();

            for component in line.unwrap().split('|') {
                line_components.push(component.to_string());
            }

            let mut lst_rates: Vec<f64> = Vec::new();

            for i in 2..11 {
                lst_rates.push(line_components[i].parse::<f64>().unwrap_or(0.0));
            }

            ftp_rates.insert(line_components[0].to_string(), lst_rates);
            let mut j = 1;
            for i in 11..=17 {
                lock_adjs.insert(j, line_components[i].to_string());
                j += 1;
            }
        }
    }

    (ftp_rates, lock_adjs)
}
