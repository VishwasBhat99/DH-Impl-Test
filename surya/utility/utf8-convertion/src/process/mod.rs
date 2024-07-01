use configuration_parameters::ConfigurationParameters;
use slog::Logger;
use std::fs;
use std::path::PathBuf;
use sdb_io::buf_file_wrtr;
use std::io::Write;

pub fn convert_txt(
    config_params: &ConfigurationParameters,
    _logger: &Logger,
    _diag_logger: &Logger,
) {
    let input_path = PathBuf::from(config_params.input_file());
    match input_path.extension().and_then(|file| file.to_str()) {
        Some("txt") => (),
        Some("csv") => (),
        _ => panic!("Expecting an txt/csv file"),
    }
    let input_reader =
        fs::read(&config_params.input_file()).unwrap();
    let valid_input = String::from_utf8_lossy(&input_reader);
    let mut output_writer = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file: `{}` : {}",
            config_params.output_file_path(),
            error
        ),
    };
    output_writer.write(valid_input.as_bytes()).unwrap();
}
