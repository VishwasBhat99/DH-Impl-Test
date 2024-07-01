use aggregator::currency::currency_converter::CurrencyConverter;
use aggregator::currency::CurrencyExchange;
use aggregator::llg_key::LLGKey;
use aggregator::reports::input_report::InputReport;
use aggregator::reports::llgs_report::LLGsReport;
use aggregator::structs::CashflowAggregatedOnDateBuilder;
use configuration_parameters::ConfigurationParameters;
use rbdate::NaiveDate;
use slog::Logger;
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
        default_overdue_llg_code: i32,
        is_consolidated: bool,
        is_account_level_exchange_rate: bool,
        llgs_report: &mut LLGsReport,
        exchange_rt: f64,
        logger: &Logger,
    ) {
        self.build_with(
            &llg,
            grouped_cashflows,
            grouped_cf_amt,
            config_params,
            is_consolidated,
            is_account_level_exchange_rate,
            llgs_report,
            exchange_rt,
            logger,
        );

        if !overdue_cashflows.is_empty() {
            let default_overdue_llg = LLGKey::new(
                llg.currency.clone(),
                default_overdue_llg_code,
                llg.cf_type.clone(),
            );
            let overdue_report = InputReport::new();
            llgs_report.add_account_totals_for_llg(&default_overdue_llg, overdue_report);
            self.build_with(
                &default_overdue_llg,
                overdue_cashflows,
                account_overdue_amount_report,
                config_params,
                is_consolidated,
                is_account_level_exchange_rate,
                llgs_report,
                exchange_rt,
                logger,
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
        is_consolidated: bool,
        is_account_level_exchange_rate: bool,
        llgs_report: &mut LLGsReport,
        ex_rt: f64,
        logger: &Logger,
    ) {
        let input_ccy = llg.currency.to_string();
        let new_llg: Option<LLGKey>;
        let mut new_base_llg: Option<LLGKey> = None;
        let mut new_llg_conv_amt_rpt = grouped_cf_amt;
        let mut new_base_llg_conv_amt_rpt = grouped_cf_amt;

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
            let conversion_rate = self.insert_in_store_converting_currency(
                &converted_llg,
                &grouped_cashflows,
                input_ccy.to_string(),
                config_params.src_local_ccy().to_string(),
                config_params,
                is_consolidated,
                is_account_level_exchange_rate,
                ex_rt,
                true,
                logger,
            );
            new_llg = Some(converted_llg);
            new_llg_conv_amt_rpt.total_interest_amount *= conversion_rate;
            new_llg_conv_amt_rpt.total_principal_amount *= conversion_rate;

            llgs_report.add_account_totals_for_llg(&base_llg, grouped_cf_amt);
            self.insert_into_store(&base_llg, grouped_cashflows);
        } else {
            let converted_llg = LLGKey::new(
                config_params.consol_ccy().to_string(),
                llg.category,
                llg.cf_type.clone(),
            );
            let conversion_rate_llg = self.insert_in_store_converting_currency(
                &converted_llg,
                &grouped_cashflows,
                input_ccy.to_string(),
                config_params.src_local_ccy().to_string(),
                config_params,
                is_consolidated,
                is_account_level_exchange_rate,
                ex_rt,
                false,
                logger,
            );
            new_llg = Some(converted_llg);
            new_llg_conv_amt_rpt.total_interest_amount *= conversion_rate_llg;
            new_llg_conv_amt_rpt.total_principal_amount *= conversion_rate_llg;

            let base_llg = LLGKey::new(llg.currency.to_string(), llg.category, llg.cf_type.clone());
            let conversion_rate_base_llg = self.insert_in_store_converting_currency(
                &base_llg,
                &grouped_cashflows,
                input_ccy.to_string(),
                llg.currency.to_string(),
                config_params,
                is_consolidated,
                is_account_level_exchange_rate,
                ex_rt,
                false,
                logger,
            );
            new_base_llg = Some(base_llg);
            new_base_llg_conv_amt_rpt.total_interest_amount *= conversion_rate_base_llg;
            new_base_llg_conv_amt_rpt.total_principal_amount *= conversion_rate_base_llg;
        }
        if let Some(llg) = new_llg {
            llgs_report.add_account_totals_for_llg(&llg, new_llg_conv_amt_rpt);
        }
        if let Some(llg) = new_base_llg {
            llgs_report.add_account_totals_for_llg(&llg, new_base_llg_conv_amt_rpt);
        }
    }

    // REVIEW: This is different across programs
    fn insert_in_store_converting_currency(
        &mut self,
        llg: &LLGKey,
        grouped_cashflows: &HashMap<NaiveDate, CashflowAggregatedOnDateBuilder>,
        input_ccy: String,
        target_currency: String,
        config_params: &ConfigurationParameters,
        is_consolidated: bool,
        is_account_level_exchange_rate: bool,
        exchange_rt: f64,
        local_ccy_equivalent: bool,
        logger: &Logger,
    ) -> f64 {
        let currency_key = if is_consolidated {
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
        let mut currency_converted_grouped_cashflows = HashMap::new();
        let mut convertion_rate = 1.0;
        for (date, builder) in grouped_cashflows {
            let currency_converted_builder = self.currency_converter.convert(
                &currency_key,
                builder,
                config_params,
                is_account_level_exchange_rate,
                exchange_rt,
                local_ccy_equivalent,
                logger,
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
