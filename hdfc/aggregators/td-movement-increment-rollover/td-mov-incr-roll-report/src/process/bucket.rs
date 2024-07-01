use std::collections::HashMap;

#[derive(Debug, Eq, Default, Clone, Hash, PartialEq)]
pub struct Bucket {
    pub init_amt: i64,
    pub final_amt: i64,
}

impl Bucket {
    pub fn new(init_amt: String, final_amt: String) -> Bucket {
        Bucket {
            init_amt: init_amt.trim().parse::<i64>().unwrap_or(0),
            final_amt: final_amt.trim().parse::<i64>().unwrap_or(0),
        }
    }
}

pub fn get_amt_cat(amt: String, bucket_map: &HashMap<Bucket, String>) -> String {
    let amt = amt.trim().parse::<i64>().unwrap_or(0);
    for (bucket, amt_cat) in bucket_map.iter() {
        if bucket.init_amt <= amt && bucket.final_amt >= amt {
            return amt_cat.to_string();
        }
    }
    String::from("0.0")
}
