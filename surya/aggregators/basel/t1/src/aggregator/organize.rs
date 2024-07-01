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

    pub fn build_width(&mut self, llg: &LLGKey, data: &mut AggregateData) {
        self.insert_into_store(llg, data);
    }

    fn insert_into_store(&mut self, llg: &LLGKey, data: &AggregateData) {
        if self.store.get_mut(llg).is_some() {
            self.store
                .get_mut(llg)
                .expect("Error while storing data to the map.")
                .add_from_builder(*data);
        } else {
            self.store.insert(llg.clone(), *data);
        }
    }
}
