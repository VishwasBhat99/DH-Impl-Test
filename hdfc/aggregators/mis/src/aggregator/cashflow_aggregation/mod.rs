use aggregator::cashflow_organizer::cashflow_aggregation::CashflowAggregatedOnDateBuilder;
use aggregator::cashflow_organizer::cashflow_aggregation::LLGAggregateOnDay;
use std::collections::HashMap;

mod aggregator_fn;

pub fn get_14_aggregates(
    mut grouped_cfs: HashMap<i64, CashflowAggregatedOnDateBuilder>,
) -> (Vec<Option<LLGAggregateOnDay>>) {
    let mut aggregate_records = Vec::with_capacity(28);
    let mut bkt = 1;
    // Daily aggregations.
    for _ in 0..28 {
        let aggregate = match aggregator_fn::aggregated(bkt, &mut grouped_cfs) {
            None => None,
            Some(val) => Some(val),
        };
        aggregate_records.push(aggregate);
        bkt += 1;
    }

    aggregate_records
}
