use calamine::{open_workbook_auto, Reader};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use rbdate::datevalue_to_naive_date;
use slog::Logger;
use std::convert::TryInto;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;
use xlsxwriter::Workbook;

pub fn excel_to_txt(
    config_params: &ConfigurationParameters,
    _logger: &Logger,
    _diag_logger: &Logger,
) {
    let input_path = PathBuf::from(config_params.input_file());
    match input_path.extension().and_then(|file| file.to_str()) {
        Some("xlsx") => (),
        _ => panic!("Expecting an excel file"),
    }
    let file_ext = get_extension_from_filename(config_params.input_file()).unwrap_or("xlsx");
    let mut input_xl_file = open_workbook_auto(&input_path).unwrap();
    let sheet_name = config_params.sheet_name();
    let op_path = format!(
        "{}_{}.{}",
        &config_params.output_file_path(),
        sheet_name,
        file_ext
    );
    let workbook = Workbook::new(&op_path);
    let mut tot_acc_encntrd = 0;
    let mut acc_pro_suc = 0;
    if let Some(Ok(range)) = input_xl_file.worksheet_range(sheet_name) {
        let mut sheet = workbook
            .add_worksheet(Some(sheet_name))
            .expect("Could not add sheet to excel.");
        for (x, row) in range.rows().enumerate() {
            let mut y = 0;
            let mut data;
            tot_acc_encntrd += 1;
            if config_params.fields_pos_vec()[0] == "" {
                while y < range.get_size().1 {
                    if config_params
                        .date_fields()
                        .to_vec()
                        .contains(&(y + 1).to_string())
                    {
                        if config_params.date_field_separator() == "-" {
                            data = datevalue_to_naive_date(&row[y as usize].to_string())
                                .unwrap_or(*config_params.as_on_date())
                                .format("%d-%m-%Y")
                                .to_string();
                        } else {
                            data = datevalue_to_naive_date(&row[y as usize].to_string())
                                .unwrap_or(*config_params.as_on_date())
                                .format("%d/%m/%Y")
                                .to_string();
                        }
                    } else {
                        data = row[y].to_string();
                    }
                    sheet
                        .write_string(x.try_into().unwrap(), y.try_into().unwrap(), &data, None)
                        .expect("Could not write to output sheet.");
                    y += 1;
                    debug!(
                        _logger,
                        "[row:{:?},column:{:?}] written to [row:{:?},column:{:?}]", x, y, x, y
                    );
                }
            } else {
                while y < range.get_size().1 {
                    for val in config_params.fields_pos_vec().into_iter() {
                        if val != "" && val.chars().next().unwrap().is_alphabetic() {
                            sheet
                                .write_string(
                                    x.try_into().unwrap(),
                                    y.try_into().unwrap(),
                                    &val,
                                    None,
                                )
                                .expect("Could not write to output sheet.");
                            y += 1;
                            debug!(_logger, "String written to [row:{:?},column:{:?}]", x, y);
                            continue;
                        } else if val.contains("_") {
                            let concat_code: Vec<&str> = val.split('_').collect();
                            let mut concat_str: String = "".to_string();
                            for index in concat_code.into_iter() {
                                let index = index.to_string().parse::<u16>().unwrap_or(0) - 1;
                                if index != 65535 {
                                    concat_str.push_str(&row[index as usize].to_string());
                                }
                                concat_str.push('_');
                            }
                            concat_str.pop();
                            sheet
                                .write_string(
                                    x.try_into().unwrap(),
                                    y.try_into().unwrap(),
                                    &concat_str,
                                    None,
                                )
                                .expect("Could not write to output sheet.");
                            y += 1;
                            debug!(
                                _logger,
                                "Concat String written to [row:{:?},column:{:?}]", x, y
                            );
                            continue;
                        } else if val == "" {
                            sheet
                                .write_string(
                                    x.try_into().unwrap(),
                                    y.try_into().unwrap(),
                                    &val,
                                    None,
                                )
                                .expect("Could not write to output sheet.");
                            y += 1;
                            debug!(
                                _logger,
                                "Empty String written to [row:{:?},column:{:?}]", x, y
                            );
                            continue;
                        }
                        let val = val.to_string().parse::<u16>().unwrap() - 1;

                        if config_params
                            .date_fields()
                            .to_vec()
                            .contains(&(val + 1).to_string())
                            && x != 0
                        {
                            if config_params.date_field_separator() == "-" {
                                data = datevalue_to_naive_date(&row[val as usize].to_string())
                                    .unwrap_or(*config_params.as_on_date())
                                    .format("%d-%m-%Y")
                                    .to_string();
                            } else {
                                data = datevalue_to_naive_date(&row[val as usize].to_string())
                                    .unwrap_or(*config_params.as_on_date())
                                    .format("%d/%m/%Y")
                                    .to_string();
                            }
                        } else {
                            data = row[val as usize].to_string();
                        }
                        sheet
                            .write_string(x.try_into().unwrap(), y.try_into().unwrap(), &data, None)
                            .expect("Could not write to output sheet.");
                        y += 1;
                        debug!(
                            _logger,
                            "[row:{:?},column:{:?}] written to [row:{:?},column:{:?}]",
                            x,
                            y,
                            x,
                            val
                        );
                    }
                }
            }
            acc_pro_suc += 1;
        }
    }
    workbook.close().expect("Failed to close workbook.");
    let health_report = HealthReport::new(
        tot_acc_encntrd,
        acc_pro_suc,
        tot_acc_encntrd - acc_pro_suc,
        0.0,
        0.0,
        0,
    );
    health_report.gen_health_rpt(&config_params.output_file_path());
}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}
