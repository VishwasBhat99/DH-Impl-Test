use self::io::*;
use configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod io;
use calamine::{open_workbook_auto, Reader};
use macros;
use std::io::Write;

pub fn process(config_params: &ConfigurationParameters, log: &Logger, _diagnostics: &Logger) {
    let mut input_file = open_workbook_auto(config_params.input_file())
        .expect("Unable to open the input xlsx file.");
    let sheet_name = config_params.input_sheet_name();

    log_info!(
        log,
        "Sheets present in Excel-File: `{:?}`",
        input_file.sheet_names()
    );

    if !input_file.sheet_names().contains(&sheet_name.to_string()) {
        panic!(
            "Sheet passed: `{}` not present in Excel-File: `{}`",
            sheet_name,
            config_params.input_file()
        );
    }

    log_info!(log, "Reading Sheet: `{}` from Excel-File", sheet_name);

    let mut op_writer = get_writer(config_params.output_file());
    if let Some(Ok(range)) = input_file.worksheet_range(sheet_name) {
        //Row and column indexing starts from 0. Hence subtract 1 from each value.
        let row = config_params.row_num() - 1;
        let col = config_params.col_num() - 1;
        if let Some(cell) = range.get_value((row as u32, col as u32)) {
            write!(op_writer, "{}", cell).expect("Unable to write to output file");
        } else {
            log_error!(log, "Cell not found at row:{}, col:{}", row, col);
        }
    }
}
