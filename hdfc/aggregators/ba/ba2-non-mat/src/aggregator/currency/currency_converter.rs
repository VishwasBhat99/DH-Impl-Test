use aggregator::structs::AggregateData;
use std::collections::HashMap;

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

    pub fn convert(&self, target_currency: &str, builder: &AggregateData) -> (AggregateData, f64) {
        let conversion_rate = *self.exchange_rate(target_currency);

        let mut new_builder = *builder;
        new_builder.values_multiplied_by(conversion_rate);

        (new_builder, conversion_rate)
    }

    pub fn exchange_rate(&self, target_currency: &str) -> &f64 {
        self.exchange_rates.get(target_currency).expect(&format!(
            "The target exchange rate requested '{}' was not found in the conversion rates file.",
            target_currency
        ))
    }
}
