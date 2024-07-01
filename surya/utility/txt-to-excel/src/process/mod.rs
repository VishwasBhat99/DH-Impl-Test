use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use slog::Logger;
use std::convert::TryInto;
use std::fs;
use std::path::PathBuf;
use xlsxwriter::Workbook;

pub fn excel_to_txt(
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
    let output_sheet_name = config_params.output_sheet_name();
    let workbook = Workbook::new(&format!("{}.{}", &config_params.output_file_path(), "xlsx"));
    let mut tot_acc_encntrd = 0;
    let mut acc_pro_suc = 0;
    let input_reader =
        fs::read_to_string(&config_params.input_file()).expect("Failed to read input file!");
    let mut skip = 0;
    if config_params.skip_header() {
        skip = 1;
    }
    let date_parser_dby = rbdate::DateParser::new("%d-%b-%Y".to_string(), false);
    let date_parser_dmy = rbdate::DateParser::new("%d-%m-%Y".to_string(), false);
    let mut sheet = workbook
        .add_worksheet(Some(output_sheet_name))
        .expect("Could not add sheet to excel.");
    for (x, line) in input_reader.lines().skip(skip).enumerate() {
        let input_fields = line
            .split(config_params.field_separator())
            .collect::<Vec<&str>>();
        let mut data;
        tot_acc_encntrd += 1;
        for (y, _item) in input_fields.iter().enumerate() {
            data = input_fields[y].to_string();
            if config_params
                .date_fields()
                .to_vec()
                .contains(&(y + 1).to_string())
            {
                if config_params.date_formats()[0] == "%d-%m-%Y"
                    && config_params.date_formats()[1] == "%d-%m-%Y"
                {
                    data = date_parser_dmy
                        .parse_opt(&data.to_string())
                        .unwrap_or(*config_params.as_on_date())
                        .format("%d-%m-%Y")
                        .to_string();
                } else if config_params.date_formats()[0] == "%d-%m-%Y"
                    && config_params.date_formats()[1] == "%d-%b-%Y"
                {
                    data = date_parser_dmy
                        .parse_opt(&data.to_string())
                        .unwrap_or(*config_params.as_on_date())
                        .format("%d-%b-%Y")
                        .to_string();
                } else if config_params.date_formats()[0] == "%d-%b-%Y"
                    && config_params.date_formats()[1] == "%d-%m-%Y"
                {
                    data = date_parser_dby
                        .parse_opt(&data.to_string())
                        .unwrap_or(*config_params.as_on_date())
                        .format("%d-%m-%Y")
                        .to_string();
                } else if config_params.date_formats()[0] == "%d-%b-%Y"
                    && config_params.date_formats()[1] == "%d-%b-%Y"
                {
                    data = date_parser_dby
                        .parse_opt(&data.to_string())
                        .unwrap_or(*config_params.as_on_date())
                        .format("%d-%b-%Y")
                        .to_string();
                } else {
                    warn!(
                        _diag_logger,
                        "Invalid date formats: `{:?}`",
                        config_params.date_formats()
                    );
                }
                if config_params.is_perf_diagnostics_enabled() {
                    debug!(
                        _logger,
                        "date from input: {:?} is converted to {:?}",
                        input_fields[y].to_string(),
                        data,
                    );
                }
            }
            sheet
                .write_string(
                    x.try_into().expect("Error writing data into a row"),
                    y.try_into().expect("Error writing data into a column"),
                    &data.trim().replace('\u{0}',""),
                    None,
                )
                .expect("Could not write to output sheet.");
            if config_params.is_perf_diagnostics_enabled() {
                debug!(
                    _logger,
                    "[line:{:?},column:{:?}] written to [row:{:?},column:{:?}]: {:?}",
                    x + 1,
                    y + 1,
                    x + 1,
                    y + 1,
                    data
                );
            }
        }
        acc_pro_suc += 1;
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
    health_report.gen_health_rpt(config_params.output_file_path());
}
