use aggregator::currency::currency_converter::CurrencyConverter;
use sdb_io;
use std::collections::HashMap;
use std::io::BufRead;

pub mod currency_converter;

pub fn create_currency_converter(
    consolidated_currency: &str,
    currency_conversion_file_path: &str,
) -> CurrencyConverter {
    let mut exchange_rate: HashMap<String, f64> = HashMap::new();

    let rdr = match sdb_io::new_buf_rdr(currency_conversion_file_path) {
        Ok(r) => r,
        Err(e) => panic!(
            "Cannot read file at path: '{}', Error: '{}'",
            currency_conversion_file_path, e
        ),
    };

    for line in rdr.lines() {
        // Lines are of the format USD|INR|67.4100000
        // If our base currency is INR, we want to store `USD` and `76.4100000` in the map
        let mut line_components: Vec<String> = Vec::new();
        for component in line
            .expect("Error while reading `exchange_rate_file`.")
            .split('|')
        {
            line_components.push(component.to_string());
        }
        if line_components[1] == consolidated_currency {
            if exchange_rate.contains_key(&line_components[0]) {
                panic!(
                    "Duplicate values for converting base currency. Duplicated target currency: {}",
                    line_components[0]
                );
            }
            exchange_rate.insert(
                line_components[0].to_string(),
                line_components[2]
                    .parse::<f64>()
                    .expect("Error while parsing `exchange rate` field."),
            );
        }
    }

    CurrencyConverter::new(consolidated_currency.to_string(), exchange_rate)
}
