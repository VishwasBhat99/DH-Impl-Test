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

    pub fn convert(&self, ccy: &str, data: f64, is_consolidated: bool, logger: &Logger) -> f64 {
        let target_currency;
        if is_consolidated {
            target_currency = CurrencyExchange {
                from_ccy: self.base_currency.to_string(),
                to_ccy: ccy.to_string(),
            };
        } else {
            target_currency = CurrencyExchange {
                from_ccy: ccy.to_string(),
                to_ccy: self.base_currency.to_string(),
            };
        }

        if target_currency.from_ccy == target_currency.to_ccy {
            data
        } else {
            data * self.exchange_rate(&target_currency, logger)
        }
    }

    fn exchange_rate(&self, target_currency: &CurrencyExchange, log: &Logger) -> f64 {
        match self.exchange_rates.get(target_currency) {
            Some(val) => *val,
            None => {
                if target_currency.from_ccy != target_currency.to_ccy {
                    let recipro_currency = CurrencyExchange {
                        from_ccy: target_currency.to_ccy.to_string(),
                        to_ccy: target_currency.from_ccy.to_string(),
                    };
                    match self.exchange_rates.get(&recipro_currency) {
                        Some(val) => {
                            if *val == 0.0 {
                                return 0.0;
                            } else {
                                let ex_rt = 1.0 / *val;
                                return ex_rt;
                            }
                        }
                        None => {
                            log_error!(log, "Exchange Not available for: `{:?}`.", target_currency);
                        }
                    }
                }
                1.0
            }
        }
    }
}
