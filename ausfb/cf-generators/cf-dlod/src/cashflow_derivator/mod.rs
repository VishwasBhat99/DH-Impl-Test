use slog::Logger;
mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod append_write_cashflows;
mod cashflow_appender;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use hashbrown::HashMap;
use rbdate::{timestamp, NaiveDate};

// use self::account_with_cashflows::AccountWithCashflows;
use self::append_write_cashflows::{append_cashflows, create_io_workers, write_cashflows};
use cashflow_derivator::account_with_cashflows::Cashflow;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use protobuf::Clear;
use sdb_io::*;
use statics::*;
use std::io::BufRead;
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq)]
pub struct CFData1 {
    pub schedule_dt: NaiveDate,
    pub schedule_limit_amt: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CFData2 {
    pub schedule_dt: NaiveDate,
    pub approved_amt_monthly: f64,
    pub approved_amt: f64,
}

pub fn generate(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut tot_acc_encntrd: i64 = DEFAULT_INT;
    let mut tot_acc_with_cfs: i64 = DEFAULT_INT;
    let mut tot_cfs: usize = 0;
    let mut tot_prin_in_ip = DEFAULT_FLOAT;
    let mut tot_prin_in_op = DEFAULT_FLOAT;
    let mut tot_int_in_op = DEFAULT_FLOAT;
    let mut tot_int_in_ip = DEFAULT_FLOAT;
    let start_derive_timer = SystemTime::now();
    //read CF file
    let mut dlod_1_cf_map: HashMap<String, Vec<CFData1>> = HashMap::new();
    let dlod_1_file = match new_buf_rdr(config_params.dlod_file_1_path()) {
        Ok(file) => file,
        Err(_error) => panic!(
            "Could not found dlod_file_1: `{}`",
            config_params.dlod_file_1_path(),
        ),
    };
    let mut dlod_2_cf_map: HashMap<String, Vec<CFData2>> = HashMap::new();
    let dlod_2_file = match new_buf_rdr(config_params.dlod_file_2_path()) {
        Ok(file) => file,
        Err(_error) => panic!(
            "Could not found dlod_file_2: `{}`",
            config_params.dlod_file_2_path(),
        ),
    };

    for (line_num, lines) in dlod_1_file.lines().enumerate().skip(1) {
        let master_line = match lines {
            Ok(master_line) => master_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.dlod_file_1_path(),
                line_num + 1,
                error
            ),
        };
        let dlod_1_cf_fields: Vec<&str> = master_line
            .split(config_params.dlod_separator_1())
            .collect();
        let loan_number = dlod_1_cf_fields[1]
            .to_string()
            .replace("'", "")
            .replace("\"", "")
            .replace("`", "")
            .strip_prefix("OD")
            .unwrap_or(dlod_1_cf_fields[1])
            .to_string();
        let sch_dt =
            NaiveDate::parse_from_str(dlod_1_cf_fields[11], config_params.dlod_date_format_1())
                .unwrap();
        if sch_dt.ge(config_params.as_on_date()) {
            dlod_1_cf_map
                .entry(loan_number)
                .and_modify(|data| {
                    data.push(CFData1 {
                        schedule_dt: sch_dt,
                        schedule_limit_amt: dlod_1_cf_fields[12].parse::<f64>().unwrap_or(0.0),
                    })
                })
                .or_insert(vec![CFData1 {
                    schedule_dt: sch_dt,
                    schedule_limit_amt: dlod_1_cf_fields[12].parse::<f64>().unwrap_or(0.0),
                }]);
        }
    }

    for (line_num, lines) in dlod_2_file.lines().enumerate().skip(1) {
        let master_line = match lines {
            Ok(master_line) => master_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.dlod_file_2_path(),
                line_num + 1,
                error
            ),
        };
        let dlod_2_cf_fields: Vec<&str> = master_line
            .split(config_params.dlod_separator_2())
            .collect();
        let loan_number = dlod_2_cf_fields[3]
            .to_string()
            .replace("'", "")
            .replace("\"", "")
            .replace("`", "")
            .trim_start_matches("A")
            .trim_start_matches("OD")
            .to_string();
        let sch_dt =
            NaiveDate::parse_from_str(dlod_2_cf_fields[5], config_params.dlod_date_format_2())
                .unwrap();
        dlod_2_cf_map
            .entry(loan_number)
            .and_modify(|data| {
                data.push(CFData2 {
                    schedule_dt: sch_dt,
                    approved_amt_monthly: dlod_2_cf_fields[6].parse::<f64>().unwrap_or(0.0),
                    approved_amt: dlod_2_cf_fields[9].parse::<f64>().unwrap_or(0.0),
                })
            })
            .or_insert(vec![CFData2 {
                schedule_dt: sch_dt,
                approved_amt_monthly: dlod_2_cf_fields[6].parse::<f64>().unwrap_or(0.0),
                approved_amt: dlod_2_cf_fields[9].parse::<f64>().unwrap_or(0.0),
            }]);
    }
    log_debug!(log, "Input DLOD CF Files Reading Completed");

    let (reader, mut writer) = create_io_workers(
        config_params.input_file_path(),
        config_params.output_file_path(),
        log,
    );
    let mut reader_iterator = reader;
    let mut a_w_cf = AccountWithCashflows::new();

    reader_iterator.next();
    loop {
        let mut cf: Cashflow = Cashflow::new();
        let account_opt = log_measurements!(
            diag_log,
            [format!(
                "Type: ReadParseInputAccount, Identifier: {}",
                tot_acc_encntrd
            )],
            reader_iterator.next()
        );

        if account_opt.is_none() {
            break;
        }
        let input_account = account_opt.expect("Unable to parse InputAccount struct.");
        tot_prin_in_ip += input_account.curr_bal_amount;
        tot_acc_encntrd += 1;
        let mut cf_vec: Vec<Cashflow> = Vec::new();
        let acc_id = input_account
            .account_id
            .to_string()
            .replace("'", "")
            .replace("\"", "")
            .replace("`", "")
            .trim_start_matches("A")
            .trim_start_matches("OD")
            .to_string();
        if dlod_1_cf_map.contains_key(&acc_id) {
            tot_acc_with_cfs += 1;
            let mut cf_data = dlod_1_cf_map.get(&acc_id).unwrap().to_vec();
            cf_data.sort_by(|a, b| a.schedule_dt.cmp(&b.schedule_dt));
            let mut cf_amount = 0.0;
            let mut cf_sum = 0.0;
            let mut last_schedule_dt = NaiveDate::parse_from_str("01-01-1970", "%d-%m-%Y").unwrap();
            for cf_fields in cf_data.iter() {
                last_schedule_dt = cf_fields.schedule_dt;
                if input_account.curr_bal_amount <= cf_fields.schedule_limit_amt {
                    cf = new_cashflow(0.0, 0.0, timestamp(cf_fields.schedule_dt));
                } else {
                    cf_amount =
                        input_account.curr_bal_amount - cf_fields.schedule_limit_amt - cf_sum;
                    if cf_amount < 0.0 {
                        cf_amount = 0.0;
                    }
                    cf = new_cashflow(0.0, cf_amount, timestamp(cf_fields.schedule_dt));
                    cf_sum += cf_amount;
                }
                tot_prin_in_op += cf.principal_amount;
                tot_cfs += 1;
                tot_int_in_op += cf.interest_amount;
                cf_vec.push(cf);
            }
            cf_vec.pop();
            tot_prin_in_op -= cf_amount;
            cf_sum -= cf_amount;
            // last cashflow derivation
            cf = new_cashflow(
                0.0,
                input_account.curr_bal_amount - cf_sum,
                timestamp(last_schedule_dt),
            );
            tot_prin_in_op += cf.principal_amount;
            tot_cfs += 1;
            tot_int_in_op += cf.interest_amount;
            cf_vec.push(cf);
            a_w_cf = append_cashflows(diag_log, &input_account, config_params, &cf_vec);
            write_cashflows(&mut writer, log, diag_log, &a_w_cf);
            a_w_cf.clear();
        } else if dlod_2_cf_map.contains_key(&acc_id) {
            tot_acc_with_cfs += 1;
            let mut cf_data = dlod_2_cf_map.get(&acc_id).unwrap().to_vec();
            cf_data.sort_by(|a, b| a.schedule_dt.cmp(&b.schedule_dt));
            let mut cf_amount = 0.0;
            let mut cf_sum = 0.0;
            let mut first_cf = true;
            for cf_fields in cf_data.iter() {
                if cf_fields.schedule_dt >= *config_params.as_on_date() {
                    if input_account.curr_bal_amount >= cf_fields.approved_amt {
                        if first_cf {
                            cf_amount = input_account.curr_bal_amount - cf_fields.approved_amt
                                + cf_fields.approved_amt_monthly
                                + cf_sum;
                            first_cf = false;
                        } else {
                            cf_amount = cf_fields.approved_amt_monthly;
                        }
                    } else {
                        cf_amount = (cf_fields.approved_amt_monthly / cf_fields.approved_amt)
                                * input_account.curr_bal_amount;
                        if first_cf {
                            cf_amount += cf_sum;
                            first_cf = false;
                        }
                    }
                } else {
                    cf_sum += (cf_fields.approved_amt_monthly / cf_fields.approved_amt)
                        * input_account.curr_bal_amount;
                    continue;
                }
                cf = new_cashflow(0.0, cf_amount, timestamp(cf_fields.schedule_dt));
                tot_prin_in_op += cf.principal_amount;
                tot_cfs += 1;
                tot_int_in_op += cf.interest_amount;
                cf_vec.push(cf);
            }
            a_w_cf = append_cashflows(diag_log, &input_account, config_params, &cf_vec);
            write_cashflows(&mut writer, log, diag_log, &a_w_cf);
            a_w_cf.clear();
        } else {
            log_debug!(
                log,
                "There are no cashflows present for account_id: {} in Both DLOD files",
                input_account.account_id
            );
        }
    }

    writer.close();

    let end_derive_timer = SystemTime::now();
    let tot_duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total duration for derive timer.");
    let report_string = format!(
        "Accounts Encountered: {}\n\
         Accounts With Cashflows: {}\n\
         Total Cashflows: {}\n\
         Total Duration: {:?}\n\
         Total outstanding amount in input: {:.2} \n\
         Total outstanding amount in output: {:.2}\n\
         Total interest in input: {:.2}\n\
         Total interest in output: {:.2}",
        tot_acc_encntrd,
        tot_acc_with_cfs,
        tot_cfs,
        tot_duration,
        tot_prin_in_ip,
        tot_prin_in_op,
        tot_int_in_ip,
        tot_int_in_op
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);
    let health_stat = HealthReport::new(
        tot_acc_with_cfs,
        tot_acc_with_cfs,
        0,
        tot_prin_in_ip,
        tot_prin_in_op,
        tot_cfs as i64,
    );
    health_stat.gen_health_rpt(config_params.output_file_path())
}

pub fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = d;
    cf
}
