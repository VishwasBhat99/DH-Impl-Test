use aggregator::structs::AggregateData;
use chrono::{Datelike, Duration, NaiveDate};
use rbdate::{incr_dt_by_mon_presrv_eom_checked, increment_date_by_months};
use std::collections::HashMap;

pub fn bucket_op(
    data: HashMap<NaiveDate, AggregateData>,
    as_on_date: NaiveDate,
    mut op_line: String,
    is_edate_req: bool,
    is_custom_bucket_req: bool,
    bucket_dt: &mut NaiveDate,
) -> String {
    //First 184 buckets are aggregated day-wise.

    let last_dt_for_daily_bucket =
        if as_on_date.month() == 0 && (as_on_date.day() == 28 || as_on_date.day() == 29) {
            increment_date_by_months(bucket_dt.pred(), 6)
        } else if is_edate_req {
            increment_date_by_months(bucket_dt.pred(), 6)
        } else {
            incr_dt_by_mon_presrv_eom_checked(bucket_dt.pred(), 6)
                .expect("Error while incrementing last date for daily bucket.")
        };

    let mut day_num = 1;
    while *bucket_dt <= last_dt_for_daily_bucket {
        let val = get_data(*bucket_dt, &data);

        if is_custom_bucket_req && *bucket_dt == last_dt_for_daily_bucket {
            op_line.push_str("0");
            op_line.push('|');
            day_num += 1;
            break;
        }
        op_line.push_str(&val.to_string());
        op_line.push('|');
        day_num += 1;
        *bucket_dt += Duration::days(1);
    }
    while day_num <= 184 {
        op_line.push_str(&0.00.to_string());
        op_line.push('|');
        day_num += 1;
    }
    //Next buckets are aggregated monthwise upto 24 months.
    let mut last_month_day_val = 0.0;
    for i in 7..=24 {
        let end_dt = if is_edate_req {
            increment_date_by_months(as_on_date, i)
        } else {
            incr_dt_by_mon_presrv_eom_checked(as_on_date, i as usize)
                .expect("Could not aggregate monthwise data. End date could not be fetched.")
        };
        let mut sum = 0.0;

        loop {
            //Check if bucket date has reached end date before fetching the value.
            if *bucket_dt > end_dt {
                break;
            }
            let val = get_data(*bucket_dt, &data);
            if is_custom_bucket_req && *bucket_dt == end_dt {
                last_month_day_val = val;
                break;
            }
            sum += val;
            *bucket_dt = bucket_dt.succ();
        }
        op_line.push_str(&sum.to_string());
        op_line.push('|');
    }

    if is_custom_bucket_req {
        *bucket_dt = bucket_dt.succ();
    } else {
        last_month_day_val = 0.0;
    }
    //Amounts after 2 years are aggregated to a single bucket.
    let val = get_data(*bucket_dt, &data) + last_month_day_val;
    op_line.push_str(&val.to_string());
    op_line
}

pub fn get_data(bucket_dt: NaiveDate, data: &HashMap<NaiveDate, AggregateData>) -> f64 {
    let data = match data.get(&bucket_dt) {
        Some(amt) => amt.total_amt,
        None => 0.00,
    };
    data
}
