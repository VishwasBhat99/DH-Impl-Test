use self::io::*;
use self::util::*;
use calamine::{open_workbook_auto, DataType, Reader};
use chrono::{Duration, NaiveDate,Datelike};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::{get_days_from_month, incr_dt_by_mon_presrv_eom_checked};
use slog::Logger;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::time::SystemTime;

mod io;
mod util;

pub fn generatesummary(
    config_params: ConfigurationParameters,
    logger: &Logger,
    _diag_logger: &Logger,
) {
    let start_time = SystemTime::now();
    let mut tot_acc_encntrd = 0;
    let skp_acc = 0;
    let ttl_amt: f64 = 0.0;
    let default_date = NaiveDate::parse_from_str("01-01-1900", "%d-%m-%Y").unwrap();
    let mut master_excel = open_workbook_auto(config_params.master_file_path())
        .expect("Unable to open Mapping Master File.");
    let mut master_map: HashMap<String, Vec<String>> = HashMap::new();
    if let Some(Ok(reader)) = master_excel.worksheet_range(config_params.master_sheet_name()) {
        for row in reader.rows().skip(1) {
            let gl_acc_no = row[0].to_string();
            let class = row[2].to_string();
            let group = row[3].to_string();
            let clsfn = row[4].to_string();
            let acc_open_logic = row[8].to_string();
            let acc_cls_logic = row[9].to_string();
            master_map.insert(
                gl_acc_no,
                [group, clsfn, class, acc_open_logic, acc_cls_logic].to_vec(),
            );
        }
    }

    let days = get_days_from_month(config_params.as_on_date);
    let year = config_params.as_on_date().year();
    let reference_date_str = format!("{}-{}",config_params.reference_date(),year).to_string();
    let reference_date = NaiveDate::parse_from_str(reference_date_str.as_str(), "%d-%m-%Y").unwrap();
    let first_quarter_date =
        incr_dt_by_mon_presrv_eom_checked(reference_date, 3).unwrap();
    let second_quarter_date =
        incr_dt_by_mon_presrv_eom_checked(reference_date, 6).unwrap();
    let third_quarter_date =
        incr_dt_by_mon_presrv_eom_checked(reference_date, 9).unwrap();
    let fourth_quarter_date =
        incr_dt_by_mon_presrv_eom_checked(reference_date, 12).unwrap();
    let mut op_writer = get_writer(config_params.output_file_path());
    let reader = fs::read_to_string(config_params.input_file_path())
        .expect("Could Not Read previous_day_file_path");
    let mut op_line = String::new();
    for line in reader.lines() {
        tot_acc_encntrd += 1;
        let derived_fields = line.split("|").collect::<Vec<&str>>();
        let acc_num = derived_fields[0].to_string();
        let mut outbal_sum = 0.0;
        let mut int_rate = 0.0;
        let mut sum_prod_outbal_intrt = 0.0;
        let mut div_posted_sum = 0.0;
        let div_posted_end_date = derived_fields[days as usize * 3].to_string().parse::<f64>().unwrap_or(0.0);
        let mut method_id = String::new();
        let gl_cd = derived_fields[97];
        let curr_status = derived_fields[94].to_string();
        let closed_dt = NaiveDate::parse_from_str(derived_fields[96], "%d-%m-%Y").unwrap_or(default_date);
        let class = derived_fields[95];
        op_line.push_str(acc_num.as_str());
        op_line.push('|');
        for index in (1..94).step_by(3) {
            let temp_index = index;
            outbal_sum += derived_fields[temp_index]
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0)
                .abs();
            int_rate += derived_fields[temp_index + 1]
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0)
                .abs();
            div_posted_sum += derived_fields[temp_index + 2]
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0)
                .abs();
            sum_prod_outbal_intrt += (derived_fields[temp_index]
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0)
                .abs()
                * derived_fields[temp_index + 1]
                    .to_string()
                    .parse::<f64>()
                    .unwrap_or(0.0))
                    .abs();
        }
        let default_vec = vec!["NA".to_string(),"NA".to_string(),"NA".to_string(),"1001".to_string(),"1001".to_string()];
        let gl_data_vec = master_map.get(gl_cd).unwrap_or(&default_vec);
        let acc_open_logic = &gl_data_vec[3];
        let acc_close_logic = &gl_data_vec[4];
        let weighted_int_rt =
            calc_weighted_int_rt(outbal_sum, int_rate, sum_prod_outbal_intrt, days as f64);
        if class == "A" && curr_status != "" && gl_cd != "" && closed_dt != default_date{
            if curr_status == "10".to_string() || curr_status == "40".to_string() || curr_status == "C".to_string(){
                method_id = acc_close_logic.to_string();
            } else {
                method_id = acc_open_logic.to_string();
            }
        } else if class == "L"  && curr_status != "" && gl_cd != "" && closed_dt != default_date{
            if curr_status == "7".to_string() || curr_status == "C".to_string() {
                method_id = acc_close_logic.to_string();
            } else {
                method_id = acc_open_logic.to_string();
            }
        } else {
            method_id = config_params.default_method_id.to_string();
        }
        let default_id = config_params.default_method_id().to_string().as_str().parse::<i32>().unwrap();
        let match_method_id = method_id.parse::<i32>().unwrap_or(default_id);
        if match_method_id == default_id {
            op_line.push_str(
                default_avg_amt_calc(outbal_sum, days as f64)
                    .to_string()
                    .as_str(),
            );
            op_line.push('|');
            op_line.push_str(weighted_int_rt.to_string().as_str());
            op_line.push('\n');

            continue;
        }
        let prev_day_cls_dt = closed_dt - Duration::days(1);
        let int_posted_index = prev_day_cls_dt.day() as usize;
        let int_posted = derived_fields[int_posted_index*3].to_string().parse::<f64>().unwrap_or(0.0).abs();
        match match_method_id {
            1002 => {
                op_line.push_str(
                    int_sub_avg_amt_calc(outbal_sum, div_posted_end_date, days as f64)
                        .to_string()
                        .as_str(),
                );
            }
            1003 => {
                if config_params.as_on_date() == &first_quarter_date
                || config_params.as_on_date() == &second_quarter_date
                || config_params.as_on_date() == &third_quarter_date
                || config_params.as_on_date() == &fourth_quarter_date
                {
                    op_line.push_str(
                    int_sub_avg_amt_calc(outbal_sum, div_posted_end_date, days as f64)
                        .to_string()
                        .as_str(),
                );
                } else {
                    op_line.push_str(
                    default_avg_amt_calc(outbal_sum, days as f64)
                        .to_string()
                        .as_str(),
                );
                }
            }
            1004 => {
                if config_params.as_on_date() == &second_quarter_date
                || config_params.as_on_date() == &fourth_quarter_date
                {
                    op_line.push_str(
                    int_sub_avg_amt_calc(outbal_sum, div_posted_end_date, days as f64)
                        .to_string()
                        .as_str(),
                );
                } else {
                    op_line.push_str(
                    default_avg_amt_calc(outbal_sum, days as f64)
                        .to_string()
                        .as_str(),
                );
                }
            }
            1005 => {
                if config_params.as_on_date() == &fourth_quarter_date {
                    op_line.push_str(
                        int_sub_avg_amt_calc(outbal_sum, div_posted_end_date, days as f64)
                            .to_string()
                            .as_str(),
                    );
                } else {
                    op_line.push_str(
                        default_avg_amt_calc(outbal_sum, days as f64)
                            .to_string()
                            .as_str(),
                    );
                }
            }
            1006 => {
                op_line.push_str(
                    int_add_avg_amt_calc(outbal_sum, div_posted_end_date, days as f64)
                        .to_string()
                        .as_str(),
                );
            }
            1007 => {
                if config_params.as_on_date() == &first_quarter_date
                || config_params.as_on_date() == &second_quarter_date
                || config_params.as_on_date() == &third_quarter_date
                || config_params.as_on_date() == &fourth_quarter_date
                {
                    op_line.push_str(
                    int_add_avg_amt_calc(outbal_sum, div_posted_end_date, days as f64)
                        .to_string()
                        .as_str(),
                );
                } else {
                    op_line.push_str(
                    default_avg_amt_calc(outbal_sum, days as f64)
                        .to_string()
                        .as_str(),
                );
                }
            }
            1008 => {
                if config_params.as_on_date() == &second_quarter_date
                || config_params.as_on_date() == &fourth_quarter_date
                {
                op_line.push_str(
                    int_add_avg_amt_calc(outbal_sum, div_posted_end_date, days as f64)
                        .to_string()
                        .as_str(),
                );
                } else {
                op_line.push_str(
                    default_avg_amt_calc(outbal_sum, days as f64)
                        .to_string()
                        .as_str(),
                );
                }
            }
            1009 => {
                if config_params.as_on_date() == &fourth_quarter_date {
                    op_line.push_str(
                        int_add_avg_amt_calc(outbal_sum, div_posted_end_date, days as f64)
                            .to_string()
                            .as_str(),
                    );
                } else {
                    op_line.push_str(
                        default_avg_amt_calc(outbal_sum, days as f64)
                            .to_string()
                            .as_str(),
                    );
                }
            }
            1010 => {
                op_line.push_str(
                    int_sub_avg_amt_calc(outbal_sum, div_posted_sum, days as f64)
                        .to_string()
                        .as_str(),
                );
            }
            1011 => {
                if config_params.as_on_date() == &first_quarter_date
                || config_params.as_on_date() == &second_quarter_date
                || config_params.as_on_date() == &third_quarter_date
                || config_params.as_on_date() == &fourth_quarter_date
                {
                op_line.push_str(
                    int_sub_avg_amt_calc(outbal_sum, div_posted_sum, days as f64)
                        .to_string()
                        .as_str(),
                );
                } else {
                op_line.push_str(
                    default_avg_amt_calc(outbal_sum, days as f64)
                        .to_string()
                        .as_str(),
                );
                }
            }
            1012 => {
                if config_params.as_on_date() == &second_quarter_date
                || config_params.as_on_date() == &fourth_quarter_date
                {
                op_line.push_str(
                    int_sub_avg_amt_calc(outbal_sum, div_posted_sum, days as f64)
                        .to_string()
                        .as_str(),
                );
                } else {
                op_line.push_str(
                    default_avg_amt_calc(outbal_sum, days as f64)
                        .to_string()
                        .as_str(),
                );
                }
            }
            1013 => {
                if config_params.as_on_date() == &fourth_quarter_date {
                    op_line.push_str(
                        int_sub_avg_amt_calc(outbal_sum, div_posted_sum, days as f64)
                            .to_string()
                            .as_str(),
                    );
                } else {
                    op_line.push_str(
                        default_avg_amt_calc(outbal_sum, days as f64)
                            .to_string()
                            .as_str(),
                    );
                }
            }
            1014 => {
                op_line.push_str(
                    int_add_avg_amt_calc(outbal_sum, div_posted_sum, days as f64)
                        .to_string()
                        .as_str(),
                );
            }
            1015 => {
                if config_params.as_on_date() == &first_quarter_date
                || config_params.as_on_date() == &second_quarter_date
                || config_params.as_on_date() == &third_quarter_date
                || config_params.as_on_date() == &fourth_quarter_date
                {
                op_line.push_str(
                    int_add_avg_amt_calc(outbal_sum, div_posted_sum, days as f64)
                        .to_string()
                        .as_str(),
                );
                } else {
                op_line.push_str(
                    default_avg_amt_calc(outbal_sum, days as f64)
                        .to_string()
                        .as_str(),
                );
                }
            }
            1016 => {
                if config_params.as_on_date() == &second_quarter_date
                || config_params.as_on_date() == &fourth_quarter_date
                {
                op_line.push_str(
                    int_add_avg_amt_calc(outbal_sum, div_posted_sum, days as f64)
                        .to_string()
                        .as_str(),
                );
                } else {
                op_line.push_str(
                    default_avg_amt_calc(outbal_sum, days as f64)
                        .to_string()
                        .as_str(),
                );
                }
            }
            1017 => {
                if config_params.as_on_date() == &fourth_quarter_date {
                    op_line.push_str(
                        int_add_avg_amt_calc(outbal_sum, div_posted_sum, days as f64)
                            .to_string()
                            .as_str(),
                    );
                } else {
                    op_line.push_str(
                        default_avg_amt_calc(outbal_sum, days as f64)
                            .to_string()
                            .as_str(),
                    );
                }
            }
            1018 => {
                op_line.push_str(
                    int_sub_avg_amt_calc(outbal_sum, int_posted, days as f64)
                        .to_string()
                        .as_str(),
                );
            }
            1019 => {
                op_line.push_str(
                    int_add_avg_amt_calc(outbal_sum, int_posted, days as f64)
                        .to_string()
                        .as_str(),
                );
            }
            _ => {
                op_line.push_str(
                    default_avg_amt_calc(outbal_sum, days as f64)
                        .to_string()
                        .as_str(),
                );
            }
        }
        op_line.push('|');
        op_line.push_str(weighted_int_rt.to_string().as_str());
        op_line.push('\n');
    }
    write!(op_writer, "{}", op_line).expect("Unable to generate summary file.");
    let total_duration = print_return_time_since!(start_time);
    log_info!(logger, "Total time for aggregation: {:?}", total_duration);
    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - skp_acc,
        skp_acc,
        ttl_amt,
        ttl_amt,
        0,
    );
    health_report.gen_health_rpt(&config_params.output_file_path());
}
