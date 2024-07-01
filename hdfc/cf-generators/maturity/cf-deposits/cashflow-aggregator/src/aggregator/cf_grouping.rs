// TODO: Lib
use std::collections::HashMap;
use chrono::NaiveDate;
use aggregator::structs::CashflowAggregatedOnDateBuilder;
use aggregator::dates::naivedate_from_timestamp;
use aggregator::reports::input_report::InputReport;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use aggregator::AccFieldNames;
use sdb_dyn_proto_rdr::compound_types::Cashflow;
use rbdate;
pub struct CashflowsGrouped {
    pub date_grouped_cashflows: HashMap<NaiveDate, CashflowAggregatedOnDateBuilder>,
    pub account_amounts_report: InputReport
}

pub fn cashflows_grouped_by_day(
    mut account: AccountWithCFs,
    k: &AccFieldNames,
    as_on_date: &NaiveDate,
    aggregation_date_limit: i64
) -> CashflowsGrouped {

    let mut account_amounts_report = InputReport::new();
    let mut map: HashMap<NaiveDate, CashflowAggregatedOnDateBuilder> = HashMap::new();
    let interest_rate = account.get_f64_for_key(&k.interest_rate).expect("fail");
    let repricing_date: Option<NaiveDate> = {
        // REVIEW: This is different across programs
        //
        // This program has no repricing date.
        // Use None for now
        // Look at git history whenever we need to restore.
        None
    };

    for cf in account.remove_cfs_for_key(&k.cashflows).expect("fail").iter_mut() {
        account_amounts_report.add_cf_values(cf);
        // If this cashflow is more than the aggregation date limit, pretend it's
        // on the day of the aggregation.
        if cf.get_date() > aggregation_date_limit {
            cf.set_date(aggregation_date_limit)
        }
        let mut cf_date = naivedate_from_timestamp(cf.get_date());
        if &cf_date < as_on_date {
            // If this cashflow was before the as-on-date
            // this account has defaulted a payment.
            // Pretend this cashflow was on the as-on-date.
            cf_date = *as_on_date;
        }
        let day_num = rbdate::num_days_start_to_end(as_on_date, &cf_date);
        insert_slr_int(cf, cf_date, day_num, interest_rate, &mut map);
        insert_irs(cf, cf_date, day_num, repricing_date, interest_rate, &mut map);
    }

    CashflowsGrouped {
        date_grouped_cashflows: map,
        account_amounts_report,
    }
}

fn insert_slr_int(
    cf: &Cashflow,
    cf_date: NaiveDate,
    day_num: i64,
    interest_rate: f64,
    map: &mut HashMap<NaiveDate, CashflowAggregatedOnDateBuilder>
) {
    if map.get(&cf_date).is_some() {
        map.get_mut(&cf_date)
            .unwrap()
            .add_slr_int_cf(
                cf,
                interest_rate,
                day_num
            );
    } else {

        let mut cf_builder = CashflowAggregatedOnDateBuilder::new();
        cf_builder
            .add_slr_int_cf(
                cf,
                interest_rate,
                day_num
            );
        map.insert(cf_date, cf_builder);
    }

}

fn insert_irs(
    cf: &Cashflow,
    cf_date: NaiveDate,
    day_num: i64,
    repricing_date: Option<NaiveDate>,
    interest_rate: f64,
    map: &mut HashMap<NaiveDate, CashflowAggregatedOnDateBuilder>
) {
    let date = date_for_irs(repricing_date, cf_date);
    map.entry(date)
        .or_insert(CashflowAggregatedOnDateBuilder::new())
        .add_irs_cf(cf, interest_rate, day_num);

}


fn date_for_irs(repricing_date: Option<NaiveDate>, cf_date: NaiveDate) -> NaiveDate {
    if repricing_date.is_none() {
        return cf_date;
    }
    let r_date_unwrapped = repricing_date.unwrap();
    if cf_date > r_date_unwrapped {
        return r_date_unwrapped;
    } else {
        return cf_date;
    }
}