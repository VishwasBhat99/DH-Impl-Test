#[derive(Debug, Eq, Default, Clone, Hash, PartialEq)]
pub struct Bucket {
    pub from_bkt_days: i64,
    pub to_bkt_days: i64,
    pub scheme_id: String,
}

impl Bucket {
    pub fn new(from_bkt_days: i64, to_bkt_days: i64, scheme_id: String) -> Bucket {
        Bucket {
            from_bkt_days: from_bkt_days,
            to_bkt_days: to_bkt_days,
            scheme_id: scheme_id,
        }
    }
}
