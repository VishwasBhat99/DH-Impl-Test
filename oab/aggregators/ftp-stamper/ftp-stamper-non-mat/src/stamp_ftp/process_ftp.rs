use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::timestamp;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_agg_rules_adj::agg_rules::AggRules_adj;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use stamp_ftp::account_with_cashflows::AccountWithCashflows;
use stamp_ftp::calc_ftp;
use stamp_ftp::rule_stamper;
use statics::DEFAULT_FLOAT;
use std::collections::HashMap;

pub fn calculate_ftp(
    acc_data: &mut AccountWithCFs,
    mut cf_data: AccountWithCashflows,
    m_rules: &AggRules,
    bc_rules: &AggRules,
    adj_rules: &AggRules_adj,
    config_params: &ConfigurationParameters,
    log: &Logger,
    diag_log: &Logger,
    avg_bal: &HashMap<String, f64>,
) -> AccountWithCashflows {
    let method = rule_stamper::get_method(
        &cf_data.account_id,
        &acc_data,
        &m_rules,
        config_params.default_method(),
        diag_log,
    );

    let basecurve = rule_stamper::get_bc(
        &cf_data.account_id,
        &acc_data,
        &bc_rules,
        config_params.default_basecurve(),
        diag_log,
    );

    let lst_adjustments =
        rule_stamper::get_adj(&cf_data.account_id, &acc_data, &adj_rules, diag_log);

    cf_data.balance_ccy = match avg_bal.get(&cf_data.account_id) {
        Some(val) => *val,
        None => DEFAULT_FLOAT,
    };
    cf_data.as_on_month = timestamp(*config_params.to_date());

    let cf_data_out = calc_ftp::calc_ftp(
        cf_data,
        method,
        basecurve,
        lst_adjustments,
        &config_params,
        log,
    );
    cf_data = cf_data_out;
    cf_data.ftp_method = method.to_string();
    cf_data.base_rate_curve_id = basecurve.to_string();

    let log_str_method = format!(
        "Method Assigned for Account {} is : {}",
        &cf_data.account_id, method
    );
    log_debug!(diag_log, "{}", log_str_method);

    let log_str_bc = format!(
        "Base curve Assigned for Account {} is : {}",
        &cf_data.account_id, basecurve
    );
    log_debug!(diag_log, "{}", log_str_bc);

    cf_data
}
