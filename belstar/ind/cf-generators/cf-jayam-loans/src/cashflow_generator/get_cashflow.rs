use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::{account_with_cashflows::AccountWithCashflows, Cashflow};

pub fn get_balance_cashflow(
    input_account: &InputAccount,
    acc_with_cf: &mut AccountWithCashflows,
    cfdate: i64,
) {
    let sec_percent = input_account.securitization_percentage.parse::<f64>().unwrap_or(100.0);
    let cashflow = new_cashflow(
        input_account.interest * (sec_percent / 100.00),
        input_account.principal * (sec_percent / 100.00),
        cfdate,
    );
    acc_with_cf.securitization_percentage = sec_percent;
    let mut cfs: Vec<Cashflow> = Vec::new();
    acc_with_cf.total_principal_amount = cashflow.principal_amount;
    acc_with_cf.total_interest_amount = cashflow.interest_amount;
    cfs.push(cashflow);
    acc_with_cf.cashflows = protobuf::RepeatedField::from_vec(cfs);
}

pub fn get_securitized_cashflow(
    input_account: &InputAccount,
    acc_with_cf_sec: &mut AccountWithCashflows,
    cfdate: i64,
) {
    let sec_percent = input_account.securitization_percentage.parse::<f64>().unwrap_or(100.0);
    let cashflow = new_cashflow(
        input_account.interest * ((100.0 - sec_percent) / 100.00),
        input_account.principal * ((100.0 - sec_percent) / 100.00),
        cfdate,
    );
    acc_with_cf_sec.securitization_percentage = sec_percent;
    let mut cfs: Vec<Cashflow> = Vec::new();
    acc_with_cf_sec.total_principal_amount = cashflow.principal_amount;
    acc_with_cf_sec.total_interest_amount = cashflow.interest_amount;
    cfs.push(cashflow);
    acc_with_cf_sec.cashflows = protobuf::RepeatedField::from_vec(cfs);
}

pub fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.principal_amount = p_a;
    cf.interest_amount = i_a;
    cf.cfdate = d;
    cf
}
