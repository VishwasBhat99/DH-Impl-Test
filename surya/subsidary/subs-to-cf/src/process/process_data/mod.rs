use super::cashflow_output::*;
use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use std::ops::Add;

pub fn get_cashflows(data: &Vec<f64>, days_range: &[&str], aod: &NaiveDate) -> Vec<Cashflow> {
    let mut cashflows: Vec<Cashflow> = Vec::new();
    for (i, amount) in data.iter().enumerate() {
        let cashflow = construct_cashflow(*amount, aod, days_range[i].to_string());
        cashflows.push(cashflow)
    }
    cashflows
}
pub fn construct_cashflow(amount: f64, aod: &NaiveDate, mut bucket_day: String) -> Cashflow {
    let day_type = bucket_day.pop().expect("unable to get day type");
    let day_range = bucket_day
        .parse::<usize>()
        .expect("unable to get range day");
    let mut cashflow = Cashflow::new();

    cashflow.set_interest_amount(0.0);
    cashflow.set_principal_amount(amount);
    cashflow.set_date(get_timestamp(aod, day_range, day_type));
    cashflow
}

pub fn get_timestamp(aod: &NaiveDate, prd: usize, prd_type: char) -> i64 {
    match prd_type {
        'D' => add_day_get_timestamp(aod, prd as i64),
        'M' => {
            let new_date = rbdate::incr_dt_by_mon_presrv_eom(*aod, prd)
                .expect("Cannot increment date by months");
            date_to_timestamp(new_date)
        }
        'Y' => {
            let new_date = rbdate::incr_dt_by_mon_presrv_eom(*aod, prd * 12)
                .expect("Cannot increment date by years");
            date_to_timestamp(new_date)
        }
        _ => panic!("Unexpected period type"),
    }
}

pub fn add_day_get_timestamp(date: &NaiveDate, days: i64) -> i64 {
    let duration_to_add = Duration::days(days);
    let zero_time = NaiveTime::from_hms_milli(0, 00, 00, 000);
    let aod_datetime: NaiveDateTime = date.and_time(zero_time);
    aod_datetime.add(duration_to_add).timestamp()
}

pub fn date_to_timestamp(date: NaiveDate) -> i64 {
    let zero_time = NaiveTime::from_hms_milli(0, 00, 00, 000);
    let aod_datetime: NaiveDateTime = date.and_time(zero_time);
    aod_datetime.timestamp()
}
