use self::io::*;
use calamine::{open_workbook, DataType, Error, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use macros;
use health_report::HealthReport;
use rbdate::date_from_timestamp;
use rbdate::NaiveDate;
use required_fields::ReqFields;
use slog::Logger;
use std::collections::HashMap;
use std::default::Default;
use std::io::Write;
mod io;

pub fn process_name(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) {
    let mut op_writer = get_writer(config_params.output_file_path());
    let mut workbook_details: Xlsx<_> = open_workbook(config_params.input_cd_details()).unwrap();
    let mut workbook_price: Xlsx<_> = open_workbook(config_params.input_cd_price_details()).unwrap();
    let req_fields = ReqFields::new_from_path(config_params.req_file_path());
    let mut isin: Vec<String> = Vec::new();
    let mut open_price: Vec<String> = Vec::new();
    let mut hign_price: Vec<String> = Vec::new();
    let mut low_price: Vec<String> = Vec::new();
    let mut close_price: Vec<String> = Vec::new();
    let mut found = false;
    let mut order_count = 0;
    let mut tot_acc_encntrd = 0;
    let mut acc_pro_suc = 0;
    let mut tot_amt = 0.0;
    if let Some(Ok(reader)) = workbook_price.worksheet_range(config_params.cd_pricedetails_sheetname()) {
        for row in reader.rows() {
            if &row[req_fields.isin_price].to_string() == "ISIN" {
                found = true;
                continue;
            }
            if found == true {
                let isin_value: String = row[req_fields.isin_price].to_string();
                isin.push(isin_value.trim().to_string());
                open_price.push(row[req_fields.open_price].to_string());
                hign_price.push(row[req_fields.high_price].to_string());
                low_price.push(row[req_fields.low_price].to_string());
                close_price.push(row[req_fields.close_price].to_string());
            }
        }
    }

    if let Some(Ok(reader)) = workbook_details.worksheet_range(config_params.cd_details_sheetname()) {
        for row in reader.rows() {
            let isin_value: String = row[req_fields.isin_detail].to_string();

            if isin.contains(&isin_value.trim().to_string()) {
                let index = isin
                    .iter()
                    .position(|reader| reader.trim().to_string() == isin_value.trim().to_string())
                    .unwrap();
                order_count += 1;
                let issue_date =
                    rbdate::datevalue_to_naive_date(&row[req_fields.issue_date].to_string());
                let mat_date =
                    rbdate::datevalue_to_naive_date(&row[req_fields.mat_date].to_string());
                tot_acc_encntrd += 1;
                acc_pro_suc += 1;
                tot_amt += hign_price[index].parse::<f64>().unwrap_or(0.0);
                write!(
                    op_writer,
                    "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
                    config_params.as_on_date().format("%d-%m-%Y"),
                    isin_value,
                    row[req_fields.isin_type],
                    row[req_fields.face_amt],
                    issue_date.unwrap().format("%d-%m-%Y"),
                    row[req_fields.outstd_amt],
                    mat_date.unwrap().format("%d-%m-%Y"),
                    row[req_fields.coupons],
                    open_price[index],
                    hign_price[index],
                    low_price[index],
                    close_price[index],
                    order_count
                );
            }
        }
    }
    let health_report = HealthReport::new(
        tot_acc_encntrd,
        acc_pro_suc,
        tot_acc_encntrd - acc_pro_suc,
        tot_amt,
        tot_amt,
        0,
    );
    health_report.gen_health_rpt(&config_params.output_file_path());

    let report_string = format!(
        "Accounts encountered: {}\n\
         Accounts proccessed suceessfully: {}\n\
         Accounts failed to process: {}",
        tot_acc_encntrd,
        acc_pro_suc,
        tot_acc_encntrd - acc_pro_suc,
    );
}
