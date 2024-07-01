use super::CurrencyExchange;
use macros;
use slog::Logger;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CurrencyConverter {
    base_currency: String,
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

    pub fn convert_to_base(&self, amt: f64, ccy: &str, log: &Logger) -> f64 {
        let target_ccy = CurrencyExchange {
            from_ccy: ccy.to_string(),
            to_ccy: self.base_currency.to_string(),
        };
        let ex_rt = self.exchange_rate(target_ccy, log);

        amt * ex_rt
    }

    pub fn convert_from_base(&self, amt: f64, ccy: &str, log: &Logger) -> f64 {
        let target_ccy = CurrencyExchange {
            from_ccy: self.base_currency.to_string(),
            to_ccy: ccy.to_string(),
        };
        let ex_rt = self.exchange_rate(target_ccy, log);

        amt * ex_rt
    }

    fn exchange_rate(&self, target_ccy: CurrencyExchange, log: &Logger) -> f64 {
        match self.exchange_rates.get(&target_ccy) {
            Some(val) => *val,
            None => {
                if target_ccy.from_ccy != target_ccy.to_ccy {
                    let recipro_currency = CurrencyExchange {
                        from_ccy: target_ccy.to_ccy.to_string(),
                        to_ccy: target_ccy.from_ccy.to_string(),
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
                            log_error!(log, "Exchange Not available for: `{:?}`.", target_ccy);
                        }
                    }
                }
                1.0
            }
        }
    }
}
