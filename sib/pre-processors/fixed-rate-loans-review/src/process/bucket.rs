use std::collections::HashMap;
#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct Bucket {
    pub from_days: i64,
    pub to_days: i64,
}

impl Bucket {
    pub fn new(from_days: String, to_days: String) -> Bucket {
        Bucket {
            from_days: from_days.trim().parse::<i64>().unwrap_or(0),
            to_days: to_days.trim().parse::<i64>().unwrap_or(0),
        }
    }
}

pub fn get_bucket_id(bucket_days: i64, bucket_map: &HashMap<Bucket, i64>) -> i64 {
    for (days, bucket_id) in bucket_map.iter() {
        if days.from_days <= bucket_days && days.to_days >= bucket_days {
            return *bucket_id;
        }
    }
    0_i64
}
