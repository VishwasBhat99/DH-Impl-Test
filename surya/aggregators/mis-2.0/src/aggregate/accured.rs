use super::aggr_data::Data;
use super::aggr_key::AggrKey;
use aggregate::Logger;
use macros;
use std::collections::HashMap;
pub fn add_int_accured(
    aggr_data: &mut HashMap<AggrKey, Data>,
    numslab_data: Vec<Vec<String>>,
    logger: &Logger,
) {
    let mut cal_map: HashMap<AggrKey, Vec<f64>> = HashMap::new();
    let mut bal_bucket_vec: Vec<f64> = Vec::new();
    let mut rate_vec: Vec<f64> = Vec::new();
    for (line, data) in numslab_data.iter().enumerate() {
        let aggr_key = AggrKey {
            dim1: data[0].to_string(),
            dim2: data[1].to_string(),
            dim3: data[2].to_string(),
            ccy: data[3].to_string(),
        };
        rate_vec.push(data[6].parse().unwrap_or(0.0) * 100.0);
        bal_bucket_vec.push(data[5].parse().unwrap_or(0.0));
        let mut default_bucket_vec = vec![0.0; numslab_data.len()];
        let mut last_bucket_total = 0.0;
        if line > 0 {
            for i in 0..line {
                default_bucket_vec[i] = aggr_data.get(&aggr_key).unwrap().no_of_depositers as f64
                    * bal_bucket_vec[i]
                    - last_bucket_total;
                last_bucket_total += default_bucket_vec[i];
            }
        }
        default_bucket_vec[line] =
            aggr_data.get(&aggr_key).unwrap().tot_prin_amt_lcy - last_bucket_total;
        cal_map.insert(aggr_key, default_bucket_vec);
    }
    let mut final_output_accure_int: HashMap<AggrKey, f64> = HashMap::new();

    for (key, data) in cal_map {
        let mut total_val = 0.0;
        for (buk_no, val) in data.iter().enumerate() {
            total_val += *val * rate_vec[buk_no] / 100.0;
        }
        final_output_accure_int.insert(key, total_val);
    }

    for (key, output_accured_int) in final_output_accure_int {
        match aggr_data.get_mut(&key) {
            Some(x) => x.int_accured = output_accured_int,
            None => log_debug!(logger, "Unable to get key: {} in aggr_data", key),
        }
    }
}
