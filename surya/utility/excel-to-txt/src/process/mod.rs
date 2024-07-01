use self::io::*;
use configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod io;
use calamine::{open_workbook_auto, Reader, Sheets};
use health_report::HealthReport;
use rbdate::{datevalue_to_naive_date, DateParser};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub fn excel_to_txt(
    config_params: &ConfigurationParameters,
    _logger: &Logger,
    _diag_logger: &Logger,
) {
    let input_path = PathBuf::from(config_params.input_file());
    match input_path.extension().and_then(|file| file.to_str()) {
        Some("xlsx") | Some("xls") | Some("xlsb") | Some("xlsm") | Some("xltx") | Some("xlt")
        | Some("ods") => (),
        _ => panic!("Expecting an excel file"),
    }
    let mut input_xl_file = match open_workbook_auto(&input_path) {
        Ok(file) => file,
        Err(err) => {
            panic!("Error reading file: {:?} due to: {}", input_path, err);
        }
    };
    let sheet_name = config_params.sheet_name();
    let op_path = format!("{}{}", &config_params.output_file_path(), ".txt");
    let date_parser = rbdate::DateParser::new("%d-%m-%Y".to_string(), false);
    let def_date = date_parser
        .parse_opt(&config_params.default_date())
        .unwrap_or(*config_params.as_on_date())
        .format("%d-%m-%Y")
        .to_string();
    let mut op_writer = get_writer(&op_path);
    let mut tot_acc_encntrd = 0;
    let mut acc_pro_suc = 0;
    if *config_params.append_required() {
        for sheet_name in input_xl_file.sheet_names() {
            let mut input_xl_file_1: calamine::Sheets = match open_workbook_auto(&input_path) {
                Ok(file) => file,
                Err(err) => {
                    panic!("Error reading file: {:?} due to: {}", input_path, err);
                }
            };
            get_output(
                &mut input_xl_file_1,
                sheet_name,
                &mut tot_acc_encntrd,
                &mut acc_pro_suc,
                &date_parser,
                &def_date,
                &mut op_writer,
                &config_params,
            );
        }
    } else {
        get_output(
            &mut input_xl_file,
            sheet_name,
            &mut tot_acc_encntrd,
            &mut acc_pro_suc,
            &date_parser,
            &def_date,
            &mut op_writer,
            &config_params,
        );
    }
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
pub fn get_output(
    input_xl_file: &mut Sheets,
    sheet_name: &str,
    tot_acc_encntrd: &mut i64,
    acc_pro_suc: &mut i64,
    date_parser: &DateParser,
    def_date: &String,
    op_writer: &mut BufWriter<std::fs::File>,
    config_params: &ConfigurationParameters,
) {
    if let Some(Ok(range)) = input_xl_file.worksheet_range(sheet_name) {
        let len = range.get_size().1;
        for (x, row) in range.rows().enumerate() {
            if config_params.skip_rows()[0] != "" {
                if config_params
                    .skip_rows()
                    .to_vec()
                    .contains(&(x + 1).to_string())
                {
                    continue;
                }
            }
            *tot_acc_encntrd += 1;
            let mut op_string = String::new();
            for index in 0..len {
                if config_params
                    .fields_with_date()
                    .to_vec()
                    .contains(&(index + 1).to_string())
                {
                    let date = datevalue_to_naive_date(&row[index].to_string())
                        .unwrap_or(date_parser.parse(&def_date))
                        .format("%d-%m-%Y");
                    op_string.push_str(&date.to_string());
                } else {
                    op_string.push_str(&row[index].to_string().trim().trim_matches(|p| p == '"').replace("\n"," ").replace("\r", " "));
                }
                if index + 1 != len {
                    op_string.push_str(config_params.field_delimeter())
                }
            }
            op_string.push_str("\n");
            write!(op_writer, "{}", op_string).expect(&format!("the output line can not be written for sheet name:{}",sheet_name));
            *acc_pro_suc += 1;
        }
    }
}
