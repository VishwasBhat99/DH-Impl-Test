use super::read_config::read_config_files;
use super::util::add_days;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::NaiveDate;
use slog::Logger;

#[derive(Debug)]
pub struct AccountData {
    pub acc_id: String,
    pub acc_open_date: NaiveDate,
    pub os_amount: f64,
    pub currency: String,
}

pub fn generate_new_acc(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) -> Vec<AccountData> {
    let dis_amt_map = read_config_files(config_params, logger, diag_logger);

    // DataStructure to store all new hypothetical accounts
    let mut new_accounts: Vec<AccountData> = Vec::new();
    let mut tot_amt_in_op = 0.0;

    let mut acc_count = 1;
    let mut current_os_amt = config_params.existing_business_value();
    for (day, day_weightage) in dis_amt_map.iter().enumerate() {
        let acc_open_date = add_days(config_params.as_on_date(), &(day as u8));
        let acc_os_amount_by_day = current_os_amt * day_weightage / 100.0;
        let new_acc = AccountData {
            acc_id: format!("ACC{}", acc_count),
            acc_open_date: acc_open_date,
            os_amount: acc_os_amount_by_day,
            currency: config_params.currency().to_string(),
        };
        new_accounts.push(new_acc);
        acc_count += 1;
        tot_amt_in_op += acc_os_amount_by_day;
        current_os_amt += acc_os_amount_by_day;
    }
    // Final Amount Settlement
    if current_os_amt != config_params.prj_business_value() {
        let mut acc = new_accounts
            .pop()
            .expect("Unable to pop a value from new account collection.");
        let final_settlement = config_params.prj_business_value() - current_os_amt;
        acc.os_amount += final_settlement;
        new_accounts.push(acc);
        tot_amt_in_op += final_settlement;
    }
    log_debug!(diag_logger, "New Accounts: \n {:#?}", new_accounts);
    log_info!(
        logger,
        "Total Amount for Accounts Generated: {}",
        tot_amt_in_op
    );
    return new_accounts;
}
