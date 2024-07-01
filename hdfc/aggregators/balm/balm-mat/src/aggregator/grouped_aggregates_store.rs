use aggregator::currency::currency_converter::CurrencyConverter;
use aggregator::llg_key::LLGKey;
use aggregator::reports::input_report::InputReport;
use aggregator::reports::llgs_report::LLGsReport;
use aggregator::structs::CashflowAggregatedOnDateBuilder;
use configuration_parameters::ConfigurationParameters;
use rbdate::NaiveDate;
use std::collections::hash_map::Drain;
use std::collections::HashMap;

pub struct CashflowOrganizer {
    store: HashMap<LLGKey, HashMap<NaiveDate, CashflowAggregatedOnDateBuilder>>,
    currency_converter: CurrencyConverter,
}

impl CashflowOrganizer {
    pub fn new(currency_converter: CurrencyConverter) -> CashflowOrganizer {
        CashflowOrganizer {
            store: HashMap::new(),
            currency_converter: currency_converter,
        }
    }
    pub fn organize(
        &mut self,
        llg: &LLGKey,
        grouped_cashflows: HashMap<NaiveDate, CashflowAggregatedOnDateBuilder>,
        grouped_cf_amt: InputReport,
        overdue_cashflows: HashMap<NaiveDate, CashflowAggregatedOnDateBuilder>,
        account_overdue_amount_report: InputReport,
        config_params: &ConfigurationParameters,
        llgs_report: &mut LLGsReport,
        exchange_rt: f64,
    ) {
        self.build_with(
            &llg,
            grouped_cashflows,
            grouped_cf_amt,
            config_params,
            llgs_report,
            exchange_rt,
        );

        if !overdue_cashflows.is_empty() {
            let default_overdue_llg = LLGKey::new(
                llg.currency.clone(),
                config_params.default_overdue_llg_code(),
                llg.cf_type.clone(),
            );
            let overdue_report = InputReport::new();
            llgs_report.add_account_totals_for_llg(&default_overdue_llg, overdue_report);
            self.build_with(
                &default_overdue_llg,
                overdue_cashflows,
                account_overdue_amount_report,
                config_params,
                llgs_report,
                exchange_rt,
            );
        }
    }
    // Needs to be thread-safe if concurrency is implemented.
    pub fn build_with(
        &mut self,
        llg: &LLGKey,
        grouped_cashflows: HashMap<NaiveDate, CashflowAggregatedOnDateBuilder>,
        grouped_cf_amt: InputReport,
        config_params: &ConfigurationParameters,
        llgs_report: &mut LLGsReport,
        ex_rt: f64,
    ) {
        let mut new_llg: Option<LLGKey> = None;
        let mut new_fcy_llg: Option<LLGKey> = None;
        let mut converted_amount_report = grouped_cf_amt;
        // If this account is in a currency different from the the base currency
        // convert its values to the base currency and insert that as well.
        if llg.currency != self.currency_converter.consolidated_currency {
            let converted_llg = LLGKey::new(
                self.currency_converter.consolidated_currency.clone(),
                llg.category,
                llg.cf_type.clone(),
            );
            let conversion_rate = self.insert_in_store_converting_currency(
                &converted_llg,
                &grouped_cashflows,
                llg.currency.clone(),
                config_params,
                ex_rt,
                true,
            );

            new_llg = Some(converted_llg);
            converted_amount_report.total_interest_amount *= conversion_rate;
            converted_amount_report.total_principal_amount *= conversion_rate;
            if llg.currency != config_params.local_consolidation_currency()
                && !config_params.is_consolidated()
            {
                let fcy_llg = LLGKey::new(
                    config_params.foreign_consolidation_currency().to_string(),
                    llg.category,
                    llg.cf_type.clone(),
                );
                let _ = self.insert_in_store_converting_currency(
                    &fcy_llg,
                    &grouped_cashflows,
                    llg.currency.clone(),
                    config_params,
                    ex_rt,
                    false,
                );
                new_fcy_llg = Some(fcy_llg);
            }
        }
        if config_params.is_consolidated() {
            self.insert_in_store_converting_currency(
                &llg,
                &grouped_cashflows,
                llg.currency.clone(),
                config_params,
                ex_rt,
                false,
            );

            if llg.currency != config_params.local_consolidation_currency() {
                let fcy_llg = LLGKey::new(
                    config_params.foreign_consolidation_currency().to_string(),
                    llg.category,
                    llg.cf_type.clone(),
                );
                let _ = self.insert_in_store_converting_currency(
                    &fcy_llg,
                    &grouped_cashflows,
                    llg.currency.clone(),
                    config_params,
                    ex_rt,
                    true,
                );
                new_fcy_llg = Some(fcy_llg);
            }
        }

        if !config_params.is_consolidated() {
            self.insert_into_store(&llg, grouped_cashflows);
        }

        if let Some(llg) = new_llg {
            llgs_report.add_account_totals_for_llg(&llg, converted_amount_report);
        }
        if let Some(llg) = new_fcy_llg {
            llgs_report.add_account_totals_for_llg(&llg, converted_amount_report);
        }
    }

    // REVIEW: This is different across programs
    fn insert_in_store_converting_currency(
        &mut self,
        llg: &LLGKey,
        grouped_cashflows: &HashMap<NaiveDate, CashflowAggregatedOnDateBuilder>,
        target_currency: String,
        config_params: &ConfigurationParameters,
        exchange_rt: f64,
        local_ccy_equivalent: bool,
    ) -> f64 {
        let mut currency_converted_grouped_cashflows = HashMap::new();
        let mut convertion_rate = 1.0;
        for (date, builder) in grouped_cashflows {
            let currency_converted_builder = self.currency_converter.convert(
                &target_currency,
                builder,
                config_params,
                exchange_rt,
                local_ccy_equivalent,
            );
            currency_converted_grouped_cashflows.insert(*date, currency_converted_builder.0);
            convertion_rate = currency_converted_builder.1;
        }

        self.insert_into_store(&llg, currency_converted_grouped_cashflows);
        convertion_rate
    }

    fn insert_into_store(
        &mut self,
        llg: &LLGKey,
        grouped_cashflows: HashMap<NaiveDate, CashflowAggregatedOnDateBuilder>,
    ) {
        if let Some(cashflows_for_llg) = self.store.get_mut(&llg) {
            add(cashflows_for_llg, grouped_cashflows);
        } else {
            self.store.insert(llg.clone(), grouped_cashflows);
        }
    }
}

impl CashflowOrganizer {
    pub fn drain(&mut self) -> Drain<LLGKey, HashMap<NaiveDate, CashflowAggregatedOnDateBuilder>> {
        self.store.drain()
    }
}

fn add(
    existing: &mut HashMap<NaiveDate, CashflowAggregatedOnDateBuilder>,
    mut new_grouped_cashflows: HashMap<NaiveDate, CashflowAggregatedOnDateBuilder>,
) {
    for (date, builder) in new_grouped_cashflows.drain() {
        // Thank you krdln, leonardo:
        // https://users.rust-lang.org/t/why-cant-i-have-mutable-references-in-if-else-branches-of-a-hashmap/19203/3?u=mayurdzk
        let existing_builder = existing
            .entry(date)
            .or_insert(CashflowAggregatedOnDateBuilder::new());
        existing_builder.add_from_builder(builder);
    }
}
