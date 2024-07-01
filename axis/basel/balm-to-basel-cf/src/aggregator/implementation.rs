use aggregator::bucket::DateRange;
use chrono::Datelike;
use chrono::Duration;
use rbdate::incr_dt_by_mon_presrv_eom_checked;
use rbdate::num_days_start_to_end;
use rbdate::NaiveDate;
use std::collections::HashMap;

pub fn get_buckted_amt(
    amt_per_day_map: HashMap<i64, f64>,
    as_on_date: &NaiveDate,
    bucket_slab: &HashMap<i64, DateRange>,
) -> Vec<f64> {
    let mut amt_vec: Vec<f64> = Vec::new();
    let mut bkt = 0;
    let mut date = *as_on_date + Duration::days(1);
    let last_daywise_date = incr_dt_by_mon_presrv_eom_checked(*as_on_date, 6).unwrap();
    while date < last_daywise_date {
        bkt += 1;
        for (key, val) in bucket_slab {
            if bkt < 185 {
                if date >= val.from_date && date < val.to_date {
                    let prin_amt = *amt_per_day_map.get(&key).unwrap_or(&0.0);
                    amt_vec.push(prin_amt);
                }
            }
        }
        date = date + Duration::days(1);
    }

    while bkt < 184 {
        amt_vec.push(0.0);
        bkt += 1;
    }

    while bkt < 190 {
        let mut next_date = incr_dt_by_mon_presrv_eom_checked(date, 1).unwrap();
        if date.month() == 2 && date.day() == 29 {
            next_date = next_date + Duration::days(2);
        }
        let check_date = next_date - Duration::days(1);
        let num_days = num_days_start_to_end(date, next_date);
        for (key, val) in bucket_slab {
            if check_date >= val.from_date && check_date < val.to_date {
                let prin_amt = *amt_per_day_map.get(&key).unwrap_or(&0.0) * num_days as f64;
                amt_vec.push(prin_amt);
            }
        }
        date = next_date;
        bkt += 1;
    }
    let mut months_pass = 0;
    let mut month_amt = 0.0;
    while bkt < 202 {
        let amt = amt_per_day_map.get(&4).unwrap_or(&0.0);
        month_amt = amt.clone();
        amt_vec.push(*amt);
        months_pass += 1;
        bkt += 1;
    }

    let last_date_range = bucket_slab.get(&4).unwrap();
    let tot_months =
        (num_days_start_to_end(last_date_range.from_date, last_date_range.to_date) / 365) * 12;
    let rem_months = tot_months - months_pass;
    let amt = month_amt * rem_months as f64;
    amt_vec.push(amt);
    amt_vec
}
