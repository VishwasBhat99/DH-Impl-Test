use self::derive_fields::*;
use self::io::*;
use self::structs::{holdings::*, sec_deal_data::*};
use calamine::{open_workbook_auto, Reader};
use configuration_parameters::ConfigurationParameters;
use macros;
use slog::Logger;
mod derive_fields;
mod io;
mod structs;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let mut op_line_td: String = String::new();
    let mut input_reader = read_file(config_params.input_file_path());
    let mut sec_data_reader = read_sec_file(config_params.sec_deal_data_file());
    let mut sec_data_map: SecDealMap = SecDealMap::new();
    for (line_num, lines) in sec_data_reader.deserialize().enumerate() {
        let ridf_input: SecDealData =
            extract_lines(line_num, lines, config_params.sec_deal_data_file(), logger);
        get_sec_deal_data(&mut sec_data_map, ridf_input);
    }
    for (line_num, lines) in input_reader.deserialize().enumerate().skip(1) {
        let input_data: InputData =
            extract_lines(line_num, lines, config_params.input_file_path(), logger);

        op_line_td.push_str(&get_op_line(input_data, &mut sec_data_map));
    }
    let mut out_file = create_file(config_params.output_file_path());
    output_writer(&mut out_file, op_line_td);
}
