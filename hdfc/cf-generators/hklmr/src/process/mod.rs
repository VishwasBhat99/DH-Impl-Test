use calamine::{open_workbook_auto, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;

use slog::Logger;
mod io;
use self::io::*;
mod struct_data;
use self::struct_data::Data;
use sdb_io::buf_file_wrtr;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let denomination: f64 = config_params.denomination().parse().unwrap();
    let mut lmr_field = 6;
    let mut curr_index;
    let mut amt;
    let mut rows_to_skip = 0;
    let currency_vec = vec!["HKD", "USD", "RUB", "EUR"];
    let mut writer = buf_file_wrtr(config_params.output_file_path(), None)
        .expect("Unable to create `.cf` file.");
    let mut workbook = open_workbook_auto(config_params.input_file_path())
        .expect("Error while opening the input file");
    if let Some(Ok(reader)) = workbook.worksheet_range(config_params.input_sheet_name()) {
        for rows in reader.rows().skip(5) {
            curr_index = 0;
            for i in 5..=8 {
                if rows_to_skip == 1 {
                    rows_to_skip = 0;
                    break;
                } else if rows[i].to_string() == "Breakdown of weighted amount by currencies" {
                    rows_to_skip = 1;
                    break;
                } else if rows[i].is_empty() {
                    amt = 0.0;
                } else {
                    amt = rows[i]
                        .to_string()
                        .parse()
                        .expect("Error while parsing the amount");
                }

                let mut data = Data::new();

                data.lmr_field = lmr_field.to_string();
                data.ccy = currency_vec[curr_index].to_string();
                data.amt = amt * denomination;
                curr_index = curr_index + 1;
                write_file(logger, &mut writer, data);
            }
            lmr_field = lmr_field + 1;
        }
    }
}
