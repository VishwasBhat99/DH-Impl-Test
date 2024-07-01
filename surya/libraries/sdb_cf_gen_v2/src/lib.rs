extern crate rbdate;
extern crate sdb_day_convention;
mod cf_date_iterator;
pub mod enums;
pub mod structs;
use enums::CashflowType;
mod calc_cfs;
use calc_cfs::compunding_cf::generate_compounding_interest_cfs;
use calc_cfs::error_cf::check_error_cases;
use calc_cfs::simple_cf::generate_simple_interest_cfs;
use structs::*;

pub fn cf_gen(acc_data: AccData, cf_type: CashflowType) -> Result<Vec<Cashflow>, ErrorMsg> {
    //check for error cases
    let error_case = check_error_cases(&acc_data);
    if error_case.error_code != 0 {
        return Err(error_case);
    }
    //check all other cases
    match cf_type {
        CashflowType::Simple => Ok(generate_simple_interest_cfs(acc_data)),
        CashflowType::Compounding => Ok(generate_compounding_interest_cfs(acc_data)),
    }
}
