use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_without_cashflows::LCAccount;
use rbdate::NaiveDate;
use slog::Logger;

pub fn create_account_without_cashflows(
    input_account: InputAccount,
    exp_date: NaiveDate,
    tenor: i64,
    acc_fin_val: &str,
    consol_amt: f64,
    interelemination_flg: &str,
    _log: &Logger,
) -> LCAccount {
    let mut out_acc = LCAccount::new();

    out_acc.natural_acc = input_account.natural_acc;
    out_acc.ref_no = input_account.ref_no;
    out_acc.acc_ccy = input_account.acc_ccy;
    out_acc.exp_dt = rbdate::timestamp(exp_date);
    out_acc.amt = consol_amt;
    out_acc.interelemination_flg = interelemination_flg.to_string();
    out_acc.tenor = tenor;
    out_acc.fin_field = acc_fin_val.to_string();

    out_acc
}
