use self::account::generate_new_acc;
use self::op_gen::cashflow_appender::create_account;
use self::op_gen::AccountWithCashflows;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use slog::Logger;

mod account;
mod op_gen;
mod read_config;
mod util;

pub fn generate(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let mut op_writer = AccountWithCashflows::new(config_params.output_file_path(), logger);

    let new_accounts = generate_new_acc(config_params, logger, diag_logger);
    let mut tot_acc = 0;
    log_info!(logger, "Total Accounts Generated: {}", new_accounts.len());
    for account in new_accounts {
        tot_acc += 1;
        let op_account = create_account(account);
        op_writer.write(op_account);
    }
    op_writer.close();
    let health_report = HealthReport::new(
        new_accounts.len(),
        tot_acc,
        0,
        config_params.existing_business_value()(),
        config_params.prj_business_value(),
        0,
    );
    health_report.gen_health_rpt(&config_params.output_file_path());
}
