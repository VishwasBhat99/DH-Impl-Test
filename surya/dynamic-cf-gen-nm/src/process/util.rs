use rbdate::incr_dt_by_mon_presrv_eom;
use rbdate::NaiveDate;

pub fn add_days(date: &NaiveDate, days: &u8) -> NaiveDate {
    let mut new_date = date.clone();
    let mut days_to_add = days.clone();
    loop {
        if days_to_add <= 0 {
            break;
        } else {
            new_date = new_date.succ();
            days_to_add -= 1;
        }
    }
    return new_date;
}
