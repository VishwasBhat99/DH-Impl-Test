use super::aggr_data::Data;
use super::aggr_key::AggrKey;
use super::ConfigurationParameters;
use sdb_io::buf_file_wrtr;
use std::collections::HashMap;
use std::env;
use std::io::Write;

pub fn write_aggr_smry(
    mut aggr_data: HashMap<AggrKey, Data>,
    op_amt: &mut f64,
    config_params: &ConfigurationParameters,
) {
    let mut output_file = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(create) => create,
        Err(error) => {
            panic!(
                "Could not create output file: `{}` due to {}",
                config_params.output_file_path(),
                error
            );
        }
    };
    for (aggr_key, data) in aggr_data.drain() {
        write!(
            output_file,
            "{}|{:.2}|{:.2}|{}|{:.2}|{:.2}|{:.2}|{:.2}|{:.2}\n",
            aggr_key,
            data.tot_amt,
            if data.tot_amt == 0.0 {
                0.0
            } else {
                data.weighted_int_rate_sum / data.tot_amt
            },
            data.count,
            data.max_amt,
            data.min_amt,
            data.max_int_rate,
            data.min_int_rate,
            if data.count == 0 {
                0.0
            } else {
                data.int_rate_sum / data.count as f64
            }
        )
        .expect("Unable to generate summary file.");
        *op_amt += data.tot_amt;
    }
}
