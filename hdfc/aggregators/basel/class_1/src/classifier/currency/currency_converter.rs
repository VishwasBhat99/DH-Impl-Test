use std::collections::HashMap;

use slog::Logger;

use macros;
// TODO: This has no need to be `Clone`. Can we get around this for perf wins?
#[derive(Debug, Clone)]
pub struct CurrencyConverter {
    pub consolidated_currency: String,
    exchange_rates: HashMap<String, f64>,
}

impl CurrencyConverter {
    pub fn new(
        consolidated_currency: String,
        exchange_rates: HashMap<String, f64>,
    ) -> CurrencyConverter {
        CurrencyConverter {
            consolidated_currency,
            exchange_rates,
        }
    }

    pub fn convert_to_lcy(&self, target_currency: &str, value: &f64, logger: &Logger) -> f64 {
        let conversion_rate = self.exchange_rate(target_currency, logger);
        value * conversion_rate
    }

    pub fn convert_from_lcy(&self, target_currency: &str, value: &f64, logger: &Logger) -> f64 {
        let conversion_rate = self.exchange_rate(target_currency, logger);
        if conversion_rate == 0.0 {
            0.0
        } else {
            value / conversion_rate
        }
    }

    fn exchange_rate(&self, target_currency: &str, logger: &Logger) -> f64 {
        match self.exchange_rates.get(target_currency) {
            Some(val) => *val,
            None => {
                log_warn!(logger, "The target exchange rate requested '{}' was not found in the conversion rates file. Using 1.0 as default exchange rate to HCY.", target_currency);
                1.0
            }
        }
    }
}
