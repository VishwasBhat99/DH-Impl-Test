use super::structs::AggregateData;
use aggregator::currency::currency_converter::CurrencyConverter;
use aggregator::llg_key::LLGKey;
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
        data: &mut AggregateData,
        foreign_consolidation_currency: &str,
        local_consolidation_currency: &str,
    ) {
        self.insert_into_store(&llg, data);

        if llg.currency != self.currency_converter.consolidated_currency {
            let converted_llg = LLGKey::new(
                self.currency_converter.consolidated_currency.clone(),
                llg.category,
                llg.cf_type.to_string(),
            );
            let mut consolidated_data = data.clone();
            self.insert_into_store_converting_currency(
                &converted_llg,
                &mut consolidated_data,
                llg.currency.to_string(),
            );
            if llg.currency != local_consolidation_currency {
                let fcy_llg = LLGKey::new(
                    foreign_consolidation_currency.to_string(),
                    llg.category,
                    llg.cf_type.to_string(),
                );
                self.insert_into_store_converting_currency(
                    &fcy_llg,
                    data,
                    llg.currency.to_string(),
                );
            }
        }
    }

    fn insert_into_store_converting_currency(
        &mut self,
        llg: &LLGKey,
        data: &mut AggregateData,
        target_currency: String,
    ) {
        let currency_converted_builder = self.currency_converter.convert(&target_currency, data);
        let convertion_rate = currency_converted_builder.1;
        data.values_multiplied_by(convertion_rate);
        self.insert_into_store(llg, data);
    }

    fn insert_into_store(&mut self, llg: &LLGKey, data: &AggregateData) {
        if self.store.get_mut(&llg).is_some() {
            self.store
                .get_mut(&llg)
                .expect("Error while storing data to the map.")
                .add_from_builder(*data);
        } else {
            self.store.insert(llg.clone(), *data);
        }
    }
}
