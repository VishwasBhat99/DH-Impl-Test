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

    pub fn convert(
        &self,
        ccy: &str,
        data: f64,
        is_consolidated: bool,
        is_account_level_exchange_rate: bool,
        ex_rt: f64,
        logger: &Logger,
    ) -> f64 {
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
            if is_account_level_exchange_rate {
                if is_consolidated {
                    if ex_rt == 0.0 {
                        data
                    } else {
                        data * (1.0 / ex_rt)
                    }
                } else {
                    data * ex_rt
                }
            } else {
                data * self.exchange_rate(&target_currency, logger)
            }
        }
    }

    pub fn convert_numslab(
        &self,
        ccy: &str,
        data: f64,
        is_consolidated: bool,
        is_account_level_exchange_rate: bool,
        ex_rt: f64,
        logger: &Logger,
    ) -> f64 {
        let target_currency;
        if is_consolidated {
            data
        } else {
            target_currency = CurrencyExchange {
                from_ccy: ccy.to_string(),
                to_ccy: self.base_currency.to_string(),
            };
            if is_account_level_exchange_rate {
                    data * ex_rt
            } else {
                let ex_rt = *self.exchange_rates.get(&target_currency).unwrap_or(&1.0);
                data * ex_rt
            }
        }
    }

    fn exchange_rate(&self, target_currency: &CurrencyExchange, log: &Logger) -> f64 {
        match self.exchange_rates.get(target_currency) {
            Some(val) => *val,
            None => {
                if target_currency.from_ccy != target_currency.to_ccy {
                    let recipro_currency = CurrencyExchange {
                        from_ccy: target_currency.to_ccy.to_string(),
                        to_ccy: target_currency.to_ccy.to_string(),
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
