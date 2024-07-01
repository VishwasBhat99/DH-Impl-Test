use chrono::NaiveDateTime;
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct LLGKey {
    pub llg_id: String,
    pub curr_code: String,
    pub bm_id: String,
    pub next_repricing_date: String,
    pub rep_freq: String,
    pub tenor: String,
}

pub fn create_key(
    next_repr_date: i64,
    llg_id: i32,
    currency: &String,
    bm_id: &String,
    rep_freq: f64,
    tenor: i64,
) -> LLGKey {
    let repr_date = NaiveDateTime::from_timestamp(next_repr_date, 0);

    LLGKey {
        llg_id: llg_id.to_string(),
        curr_code: currency.to_string(),
        bm_id: bm_id.to_string(),
        next_repricing_date: repr_date.date().format("%d-%m-%Y").to_string(),
        rep_freq: rep_freq.to_string(),
        tenor: tenor.to_string(),
    }
}
