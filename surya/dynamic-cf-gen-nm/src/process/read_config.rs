use configuration_parameters::ConfigurationParameters;
use macros;
use slog::Logger;
use std::io::prelude::*;

pub fn read_config_files(
    config_params: &ConfigurationParameters,
    _logger: &Logger,
    diag_logger: &Logger,
) -> Vec<f64> {
    // Read Amount Disbursement by Day File into Vec
    let dis_day_reader = sdb_io::new_buf_rdr(config_params.disbursement_by_day_file_path())
        .expect("Cannot open Disbursement by Day File!");
    let mut dis_day_map: Vec<f64> = Vec::with_capacity(31);
    for (_, lines) in dis_day_reader.lines().enumerate() {
        let line = lines.expect("Cannot read data from amt disbursement by day file!");
        let line_info: Vec<&str> = line.split('|').collect();
        if line_info[0]
            == config_params
                .as_on_date()
                .succ()
                .format("%d-%m-%Y")
                .to_string()
        {
            for idx in 1..32 {
                let weightage: f64 = line_info[idx].parse().unwrap_or(0.0);
                dis_day_map.push(weightage);
            }
        } else {
            continue;
        }
    }
    log_debug!(diag_logger, "Day Disbursement Mapping{:?}", dis_day_map);

    dis_day_map
}
