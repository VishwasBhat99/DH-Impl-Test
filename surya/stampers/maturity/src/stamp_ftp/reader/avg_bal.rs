use super::*;

#[derive(Debug, PartialEq)]
pub struct Balances {
    pub bal: f64,
    pub rate: f64,
    pub int_income_expense: f64,
}

#[derive(Debug)]
pub struct AverageBalance {
    pub avg_bal: AvgBalMap,
}

pub type AvgBalMap = HashMap<String, Balances>;
impl AverageBalance {
    pub fn new(file_path: &str, skip_amb_header: bool) -> Self {
        let mut avg_bal: AvgBalMap = HashMap::new();
        let reader = read_file(file_path);
        let mut skip_line = 0;
        if skip_amb_header {
            skip_line = 1;
        }
        for (line_num, lines) in reader.lines().enumerate().skip(skip_line) {
            let line = extract_lines(line_num, lines, file_path);
            let fields: Vec<&str> = line.split('|').collect();
            if fields.len() > 3 {
                avg_bal.insert(
                    fields[0].to_string(),
                    Balances {
                        bal: parse_f64(fields[1].trim()),
                        rate: parse_f64(fields[2].trim()),
                        int_income_expense: parse_f64(fields[3].trim()),
                    },
                );
            } else if fields.len() == 3 {
                avg_bal.insert(
                    fields[0].to_string(),
                    Balances {
                        bal: parse_f64(fields[1].trim()),
                        rate: parse_f64(fields[2].trim()),
                        int_income_expense: DEFAULT_FLOAT,
                    },
                );
            }
        }
        Self { avg_bal }
    }
}
