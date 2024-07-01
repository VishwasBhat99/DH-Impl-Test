use sdb_io;
use slog::Logger;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io::BufRead;
use std::path::Path;

pub fn read_avg_bal(amb_file_path: &str) -> (HashMap<String, AmbVal>) {
    let mut avg_bal_list: HashMap<String, AmbVal> = HashMap::new();

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

            for component in line.unwrap().split('|') {
                line_components.push(component.to_string());
            }
            if line_components.len() >= 3 {
                let avg_bal = line_components[1].parse::<f64>().expect(&format!(
                    "couldn't parse Average balance for account : {} at line number :{} 
                    while reading Average balance. ",
                    line_components[0].to_string(),
                    line_num + 1
                ));
                let int_amt = line_components[2].parse::<f64>().expect(&format!(
                    "couldn't parse Average balance for account : {} at line number :{} 
                    while reading Average balance. ",
                    line_components[0].to_string(),
                    line_num + 1
                ));

                let value = AmbVal::new(avg_bal, int_amt);

                avg_bal_list.insert(line_components[0].to_string(), value);
            }
        }
    }
    return avg_bal_list;
}

#[derive(Debug, Clone, PartialEq)]
pub struct AmbVal {
    pub avg_bal: f64,
    pub int_amt: f64,
}

impl AmbVal {
    pub fn new(avg_bal: f64, int_amt: f64) -> AmbVal {
        AmbVal { avg_bal, int_amt }
    }
}

impl Display for AmbVal {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}|{}", self.avg_bal, self.int_amt)
    }
}
