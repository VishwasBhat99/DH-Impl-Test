use crate::macros;
use configuration_parameters::ConfigurationParameters;
use slog::Logger;

mod file_writer;

pub fn data_read_writer(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) {
    let input_data = file_writer::file_reader::file_read(config_params);
    log_info!(
        logger,
        "Read the data from {:?} and {:?}",
        config_params.input_file_path(),
        config_params.rule_file_path()
    );
    file_writer::file_write(input_data, config_params);
    log_info!(
        diag_logger,
        "Written the data {:?}",
        config_params.output_file_path()
    );
}
