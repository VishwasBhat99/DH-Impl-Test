// TODO: Lib
use super::aggregator_fn;
use std::collections::HashMap;
use chrono::NaiveDate;
use chrono::Duration;
use aggregator::structs::LLGAggregateOnDay;
use aggregator::structs::CashflowAggregatedOnDateBuilder;
use rbdate::increment_date_by_months_unchecked;

pub fn get_717_aggregates(
    as_on_day: &NaiveDate,
    mut grouped_cfs: HashMap<NaiveDate, CashflowAggregatedOnDateBuilder>,
) -> Vec<Option<LLGAggregateOnDay>> {

    // 717 is a known size
    let mut aggregate_records = Vec::with_capacity(717);
    let mut date = as_on_day.clone();

    // Daily aggregations.
    for i in 0..366 {
        let aggregate = match aggregator_fn::aggregated(
            date,
            false,
            &mut grouped_cfs
        ) {
            None => { None },
            Some(mut val) => {
                // The date needs to be set to the index of the day from 0...366
                val.int.date = i as f64;
                val.slr.date = i as f64;
                val.irs.date = i as f64;

                (Some(val))
            }
        };
        aggregate_records.push(
            aggregate
        );
        date += Duration::days(1);
    }

    // Monthly Aggregations.
    for _ in 0..351 {
        aggregate_records.push(
            aggregator_fn::aggregated(
                date,
                true,
                &mut grouped_cfs
            )
        );
        date = increment_date_by_months_unchecked(date, 1);
    }

    aggregate_records
}