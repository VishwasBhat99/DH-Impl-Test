use aggregator::RequiredFields;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;

pub fn get_amount(mut account: AccountWithCFs, req_fields: &RequiredFields) -> f64 {
    let mut amt: f64 = 0.0;
    for cf in account
        .remove_cfs_for_key(&req_fields.cashflows)
        .expect("cashflow read json error")
        .iter_mut()
    {
        amt = cf.principal_amount;
    }

    amt
}
