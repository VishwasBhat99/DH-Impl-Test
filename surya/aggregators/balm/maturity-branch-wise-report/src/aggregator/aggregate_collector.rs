use super::aggregator_fn;
use aggregator::structs::CashflowAggregatedOnDateBuilder;
use aggregator::structs::LLGAggregateOnDay;
use chrono::Duration;
use rbdate::incr_dt_by_mon_presrv_eom;
use rbdate::NaiveDate;
use std::collections::HashMap;

pub fn get_717_aggregates(
    as_on_day: &NaiveDate,
    mut grouped_cfs: HashMap<NaiveDate, CashflowAggregatedOnDateBuilder>,
) -> (
    Vec<Option<LLGAggregateOnDay>>,
    HashMap<NaiveDate, CashflowAggregatedOnDateBuilder>,
) {
    // 717 is a known size
    let mut aggregate_records = Vec::with_capacity(717);
    let last_dt_for_daily_bucket = incr_dt_by_mon_presrv_eom(as_on_day.pred(), 12)
        .expect("Error while incrementing last date for daily bucket.");
    let mut st_dt_mon_bucket = last_dt_for_daily_bucket.pred();
    let mut day_num = 1;
    let mut date = *as_on_day;
    // Daily aggregations.
    while date <= last_dt_for_daily_bucket {
        let aggregate =
            match aggregator_fn::aggregated(date, false, &mut grouped_cfs, st_dt_mon_bucket.succ())
            {
                None => None,
                Some(val) => (Some(val)),
            };
        aggregate_records.push(aggregate);
        day_num += 1;
        date += Duration::days(1);
    }
    if day_num == 366 {
        aggregate_records.push(None);
    }
    // Monthly Aggregations.

    for i in 1..=351 {
        let monthly_date = incr_dt_by_mon_presrv_eom(as_on_day.pred(), 12 + i)
            .expect("Error while incrementing monthly date.");
        let aggregate =
            match aggregator_fn::aggregated(monthly_date, true, &mut grouped_cfs, st_dt_mon_bucket)
            {
                None => None,
                Some(val) => (Some(val)),
            };
        aggregate_records.push(aggregate);
        st_dt_mon_bucket = monthly_date;
    }

    (aggregate_records, grouped_cfs)
}
