pub fn str_to_flt(val: &str) -> f64 {
    val.to_string().parse().unwrap_or(0.0)
}

pub fn str_to_int(val: &str) -> i64 {
    val.to_string().parse().unwrap_or(0)
}
