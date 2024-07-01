use std::io::BufRead;
use std::collections::HashMap;
use aggregator::currency::currency_converter::CurrencyConverter;
use sdb_io;

pub mod currency_converter;

pub fn create_currency_converter(
    base_currency: &str,
    currency_conversion_file_path: &str
) -> CurrencyConverter {

    let mut exchange_rate: HashMap<String, f64> = HashMap::new();

    let rdr = match sdb_io::new_buf_rdr(currency_conversion_file_path) {
        Ok(r) => {
            r
        },
        Err(e) => {
            panic!(
                format!(
                    "Cannot read file at path: '{}', Error: '{}'", currency_conversion_file_path, e
                )
            )
        }
    };

    for line in rdr.lines() {

        // Lines are of the format USD|INR|67.4100000
        // If our base currency is INR, we want to store `USD` and `76.4100000` in the map
        let mut line_components: Vec<String> = Vec::new();
        for component in line.unwrap().split('|') {
            line_components.push(component.to_string());
        }
        if line_components[1] == base_currency {
            if exchange_rate.contains_key(&line_components[0]) {
                panic!(
                    "Duplicate values for converting base currency. Duplicated target currency: {}",
                    line_components[0]
                );
            }
            exchange_rate.insert(
                line_components[0].to_string(),
                line_components[2].parse::<f64>().unwrap()
            );
        }
    }

    CurrencyConverter::new(
        base_currency.to_string(),
        exchange_rate
    )
}