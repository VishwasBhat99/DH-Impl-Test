use rbdate::incr_dt_by_mon_presrv_eom_checked;
use rbdate::NaiveDate;

pub fn get_aggregation_date_limit(as_on_date: &NaiveDate, months: i64) -> NaiveDate {
    incr_dt_by_mon_presrv_eom_checked(*as_on_date, months as usize).expect(&format!(
        "Error while incrementing {} months from now.",
        months
    ))
}
