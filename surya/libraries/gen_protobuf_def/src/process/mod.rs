mod message_generator;
mod proto_generator;

use super::*;
use configuration_parameters::ConfigurationParameters;
use slog::Logger;

pub fn gen_proto(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) {
    proto_generator::generate_proto_file(config_params, logger, diag_logger);
    message_generator::generate_message(config_params, logger, diag_logger);
}
