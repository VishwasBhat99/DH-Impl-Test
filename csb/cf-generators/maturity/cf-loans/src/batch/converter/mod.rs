use macros;
pub mod account;
mod cashflow_appender;
pub mod generate_cashflows;
pub mod input;

use self::cashflow_appender::create_account_with_cashflows;
use self::generate_cashflows::generate;

use self::account::Account;
use self::account::Cashflow;
use self::input::Input;
use super::super::statics::*;
use super::output_descriptor::AccountDescriptor;
use rbdate::{timestamp, NaiveDate};
use sdb_day_convention::conventions::Conventions;
use sdb_day_convention::days_with_convn;
use slog::Logger;

pub fn convert(
    input_account: Input,
    convention: Conventions,
    as_on_date: NaiveDate,
    is_contractual: bool,
    log: &Logger,
) -> Result<(Account, AccountDescriptor), String> {
    let cfs_vec = generate(&input_account, convention, as_on_date, is_contractual, log);

    Ok(create_account_with_cashflows(input_account, cfs_vec))
}
