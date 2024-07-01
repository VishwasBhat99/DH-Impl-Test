use aggregator::currency::currency_converter::CurrencyConverter;
use aggregator::currency::CurrencyExchange;
use aggregator::llg_key::LLGKey;
use aggregator::structs::AggregateData;
use chrono::Duration;
use configuration_parameters::ConfigurationParameters;
use rbdate::{num_days_start_to_end, NaiveDate};
use slog::Logger;
use std::collections::hash_map::Drain;
use std::collections::HashMap;

use super::overdue_llg::{get_overdue_llg, ResidualPeriod};

pub struct CashflowOrganizer {
    store: HashMap<LLGKey, HashMap<NaiveDate, AggregateData>>,
    currency_converter: CurrencyConverter,
}

impl CashflowOrganizer {
    pub fn new(currency_converter: CurrencyConverter) -> CashflowOrganizer {
        CashflowOrganizer {
            store: HashMap::new(),
            currency_converter,
        }
    }
    pub fn organize(
        &mut self,
        llg: &LLGKey,
        grouped_cashflows: HashMap<NaiveDate, AggregateData>,
        config_params: &ConfigurationParameters,
        logger: &Logger,
        overdue_llg_map: &HashMap<ResidualPeriod, i32>,
    ) {
        self.insert_in_store_converting_currency(
            llg,
            &grouped_cashflows,
            config_params,
            logger,
            overdue_llg_map,
        );
    }

    fn insert_in_store_converting_currency(
        &mut self,
        llg: &LLGKey,
        grouped_cashflows: &HashMap<NaiveDate, AggregateData>,
        config_params: &ConfigurationParameters,
        logger: &Logger,
        overdue_llg_map: &HashMap<ResidualPeriod, i32>,
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
        let mut currency_converted_grouped_cashflows = HashMap::new();
        let mut default_overdue_cashflows = HashMap::new();
        for (date, builder) in grouped_cashflows {
            let currency_converted_builder = if config_params.is_consolidated() {
                *builder // If consolidation flag is true, don't perform currency conversion
            } else {
                self.currency_converter
                    .convert(&currency_key, builder, logger)
            };
            if date <= config_params.as_on_date() {
                if config_params.is_overdue_req()
                    && config_params.default_overdue_llg_path() != "NA"
                {
                    default_overdue_cashflows.insert(*date, currency_converted_builder);
                } else if config_params.is_overdue_req() && *date <= *config_params.as_on_date() {
                    currency_converted_grouped_cashflows.insert(
                        *config_params.as_on_date() + Duration::days(1),
                        currency_converted_builder,
                    );
                }
            } else {
                currency_converted_grouped_cashflows.insert(*date, currency_converted_builder);
            }
        }
        let ccy = &llg.currency;
        if !default_overdue_cashflows.is_empty() {
            for (date, aggr_data) in default_overdue_cashflows {
                let residual_period =
                    num_days_start_to_end(date.clone(), *config_params.as_on_date()) as i32;
                let overdue_llg = match get_overdue_llg(residual_period, &overdue_llg_map) {
                    Some(value) => value,
                    None => {
                        panic!("Unale to get default overdue llg, please check overdue-config file")
                    }
                };

                let new_llg = &LLGKey {
                    currency: ccy.to_string(),
                    category: overdue_llg,
                };
                let mut overdue_map = HashMap::new();
                overdue_map.insert(*config_params.as_on_date(), aggr_data);
                self.insert_into_store(new_llg, overdue_map);
            }
        }
        self.insert_into_store(llg, currency_converted_grouped_cashflows);
    }

    fn insert_into_store(
        &mut self,
        llg: &LLGKey,
        grouped_cashflows: HashMap<NaiveDate, AggregateData>,
    ) {
        if let Some(cashflows_for_llg) = self.store.get_mut(llg) {
            add(cashflows_for_llg, grouped_cashflows);
        } else {
            self.store.insert(llg.clone(), grouped_cashflows);
        }
    }
}

impl CashflowOrganizer {
    pub fn drain(&mut self) -> Drain<LLGKey, HashMap<NaiveDate, AggregateData>> {
        self.store.drain()
    }
}

fn add(
    existing: &mut HashMap<NaiveDate, AggregateData>,
    mut new_grouped_cashflows: HashMap<NaiveDate, AggregateData>,
) {
    for (date, builder) in new_grouped_cashflows.drain() {
        let existing_builder = existing.entry(date).or_insert_with(AggregateData::new);
        existing_builder.add_from_builder(builder);
    }
}
