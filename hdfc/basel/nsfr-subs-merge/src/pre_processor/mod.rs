extern crate csv;
extern crate serde;
use calamine::DataType;
use calamine::{open_workbook_auto, Reader};
use configuration_parameters::ConfigurationParameters;
use indexmap::IndexMap;
use slog::Logger;
use xlsxwriter::Workbook;
mod config;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let output_sheet_name = config_param.output_sheet_name();
    let op_path = format!("{}.{}", &config_param.output_file_path(), "xlsx");
    let workbook = Workbook::new(&op_path).expect("Can not create xlsx file");
    let mut worksheet = workbook
        .add_worksheet(Some(output_sheet_name))
        .expect("Can not add the sheet to output excel");

    let files_config = config::get_files(config_param.config_file_path());
    let mut global_map: IndexMap<(i64, String), Vec<f64>> = IndexMap::new();
    for config_fields in files_config.files {
        let input_file_path = config_fields.input_file_path;
        let input_sheet_name = config_fields.input_sheet_name;
        let key_num = config_fields.key_num;
        let col_to_skip = config_fields.col_to_skip;
        let row_to_skip = config_fields.row_to_skip;
        let mut input_file_path =
            open_workbook_auto(input_file_path).expect("Unable to open the input xlsx file.");
        println!(
            "Sheets present in Input-File: `{:?}`",
            input_file_path.sheet_names()
        );
        if !input_file_path.sheet_names().contains(&input_sheet_name) {
            panic!(
                "Sheet passed: `{}` not present in Input-File: `{}`",
                input_sheet_name, input_sheet_name
            );
        }
        println!("Reading Sheet: `{}` from Input-File", input_sheet_name,);

        if let Some(Ok(input_file_reader)) = input_file_path.worksheet_range(&input_sheet_name) {
            for (row_no, row) in input_file_reader.rows().enumerate() {
                if row_to_skip.contains(&row_no) {
                    continue; // Skip the row
                }
                let len = row.len() - col_to_skip;
                let mut amount_vec: Vec<f64> = vec![0.0; len];
                let slno = get_str_from_xlsx(row, 0).parse::<i64>().unwrap_or(0);
                let key_value: String = get_str_from_xlsx(row, key_num as usize);
                for (col_no, cell) in row.iter().enumerate().skip(col_to_skip) {
                    let balance = get_str_from_xlsx(row, col_no);
                    let amt = balance.parse::<f64>().unwrap_or(0.0);
                    amount_vec[col_no - col_to_skip] = amt;
                }
                global_map
                    .entry((slno, key_value))
                    .and_modify(|prev_amt_vec| {
                        for (prev, val) in prev_amt_vec.iter_mut().zip(&amount_vec) {
                            *prev += val;
                        }
                    })
                    .or_insert(amount_vec.clone());
            }
        }
    }
    let mut row = 0;
    let col = 0;
    for (key, values) in global_map.iter() {
        let sl_no = (row + 1) as f64;
        worksheet
            .write_number(row, col, sl_no, None)
            .expect("Could not write serial number to output sheet.");

        worksheet
            .write_string(row, col + 1, &key.1, None)
            .expect("Could not write key to output sheet.");

        for (i, value) in values.iter().enumerate() {
            worksheet
                .write_number(row, col + i as u16 + 2, *value, None)
                .expect("Could not write value to output sheet.");
        }

        row += 1;
    }
    workbook.close().expect("Failed to close workbook.");
}
pub fn get_str_from_xlsx(data: &[DataType], index: usize) -> String {
    data.get(index)
        .unwrap_or_else(|| {
            panic!(
                "Could not get data at column-no: `{}` for row: `{:?}`",
                index + 1,
                data
            )
        })
        .to_string()
        .replace("\n", " ")
        .trim()
        .to_string()
}
