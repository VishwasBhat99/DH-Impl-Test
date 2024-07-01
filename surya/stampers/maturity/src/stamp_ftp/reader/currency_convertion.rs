use super::*;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Currency {
    pub source: String,
    pub target: String,
}

#[derive(Debug)]
pub struct ExchangeRates {
    pub exrt_map: CurrencyMap,
}

pub type CurrencyMap = HashMap<Currency, f64>;
impl ExchangeRates {
    pub fn new(file_path: &str) -> Self {
        let mut exrt_map: CurrencyMap = HashMap::new();
        let reader = read_file(file_path);
        for (line_num, lines) in reader.lines().enumerate() {
            let line = extract_lines(line_num, lines, file_path);
            let fields: Vec<&str> = line.split('|').collect();
            exrt_map.insert(
                Currency {
                    source: fields[0].to_string(),
                    target: fields[1].to_string(),
                },
                parse_f64(fields[2].trim()),
            );
        }
        Self { exrt_map }
    }
}
