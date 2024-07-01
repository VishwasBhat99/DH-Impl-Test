use chrono::NaiveDateTime;
use rbdate::incr_dt_by_mon_presrv_eom_checked;
use rbdate::NaiveDate;

pub fn get_aggregation_date_limit(as_on_date: &NaiveDate) -> NaiveDate {
    incr_dt_by_mon_presrv_eom_checked(*as_on_date, 24)
        .expect("Error while incrementing 2 years from now.")
}

pub fn naivedate_from_timestamp(t: i64) -> NaiveDate {
    let naive_date_time = NaiveDateTime::from_timestamp(t, 0);
    naive_date_time.date()
}
