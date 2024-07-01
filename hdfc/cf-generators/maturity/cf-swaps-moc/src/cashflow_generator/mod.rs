extern crate serde;
use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
mod account_reader;
use self::account_reader::InputAccountReader;
use rbdate::*;
mod io;
use calamine::{open_workbook_auto, Reader, Sheets};
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
mod gen_cashflows;
use cashflow_generator::gen_cashflows::generate_cashflows;
use std::path::PathBuf;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod req_fields;
use cashflow_generator::account_with_cashflows_writer::AccountWithCashflowsWriter;
mod structure;
use self::req_fields::ReqiredFields;
use self::structure::FieldStructure;

pub fn generate(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut tot_input_acc_encntrd: i64 = 0;
    let mut tot_acc_skippd: i64 = 0;
    let mut tot_succ_rec: i64 = 0;
    let as_on_date = *config_param.as_on_date();

    let required_fields = ReqiredFields::new_from_path(config_param.req_fields_file());

    let input_path_rec = PathBuf::from(config_param.input_rec_file_path());
    match input_path_rec.extension().and_then(|file| file.to_str()) {
        Some("xlsx") | Some("xls") | Some("xlsb") | Some("xlsm") | Some("xltx") | Some("xlt")
        | Some("ods") => (),
        _ => panic!("Expecting an excel for REC file."),
    }

    let input_path_pay = PathBuf::from(config_param.input_pay_file_path());
    match input_path_pay.extension().and_then(|file| file.to_str()) {
        Some("xlsx") | Some("xls") | Some("xlsb") | Some("xlsm") | Some("xltx") | Some("xlt")
        | Some("ods") => (),
        _ => panic!("Expecting an excel for PAY file."),
    }

    let mut cf_date_bucket: HashMap<String, NaiveDate> = HashMap::new();

    let bkt_data_files = match new_buf_rdr(config_param.bucket_config_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found npa data file: `{}` on location `{}` : {}.",
            config_param.bucket_config_file(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    for (line_num, lines) in bkt_data_files.lines().enumerate() {
        let bkt_data_line = match lines {
            Ok(bkt_data_line) => bkt_data_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.bucket_config_file(),
                line_num + 1,
                error
            ),
        };
        let bkt_data_fields = bkt_data_line.split("|").collect::<Vec<&str>>();
        if bkt_data_fields.len() == 2 {
            cf_date_bucket.insert(
                (bkt_data_fields[0].parse::<i64>().unwrap_or(0) + 1).to_string(),
                incr_dt_by_days(as_on_date, bkt_data_fields[1].parse::<i64>().unwrap_or(0)),
            );
        } else {
            continue;
        }
    }

    // rec file
    data_generation(
        &config_param,
        required_fields.clone(),
        input_path_rec,
        &cf_date_bucket.clone(),
        log,
        "rec",
        &mut tot_input_acc_encntrd,
        &mut tot_succ_rec,
    );

    // pay file
    data_generation(
        &config_param,
        required_fields.clone(),
        input_path_pay,
        &cf_date_bucket,
        log,
        "pay",
        &mut tot_input_acc_encntrd,
        &mut tot_succ_rec,
    );

    let health_report = HealthReport::new(
        tot_input_acc_encntrd,
        tot_succ_rec,
        tot_input_acc_encntrd - tot_acc_skippd,
        0.0,
        0.0,
        0,
    );
    health_report.gen_health_rpt(config_param.output_file_path());
}

pub fn excel_data(
    input_xl_file: &mut Sheets,
    sheet_name: &str,
    start_col: usize,
    end_col: usize,
    config_params: &ConfigurationParameters,
    mut tot_input_acc_encntrd: &mut i64,
    mut tot_succ_rec: &mut i64,
) -> HashMap<String, FieldStructure> {
    let mut data_map: HashMap<String, FieldStructure> = HashMap::new();

    if let Some(Ok(range)) = input_xl_file.worksheet_range(&sheet_name) {
        let mut mapper_track = String::new();
        for (x, row) in range.rows().enumerate() {
            *tot_input_acc_encntrd += 1;
            if config_params.skip_rows()[0] != "" {
                if config_params
                    .skip_rows()
                    .to_vec()
                    .contains(&(x + 1).to_string())
                {
                    if row[0].to_string().trim() == "Notionals"
                        || row[0].to_string().trim() == "Modified Duration"
                        || row[0].to_string().trim() == "Yields"
                        || row[0].to_string().trim() == "Coupons"
                    {
                        mapper_track = row[0].to_string().trim().to_owned();
                    }
                    continue;
                }
            }
            if x >= start_col && x <= end_col {
                *tot_succ_rec += 1;
                for index in 2..12 {
                    let data = data_map
                        .entry(row[1].to_string() + "-" + &index.to_string())
                        .or_insert(FieldStructure {
                            notional: row[index].to_string().parse::<f64>().unwrap_or(0.0),
                            coupons: 0.0,
                            yields: 0.0,
                            mod_duration: 0.0,
                        });

                    if mapper_track == "Coupons" {
                        data.coupons = row[index].to_string().parse::<f64>().unwrap_or(0.0);
                    } else if mapper_track == "Yields" {
                        data.yields = row[index].to_string().parse::<f64>().unwrap_or(0.0);
                    } else if mapper_track == "Modified Duration" {
                        data.mod_duration = row[index].to_string().parse::<f64>().unwrap_or(0.0);
                    }
                }
            }
        }
    }
    data_map
}

fn create_io_workers(
    input_path: &str,
    output_path: &str,
    log: &Logger,
) -> (InputAccountReader, AccountWithCashflowsWriter) {
    let reader = InputAccountReader::new(input_path, log);
    let writer = AccountWithCashflowsWriter::new(output_path, log);

    (reader, writer)
}

pub fn data_generation(
    config_param: &ConfigurationParameters,
    required_fields: ReqiredFields,
    input_path: PathBuf,
    cf_date_bucket: &HashMap<String, NaiveDate>,
    log: &Logger,
    file_name: &str,
    mut tot_input_acc_encntrd: &mut i64,
    mut tot_succ_rec: &mut i64,
) {
    let mut input_file = match open_workbook_auto(&input_path) {
        Ok(file) => file,
        Err(err) => {
            panic!("Error reading file: {:?} due to: {}", input_path, err);
        }
    };
    let sheet_name = required_fields.sheet_name.split(",").collect::<Vec<&str>>();
    let currency = required_fields.currency.split(",").collect::<Vec<&str>>();
    for index in 0..sheet_name.len() {
        let op_path_bank = config_param.output_file_path().to_string()
            + "-"
            + &currency[index]
            + "-"
            + file_name
            + "-book";
        let (reader, mut writer_bank) =
            create_io_workers(config_param.input_rec_file_path(), &op_path_bank, log);
        let mut input_xl_file_1: calamine::Sheets = match open_workbook_auto(&input_path) {
            Ok(file) => file,
            Err(err) => {
                panic!("Error reading file: {:?} due to: {}", input_path, err);
            }
        };
        let bank_rows = required_fields
            .input_rec_banking_row
            .split("-")
            .collect::<Vec<&str>>();
        let bank_start_row = bank_rows[0].parse::<usize>().unwrap_or(1) - 1;
        let bank_end_row = bank_rows[1].parse::<usize>().unwrap_or(1) - 1;

        let bank_data_map = excel_data(
            &mut input_xl_file_1,
            &sheet_name[index],
            bank_start_row,
            bank_end_row,
            &config_param,
            &mut tot_input_acc_encntrd,
            &mut tot_succ_rec,
        );

        generate_cashflows(
            &config_param,
            cf_date_bucket.clone(),
            bank_data_map,
            &mut writer_bank,
            &currency[index],
        );

        let op_path_trade = config_param.output_file_path().to_string()
            + "-"
            + &currency[index]
            + "-"
            + file_name
            + "-trade";
        let (reader, mut writer_trade) =
            create_io_workers(config_param.input_rec_file_path(), &op_path_trade, log);

        let trade_row = required_fields
            .input_rec_trading_row
            .split("-")
            .collect::<Vec<&str>>();
        let trade_start_row = trade_row[0].parse::<usize>().unwrap_or(1) - 1;
        let trade_end_row = trade_row[1].parse::<usize>().unwrap_or(1) - 1;

        let trade_data_map = excel_data(
            &mut input_xl_file_1,
            &sheet_name[index],
            trade_start_row,
            trade_end_row,
            &config_param,
            &mut tot_input_acc_encntrd,
            &mut tot_succ_rec,
        );
        generate_cashflows(
            &config_param,
            cf_date_bucket.clone(),
            trade_data_map,
            &mut writer_trade,
            &currency[index],
        );
        writer_bank.close();
        writer_trade.close();
    }
}
