use crate::aggregator::currency::currency_converter::CurrencyConverter;
use crate::aggregator::currency::CurrencyExchange;
use crate::aggregator::llg_key::LLGKey;
use crate::aggregator::structs::AggregateData;
use crate::configuration_parameters::ConfigurationParameters;
use slog::Logger;
use std::collections::hash_map::Drain;
use std::collections::HashMap;

pub struct Organizer {
    store: HashMap<LLGKey, AggregateData>,
    currency_converter: CurrencyConverter,
}

impl Organizer {
    pub fn new(currency_converter: CurrencyConverter) -> Organizer {
        Organizer {
            store: HashMap::new(),
            currency_converter,
        }
    }
    pub fn organize(
        &mut self,
        llg: &LLGKey,
        data: AggregateData,
        config_params: &ConfigurationParameters,
        logger: &Logger,
    ) {
        self.insert_in_store_converting_currency(llg, data, config_params, logger);
    }

    fn insert_in_store_converting_currency(
        &mut self,
        llg: &LLGKey,
        data: AggregateData,
        config_params: &ConfigurationParameters,
        logger: &Logger,
    ) {
        let currency_key = if config_params.is_consolidated() {
            CurrencyExchange {
                from_ccy: config_params.base_currency().to_string(),
                to_ccy: llg.currency.to_owned(),
            }
        } else {
            CurrencyExchange {
                from_ccy: llg.currency.to_owned(),
                to_ccy: config_params.base_currency().to_string(),
            }
        };
        let _conv_data = self
            .currency_converter
            .convert(&currency_key, &data, logger);
        self.insert_into_store(llg, _conv_data);
    }

    fn insert_into_store(&mut self, llg: &LLGKey, data: AggregateData) {
        if self.store.get_mut(&llg).is_some() {
            self.store
                .get_mut(&llg)
                .expect("Error while storing data to the map.")
                .add_from_builder(data);
        } else {
            self.store.insert(llg.clone(), data);
        }
    }
}

impl Organizer {
    pub fn drain(&mut self) -> Drain<LLGKey, AggregateData> {
        self.store.drain()
    }
}
