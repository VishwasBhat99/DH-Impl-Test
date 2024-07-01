use aggregator::account_field_names::AccFieldNames;
use rbdate::NaiveDate;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;

pub fn group_cfs(
    mut account: AccountWithCFs,
    keys: &AccFieldNames,
    as_on_date: &NaiveDate,
) -> Vec<f64> {
    let mut buckets: Vec<f64> = vec![0.0, 0.0, 0.0];
    let limit_1 = rbdate::timestamp(
        rbdate::incr_dt_by_mon_presrv_eom(*as_on_date, 6).expect("Cannot set bucket 1 limit."),
    );
    let limit_2 = rbdate::timestamp(
        rbdate::incr_dt_by_mon_presrv_eom(*as_on_date, 12).expect("Cannot set bucket 2 limit."),
    );
    for cf in account
        .remove_cfs_for_key(&keys.cashflows)
        .expect("Error while removing cashflow from the pool of cashflows.")
        .iter_mut()
    {
        let cf_dt = cf.get_date();
        if cf_dt <= limit_1 {
            buckets[0] += cf.principal_amount;
        } else if cf_dt > limit_1 && cf_dt <= limit_2 {
            buckets[1] += cf.principal_amount;
        } else {
            buckets[2] += cf.principal_amount;
        }
    }
    buckets
}
