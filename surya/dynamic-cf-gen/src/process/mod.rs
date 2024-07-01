use self::account::generate_new_acc;
use self::cashflows::generate_cfs;
use self::op_gen::cashflow_appender::create_account_with_cashflows;
use self::op_gen::AccountWithCashflows;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_day_convention::Conventions;
use slog::Logger;
use std::io::prelude::*;

mod account;
mod cashflows;
mod op_gen;
mod read_config;
mod util;

pub fn generate(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let mut op_writer = AccountWithCashflows::new(config_params.output_file_path(), logger);

    // Generate collection of new accounts based on the split logic (https://docs.google.com/spreadsheets/d/1fs4UarKkr6lv1PNPfJfWhpmbfF0ObBa-/edit#gid=281035490)
    let new_accounts = generate_new_acc(config_params, logger, diag_logger);
    // Map interest basis convention with cf gen variant type
    let convection = match config_params.interest_basis() {
        "ACTby360" => Conventions::ACTby360,
        "ACTbyACT" => Conventions::ACTbyACT,
        "ACTby365" => Conventions::ACTby365,
        "Thirtyby360" => Conventions::Thirtyby360,
        _ => {
            log_warn!(logger,"Using default interest basis convention: ACT/ACT due to error reading from config file.");
            Conventions::ACTbyACT
        }
    };
    let mut total_prin_amt = 0.0;
    let mut total_cfs = 0;
    let mut total_prin_cfs = 0;
    let mut tot_acc = 0;
    // Read Pre Payment rates file
    let pp_map = get_pp_rates(config_params.pre_payment_rates_file());
    log_info!(logger, "Total Accounts Generated: {}", new_accounts.len());
    for account in new_accounts {
        tot_acc += 1;
        let (cfs, acc_prin_amt, _acc_int_amt) =
            generate_cfs(&account, convection, &pp_map, &config_params, logger);
        total_prin_amt += acc_prin_amt;
        for cf in &cfs {
            if cf.principal_amount != 0.0 {
                total_prin_cfs += 1;
            }
        }
        total_cfs += cfs.len();
        log_debug!(diag_logger, "Account CF's: {:?}\n{:?}", account.acc_id, cfs);

        let account_with_cashflows = create_account_with_cashflows(account, cfs);
        op_writer.write(account_with_cashflows);
    }
    op_writer.close();
    log_info!(logger, "Total Cashflows Generated: {}", total_cfs);
    let health_report = HealthReport::new(
        1,
        tot_acc,
        0,
        *config_params.new_business_value(),
        total_prin_amt,
        total_prin_cfs as i64,
    );
    health_report.gen_health_rpt(&config_params.output_file_path());
}

pub fn get_pp_rates(path: &str) -> Vec<f64> {
    let mut data: Vec<f64> = Vec::new();
    let pp_reader = sdb_io::new_buf_rdr(path).expect("Cannot open pre payment rates file!");
    for (_, lines) in pp_reader.lines().enumerate() {
        let line = lines.expect("Cannot read data from pre payment rates file!");
        let line_info: Vec<&str> = line.split('|').collect();
        for v in line_info {
            data.push(v.parse().expect("Cannot convert pp rate to f64."));
        }
    }
    data
}
