use std::collections::HashMap;
use std::fs;

pub fn get_exchange_rate(cons_curr: &str, ccy_path: &str) -> HashMap<String, f64> {
    let ccy_file_contents = fs::read_to_string(ccy_path).expect("cannot read currency file");
    let mut currency_map: HashMap<String, f64> = HashMap::new();
    for line in ccy_file_contents.lines() {
        let each_line: Vec<&str> = line.split("|").collect();
        if each_line[1] == cons_curr {
            currency_map.insert(
                each_line[0].to_string(),
                each_line[2]
                    .parse::<f64>()
                    .expect("each_line[2].parse::<f64>"),
            );
        }
    }
    return currency_map;
}
