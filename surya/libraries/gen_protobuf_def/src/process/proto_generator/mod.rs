mod processing;
mod proto;
mod reader;
mod writer;
use super::*;
pub fn generate_proto_file(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) {
    processing::generate_protofile(config_params,logger,diag_logger);
}
