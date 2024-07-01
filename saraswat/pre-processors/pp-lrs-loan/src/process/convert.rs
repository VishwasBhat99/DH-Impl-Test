pub fn to_i64(val: &str) -> i64 {
    val.to_string().parse::<i64>().unwrap_or(0)
}

pub fn to_date(val: &str) -> rbdate::NaiveDate {
    rbdate::NaiveDate::parse_from_str(val, "%d-%m-%Y").expect("Invalid Date")
}
