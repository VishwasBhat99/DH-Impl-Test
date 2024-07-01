use std::collections::HashMap;

// TODO: This has no need to be `Clone`. Can we get around this for perf wins?
#[derive(Debug, Clone)]
pub struct CurrencyConverter {
    pub consolidated_currency: String,
    pub exchange_rates: HashMap<String, f64>,
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
}
