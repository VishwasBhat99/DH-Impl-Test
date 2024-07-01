use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LLGKey {
    pub as_on: String,
    pub llg_id: String,
    pub ccy: String,
    pub ex_rt: String,
    pub bm_id: String,
    pub tenor: String,
    pub rep_dt: String,
    pub rep_freq: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BucketValue {
    pub spread: f64,
    pub principal_vec: Vec<f64>,
    pub rate_vec: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SummaryValue {
    pub spread: f64,
    pub principal_smry: f64,
    pub rate_smry: f64,
}

pub fn organize_aggregate(
    map: &mut HashMap<LLGKey, BucketValue>,
    principal_line: &str,
    rate_line: &str,
) {
    let header_size = 9;
    let mut principal_key: Vec<String> = principal_line.split('|').map(String::from).collect();
    let mut rate_key: Vec<String> = rate_line.split('|').map(String::from).collect();
    let principal_val = principal_key.split_off(header_size);
    let rate_val = rate_key.split_off(header_size);
    let key = get_key(principal_key.clone());
    let spread = principal_key[6].parse::<f64>().expect(&format!(
        "Cannot parse spread for llg : {}",
        principal_key[1],
    ));
    let len = principal_val.len();
    let mut new_prin_value: Vec<f64> = Vec::with_capacity(len);
    let mut new_rate_value: Vec<f64> = Vec::with_capacity(len);
    let mut new_spread: f64 = 0.0;
    if map.contains_key(&key) {
        let existing_value = map.get(&key).unwrap().clone();
        let existing_prin_value = existing_value.principal_vec;
        let existing_rate_value = existing_value.rate_vec;
        let existing_spread = existing_value.spread;
        new_spread = existing_spread
            + (spread
                * principal_val[0].parse::<f64>().expect(&format!(
                    "Cannot parse principal amount at bucket no : 0 for llg : {}",
                    principal_key[1]
                )));
        for bucket_no in 0..len {
            new_prin_value.push(
                principal_val[bucket_no].parse::<f64>().expect(&format!(
                    "Cannot parse principal amount at bucket no : {} for llg : {}",
                    bucket_no, principal_key[1]
                )) + existing_prin_value[bucket_no],
            );
            new_rate_value.push(
                existing_rate_value[bucket_no]
                    + (principal_val[bucket_no].parse::<f64>().expect(&format!(
                        "Cannot parse principal amount at bucket no : {} for llg : {}",
                        bucket_no, principal_key[1]
                    )) * rate_val[bucket_no].parse::<f64>().expect(&format!(
                        "Cannot parse rate at bucket no : {} for llg : {}",
                        bucket_no, principal_key[1]
                    ))),
            );
        }
    } else {
        new_spread = spread
            * principal_val[0].parse::<f64>().expect(&format!(
                "Cannot parse principal amount at bucket no : 0 for llg : {}",
                principal_key[1]
            ));
        for bucket_no in 0..len {
            new_prin_value.push(principal_val[bucket_no].parse::<f64>().expect(&format!(
                "Cannot parse principal amount at bucket no : {} for llg : {}",
                bucket_no, principal_key[1]
            )));
            new_rate_value.push(
                principal_val[bucket_no].parse::<f64>().expect(&format!(
                    "Cannot parse principal amount at bucket no : {} for llg : {}",
                    bucket_no, principal_key[1]
                )) * rate_val[bucket_no].parse::<f64>().expect(&format!(
                    "Cannot parse rate at bucket no : {} for llg : {}",
                    bucket_no, principal_key[1]
                )),
            );
        }
    }
    let bucket_value = get_bucket_value(new_prin_value, new_rate_value, new_spread);
    map.insert(key, bucket_value);
}

pub fn organize_smry(map: &mut HashMap<LLGKey, SummaryValue>, smry_line: &str) {
    let header_size = 9;
    let mut smry_key: Vec<String> = smry_line.split('|').map(String::from).collect();
    let smry_val = smry_key.split_off(header_size);
    let key = get_key(smry_key.clone());
    let mut spread = smry_key[6]
        .parse::<f64>()
        .expect(&format!("Cannot parse spread for llg : {}", smry_key[1],));
    let mut new_prin_value: f64 = 0.0;
    let mut new_rate_value: f64 = 0.0;
    let mut new_spread: f64 = 0.0;
    if map.contains_key(&key) {
        let existing_value = map.get(&key).unwrap().clone();
        let existing_prin_value = existing_value.principal_smry;
        let existing_rate_value = existing_value.rate_smry;
        let existing_spread = existing_value.spread;
        new_prin_value = existing_prin_value
            + smry_val[0].parse::<f64>().expect(&format!(
                "Cannot parse principal amount for llg : {}",
                smry_key[1]
            ));
        new_spread = existing_spread
            + (spread
                * smry_val[0].parse::<f64>().expect(&format!(
                    "Cannot parse principal amount for llg : {}",
                    smry_key[1]
                )));
        new_rate_value = existing_rate_value
            + (smry_val[0].parse::<f64>().expect(&format!(
                "Cannot parse principal amount for llg : {}",
                smry_key[1]
            )) * smry_val[1]
                .parse::<f64>()
                .expect(&format!("Cannot parse rate for llg : {}", smry_val[1])));
    } else {
        new_prin_value = smry_val[0].parse::<f64>().expect(&format!(
            "Cannot parse principal amount for llg : {}",
            smry_key[1]
        ));
        new_spread = spread
            * smry_val[0].parse::<f64>().expect(&format!(
                "Cannot parse principal amount for llg : {}",
                smry_key[1]
            ));
        new_rate_value = smry_val[0].parse::<f64>().expect(&format!(
            "Cannot parse principal amount for llg : {}",
            smry_key[1]
        )) * smry_val[1]
            .parse::<f64>()
            .expect(&format!("Cannot parse rate for llg : {}", smry_val[1]));
    }
    let summary_value = get_smry_value(new_prin_value, new_rate_value, new_spread);
    map.insert(key, summary_value);
}

fn get_key(key_values: Vec<String>) -> LLGKey {
    LLGKey {
        as_on: key_values[0].to_string(),
        llg_id: key_values[1].to_string(),
        ccy: key_values[2].to_string(),
        ex_rt: key_values[3].to_string(),
        bm_id: key_values[4].to_string(),
        tenor: key_values[5].to_string(),
        rep_dt: key_values[7].to_string(),
        rep_freq: key_values[8].to_string(),
    }
}

fn get_smry_value(principal: f64, rate: f64, spread: f64) -> SummaryValue {
    SummaryValue {
        spread: spread,
        principal_smry: principal,
        rate_smry: rate,
    }
}

fn get_bucket_value(principal: Vec<f64>, rate: Vec<f64>, spread: f64) -> BucketValue {
    BucketValue {
        spread: spread,
        principal_vec: principal,
        rate_vec: rate,
    }
}
