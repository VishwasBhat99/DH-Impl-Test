use core::f32;

use rbdate::*;

pub static DEFAULT_INT: i64 = 0;
pub static DEFAULT_FLOAT: f64 = 0.0;
lazy_static! {
    pub static ref DEFAULT_DATE: NaiveDate = NaiveDate::from_ymd(1900, 01, 01);
}
use math::round::half_away_from_zero;

pub fn get_def_date(date: &mut NaiveDate, as_on_date: &NaiveDate) {
    if *date == NaiveDate::from_ymd(1970, 01, 01) {
        *date = *as_on_date;
    }
}

pub fn parse_i64(val: &str) -> i64 {
    val.parse::<i64>().unwrap_or(DEFAULT_INT)
}

pub fn parse_f64(val: &str) -> f64 {
    val.parse::<f64>().unwrap_or(DEFAULT_FLOAT)
}

pub fn parse_i32(val: &str) -> i32 {
    parse_i64(val) as i32
}

pub fn parse_f32(val: &str) -> f32 {
    parse_f64(val) as f32
}

pub fn parse_date(val: &str) -> NaiveDate {
    NaiveDate::parse_from_str(val, "%d-%m-%Y").unwrap_or(*DEFAULT_DATE)
}

pub fn ig_neg_val_f64(val: &mut f64) {
    *val = if *val < 0.0 { DEFAULT_FLOAT } else { *val };
}

pub fn ig_neg_val_i64(val: &mut i64) {
    *val = if *val < 0 { DEFAULT_INT } else { *val };
}

pub fn get_file_path(file_path: String, as_on_date: NaiveDate) -> String {
    if file_path.contains("{ddmmyyyy}") {
        file_path.replace("{ddmmyyyy}", &as_on_date.format("%d%m%Y").to_string())
    } else if file_path.contains("{AsOnDate}") {
        file_path.replace("{AsOnDate}", &as_on_date.format("%d-%m-%Y").to_string())
    } else {
        file_path
    }
}

pub fn rounded_f64(val: f64, prec: i8) -> f64 {
    half_away_from_zero(val, prec)
}

pub fn rounded_f32(val: f32, prec: i8) -> f32 {
    half_away_from_zero(val as f64, prec) as f32
}
