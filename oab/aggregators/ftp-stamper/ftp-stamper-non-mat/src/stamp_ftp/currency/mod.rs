use sdb_io;
use stamp_ftp::currency::currency_converter::CurrencyConverter;
use std::collections::HashMap;
use std::io::BufRead;

pub mod currency_converter;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct CurrencyExchange {
    pub from_ccy: String,
    pub to_ccy: String,
}

pub fn create_currency_converter(
    base_currency: &str,
    currency_conversion_file_path: &str,
) -> CurrencyConverter {
    let mut exchange_rate: HashMap<CurrencyExchange, f64> = HashMap::new();

    let reader = match sdb_io::new_buf_rdr(currency_conversion_file_path) {
        Ok(r) => r,
        Err(e) => panic!(format!(
            "Cannot read file at path: '{}', Error: '{}'",
            currency_conversion_file_path, e
        )),
    };

    for (line_num, lines) in reader.lines().enumerate() {
        // Lines are of the format USD|INR|67.4100000
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read rules file at line number: `{}` : {}",
                line_num + 1,
                error
            ),
        };
        let mut line_components: Vec<String> = Vec::new();
        for component in line.split('|') {
            line_components.push(component.to_string());
        }
        let exchange_key = CurrencyExchange {
            from_ccy: line_components[0].to_string(),
            to_ccy: line_components[1].to_string(),
        };
        exchange_rate.insert(
            exchange_key,
            line_components[2]
                .parse::<f64>()
                .expect("Error while getting coversion rate."),
        );
    }

    CurrencyConverter::new(base_currency.to_string(), exchange_rate)
}
