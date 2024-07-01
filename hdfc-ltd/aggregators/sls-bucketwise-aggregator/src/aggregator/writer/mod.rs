use super::aggr_data::AggrData;
use super::aggr_key::AggrKey;
use super::ConfigurationParameters;
use sdb_io::buf_file_wrtr;
use std::collections::HashMap;
use std::env;
use std::io::Write;

pub fn write_aggr_smry(
    mut aggr_data: HashMap<AggrKey, AggrData>,
    config_params: &ConfigurationParameters,
) {
    let mut output_file = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(create) => create,
        Err(error) => {
            panic!(
                "Could not create output file: `{}` on location `{}`: {}.",
                config_params.output_file_path(),
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    for (aggr_key, data) in aggr_data.drain() {
        write!(output_file, "{}|{}\n", aggr_key, data).expect("Unable to generate summary file.");
    }
}
