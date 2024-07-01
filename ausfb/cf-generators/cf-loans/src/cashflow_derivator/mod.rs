use slog::Logger;
mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod append_write_cashflows;
mod cashflow_appender;

use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use rbdate::{date_from_timestamp, NaiveDate};

// use self::account_with_cashflows::AccountWithCashflows;
use self::append_write_cashflows::{append_cashflows, create_io_workers, write_cashflows};
use cashflow_derivator::account_with_cashflows::Cashflow;
use chrono::Datelike;
use configuration_parameters::ConfigurationParameters;
use hashbrown::HashMap;
use health_report::HealthReport;
use macros;
use protobuf::Clear;
use sdb_io::*;
use statics::*;
use std::io::BufRead;
use std::time::SystemTime;

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
    log_debug!(log, "Input CF File Reading Started");
    let mut cf_map: HashMap<String, Vec<Vec<String>>> = HashMap::new();
    let input_cf_file = match new_buf_rdr(config_params.input_cf_file_path()) {
        Ok(file) => file,
        Err(_error) => panic!(
            "Could not found input_cf_file: `{}`",
            config_params.input_cf_file_path(),
        ),
    };

    for (line_num, lines) in input_cf_file.lines().enumerate().skip(1) {
        let master_line = match lines {
            Ok(master_line) => master_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.input_cf_file_path(),
                line_num + 1,
                error
            ),
        };
        let cf_fields = master_line
            .split('|')
            .map(|s| {
                s.trim()
                    .to_string()
                    .replace("'", "")
                    .replace("\"", "")
                    .replace("`", "")
            })
            .collect::<Vec<String>>();

        let loan_number = cf_fields[0].to_string();
        cf_map
            .entry(loan_number)
            .and_modify(|data| {
                data.push(vec![
                    cf_fields[4].to_string(),
                    cf_fields[7].to_string(),
                    cf_fields[8].to_string(),
                    cf_fields[9].to_string(),
                ])
            })
            .or_insert(vec![vec![
                cf_fields[4].to_string(),
                cf_fields[7].to_string(),
                cf_fields[8].to_string(),
                cf_fields[9].to_string(),
            ]]);
    }

    log_debug!(log, "Input CF File Reading Completed");

    let (reader, mut writer) = create_io_workers(
        config_params.input_file_path(),
        config_params.output_file_path(),
        log,
    );
    let mut reader_iterator = reader;
    let mut a_w_cf = AccountWithCashflows::new();

    // reader_iterator.next();
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
        tot_acc_encntrd += 1;
        tot_int_in_ip += input_account.gl_account_interest;
        tot_prin_in_ip += input_account.gl_account_principal;
        let mut cf_vec: Vec<Cashflow> = Vec::new();
        let mut last_repricing_date: String = "".to_string();
        let eop_bal = input_account.eop_balance;
        let mut total_cf_prin = 0.0;
        if cf_map.contains_key(&input_account.cod_acct_no) {
            tot_acc_with_cfs += 1;
            let cf_data: Vec<Vec<String>> =
                cf_map.get(&input_account.cod_acct_no).unwrap().to_vec();
            if eop_bal == 0.0 {
                log_debug!(
                    log,
                    "skipped cashflow for cod_acct_no:{} because eop_balnace is 0.0",
                    input_account.cod_acct_no
                );
                continue;
            }
            let mut is_adjusted_flag = false;
            for cf_fields in cf_data.iter() {
                total_cf_prin += cf_fields[2].parse().unwrap_or(0.0);
                let cf_date = NaiveDate::parse_from_str(&cf_fields[1], "%d-%m-%y");
                cf = new_cashflow(
                    cf_fields[3].parse().unwrap_or(0.0),
                    if total_cf_prin < eop_bal {
                        cf_fields[2].parse().unwrap_or(0.0)
                    } else {
                        let adjusted_amt =
                            eop_bal - (total_cf_prin - cf_fields[2].parse().unwrap_or(0.0));
                        is_adjusted_flag = true;
                        log_debug!(
                            log,
                            "for cod_acct_no:{} eop_balance:{} is lesser than total_cf_prin:{}",
                            input_account.cod_acct_no,
                            input_account.eop_balance,
                            total_cf_prin
                        );
                        adjusted_amt
                    },
                    if cf_date.is_err() {
                        rbdate::timestamp(
                            input_account
                                .maturity_date
                                .unwrap_or(*config_params.as_on_date()),
                        )
                    } else {
                        rbdate::timestamp(update_year(
                            &cf_fields[1].to_string(),
                            config_params.as_on_date(),
                        ))
                    },
                );
                tot_prin_in_op += cf.principal_amount;
                tot_cfs += 1;
                tot_int_in_op += cf.interest_amount;
                cf_vec.push(cf);
                last_repricing_date = cf_fields[0].to_string();
                if is_adjusted_flag {
                    break;
                }
            }
        } else {
            log_debug!(
                log,
                "There is no cashflow present for {} customer_id",
                input_account.customer_id
            );
            continue;
        }
        if total_cf_prin < eop_bal {
            log_debug!(
                log,
                "for cod_acct_no:{} eop_balance:{} is greater than total_cf_prin:{}",
                input_account.cod_acct_no,
                input_account.eop_balance,
                total_cf_prin
            );
            let no_of_cfs = cf_vec.len();
            cf_vec[no_of_cfs - 1] = new_cashflow(
                cf_vec[no_of_cfs - 1].interest_amount,
                cf_vec[no_of_cfs - 1].principal_amount + (eop_bal - total_cf_prin),
                cf_vec[no_of_cfs - 1].date,
            );
        }
        a_w_cf = append_cashflows(
            diag_log,
            &input_account,
            config_params,
            last_repricing_date,
            &cf_vec,
        );
        write_cashflows(&mut writer, log, diag_log, &a_w_cf);
        a_w_cf.clear();
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
fn update_year(date_string: &str, as_on_date: &NaiveDate) -> NaiveDate {
    let date = NaiveDate::parse_from_str(date_string, "%d-%m-%y").unwrap();
    let current_year = date.year() % 100;
    let current_century = (as_on_date.year() / 100) * 100;
    let final_year = current_century + current_year;
    let updated_date = NaiveDate::from_ymd(final_year, date.month(), date.day());
    updated_date
}
