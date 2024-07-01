use sdb_io;
use slog::Logger;
use std::collections::HashMap;
use std::io::BufRead;
use std::path::Path;

pub fn read_avg_bal(amb_file_path: &str, log: &Logger) -> (HashMap<String, f64>) {
    let mut avg_bal: HashMap<String, f64> = HashMap::new();

    if Path::new(&amb_file_path).exists() {
        let rdr = match sdb_io::new_buf_rdr(amb_file_path) {
            Ok(r) => r,
            Err(e) => panic!(format!(
                "Cannot read file at path: '{}', Error: '{}'",
                amb_file_path, e
            )),
        };

        for (line_num, line) in rdr.lines().enumerate() {
            let mut line_components: Vec<String> = Vec::new();

            for component in line.unwrap().split(',') {
                line_components.push(component.to_string());
            }

            if line_components.len() >= 2 {
                avg_bal.insert(
                    line_components[0].to_string(),
                    line_components[1].parse::<f64>().expect(&format!(
                        "couldn't parse Average balance for account : {} at line number :{} 
                            while reading Average balance. ",
                        line_components[0].to_string(),
                        line_num + 1
                    )),
                );
            }
        }
    }
    return avg_bal;
}
