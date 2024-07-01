use self::io::*;
use configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod io;
use calamine::{open_workbook, Reader, Xlsx};
use health_report::HealthReport;
use rbdate::DateParser;
use rbdate::{
    datevalue_to_naive_date, get_month_end_date, incr_dt_by_mon_presrv_eom,
    increment_date_by_months, num_days_start_to_end,
};
use sdb_io::new_buf_rdr;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let mut op_writer = get_writer(&config_params.output_file_path());
    let date_parser = DateParser::new("%d-%b-%y".to_string(), true);
    let mut tot_acc_encntrd = 0;
    let mut acc_pro_suc = 0;
    let mut tot_amt = 0.0;
    let mut input_file: Xlsx<_> =
        open_workbook(&config_params.input_file()).expect("Unable to open `Input File`.");
    let sheet1 = input_file
        .sheet_names()
        .first()
        .expect("excel is empty")
        .to_owned();
    if let Some(Ok(reader)) = input_file.worksheet_range(sheet1.as_str()) {
        for row in reader.rows().skip(2) {
            tot_acc_encntrd += 1;
            let val_date = datevalue_to_naive_date(&row[1].to_string())
                .expect("Date is not in correct format");
            let mat_date = datevalue_to_naive_date(&row[2].to_string())
                .expect("Date is not in correct format")
                .format("%d-%m-%Y");

            let no_of_days = num_days_start_to_end(val_date, *config_params.as_on_date());
            let mut acrd_int = 0.0;
            let mut broken_quaterly_int = 0.0;
            if row[7].to_string().replace(" ", "").to_lowercase() != "compoundedquarterly" {
                acrd_int = row[3].to_string().parse::<f64>().unwrap_or(0.0)
                    * (row[6].to_string().parse::<f64>().unwrap_or(0.0) / 100.0)
                    * ((no_of_days as f64 + 1.0) / 365.0);
            } else {
                let as_on = *config_params.as_on_date();
                let mut initial_val = row[3].to_string().parse::<f64>().unwrap_or(0.0);
                let roi = row[6].to_string().parse::<f64>().unwrap_or(0.0);
                let mut quaterly_int = 0.0;
                let month = val_date.to_string()[5..7].parse::<f32>().unwrap_or(0.0);
                let mut end_date;
                if (month <= 3.0) {
                    end_date = get_month_end_date(increment_date_by_months(
                        val_date,
                        (3.0 - month) as u16,
                    ));
                } else if (month > 3.0 && month <= 6.0) {
                    end_date = get_month_end_date(increment_date_by_months(
                        val_date,
                        (6.0 - month) as u16,
                    ));
                } else if (month > 6.0 && month <= 9.0) {
                    end_date = get_month_end_date(increment_date_by_months(
                        val_date,
                        (9.0 - month) as u16,
                    ));
                } else {
                    end_date = get_month_end_date(increment_date_by_months(
                        val_date,
                        (12.0 - month) as u16,
                    ));
                }
                let mut no_of_quaterly_days = num_days_start_to_end(val_date, end_date);
                quaterly_int = initial_val * roi * (no_of_quaterly_days as f64 + 1.0) / 36500.0;
                acrd_int = acrd_int + quaterly_int;
                while incr_dt_by_mon_presrv_eom(end_date, 3).expect("Couldnt find date") < as_on {
                    let prev_end = end_date;
                    end_date = incr_dt_by_mon_presrv_eom(end_date, 3).expect("Couldnt find date");
                    no_of_quaterly_days = num_days_start_to_end(prev_end, end_date);
                    initial_val = initial_val + quaterly_int;
                    quaterly_int = initial_val * roi * (no_of_quaterly_days as f64) / 36500.0;
                    acrd_int = acrd_int + quaterly_int;
                }
                no_of_quaterly_days = num_days_start_to_end(end_date, as_on);
                initial_val = initial_val + quaterly_int;
                broken_quaterly_int = initial_val * roi * (no_of_quaterly_days as f64) / 36500.0;
                acrd_int = acrd_int + broken_quaterly_int;
            }
            acc_pro_suc += 1;
            write!(
                op_writer,
                "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|||{}|{}|{}|||||||\n",
                row[0],
                val_date.format("%d-%m-%Y"),
                mat_date,
                row[3],
                row[3],
                row[6],
                row[7],
                row[8],
                row[10],
                row[5],
                "INR",
                row[9],
                acrd_int,
                broken_quaterly_int,
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
    }
}
