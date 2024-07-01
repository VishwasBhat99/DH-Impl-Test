use slog::Logger;
mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod cashflow_appender;
mod gen_cashflows;
use self::account_reader::InputAccountReader;
use cashflow_generator::account_with_cashflows_writer::AccountWithCashflowsWriter;
use cashflow_generator::cashflow_appender::create_account_with_cashflows;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::{incr_dt_by_mon_presrv_eom, NaiveDate};
use sdb_io::new_buf_rdr;
use std::collections::HashMap;
use std::io::{prelude::*, BufReader};
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct CompData {
    pub limit_amt: f64,
    pub days: i64,
}

pub fn generate(config_params: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut total_accounts_encountered: i64 = 0;
    let mut total_cfs: usize = 0;
    let mut tot_amt_ip: f64 = 0.0;
    let mut tot_amt_op = 0.0;
    let mut acc_read_fail = 0;
    let mut total_accounts_with_cashflows = 0;
    let mut config_map: HashMap<String, CompData> = HashMap::new();
    let start_time = SystemTime::now();
    let (reader, mut writer) = create_io_workers(
        config_params.input_file_path(),
        config_params.output_file_path(),
        log,
    );
    let config_file = match new_buf_rdr(config_params.config_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found config_file: `{}`: {}.",
            config_params.config_file_path(),
            error
        ),
    };
    for line in BufReader::new(config_file).lines().skip(1) {
        let record = match line {
            Ok(ln) => ln,
            Err(error) => {
                panic!("Error while reading input file : {:?}", error);
            }
        };
        let fields: Vec<&str> = record.split("|").collect();
        if fields.len() < 3 {
            continue;
        }
        let data = CompData {
            limit_amt: fields[1].parse::<f64>().unwrap_or(0.0),
            days: num_days(fields[2], config_params.as_on_date()),
        };
        config_map.insert(fields[0].to_string(), data);
    }
    let mut reader_iterator = reader.into_iter();
    if config_params.skip_header() {
        reader_iterator.next();
    }
    loop {
        let account_opt = log_measurements!(
            diag_log,
            [format!(
                "Type: ReadParseInputAccount, Identifier: `{}`",
                total_accounts_encountered
            )],
            reader_iterator.next()
        );

        if account_opt.is_none() {
            break;
        }
        total_accounts_encountered += 1;

        let input_account = match account_opt {
            Some(val) => val,
            None => {
                log_info!(log, "Unable to parse `record`.");
                acc_read_fail += 1;
                continue;
            }
        };
        tot_amt_ip += input_account.amount;
        let account_with_cashflows = log_measurements!(
            diag_log,
            [format!(
                "Type: GenCFs, Identifier: `{}`",
                input_account.account_id
            )],
            create_account_with_cashflows(
                input_account,
                *config_params.as_on_date(),
                config_map.clone()
            )
        );
        if account_with_cashflows.cashflows.len() > 0 {
            total_cfs += account_with_cashflows.cashflows.len();
            total_accounts_with_cashflows += 1;
        }

        log_measurements!(
            diag_log,
            [format!(
                "Type: WriteAccWithCFs, Identifier: {}",
                account_with_cashflows.account_id
            )],
            writer.write(account_with_cashflows)
        );
    }

    writer.close();

    let end_time = SystemTime::now();
    let total_duration = end_time
        .duration_since(start_time)
        .expect("Could not calculate total duration.");
    let report_string = format!(
        "Accounts Encountered: {}\n\
         Accounts With Cashflows: {}\n\
         Total Cashflows: {}\n\
         Total Duration: {:?}\n\
         Total outstanding amount in input: {:.2} \n\
         Total outstanding amount in output: {:.2}\n",
        total_accounts_encountered,
        total_accounts_with_cashflows,
        total_cfs,
        total_duration,
        tot_amt_ip,
        tot_amt_op
    );
    log_info!(log, "{}", report_string);

    let health_stat = HealthReport::new(
        total_accounts_encountered,
        total_cfs as i64,
        acc_read_fail,
        tot_amt_ip,
        tot_amt_op,
        total_cfs as i64,
    );
    health_stat.gen_health_rpt(config_params.output_file_path());
}

fn create_io_workers(
    input_path: &str,
    output_path: &str,
    log: &Logger,
) -> (InputAccountReader, AccountWithCashflowsWriter) {
    let reader = InputAccountReader::new(input_path, log);
    let writer = AccountWithCashflowsWriter::new(output_path, log);

    (reader, writer)
}

fn num_days(info: &str, as_on_date: &NaiveDate) -> i64 {
    if info.contains("D") {
        let period: i64 = info
            .trim_matches('D')
            .parse::<i64>()
            .expect("Invalid from day format");
        return period;
    } else if info.contains("M") {
        let period: usize = info
            .trim_matches('M')
            .parse::<usize>()
            .expect("Invalid from month format");
        let new_date = incr_dt_by_mon_presrv_eom(*as_on_date, period)
            .expect("Cannot add month to as on date as per config");
        return rbdate::num_days_start_to_end(*as_on_date, new_date);
    } else if info.contains("Y") {
        let period: usize = info
            .trim_matches('Y')
            .parse::<usize>()
            .expect("Invalid from year format");
        let new_date = incr_dt_by_mon_presrv_eom(*as_on_date, period * 12)
            .expect("Cannot add month to as on date as per config");
        return rbdate::num_days_start_to_end(*as_on_date, new_date);
    } else {
        panic!("Invalid period type in config file.");
    }
}
