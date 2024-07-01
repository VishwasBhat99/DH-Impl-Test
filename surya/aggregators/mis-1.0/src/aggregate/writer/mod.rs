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
        if !config_params.write_aggr_vals(){
            write!(
            output_file,
            "{}|{}|{}\n",
            config_params.country_code(),
            config_params.as_on_date().format("%d-%m-%Y"),
            aggr_key
        )
        .expect("Unable to generate summary file.");
        }else{
            write!(
                output_file,
                "{}|{}|{}|{:.2}|{:.2}|{:.2}\n",
                config_params.country_code(),
                config_params.as_on_date().format("%d-%m-%Y"),
                aggr_key,
                data.tot_prin_amt,
                data.tot_prin_amt_lcy,
                data.rt_prin_amt_weighted,
            )
            .expect("Unable to generate summary file.");
        }
        *op_amt += data.tot_prin_amt_lcy;
    }
}