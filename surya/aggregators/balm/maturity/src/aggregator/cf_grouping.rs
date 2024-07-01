use aggregator::dates::naivedate_from_timestamp;
use aggregator::reports::input_report::InputReport;
use aggregator::structs::CashflowAggregatedOnDateBuilder;
use aggregator::AccFieldNames;
use rbdate;
use rbdate::NaiveDate;
use sdb_dyn_proto_rdr::compound_types::Cashflow;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use std::collections::HashMap;
pub struct CashflowsGrouped {
    pub date_grouped_cashflows: HashMap<NaiveDate, CashflowAggregatedOnDateBuilder>,
    pub overdue_cashflows: HashMap<NaiveDate, CashflowAggregatedOnDateBuilder>,
    pub account_amounts_report: InputReport,
    pub account_overdue_amount_report: InputReport,
}

pub fn cashflows_grouped_by_day(
    mut account: AccountWithCFs,
    keys: &AccFieldNames,
    as_on_date: &NaiveDate,
    aggregation_date_limit: NaiveDate,
    default_overdue_llg_code: i32,
    is_rep_mandatory: bool,
    is_npa: bool,
) -> CashflowsGrouped {
    let mut account_amounts_report = InputReport::new();
    //Overdue amount report structure is the same as the Input Report
    let mut account_overdue_amount_report = InputReport::new();
    let mut map: HashMap<NaiveDate, CashflowAggregatedOnDateBuilder> = HashMap::new();
    let mut overdue_map: HashMap<NaiveDate, CashflowAggregatedOnDateBuilder> = HashMap::new();
    let mut interest_rate = account
        .get_f64_for_key(&keys.interest_rate)
        .expect("Cannot read interest rate from .cf file.");
    let mut repricing_date: Option<NaiveDate> = match account.get_i64_for_key(&keys.next_rep_date) {
        Ok(val) => {
            if val == 0 {
                None
            } else {
                Some(naivedate_from_timestamp(val))
            }
        }
        Err(_err) => None,
    };
    if is_npa {
        interest_rate = 0.0;
        repricing_date = None;
    }
    for cf in account
        .remove_cfs_for_key(&keys.cashflows)
        .expect("Error while removing cashflow from the pool of cashflows.")
        .iter_mut()
    {
        account_amounts_report.add_cf_values(cf);
        // If this cashflow date is after the aggregation date limit, pretend it's
        // on the last day of the aggregation.
        let mut cf_date = naivedate_from_timestamp(cf.get_date());
        if cf_date > aggregation_date_limit {
            cf_date = aggregation_date_limit;
        }
        if let Some(rep_dt) = repricing_date {
            if &rep_dt <= as_on_date {
                repricing_date = Some(as_on_date.succ());
            }
            if rep_dt > aggregation_date_limit {
                repricing_date = Some(aggregation_date_limit);
            }
        }
        if &cf_date <= as_on_date {
            cf_date = as_on_date.succ();
            if default_overdue_llg_code == 0 {
                insert_slr_int(cf, cf_date, 1, interest_rate, &mut map);
                insert_irs(
                    as_on_date,
                    cf,
                    cf_date,
                    repricing_date,
                    interest_rate,
                    &mut map,
                    is_rep_mandatory,
                );
            } else {
                account_overdue_amount_report.add_cf_values(cf);
                //passed 1 as the day_num value because cf date is before as on date
                insert_slr_int(cf, cf_date, 1, interest_rate, &mut overdue_map);
                insert_irs(
                    as_on_date,
                    cf,
                    cf_date,
                    repricing_date,
                    interest_rate,
                    &mut overdue_map,
                    is_rep_mandatory,
                );
            }
        } else {
            let day_num = rbdate::num_days_start_to_end(*as_on_date, cf_date);
            insert_slr_int(cf, cf_date, day_num, interest_rate, &mut map);
            insert_irs(
                as_on_date,
                cf,
                cf_date,
                repricing_date,
                interest_rate,
                &mut map,
                is_rep_mandatory,
            );
        }
    }
    CashflowsGrouped {
        date_grouped_cashflows: map,
        overdue_cashflows: overdue_map,
        account_amounts_report,
        account_overdue_amount_report,
    }
}

fn insert_slr_int(
    cf: &Cashflow,
    cf_date: NaiveDate,
    day_num: i64,
    interest_rate: f64,
    map: &mut HashMap<NaiveDate, CashflowAggregatedOnDateBuilder>,
) {
    let mut cf_builder = CashflowAggregatedOnDateBuilder::new();
    cf_builder.add_slr_int_cf(cf, interest_rate, day_num);
    map.entry(cf_date)
        .and_modify(|m| m.add_slr_int_cf(cf, interest_rate, day_num))
        .or_insert(cf_builder);
}

fn insert_irs(
    as_on_date: &NaiveDate,
    cf: &Cashflow,
    cf_date: NaiveDate,
    repricing_date: Option<NaiveDate>,
    interest_rate: f64,
    map: &mut HashMap<NaiveDate, CashflowAggregatedOnDateBuilder>,
    is_rep_mandatory: bool,
) {
    let date = date_for_irs(repricing_date, cf_date, is_rep_mandatory);
    let day_num = rbdate::num_days_start_to_end(*as_on_date, date);
    map.entry(date)
        .or_insert(CashflowAggregatedOnDateBuilder::new())
        .add_irs_cf(cf, interest_rate, day_num);
}

fn date_for_irs(
    repricing_date: Option<NaiveDate>,
    cf_date: NaiveDate,
    is_rep_mandatory: bool,
) -> NaiveDate {
    if let Some(rep_date) = repricing_date {
        if is_rep_mandatory {
            rep_date
        } else {
            if cf_date > rep_date {
                rep_date
            } else {
                cf_date
            }
        }
    } else {
        cf_date
    }
}
