use std::collections::HashMap;

use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use cashflow_generator::account_with_cashflows_writer::AccountWithCashflowsWriter;
use rbdate::timestamp;
use rbdate::NaiveDate;

use crate::configuration_parameters::ConfigurationParameters;

use super::structure::FieldStructure;

pub fn generate_cashflows(
    config_param: &ConfigurationParameters,
    cf_date_bucket: HashMap<String, NaiveDate>,
    data_map: HashMap<String, FieldStructure>,
    writer: &mut AccountWithCashflowsWriter,
    sheet_name: &str,
) {
    for (key, value) in data_map {
        let split_key_data = key.split("-").collect::<Vec<&str>>();
        let cf_date = cf_date_bucket
            .get(split_key_data[1])
            .unwrap_or(config_param.as_on_date());
        let mut account_with_cashflows = AccountWithCashflows::new();
        account_with_cashflows.llg_desp = split_key_data[0].to_string();
        account_with_cashflows.currency = sheet_name.to_string();
        account_with_cashflows.notional = value.notional;
        account_with_cashflows.coupons = value.coupons;
        account_with_cashflows.field_yield = value.yields;
        account_with_cashflows.mod_duration = value.mod_duration;

        let mut cf: Cashflow = Cashflow::new();
        cf.interest_amount = 0.0;
        cf.principal_amount = value.notional;
        cf.date = timestamp(*cf_date);

        account_with_cashflows.cashflows = protobuf::RepeatedField::from_vec(vec![cf]);
        writer.write(account_with_cashflows);
    }
}
