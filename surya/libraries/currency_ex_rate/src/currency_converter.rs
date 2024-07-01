use super::CurrencyExchange;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CurrencyConverter {
    from_ccy: String,
    to_ccy: String,
    exchange_rates: HashMap<CurrencyExchange, f64>,
}
impl CurrencyConverter {
    pub fn new(
        from_ccy: String,
        to_ccy: String,
        exchange_rates: HashMap<CurrencyExchange, f64>,
    ) -> CurrencyConverter {
        CurrencyConverter {
            from_ccy,
            to_ccy,
            exchange_rates,
        }
    }

    pub fn convert(&self) -> f64 {
        let target_ccy = CurrencyExchange {
            from_ccy: self.from_ccy.to_string(),
            to_ccy: self.to_ccy.to_string(),
        };
        let ex_rt = self.get_exchange_rate(target_ccy);

        ex_rt
    }

    fn get_exchange_rate(&self, target_ccy: CurrencyExchange) -> f64 {
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
                            panic!("Exchange Not available for: `{:?}`.", target_ccy);

                        }
                    }
                }
                1.0
            }
        }
    }
}

