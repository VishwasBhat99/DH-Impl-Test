use rbdate::*;

pub fn get_months(start_date_opt: Option<NaiveDate>, end_date_opt: Option<NaiveDate>) -> i64 {
    if let Some(start_date) = start_date_opt {
        if let Some(end_date) = end_date_opt {
            (num_days_start_to_end(start_date, end_date) as f64 * 12.0 / 365.0).round() as i64
        } else {
            0
        }
    } else {
        0
    }
}
