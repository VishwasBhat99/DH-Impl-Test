use self::cashflow_grouped_by_day::cashflows_grouped_by_day;
use aggregator::cashflow_organizer::cashflow_aggregation::CashflowAggregatedOnDateBuilder;
use aggregator::currency::currency_converter::CurrencyConverter;
use aggregator::llg::llg_key::LLGKey;
use aggregator::reports::input_report::InputReport;
use aggregator::reports::llgs_report::LLGsReport;
use aggregator::AccFieldNames;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use std::collections::hash_map::Drain;
use std::collections::HashMap;

mod bucket;
pub mod cashflow_aggregation;
pub mod cashflow_grouped_by_day;

pub struct CashflowOrganizer {
    store: HashMap<LLGKey, HashMap<i64, CashflowAggregatedOnDateBuilder>>,
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
        llg: &mut LLGKey,
        account: AccountWithCFs,
        k: &AccFieldNames,
        mut input_report: InputReport,
        llgs_report: &mut LLGsReport,
        is_acc_level_ex_rt: bool,
        ex_rt: f64,
    ) {
        let grouped_cashflows = cashflows_grouped_by_day(account, k, llg.tenor);

        input_report.add_account_totals(grouped_cashflows.acc_amts_rpt);

        llgs_report.add_account_totals_for_llg(&llg, grouped_cashflows.acc_amts_rpt);

        self.build_with(
            llg,
            grouped_cashflows.bkt_grpd_cfs,
            grouped_cashflows.acc_amts_rpt,
            llgs_report,
            is_acc_level_ex_rt,
            ex_rt,
        )
    }
    // Needs to be thread-safe if concurrency is implemented.
    pub fn build_with(
        &mut self,
        llg: &mut LLGKey,
        grouped_cashflows: HashMap<i64, CashflowAggregatedOnDateBuilder>,
        grouped_cf_amt: InputReport,
        llgs_report: &mut LLGsReport,
        is_acc_level_ex_rt: bool,
        ex_rt: f64,
    ) {
        let mut new_llg: Option<LLGKey> = None;
        let mut converted_amount_report = grouped_cf_amt;
        // If this account is in a currency different from the the base currency
        // convert its values to the base currency and insert that as well.
        if llg.ccy != self.currency_converter.consolidated_currency {
            let mut converted_llg = LLGKey::new(
                llg.llg_cd,
                self.currency_converter.consolidated_currency.to_string(),
                llg.tenor,
            );

            let conversion_rate = self.insert_in_store_converting_currency(
                &mut converted_llg,
                &grouped_cashflows,
                llg.ccy.to_string(),
                is_acc_level_ex_rt,
                ex_rt,
            );
            new_llg = Some(converted_llg);

            converted_amount_report.total_interest_amount *= conversion_rate;
            converted_amount_report.total_principal_amount *= conversion_rate;
        }

        self.insert_into_store(&llg, grouped_cashflows);
        if let Some(l) = new_llg {
            llgs_report.add_account_totals_for_llg(&l, converted_amount_report);
        }
    }

    // REVIEW: This is different across programs
    fn insert_in_store_converting_currency(
        &mut self,
        llg: &mut LLGKey,
        grouped_cashflows: &HashMap<i64, CashflowAggregatedOnDateBuilder>,
        target_currency: String,
        is_acc_level_ex_rt: bool,
        exchange_rt: f64,
    ) -> f64 {
        let mut currency_converted_grouped_cashflows = HashMap::new();
        let mut convertion_rate = 1.0;
        for (bkt, builder) in grouped_cashflows {
            let currency_converted_builder = self.currency_converter.convert(
                &target_currency,
                builder,
                is_acc_level_ex_rt,
                exchange_rt,
            );
            currency_converted_grouped_cashflows.insert(*bkt, currency_converted_builder.0);
            convertion_rate = currency_converted_builder.1;
        }
        self.insert_into_store(&llg, currency_converted_grouped_cashflows);
        convertion_rate
    }

    fn insert_into_store(
        &mut self,
        llg: &LLGKey,
        grouped_cashflows: HashMap<i64, CashflowAggregatedOnDateBuilder>,
    ) {
        if self.store.get(&llg).is_some() {
            let cashflows_for_llg = self.store.get_mut(&llg).unwrap();
            add(cashflows_for_llg, grouped_cashflows);
        } else {
            self.store.insert(llg.clone(), grouped_cashflows);
        }
    }
}

impl CashflowOrganizer {
    pub fn drain(&mut self) -> Drain<LLGKey, HashMap<i64, CashflowAggregatedOnDateBuilder>> {
        self.store.drain()
    }
}

fn add(
    existing: &mut HashMap<i64, CashflowAggregatedOnDateBuilder>,
    mut new_grouped_cashflows: HashMap<i64, CashflowAggregatedOnDateBuilder>,
) {
    for (bkt, builder) in new_grouped_cashflows.drain() {
        // Thank you krdln, leonardo:
        // https://users.rust-lang.org/t/why-cant-i-have-mutable-references-in-if-else-branches-of-a-hashmap/19203/3?u=mayurdzk
        let existing_builder = existing
            .entry(bkt)
            .or_insert(CashflowAggregatedOnDateBuilder::new());
        existing_builder.add_from_builder(builder);
    }
}
