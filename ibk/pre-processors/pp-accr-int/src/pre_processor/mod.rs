use self::derive_maturity::get_maturity_date;
use self::get_holiday_data::get_holiday_map;
use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use multimap::MultiMap;
use rbdate::DateParser;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::SystemTime;
mod derive_maturity;
mod get_holiday_data;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_timer = SystemTime::now();
    let mut tot_acc_encntrd: i64 = 0;
    let mut tot_acc_skipped: i64 = 0;
    let mut tot_acc_failed: i64 = 0;
    let mut tot_acc_succ: i64 = 0;
    let mut input_map: MultiMap<String, String> = MultiMap::new();

    let date_format = match config_param.date_format() {
        "dd-mm-yyyy" => "%d-%m-%Y",
        "dd.mm.yyyy" => "%d.%m.%Y",
        "dd/mm/yyyy" => "%d/%m/%Y",
        "dd-mmm-yyyy" => "%d-%b-%Y",
        "dd.mmm.yyyy" => "%d.%b.%Y",
        "dd/mmm/yyyy" => "%d/%b/%Y",
        "yyyy-mm-dd" => "%Y-%m-%d",
        "yyyy.mm.dd" => "%Y.%m.%d",
        "yyyy/mm/dd" => "%Y/%m/%d",
        "yyyy-mmm-dd" => "%Y-%b-%d",
        "yyyy.mmm.dd" => "%Y.%b.%d",
        "yyyy/mmm/dd" => "%Y/%b/%d",
        "dd-mm-yy" => "%d-%m-%y",
        "dd.mm.yy" => "%d.%m.%y",
        "dd/mm/yy" => "%d/%m/%y",
        "yy-mm-dd" => "%y-%m-%d",
        "yy.mm.dd" => "%y.%m.%d",
        "yy/mm/dd" => "%y/%m/%d",
        _ => "%d-%m-%Y",
    };
    let dt_parser = DateParser::new(date_format.to_string(), false);

    let input_file = match new_buf_rdr(config_param.input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            config_param.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let holiday_file = match new_buf_rdr(config_param.holiday_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input holiday file: `{}` on location `{}` : {}.",
            config_param.holiday_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut writer = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file: `{}` on location `{}` : {}",
            config_param.output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };
    let mut holiday_map: HashMap<rbdate::NaiveDate, String> = HashMap::new();

    for (line_num, lines) in holiday_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => {
                log_info!(
                    log,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.holiday_file_path(),
                    line_num + 1,
                    error
                );
                continue;
            }
        };
        let mut holiday_fields: Vec<&str> = line.split(config_param.holiday_delimiter()).collect();
        get_holiday_map(
            &mut holiday_fields,
            config_param.currency(),
            &mut holiday_map,
        );
    }

    for (line_num, lines) in input_file.lines().enumerate() {
        tot_acc_encntrd += 1;
        let line = match lines {
            Ok(line) => line,
            Err(error) => {
                log_info!(
                    log,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.input_file_path(),
                    line_num + 1,
                    error
                );
                tot_acc_failed += 1;
                continue;
            }
        };
        let fields: Vec<&str> = line.split(config_param.input_delimiter()).collect();
        if fields[14].to_uppercase() != "INDEM" {
            tot_acc_skipped += 1;
            continue;
        }
        input_map.insert(fields[1].to_string(), line);
    }

    for (acc_no, _) in input_map.iter() {
        let accounts = input_map.get_vec(acc_no).unwrap();
        let mut mat_dt: NaiveDate;
        let mut op_line = String::new();
        // Case for single account data
        if accounts.len() == 1 {
            let mut ip: Vec<&str> = accounts[0].split(config_param.input_delimiter()).collect();
            if ip[2] == ip[3] {
                mat_dt = dt_parser.parse(&ip[4]);
                while mat_dt < *config_param.as_on_date() {
                    if get_freq(ip[6]) == 0 {
                        mat_dt = rbdate::incr_dt_by_days(mat_dt, 7);
                    } else {
                        mat_dt = rbdate::increment_date_by_months(mat_dt, get_freq(ip[6]));
                    }
                }
                if !holiday_map.contains_key(&mat_dt) {
                    op_line = write_output(mat_dt, &mut ip);
                    continue;
                }
                while holiday_map.get(&mat_dt).unwrap_or_else(|| {
                    panic!("Could Not Get Holiday Data for Mat-Date: {}", mat_dt)
                }) != "W"
                {
                    mat_dt = rbdate::incr_dt_by_days(mat_dt, 1);
                }
                op_line = write_output(mat_dt, &mut ip);
            } else {
                mat_dt = get_maturity_date(
                    &mut ip,
                    &mut holiday_map,
                    &config_param,
                    date_format.to_string(),
                );
                op_line = write_output(mat_dt, &mut ip);
            }
            tot_acc_succ += 1;
            writer.write_all(op_line.as_bytes()).unwrap();
        }
        // Case for multiple account data
        else {
            let mut accs: Vec<Vec<&str>> = Vec::new();
            for account in accounts.iter() {
                let acc: Vec<&str> = account.split(config_param.input_delimiter()).collect();
                let no_of_flows = acc[2].parse::<f64>().unwrap_or(0.0) as i64;
                let no_of_demands = acc[3].parse::<f64>().unwrap_or(0.0) as i64;
                if no_of_flows != 0 && no_of_demands != 0 && no_of_flows == no_of_demands {
                    continue;
                }
                accs.push(acc);
            }
            // Sort by Flow_Start_Date
            accs.sort_by(|x, y| x[4].cmp(&y[4]));
            for mut acc in accs {
                let no_of_flows = acc[2].parse::<f64>().unwrap_or(0.0) as i64;
                let no_of_demands = acc[3].parse::<f64>().unwrap_or(0.0) as i64;
                if no_of_flows == 0 && no_of_demands == 0 {
                    mat_dt = dt_parser.parse(&acc[4]);
                    while mat_dt < *config_param.as_on_date() {
                        if get_freq(acc[6]) == 0 {
                            mat_dt = rbdate::incr_dt_by_days(mat_dt, 7);
                        } else {
                            mat_dt = rbdate::increment_date_by_months(mat_dt, get_freq(acc[6]));
                        }
                    }
                    if !holiday_map.contains_key(&mat_dt) {
                        let op_line = write_output(mat_dt, &mut acc);
                        tot_acc_succ += 1;
                        writer.write_all(op_line.as_bytes()).unwrap();
                        break;
                    }
                    while holiday_map.get(&mat_dt).unwrap_or_else(|| {
                        panic!("Could Not Get Holiday Data for Mat-Date: {}", mat_dt)
                    }) != "W"
                    {
                        mat_dt = rbdate::incr_dt_by_days(mat_dt, 1);
                    }
                    let op_line = write_output(mat_dt, &mut acc);
                    tot_acc_succ += 1;
                    writer.write_all(op_line.as_bytes()).unwrap();
                    break;
                } else {
                    mat_dt = get_maturity_date(
                        &mut acc,
                        &mut holiday_map,
                        &config_param,
                        date_format.to_string(),
                    );
                    let op_line = write_output(mat_dt, &mut acc);
                    tot_acc_succ += 1;
                    writer.write_all(op_line.as_bytes()).unwrap();
                    break;
                }
            }
        }
    }
    let end_timer = SystemTime::now();
    let duration = end_timer
        .duration_since(start_timer)
        .expect("Could not calculate total derive process duration.");
    debug!(diag_log, "Derive Process Total Duration: {:?}.", duration);

    let report_string = format!(
        "Accounts encountered: {}\n\
         Accounts proccessed suceessfully: {}\n\
         Accounts skipped in the process: {}\n\
         Accounts failed to process: {}",
        tot_acc_encntrd, tot_acc_succ, tot_acc_skipped, tot_acc_failed
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);
    let health_stat = HealthReport::new(tot_acc_encntrd, tot_acc_succ, tot_acc_failed, 0.0, 0.0, 0);
    health_stat.gen_health_rpt(config_param.output_file_path());
}

pub fn write_output(mat_dt: NaiveDate, input: &mut Vec<&str>) -> String {
    let mut op_line: String = String::new();
    let no_of_flows = input[2].parse::<f64>().unwrap_or(0.0) as i64;
    let no_of_demands = input[3].parse::<f64>().unwrap_or(0.0) as i64;
    let cf_shdl_num = no_of_flows - no_of_demands;
    op_line.push_str(&input[1].to_string()); //AccID
    op_line.push_str("|");
    op_line.push_str(&no_of_flows.to_string()); //NO_OF_FLOWS
    op_line.push_str("|");
    op_line.push_str(&no_of_demands.to_string()); //NO_OF_DMDS
    op_line.push_str("|");
    op_line.push_str(&cf_shdl_num.to_string()); //CF_SHDL_NUM
    op_line.push_str("|");
    op_line.push_str(&input[5].to_string()); //FLOW_AMT
    op_line.push_str("|");
    op_line.push_str(&(input[5].parse::<f64>().unwrap_or(0.0) * (cf_shdl_num as f64)).to_string()); //CF_AMT
    op_line.push_str("|");
    op_line.push_str(&input[14].to_string()); //FLOW_ID
    op_line.push_str("|");
    op_line.push_str(&mat_dt.format("%d-%m-%Y").to_string()); //MATURITY_DATE
    op_line.push_str("\n");
    op_line
}
pub fn get_freq(freq: &str) -> u16 {
    match freq {
        "Y" => 12,
        "H" => 6,
        "Q" => 3,
        "M" => 1,
        _ => 0,
    }
}
