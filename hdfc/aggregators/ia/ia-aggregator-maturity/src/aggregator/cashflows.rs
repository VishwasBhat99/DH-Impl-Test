use super::macros;
use aggregator::RequiredFields;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct CashflowVec {
    pub date: Vec<i64>,
    pub date_amt_map: HashMap<i64, f64>,
}

pub fn get_cashflow(
    mut account: AccountWithCFs,
    req_fields: &RequiredFields,
    log: &Logger,
) -> CashflowVec {
    let mut date_amt_map = HashMap::new();
    let mut date_vec: Vec<i64> = Vec::new();
    let mut cfs = match account.remove_cfs_for_key(&req_fields.cashflows) {
        Ok(cfs) => cfs,
        Err(_) => {
            log_info!(log, "cashflows are not present for the account.");
            return CashflowVec {
                date: date_vec,
                date_amt_map: date_amt_map,
            };
        }
    };
    for cf in cfs.iter_mut() {
        if !date_vec.contains(&cf.date) {
            date_vec.push(cf.date);
        }
        if date_amt_map.contains_key(&cf.date) {
            let amt = date_amt_map
                .get(&cf.date)
                .expect("amt for date from date amt map");
            cf.principal_amount = cf.principal_amount + amt;
        }
        date_amt_map.insert(cf.date, cf.principal_amount);
    }

    CashflowVec {
        date: date_vec,
        date_amt_map: date_amt_map,
    }
}
