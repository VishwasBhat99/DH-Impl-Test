use macros;
use sdb_io::new_buf_rdr;
use sdb_util::expand;
use slog::Logger;
use statics::DEFAULT_FLOAT;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::BufRead;
use std::io::BufReader;

pub fn read_avg_bal(avg_bal_path: &str, log: &Logger) -> HashMap<String, f64> {
    let avg_bal_file = match new_buf_rdr(avg_bal_path) {
        Ok(file) => file,
        Err(error) => {
            log_error!(
                log,
                "Could not find Average Balance file: `{}`",
                avg_bal_path
            );
            panic!(
                "Could not find Average Balance file: `{}` on location `{}` : {}.",
                avg_bal_path,
                current_dir()
                    .expect("Error while getting current directory path.")
                    .display(),
                error
            )
        }
    };
    let reader = BufReader::new(avg_bal_file);

    let mut avg_bal: HashMap<String, f64> = HashMap::new();
    for (line_no, line) in reader.lines().enumerate() {
        let acc_line: String = match line {
            Ok(value) => value,
            Err(error) => {
                log_error!(
                    log,
                    "Cannot read line from input file: {}\n Error: {}",
                    line_no,
                    error
                );
                panic!("Cannot read line from input file: {}", line_no)
            }
        };
        let fields: Vec<String> = expand(acc_line, '|');
        let amount = fields[1].parse().unwrap_or(DEFAULT_FLOAT);
        avg_bal.insert(fields[0].to_string(), amount);
    }

    avg_bal
}
