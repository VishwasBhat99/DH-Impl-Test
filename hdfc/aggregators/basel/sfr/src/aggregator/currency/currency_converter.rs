use std::collections::HashMap;

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

    pub fn get_consol_data(&self, target_currency: &str, amount: &f64) -> f64 {
        let conversion_rate = *self.exchange_rate(target_currency);

        amount * conversion_rate
    }

    fn exchange_rate(&self, target_currency: &str) -> &f64 {
        match self.exchange_rates.get(target_currency) {
            Some(val) => val,
            None => {
                // Todo: Log
                &1.0
            }
        }
    }
}
