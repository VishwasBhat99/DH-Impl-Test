use super::bucket::get_bucket_no;
use aggregator::cashflow_organizer::cashflow_aggregation::CashflowAggregatedOnDateBuilder;
use aggregator::reports::input_report::InputReport;
use aggregator::AccFieldNames;
use sdb_dyn_proto_rdr::compound_types::Cashflow;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use std::collections::HashMap;

#[derive(Debug)]
pub struct CashflowsGrouped {
    pub bkt_grpd_cfs: HashMap<i64, CashflowAggregatedOnDateBuilder>,
    pub acc_amts_rpt: InputReport,
}

pub fn cashflows_grouped_by_day(
    mut account: AccountWithCFs,
    k: &AccFieldNames,
    tenor: i64,
) -> CashflowsGrouped {
    let mut acc_amts_rpt = InputReport::new();
    let mut map: HashMap<i64, CashflowAggregatedOnDateBuilder> = HashMap::new();
    let interest_rate = account
        .get_f64_for_key(&k.int_rt)
        .expect("Error while getting interest rate.");

    for cf in account
        .remove_cfs_for_key(&k.cashflows)
        .expect("Error while reading cashflows.")
        .iter_mut()
    {
        acc_amts_rpt.add_cf_values(cf);
        let bkt_no = get_bucket_no(tenor);
        insert_outstanding(cf, bkt_no, interest_rate, &mut map);
    }

    CashflowsGrouped {
        bkt_grpd_cfs: map,
        acc_amts_rpt,
    }
}

fn insert_outstanding(
    cf: &Cashflow,
    bkt_no: i64,
    interest_rate: f64,
    map: &mut HashMap<i64, CashflowAggregatedOnDateBuilder>,
) {
    let mut cf_builder = CashflowAggregatedOnDateBuilder::new();
    cf_builder.add_outstanding_cf(cf, interest_rate);
    map.entry(bkt_no)
        .and_modify(|m| m.add_outstanding_cf(cf, interest_rate))
        .or_insert(cf_builder);
}
