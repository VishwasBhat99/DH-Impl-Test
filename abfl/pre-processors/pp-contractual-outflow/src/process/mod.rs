use calamine::{open_workbook, Reader, Xlsx};
use chrono::{Datelike, Duration};
use configuration_parameters::ConfigurationParameters;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::SystemTime;

pub fn process(config_param: ConfigurationParameters, _log: &Logger, diag_log: &Logger) {
    let start_read_timer = SystemTime::now();
    let as_on_date = config_param.as_on_date();
    let ason_str = as_on_date.format("%d-%m-%Y");
    let one_day_less_ason = as_on_date - Duration::days(1);
    let thirty_days_after_ason = as_on_date + Duration::days(30);
    let day = one_day_less_ason.day();
    let mut month_str = String::new();
    let month = one_day_less_ason.month();
    if month < 10 {
        month_str = format!("0{}", month);
    } else {
        month_str = month.to_string();
    }
    let year = one_day_less_ason.year();
    let directory = format!("{}{}{}", day, month_str, year);
    let inp_file_path = format!(
        "{}{}{}{}{}",
        config_param.input_file(),
        config_param.path_sep(),
        directory,
        config_param.path_sep(),
        config_param.mis_input_file()
    );
    let file = match new_buf_rdr(&inp_file_path.as_str()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            inp_file_path,
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut app_dt_str = String::new();
    let mut cashflow_file: Xlsx<_> = open_workbook(config_param.input_cashflow_file())
        .expect("Unable to open `input_cashflow_file.Xlsx`.");
    let mut cashflow_map_project_amt: HashMap<String, f64> = HashMap::new();
    let mut cashflow_map_thirty_days_amt: HashMap<String, f64> = HashMap::new();
    if let Some(Ok(reader)) = cashflow_file.worksheet_range(config_param.cashflow_sheet()) {
        for row in reader.rows().skip(1) {
            let applicabe_dt =
                rbdate::datevalue_to_naive_date(&row[1].to_string().as_str()).unwrap_or(as_on_date);
            let date =
                rbdate::datevalue_to_naive_date(&row[0].to_string().as_str()).unwrap_or(as_on_date);
            let projected_amt = row[3].to_string().parse::<f64>().unwrap_or(0.0);
            if applicabe_dt < as_on_date {
                panic!("Applicable date is less than Ason date is present in file");
            }
            if applicabe_dt > as_on_date && date == one_day_less_ason {
                cashflow_map_project_amt.insert(row[2].to_string(), projected_amt);
            }
            if date >= as_on_date && date < thirty_days_after_ason && applicabe_dt > as_on_date {
                cashflow_map_thirty_days_amt
                    .entry(row[2].to_string())
                    .and_modify(|amt| *amt += projected_amt)
                    .or_insert(projected_amt);
            }
            app_dt_str = applicabe_dt.format("%d-%m-%Y").to_string();
        }
    }
    let mut master_file: Xlsx<_> = open_workbook(config_param.mapping_master_file())
        .expect("Unable to open `mapping_master.xlsx`.");
    let mut master_map: HashMap<String, String> = HashMap::new();
    let mut serial_map: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = master_file.worksheet_range(config_param.master_sheet()) {
        for row in reader.rows().skip(1) {
            master_map.insert(row[1].to_string(), row[2].to_string());
            serial_map.insert(row[2].to_string(), row[0].to_string());
        }
    }

    let end_read_timer = SystemTime::now();
    let duration = end_read_timer
        .duration_since(start_read_timer)
        .expect("Could not calculate total duration read timer.");
    debug!(
        diag_log,
        "Reading Reference Files, Total Duration: {:?}.", duration
    );

    let start_derive_timer = SystemTime::now();
    let mut output_line: String = String::new();
    for (line_num, lines) in file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                inp_file_path,
                line_num + 1,
                error
            ),
        };

        let fields: Vec<&str> = line.split('|').collect();
        let prd_slab = fields[2].to_string();
        let naive_amt = fields[6].to_string().parse::<f64>().unwrap_or(0.0);

        let lob = master_map
            .get(&prd_slab)
            .unwrap_or(&"NA".to_string())
            .to_owned();
        let projected_amt = cashflow_map_project_amt
            .get(&lob)
            .unwrap_or(&0.0)
            .to_owned();
        cashflow_map_project_amt.remove(&lob);
        let incr_amt = projected_amt - naive_amt;
        let thirty_day_projected_amt = cashflow_map_thirty_days_amt
            .get(&lob)
            .unwrap_or(&0.0)
            .to_owned();
        cashflow_map_thirty_days_amt.remove(&lob);
        let serial_no = serial_map.get(&lob).unwrap_or(&"NA".to_string()).to_owned();

        output_line.push_str(serial_no.to_string().as_str());
        output_line.push('|');

        output_line.push_str(ason_str.to_string().as_str());
        output_line.push('|');

        if incr_amt > 0.0 {
            output_line.push_str(incr_amt.to_string().as_str());
            output_line.push('|');
        } else {
            output_line.push_str("0.0");
            output_line.push('|');
        }
        output_line.push_str(app_dt_str.as_str());
        output_line.push('|');

        output_line.push_str(thirty_day_projected_amt.to_string().as_str());
        output_line.push('|');

        output_line.push_str(lob.as_str());
        output_line.push('|');

        output_line.push_str(fields[5]);
        output_line.push('|');

        output_line.push_str("||||");

        output_line.push('\n');
    }
    for (key, val) in cashflow_map_project_amt.iter() {
        let serial_no = serial_map.get(key).unwrap_or(&"NA".to_string()).to_owned();

        output_line.push_str(serial_no.to_string().as_str());
        output_line.push('|');

        output_line.push_str(ason_str.to_string().as_str());
        output_line.push('|');

        if val.to_owned() > 0.0 {
            output_line.push_str(val.to_string().as_str());
            output_line.push('|');
        } else {
            output_line.push_str("0.0");
            output_line.push('|');
        }

        let thirty_day_projected_amt = cashflow_map_thirty_days_amt
            .get(key)
            .unwrap_or(&0.0)
            .to_owned();

        output_line.push_str(app_dt_str.as_str());
        output_line.push('|');

        output_line.push_str(thirty_day_projected_amt.to_string().as_str());
        output_line.push('|');

        output_line.push_str(key.as_str());
        output_line.push('|');

        output_line.push_str("INR");
        output_line.push('|');

        output_line.push_str("||||");

        output_line.push('\n');
    }
    let end_derive_timer = SystemTime::now();
    let duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total derive process duration.");
    debug!(diag_log, "Derive Process Total Duration: {:?}.", duration);

    let start_write_timer = SystemTime::now();
    let mut writer = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file: `{}` on location `{}` : {}",
            config_param.output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error,
        ),
    };

    match writer.write_all(output_line.as_bytes()) {
        Ok(_) => println!("Successfully processed all accounts."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`: {}.",
            config_param.output_file_path(),
            error,
        ),
    }

    let end_write_timer = SystemTime::now();
    let duration = end_write_timer
        .duration_since(start_write_timer)
        .expect("Could not calculate total duration for writing pre-processed output.");
    debug!(
        diag_log,
        "Writing Records and Reconcilation File, Total Duration: {:?}.", duration
    );
}
