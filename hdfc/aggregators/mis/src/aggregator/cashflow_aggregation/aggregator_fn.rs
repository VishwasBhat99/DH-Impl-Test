use aggregator::cashflow_organizer::cashflow_aggregation::CashflowAggregatedOnDateBuilder;
use aggregator::cashflow_organizer::cashflow_aggregation::LLGAggregateOnDay;
use std::collections::HashMap;

// Proceeding operations on the hashmap all remove the key/value pair, making
// successive computations faster.

pub fn aggregated(
    bkt: i64,
    cashflows_map: &mut HashMap<i64, CashflowAggregatedOnDateBuilder>,
) -> Option<LLGAggregateOnDay> {
    // If Hashmap is empty, return zero-valued CF
    if cashflows_map.is_empty() {
        return None;
    }

    if let Some(cf_builder_on_bkt) = cashflows_map.remove(&bkt) {
        Some(cf_builder_on_bkt.to_cf_aggregated())
    } else {
        None
    }
}
