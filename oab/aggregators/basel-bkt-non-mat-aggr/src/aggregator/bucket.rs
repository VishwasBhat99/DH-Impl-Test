use crate::aggregator::structs::AggregateData;
use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use slog::Logger;

pub fn write_bucket(
    logger: &Logger,
    data: AggregateData,
    config_params: &ConfigurationParameters,
    mut op_line: String,
) -> String {
    if let 1..=203 = config_params.bucket_id() {
        for bkt_id in 1..=203 {
            if config_params.bucket_id() == bkt_id {
                op_line.push_str(&data.total_amt.to_string());
                if bkt_id != 203 {
                    op_line.push('|');
                }
            } else {
                op_line.push_str(&0.00.to_string());
                if bkt_id != 203 {
                    op_line.push('|');
                }
            }
        }
    } else {
        log_error!(logger, "Invalid Bucket-ID `{}`", config_params.bucket_id());
        for bkt_id in 1..=203 {
            op_line.push_str(&0.00.to_string());
                if bkt_id != 203 {
                    op_line.push('|');
                }
        }
    }

    op_line
}
