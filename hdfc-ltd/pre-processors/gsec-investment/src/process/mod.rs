use self::io::*;
use configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod io;
use health_report::HealthReport;
use rbdate::DateParser;
use sdb_io::new_buf_rdr;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;
use std::process::exit;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let mut op_writer = get_writer(&config_params.output_file_path());
    let date_parser = DateParser::new("%d-%b-%Y".to_string(), false);
    let mut tot_acc_encntrd = 0;
    let mut acc_pro_suc = 0;
    let mut tot_amt = 0.0;

    let input = File::open(&config_params.input_file()).expect("Could Not Read File");
    let input_reader = BufReader::new(input);

    for (index, line) in input_reader.lines().enumerate().skip(1) {
        tot_acc_encntrd += 1;
        acc_pro_suc += 1;
        let line = line.expect("Could Not Read Line").to_string();
        let input_fields: Vec<&str> = line.split('|').collect();

        let issue_date = input_fields[11];
        let coupon_date = input_fields[28];
        let face_value = input_fields[17].parse::<f64>().unwrap_or(0.0);
        let coupon_rate = input_fields[13].parse::<f64>().unwrap_or(0.0);
        let mut max_date;
        let mut int_accrued;
        let date_parser = DateParser::new("%d-%b-%y".to_string(), false);
        let as_on = config_params.as_on_date();
        if date_parser.parse(issue_date) > date_parser.parse(coupon_date) {
            max_date = date_parser.parse(issue_date)
        } else {
            max_date = date_parser.parse(coupon_date)
        }
        //get day from coupon date
        let coupon_day = &(date_parser.parse(coupon_date)).to_string()[8..]
            .parse::<f32>()
            .unwrap_or(0.0);
        let int_accrued_days = rbdate::num_days_start_to_end(max_date, *as_on);
        if max_date == rbdate::get_month_end_date(max_date) {
            //divide by 29 to get no. of months from total days
            let no_mon = (rbdate::num_days_start_to_end(
                rbdate::get_month_end_date(max_date),
                rbdate::get_month_end_date(*as_on),
            ) / 29)
                - 1;
            let broke_days = &as_on.to_string()[8..].parse::<f32>().unwrap_or(0.0);
            if no_mon >= 1 {
                //30=no. of days in a month, 360=total days in a year
                int_accrued = face_value * (coupon_rate / 100.0) * (*broke_days as f64 / 360.0)
                    + face_value * (coupon_rate / 100.0) * ((no_mon as f64) * 30.0 / 360.0);
            } else {
                int_accrued = face_value * (coupon_rate / 100.0) * (*broke_days as f64 / 360.0);
            }
        } else {
            if as_on > &rbdate::get_month_end_date(max_date)
                && *as_on != rbdate::get_month_end_date(*as_on)
            {
                let no_mon = (rbdate::num_days_start_to_end(
                    rbdate::get_month_end_date(max_date),
                    rbdate::get_month_end_date(*as_on),
                ) / 29)
                    - 1;
                //get day from AsOn date
                let ason_broke_days = &as_on.to_string()[8..].parse::<f64>().unwrap_or(0.0);
                let max_broke_days = &rbdate::get_days_from_month(max_date)
                    - &max_date.to_string()[8..].parse::<i64>().unwrap_or(0);
                if no_mon >= 1 {
                    int_accrued = face_value
                        * (coupon_rate / 100.0)
                        * (max_broke_days as f64 / 360.0)
                        + face_value * (coupon_rate / 100.0) * (*ason_broke_days as f64 / 360.0)
                        + face_value * (coupon_rate / 100.0) * ((no_mon as f64) * 30.0 / 360.0);
                } else {
                    int_accrued = face_value
                        * (coupon_rate / 100.0)
                        * (max_broke_days as f64 / 360.0)
                        + face_value * (coupon_rate / 100.0) * (*ason_broke_days as f64 / 360.0);
                }
            } else if as_on > &rbdate::get_month_end_date(max_date)
                && *as_on == rbdate::get_month_end_date(*as_on)
            {
                //divide by 29 to get no. of months from total days
                let no_mon = (rbdate::num_days_start_to_end(
                    rbdate::get_month_end_date(max_date),
                    rbdate::get_month_end_date(*as_on),
                ) / 29);
                let max_broke_days = &rbdate::get_days_from_month(max_date)
                    - &max_date.to_string()[8..].parse::<i64>().unwrap_or(0);
                int_accrued = face_value * (coupon_rate / 100.0) * (max_broke_days as f64 / 360.0)
                    + face_value * (coupon_rate / 100.0) * ((no_mon as f64) * 30.0 / 360.0);
            } else {
                let no_broke_days = rbdate::num_days_start_to_end(max_date, *as_on);
                int_accrued = face_value * (coupon_rate / 100.0) * (no_broke_days as f64 / 360.0);
            }
        }

        write!(
            op_writer,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}||||||||| \n",
            input_fields[0],
            input_fields[1],
            input_fields[2],
            input_fields[3],
            input_fields[4],
            input_fields[5],
            input_fields[6],
            input_fields[7],
            input_fields[8],
            input_fields[9],
            input_fields[10],
            date_parser.parse_opt(&input_fields[11]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            date_parser.parse_opt(&input_fields[12]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            input_fields[13],
            input_fields[14],
            input_fields[15],
            input_fields[16],
            input_fields[17],
            input_fields[18],
            input_fields[19],
            input_fields[20],
            input_fields[21],
            input_fields[22],
            int_accrued_days,
            int_accrued,
            coupon_day,
            input_fields[24],
            input_fields[25],
            input_fields[26],
            input_fields[27],
            date_parser.parse_opt(&input_fields[28]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            date_parser.parse_opt(&input_fields[29]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            date_parser.parse_opt(&input_fields[30]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            date_parser.parse_opt(&input_fields[31]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            input_fields[32],
            input_fields[33],
            input_fields[34],
            input_fields[35],
            input_fields[36],
            input_fields[37],
            input_fields[38],
            input_fields[39],
            input_fields[42],
            input_fields[43],
            input_fields[44],
            input_fields[45],
            input_fields[46],
            input_fields[47],
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
