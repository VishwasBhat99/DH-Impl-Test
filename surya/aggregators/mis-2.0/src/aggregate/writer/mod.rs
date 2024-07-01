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
        write!(
            output_file,
            "{}|{}|{}|{:.3}|{:.3}|{:.3}|{:.3}|{:.3}|{:.3}|{:.3}|{:.3}|{:.3}|{:.3}|{:.3}|{:.3}|{}\n",
            config_params.report_id(),
            config_params.as_on_date().format("%d-%m-%Y"),
            aggr_key,
            data.tot_prin_amt,
            data.tot_prin_amt_lcy,
            data.min_amount_ccy,
            data.min_amount_hcy,
            data.max_amount_ccy,
            data.max_amount_hcy,
            data.int_accured,
            if data.tot_prin_amt_lcy == 0.0 {
                0.0
            } else {
                data.rt_prin_amt_weighted / data.tot_prin_amt_lcy
            },
            data.min_int_rate,
            data.max_int_rate,
            if data.total_bal == 0.0 {
                0.0
            } else {
                data.avg_days_contract_mat_sum / data.total_bal
            },
            if data.total_bal == 0.0 {
                0.0
            } else {
                data.avg_days_residual_mat_sum / data.total_bal
            },
            data.no_of_depositers,
        )
        .expect("Unable to generate summary file.");
        *op_amt += data.tot_prin_amt_lcy;
    }
}
