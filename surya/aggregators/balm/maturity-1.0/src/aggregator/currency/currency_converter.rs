use super::CurrencyExchange;
use aggregator::structs::CashflowAggregatedOnDateBuilder;
use configuration_parameters::ConfigurationParameters;
use macros;
use slog::Logger;
use std::collections::HashMap;

// TODO: This has no need to be `Clone`. Can we get around this for perf wins?
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
        builder: &CashflowAggregatedOnDateBuilder,
        config_params: &ConfigurationParameters,
        ex_rt: f64,
        local_ccy_equivalent: bool,
        logger: &Logger,
    ) -> (CashflowAggregatedOnDateBuilder, f64) {
        let conversion_rate: f64;
        let mut exrt_flag = local_ccy_equivalent;
        (conversion_rate, exrt_flag) = if config_params.is_account_level_exchange_rate() {
            if config_params.is_consolidated() {
                (1.0 / ex_rt, false)
            } else {
                (ex_rt, exrt_flag)
            }
        } else {
            (*self.exchange_rate(target_currency, logger), exrt_flag)
        };
        let mut new_builder = *builder;
        if exrt_flag {
            new_builder.values_multiplied_by(1.0);
        } else {
            new_builder.values_multiplied_by(conversion_rate);
        }

        (new_builder, conversion_rate)
    }

    fn exchange_rate(&self, target_currency: &CurrencyExchange, log: &Logger) -> &f64 {
        match self.exchange_rates.get(target_currency) {
            Some(val) => val,
            None => {
                if target_currency.from_ccy != target_currency.to_ccy {
                    log_error!(log, "Exchange Not available for: `{:?}`.", target_currency);
                }
                &1.0
            }
        }
    }
}
