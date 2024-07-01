use aggregator::structs::CashflowAggregatedOnDateBuilder;
use configuration_parameters::ConfigurationParameters;
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

    pub fn convert(
        &self,
        target_currency: &str,
        builder: &CashflowAggregatedOnDateBuilder,
        config_params: &ConfigurationParameters,
        ex_rt: f64,
        local_ccy_equivalent: bool,
    ) -> (CashflowAggregatedOnDateBuilder, f64) {
        let conversion_rate: f64;
        if config_params.is_account_level_exchange_rate() {
            conversion_rate = ex_rt;
        } else {
            conversion_rate = *self.exchange_rate(target_currency);
        }

        let mut new_builder = *builder;
        if config_params.is_consolidated() && local_ccy_equivalent {
            new_builder.values_multiplied_by(1.0);
        } else if config_params.is_consolidated() && !local_ccy_equivalent {
            new_builder.values_multiplied_by(1.0 / conversion_rate);
        } else {
            new_builder.values_multiplied_by(conversion_rate);
        }

        (new_builder, conversion_rate)
    }

    fn exchange_rate(&self, target_currency: &str) -> &f64 {
        self.exchange_rates.get(target_currency).expect(&format!(
            "The target exchange rate requested '{}' was not found in the conversion rates file.",
            target_currency
        ))
    }
}
