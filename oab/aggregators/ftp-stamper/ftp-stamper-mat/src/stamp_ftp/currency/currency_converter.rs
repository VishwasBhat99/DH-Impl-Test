use super::CurrencyExchange;
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

    pub fn convert(&self, ccy: &str, data: f64, logger: &Logger) -> f64 {
        let target_currency = CurrencyExchange {
            from_ccy: ccy.to_string(),
            to_ccy: self.base_currency.to_string(),
        };
        let conversion_rate = self.exchange_rate(&target_currency, logger);
        let conv_data = data * conversion_rate;

        conv_data
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
