use super::CurrencyExchange;
use aggregator::structs::AggregateData;
use macros;
use slog::Logger;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CurrencyConverter {
    pub base_currency: String,
    exchange_rates: HashMap<CurrencyExchange, f64>,
}
impl CurrencyConverter {
    pub fn new(
        base_currency: String,
        exchange_rates: HashMap<CurrencyExchange, f64>,
    ) -> CurrencyConverter {
        CurrencyConverter {
            base_currency,
            exchange_rates,
        }
    }

    pub fn convert(
        &self,
        target_currency: &CurrencyExchange,
        builder: &AggregateData,
        logger: &Logger,
    ) -> AggregateData {
        let conversion_rate: f64 = self.exchange_rate(target_currency, logger);
        let mut new_builder = *builder;
        new_builder.values_multiplied_by(conversion_rate);

        new_builder
    }

    fn exchange_rate(&self, target_currency: &CurrencyExchange, log: &Logger) -> f64 {
        match self.exchange_rates.get(target_currency) {
            Some(val) => *val,
            None => {
                if target_currency.from_ccy != target_currency.to_ccy {
                    log_error!(log, "Exchange Not available for: `{:?}`.", target_currency);
                }
                1.0
            }
        }
    }
}
