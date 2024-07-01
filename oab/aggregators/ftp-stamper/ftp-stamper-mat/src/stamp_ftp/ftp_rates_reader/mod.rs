use sdb_io;
use std::collections::HashMap;
use std::io::BufRead;
use std::path::Path;

#[derive(Default)]
pub struct FtpRateLock {
    pub ftp_rates: HashMap<String, Vec<f64>>,
    pub lock_adjs: HashMap<i32, String>,
}

pub fn read_ftp_rates(ftp_rates_file_path: &str) -> FtpRateLock {
    let mut ftp_rate_lock: FtpRateLock = Default::default();

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

            for component in line.expect("Error in reading line").split('|') {
                line_components.push(component.to_string());
            }

            let mut lst_rates: Vec<f64> = Vec::new();

            for index in 2..11 {
                lst_rates.push(
                    line_components[index]
                        .parse::<f64>()
                        .expect("couldn't parse number"),
                );
            }

            ftp_rate_lock
                .ftp_rates
                .insert(line_components[0].to_string(), lst_rates);
            let mut position = 1;
            for index in 11..=17 {
                ftp_rate_lock
                    .lock_adjs
                    .insert(position, line_components[index].to_string());
                position += 1;
            }
        }
    }

    ftp_rate_lock
}
