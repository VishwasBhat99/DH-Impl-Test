use chrono::{NaiveDate, NaiveDateTime};
use rbdate::date_from_timestamp;
use std::collections::HashMap;
pub fn get_spread(
    next_repr_dt: i64,
    bm_id: String,
    int_rt: f64,
    rep_freq: i64,
    bm_rt_map: HashMap<String, Vec<f64>>,
    date_vec: Vec<String>,
    last_repr_dt: i64,
) -> f64 {
    if !bm_rt_map.contains_key(&bm_id) {
        return 0.0;
    }
    let len = date_vec.len();
    let rate_vec = bm_rt_map.get(&bm_id).unwrap();
    let nxt_rep_dt = NaiveDateTime::from_timestamp(next_repr_dt, 0);
    let last_rep_dt = date_from_timestamp(last_repr_dt);
    let mut vec_count = 0;
    let mut vec_date;
    for date in date_vec {
        if NaiveDate::parse_from_str(&date, "%d-%m-%Y").is_err() {
            vec_date =
                datevalue_to_naive_date(&date).expect("cannot read date in format DD/MM/YYYY");
        } else {
            vec_date = NaiveDate::parse_from_str(&date, "%d-%m-%Y")
                .expect("cannot read date in format DD-MM-YYYY");
        }
        if last_rep_dt <= vec_date {
            break;
        }
        vec_count += 1;
    }
    if vec_count == len {
        vec_count -= 1;
    }
    let bm_rt = rate_vec[vec_count];
    let spread = int_rt - bm_rt;
    return spread;
}
fn datevalue_to_naive_date(date: &str) -> Option<NaiveDate> {
    if let Ok(timestamp) = date.parse::<f64>() {
        Some(date_from_timestamp(((timestamp as i64) - 25569) * 86400))
    } else {
        None
    }
}
