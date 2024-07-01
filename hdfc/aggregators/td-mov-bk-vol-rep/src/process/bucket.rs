use std::collections::HashMap;

#[derive(Debug, Eq, Default, Clone, Hash, PartialEq)]
pub struct Bucket {
    pub init_amt: String,
    pub final_amt: String,
}

impl Bucket {
    pub fn new(init_amt: String, final_amt: String) -> Bucket {
        Bucket {
            init_amt: init_amt,
            final_amt: final_amt,
        }
    }
}

pub fn get_amt_cat(amt: String, bucket_map: &HashMap<Bucket, String>) -> String {
    let amt = amt.to_string().trim().parse::<f64>().unwrap_or(0.0);
    for (bucket, amt_cat) in bucket_map.iter() {
        let init_amt = bucket.init_amt.trim().parse::<f64>().unwrap_or(0.0);
        let final_amt = bucket.final_amt.trim().parse::<f64>().unwrap_or(0.0);
        if init_amt <= amt && final_amt >= amt {
            return amt_cat.to_string();
        }
    }
    String::from("0.0")
}
