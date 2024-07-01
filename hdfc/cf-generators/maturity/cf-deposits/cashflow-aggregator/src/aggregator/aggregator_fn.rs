// TODO: Lib
use rbdate;
use chrono::NaiveDate;
use std::collections::HashMap;
use chrono::Datelike;
use chrono::Duration;
use aggregator::structs::LLGAggregateOnDay;
use aggregator::structs::CashflowAggregatedOnDateBuilder;


// Proceeding operations on the hashmap all remove the key/value pair, making
// successive computations faster.

pub fn aggregated(
    date: NaiveDate,
    is_by_month: bool,
    cashflows_map: &mut HashMap<NaiveDate, CashflowAggregatedOnDateBuilder>
) -> Option<LLGAggregateOnDay> {

    // If Hashmap is empty, return zero-valued CF
    if cashflows_map.is_empty() {
        return None
    }

    // If not by month, get the date and accompanying cfs. Pass to aggregate.
    if !is_by_month {
        if let Some(cf_builder_on_date) = cashflows_map.remove(&date) {
            return Some(cf_builder_on_date.to_cf_aggregated());
        } else{
            return None;
        }
    }

    // If by month, aggregate all cfs between date and 30 days from date.
    /*
        Loop
        1. Increment date
        2. Get builders on date
        3. If they exist add them to the monthly CF Builder.
    */

    let mut day_of_month = NaiveDate::from_ymd(date.year(), date.month(), 1);
    let start_of_next_month = rbdate::increment_date_by_months_unchecked(day_of_month, 1);

    let mut cf_builder_for_month = CashflowAggregatedOnDateBuilder::new();
    let mut does_month_contain_cf = false;

    // TODO: This code can be more performant.
    loop {
        if let Some(cf_builder_on_date) = cashflows_map.remove(&day_of_month) {
            does_month_contain_cf = true;
            cf_builder_for_month.add_from_builder(cf_builder_on_date);
        }
        day_of_month += Duration::days(1);

        if day_of_month == start_of_next_month {
            // Next month has started. Stop looping.
            break;
        }
    }


    if does_month_contain_cf == false {
        return None;
    } else {
        let v = cf_builder_for_month.to_cf_aggregated();
        return Some(v);
    }
}