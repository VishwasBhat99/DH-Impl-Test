use std::{collections::HashMap, f64::MAX_EXP};

use slog::Logger;

use crate::configuration_parameters::ConfigurationParameters;

#[derive(Debug, Eq, Default, Clone, Hash, PartialEq)]
pub struct Bucket {
    pub init_amt: String,
    pub final_amt: String,
}

impl Bucket {
    pub fn new(init_amt: String, final_amt: String) -> Bucket {
        Bucket {
            init_amt,
            final_amt,
        }
    }
}

pub fn get_amt_cat(
    yield_rate: f64,
    rate_bucket_map: &HashMap<Bucket, String>,
    config_params: &ConfigurationParameters,
    logger: &Logger,
) -> String {
    let mut rate_amt = "NA".to_string();
    for (rate_bucket, amt_cat) in rate_bucket_map.iter() {
        let init_amt = rate_bucket
            .init_amt
            .trim()
            .parse::<f64>()
            .unwrap_or(MAX_EXP.into());
        let final_amt = rate_bucket
            .final_amt
            .trim()
            .parse::<f64>()
            .unwrap_or(MAX_EXP.into());
        if init_amt <= yield_rate && final_amt > yield_rate {
            rate_amt = amt_cat.to_string();
            break;
        }
    }
    rate_amt
}
