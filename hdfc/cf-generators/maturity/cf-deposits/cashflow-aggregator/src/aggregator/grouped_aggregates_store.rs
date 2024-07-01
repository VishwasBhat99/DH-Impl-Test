// TODO: Lib; Bills is different
use std::collections::HashMap;
use chrono::NaiveDate;
use aggregator::structs::CashflowAggregatedOnDateBuilder;
use std::collections::hash_map::Drain;
use aggregator::llg_key::LLGKey;
use aggregator::currency::currency_converter::CurrencyConverter;


pub struct CashflowOrganizer {
    store: HashMap<
        LLGKey,
        HashMap<NaiveDate, CashflowAggregatedOnDateBuilder>
    >,
    currency_converter: CurrencyConverter,
}

impl CashflowOrganizer {

    pub fn new(
        currency_converter: CurrencyConverter
    ) -> CashflowOrganizer {
        CashflowOrganizer {
            store: HashMap::new(),
            currency_converter: currency_converter
        }
    }

    // Needs to be thread-safe if concurrency is implemented.
    pub fn build_with(
        &mut self,
        llg: LLGKey,
        grouped_cashflows: HashMap<NaiveDate, CashflowAggregatedOnDateBuilder>
    ) {

        // If this account is in a currency different from the the base currency
        // convert its values to the base currency and insert that as well.
        if llg.currency != self.currency_converter.base_currency {
            self.insert_in_store_converting_currency(&llg, &grouped_cashflows)
        }

        self.insert_into_store(
            &llg,
            grouped_cashflows
        );

    }

    // REVIEW: This is different across programs
    fn insert_in_store_converting_currency(
        &mut self,
        llg: &LLGKey,
        grouped_cashflows: &HashMap<NaiveDate, CashflowAggregatedOnDateBuilder>
    ) {
        let mut currency_converted_grouped_cashflows = HashMap::new();

        for (date, builder) in grouped_cashflows {
            let currency_converted_builder = self.currency_converter.convert(
                &llg.currency[..],
                builder
            );
            currency_converted_grouped_cashflows.insert(
                *date,
                currency_converted_builder
            );
        }

        self.insert_into_store(
            llg,
            currency_converted_grouped_cashflows
        )
    }

    fn insert_into_store(
        &mut self,
        llg: &LLGKey,
        grouped_cashflows: HashMap<NaiveDate, CashflowAggregatedOnDateBuilder>
    ) {

        if self.store.get(&llg).is_some() {
            let cashflows_for_llg = self.store.get_mut(&llg).unwrap();
            add(cashflows_for_llg, grouped_cashflows);
        } else {
            self.store.insert(
                llg.clone()
                , grouped_cashflows
            );
        }

    }
}

impl  CashflowOrganizer {
    pub fn drain(&mut self) -> Drain<
        LLGKey,
        HashMap<NaiveDate, CashflowAggregatedOnDateBuilder>
    > {
        self.store.drain()
    }
}

fn add(
    existing: &mut HashMap<NaiveDate, CashflowAggregatedOnDateBuilder>,
    mut new_grouped_cashflows: HashMap<NaiveDate, CashflowAggregatedOnDateBuilder>
) {

    for (date, builder) in new_grouped_cashflows.drain() {

        // Thank you krdln, leonardo:
        // https://users.rust-lang.org/t/why-cant-i-have-mutable-references-in-if-else-branches-of-a-hashmap/19203/3?u=mayurdzk
        let existing_builder = existing
            .entry(date)
            .or_insert(
                CashflowAggregatedOnDateBuilder::new()
            );
        existing_builder.add_from_builder(builder);

    }

}