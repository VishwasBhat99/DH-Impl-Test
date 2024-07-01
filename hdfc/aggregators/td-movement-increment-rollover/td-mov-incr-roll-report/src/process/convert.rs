pub static DEFAULT_INT: i64 = 0;
pub static DEFAULT_FLOAT: f64 = 0.0;

pub fn to_f64(val: String) -> f64 {
    return val.to_string().parse::<f64>().unwrap_or(DEFAULT_FLOAT);
}

pub fn to_i64(val: String) -> i64 {
    return val.to_string().parse::<i64>().unwrap_or(DEFAULT_INT);
}
