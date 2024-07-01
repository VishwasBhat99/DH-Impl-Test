use chrono::NaiveDateTime;
use rbdate::incr_dt_by_mon_presrv_eom;
use rbdate::NaiveDate;

pub fn get_aggregation_date_limit(as_on_date: &NaiveDate) -> NaiveDate {
    let thirty_years_from_now = incr_dt_by_mon_presrv_eom(*as_on_date, 360)
        .expect("Error while incrementing 30 years from now.");

    thirty_years_from_now
}

pub fn naivedate_from_timestamp(t: i64) -> NaiveDate {
    let naive_date_time = NaiveDateTime::from_timestamp(t, 0);
    naive_date_time.date()
}
