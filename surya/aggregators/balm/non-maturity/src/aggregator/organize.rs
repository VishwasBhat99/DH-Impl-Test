use super::structs::AggregateData;
use aggregator::currency::currency_converter::CurrencyConverter;
use aggregator::currency::CurrencyExchange;
use aggregator::llg_key::LLGKey;
use configuration_parameters::ConfigurationParameters;
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
            currency_converter: currency_converter,
        }
    }

    pub fn drain(&mut self) -> Drain<LLGKey, AggregateData> {
        self.store.drain()
    }

    pub fn build_width(
        &mut self,
        llg: &LLGKey,
        mut data: AggregateData,
        config_params: &ConfigurationParameters,
        logger: &Logger,
    ) {
        let input_ccy = llg.currency.to_string();
        if input_ccy == config_params.src_local_ccy() {
            let base_llg = LLGKey::new(
                config_params.consol_ccy().to_string(),
                llg.category,
                llg.cf_type.clone(),
            );
            let converted_llg = LLGKey::new(
                config_params.display_local_ccy().to_string(),
                llg.category,
                llg.cf_type.clone(),
            );
            self.insert_into_store(&converted_llg, data);
            self.insert_into_store_converting_currency(
                &base_llg,
                &mut data,
                input_ccy.to_string(),
                config_params.src_local_ccy().to_string(),
                config_params,
                true,
                logger,
            );
        } else {
            let converted_llg = LLGKey::new(
                config_params.consol_ccy().to_string(),
                llg.category,
                llg.cf_type.clone(),
            );
            self.insert_into_store_converting_currency(
                &converted_llg,
                &mut data,
                input_ccy.to_string(),
                config_params.src_local_ccy().to_string(),
                config_params,
                false,
                logger,
            );
            let base_llg = LLGKey::new(llg.currency.to_string(), llg.category, llg.cf_type.clone());
            self.insert_into_store_converting_currency(
                &base_llg,
                &mut data,
                input_ccy.to_string(),
                llg.currency.to_string(),
                config_params,
                false,
                logger,
            );
        }
    }

    fn insert_into_store_converting_currency(
        &mut self,
        llg: &LLGKey,
        data: &mut AggregateData,
        input_ccy: String,
        target_currency: String,
        config_params: &ConfigurationParameters,
        local_ccy_equivalent: bool,
        logger: &Logger,
    ) {
        let mut conv_data = data.clone();
        let currency_key = if config_params.is_consolidated() {
            CurrencyExchange {
                from_ccy: config_params.src_local_ccy().to_string(),
                to_ccy: target_currency,
            }
        } else {
            CurrencyExchange {
                from_ccy: input_ccy,
                to_ccy: target_currency,
            }
        };
        let currency_converted_builder =
            self.currency_converter
                .convert(&currency_key, data, local_ccy_equivalent, logger);
        let convertion_rate = currency_converted_builder.1;
        conv_data.values_multiplied_by(convertion_rate);
        self.insert_into_store(llg, conv_data);
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
