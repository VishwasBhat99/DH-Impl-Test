extern crate csv;
extern crate serde;
use chrono::Datelike;
use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::{decr_dt_by_mon_presrv_eom, get_month_end_date};
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufWriter;
use std::time::SystemTime;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let output_file = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{} Cannot read output file path", error),
    };
    let mut output_writer = BufWriter::new(output_file);
    let output_header="holding_type|book|portfolio|security|asset_class|asset_class_name|securitytype|issuer|settlement|face_value|cl_face_value|book_value_wac|compounding_bookvalue|cl_book_value|quantity|cl_quantity|wtgcost|accrintt|valuation|amortization|amortization_acretion|amortization_date|accretion_date|days_in_holding|maturity|earliest_maturity|avg_maturity|sector|promoter|industry|os_date|coupon|accrintt_manual|valuation_manual|amortization_manual|mis_category_name|mtm_value|mtm_date|accrual_perday|accrual|accrual_date|orientation|rbi_category_name|isin_qty|issuer_fimmdacategory|objective|isin_no|ex_interest_qty|coupyear|rating1|listing_status|npi_status|guarantee_type|secured_unsecured|as_on_date|currency|derived_date_1|derived_date_2";
    let start_timer = SystemTime::now();
    writeln!(output_writer, "{}", output_header).expect("outptut file header can not be written");
    let mut tot_rec = 0;
    let mut succ_rec = 0;

    let date_parser = rbdate::DateParser::new(config_param.input_date_format().to_string(), false);

    let input_file = match new_buf_rdr(config_param.input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input_file: `{}`",
            config_param.input_file_path(),
        ),
    };
    //Date Fields 22,23,25,26,31,38,41,55
    let date_fields_vec: Vec<usize> = vec![22, 23, 25, 26, 31, 38, 41, 55];
    //Get month from freq
    let month_freq_map: HashMap<String, i32> = HashMap::from([
        ("1".to_string(), 12),
        ("2".to_string(), 6),
        ("4".to_string(), 3),
        ("12".to_string(), 1),
    ]);
    //input file reading started
    for (line_num, lines) in input_file.lines().enumerate().skip(1) {
        let input_line = match lines {
            Ok(input_line) => input_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.input_file_path(),
                line_num + 1,
                error
            ),
        };
        tot_rec += 1;
        let mut input_fields: Vec<String> = input_line
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
        succ_rec += 1;
        let mut output_line = "".to_string();
        let ip_mat_date = input_fields[24].to_string();
        let ip_earliest_mat_date = input_fields[25].to_string();

        for date_index in date_fields_vec.iter() {
            input_fields[date_index - 1] = date_parser
                .parse_opt(&input_fields[date_index - 1])
                .unwrap_or(*config_param.as_on_date())
                .format("%d-%m-%Y")
                .to_string();
        }
        //Maturity (PP25),os_date(PP31) and coupyear(PP49)
        let mat_date = if ip_mat_date.is_empty() {
            *config_param.as_on_date()
        } else if ip_mat_date.trim().len() == 8 && !config_param.input_date_format().contains("%y")
        {
            panic!(
                "Unable to read input date :{} expected date format: {}",
                ip_mat_date,
                config_param.input_date_format()
            );
        } else {
            NaiveDate::parse_from_str(&ip_mat_date, config_param.input_date_format()).expect(
                &format!(
                    "Unable to read input date :{} expected date format: {}",
                    ip_mat_date,
                    config_param.input_date_format(),
                ),
            )
        };
        let earliest_mat_date = if ip_earliest_mat_date.is_empty() {
            *config_param.as_on_date()
        } else if ip_earliest_mat_date.trim().len() == 8
            && !config_param.input_date_format().contains("%y")
        {
            panic!(
                "Unable to read input date :{} expected date format: {}",
                ip_earliest_mat_date,
                config_param.input_date_format()
            );
        } else {
            NaiveDate::parse_from_str(&ip_earliest_mat_date, config_param.input_date_format())
                .expect(&format!(
                    "Unable to read input date :{} expected date format: {}",
                    ip_earliest_mat_date,
                    config_param.input_date_format(),
                ))
        };
        let coupyear = input_fields[48].to_owned();
        let input_line = input_fields.join("|");
        output_line.push_str(&input_line);
        //Derivation of derive_date_1
        let freq = month_freq_map.get(&coupyear.to_string()).unwrap_or(&0);
        let next_coupen_date_1 = if *freq == 0 {
            mat_date
        } else {
            get_next_coupen_date(mat_date, *config_param.as_on_date(), *freq as usize)
        };
        //Derivation of derive_date_2
        let next_coupen_date_2 = if *freq == 0 {
            earliest_mat_date
        } else {
            get_next_coupen_date(
                earliest_mat_date,
                *config_param.as_on_date(),
                *freq as usize,
            )
        };
        //append the derivation_1 and derivation_2 in output line
        output_line.push_str(&format!(
            "|{}|{}|{}",
            config_param.currency(),
            next_coupen_date_1.format("%d-%m-%Y").to_string(),
            next_coupen_date_2.format("%d-%m-%Y").to_string()
        ));
        writeln!(output_writer, "{}", output_line).expect("output_line can not be written");
    }
    let end_timer = SystemTime::now();
    let duration = end_timer
        .duration_since(start_timer)
        .expect("Could not calculate total process duration.");
    log_debug!(
        log,
        "Total Duration for preprocess the data: {:?}.",
        duration
    );
    info!(
        diag_log,
        "Total Duration for preprocess the data: {:?}.", duration
    );
    let health_report = HealthReport::new(tot_rec, succ_rec, tot_rec - succ_rec, 0.0, 0.0, 0);
    health_report.gen_health_rpt(&config_param.output_file_path());
}
//function to get the next coupen date
pub fn get_next_coupen_date(mat_date: NaiveDate, as_on_date: NaiveDate, freq: usize) -> NaiveDate {
    if mat_date <= as_on_date {
        return mat_date;
    }
    let curr_mat_date = mat_date;
    let mut temp_mat_date = mat_date;
    let mut months = 0;

    while temp_mat_date > as_on_date {
        months += freq;
        temp_mat_date = decr_dt_by_mon_presrv_eom(curr_mat_date, months).unwrap_or(as_on_date);
    }
    let cal_mat_date =
        decr_dt_by_mon_presrv_eom(curr_mat_date, months - freq).unwrap_or(as_on_date);
    if curr_mat_date == get_month_end_date(curr_mat_date) {
        get_month_end_date(cal_mat_date)
    } else {
        NaiveDate::from_ymd_opt(
            cal_mat_date.year(),
            cal_mat_date.month(),
            curr_mat_date.day(),
        )
        .unwrap_or(cal_mat_date)
    }
}
