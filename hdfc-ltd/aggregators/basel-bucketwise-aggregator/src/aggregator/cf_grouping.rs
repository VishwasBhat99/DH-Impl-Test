use aggregator::dates::naivedate_from_timestamp;
use aggregator::input_report::InputReport;
use aggregator::structs::AggregateData;
use aggregator::AccFieldNames;
use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::incr_dt_by_days;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use std::collections::HashMap;
pub struct CashflowsGrouped {
    pub date_grouped_cashflows: HashMap<NaiveDate, AggregateData>,
    pub account_amounts_report: InputReport,
}

pub fn cashflows_grouped_by_day(
    mut account: AccountWithCFs,
    k: &AccFieldNames,
    config_params: &ConfigurationParameters,
    aggregation_date_limit: NaiveDate,
    logger: &Logger,
) -> CashflowsGrouped {
    let mut map: HashMap<NaiveDate, AggregateData> = HashMap::new();
    let mut account_amounts_report = InputReport::new();
    let mut cfs = match account.remove_cfs_for_key(&k.cashflows) {
        Ok(val) => val,
        Err(err) => {
            log_info!(
                logger,
                "Account: {} \n Error while removing cashflow from the pool of cashflows.{:#?}",
                account
                    .get_string_for_key(&k.account_number)
                    .unwrap_or(&"-1".to_string()),
                err
            );
            Vec::new()
        }
    };
    for cf in cfs.iter_mut() {
        account_amounts_report.add_cf_values(cf);
        let mut cf_date = naivedate_from_timestamp(cf.get_date());
        //Aggregation for amounts after 2 years is done to a single bucket.
        if cf_date > aggregation_date_limit {
            cf_date = aggregation_date_limit.succ();
        }

        let amt = if config_params.amt_type() == "PRIN" {
            cf.get_principal_amount()
        } else {
            cf.get_interest_amount()
        };
        insert_prin_amt(amt, cf_date, &mut map);
    }
    CashflowsGrouped {
        date_grouped_cashflows: map,
        account_amounts_report,
    }
}

fn insert_prin_amt(amt: f64, cf_date: NaiveDate, map: &mut HashMap<NaiveDate, AggregateData>) {
    let mut cf_builder = AggregateData::new();
    cf_builder.add_amount(amt);
    map.entry(cf_date)
        .and_modify(|m| m.add_amount(amt))
        .or_insert(cf_builder);
}
