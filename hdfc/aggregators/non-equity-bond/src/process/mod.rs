use self::io::*;
use calamine::{open_workbook, DataType, Error, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::date_from_timestamp;
use health_report::HealthReport;
use rbdate::NaiveDate;
use required_fields::ReqFields;
use slog::Logger;
use std::collections::HashMap;
use std::default::Default;
use std::io::Write;
mod io;
pub const MAX: f64 = f64::MAX;
mod exchange_rate;

pub fn process_name(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) {
    let exchange_rate_map =
        exchange_rate::read_exchange_rate(config_params.currency_conversion_file_path());

    let mut op_writer = get_writer(config_params.output_file_path());
    let mut workbook: Xlsx<_> = open_workbook(config_params.input_file_path()).expect("Could Not Read Input File");
    let req_fields = ReqFields::new_from_path(config_params.req_file_path());
    let mut isin_found = false;
    let mut date_found = false;

    let mut open_date = MAX;
    let mut open_price = 0.0;

    let mut high_price = -1.0;
    let mut low_price = MAX;

    let mut close_date = -1.0;
    let mut close_price = 0.0;

    let mut order_count = 0;
    let exchange_rate = exchange_rate::get_exch_rate(
        config_params.foreign_currency().to_string(),
        config_params.base_currency(),
        &exchange_rate_map,
    );
    let mut tot_acc_encntrd = 0;
    let mut acc_pro_suc = 0;
    let mut tot_amt = 0.0;

    if let Some(Ok(r)) = workbook.worksheet_range(config_params.input_sheet_name()) {
        for row in r.rows() {
            if &row[req_fields.isin] == "ISIN" {
                continue;
            }
            if &row[req_fields.isin] == &DataType::Empty
                && &row[req_fields.open_date] == &DataType::Empty
            {
                isin_found = false;
                date_found = false;
                if high_price != -1.0 {
                    tot_acc_encntrd +=1;
                    acc_pro_suc += 1;
                    tot_amt +=high_price;
                    write!(
                        op_writer,
                        "{}|{}|{}|{}|{}\n",
                        open_price, high_price, low_price, close_price, order_count
                    );
                    high_price = -1.0;
                    low_price = MAX;
                    open_date = MAX;
                    open_price = 0.0;
                    close_date = -1.0;
                    close_price = 0.0;
                }
                continue;
            }

            if &row[req_fields.isin] != &DataType::Empty
                && DataType::is_float(&row[req_fields.close_date]) == true
            {
                date_found = true;
                order_count += 1;
                let issue_date =
                    rbdate::datevalue_to_naive_date(&row[req_fields.issue_date].to_string());

                let mat_date =
                    rbdate::datevalue_to_naive_date(&row[req_fields.mat_date].to_string());
                write!(
                    op_writer,
                    "{}|{}|{}|{}|{}|{}|{}|{}|",
                    config_params.as_on_date().format("%d-%m-%Y"),
                    row[req_fields.isin],
                    row[req_fields.isin_type],
                    (row[req_fields.face_amt]
                        .to_string()
                        .parse::<f64>()
                        .unwrap_or(0.0)
                        * exchange_rate),
                    issue_date.expect("Could Not Read Issue Date").format("%d-%m-%Y"),
                    (row[req_fields.outsdng_amt]
                        .to_string()
                        .parse::<f64>()
                        .unwrap_or(0.0)
                        * exchange_rate),
                    mat_date.expect("Could Not Read Maturity Date").format("%d-%m-%Y"),
                    row[req_fields.coupons]
                )
                .expect("Unable to generate summary file.");
            }

            if date_found == true {
                let temp_open_date = row[req_fields.open_date]
                    .to_string()
                    .parse::<f64>()
                    .unwrap_or(0.0);
                let temp_high_price = row[req_fields.high_price]
                    .to_string()
                    .parse::<f64>()
                    .unwrap_or(0.0);
                let temp_open_price = row[req_fields.open_price]
                    .to_string()
                    .parse::<f64>()
                    .unwrap_or(0.0);
                let temp_close_date = row[req_fields.close_date]
                    .to_string()
                    .parse::<f64>()
                    .unwrap_or(0.0);
                let temp_close_price = row[req_fields.close_price]
                    .to_string()
                    .parse::<f64>()
                    .unwrap_or(0.0);
                let temp_low_price = row[req_fields.low_price]
                    .to_string()
                    .parse::<f64>()
                    .unwrap_or(0.0);
                if temp_open_date < open_date {
                    open_date = temp_open_date;
                    open_price = temp_open_price;
                }

                if &temp_high_price >= &high_price {
                    high_price = temp_high_price;
                }

                if &temp_low_price <= &low_price {
                    low_price = temp_low_price;
                }

                if temp_close_date > close_date {
                    close_date = temp_close_date;
                    close_price = temp_close_price;
                }
            }
        }
        tot_acc_encntrd +=1;
        acc_pro_suc += 1;
        tot_amt +=high_price;
        write!(
            op_writer,
            "{}|{}|{}|{}|{}\n",
            open_price, high_price, low_price, close_price, order_count
        );
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
