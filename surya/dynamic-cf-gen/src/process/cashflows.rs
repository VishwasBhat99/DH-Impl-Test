use super::account::AccountData;
use super::op_gen::op_account::Cashflow;
use configuration_parameters::ConfigurationParameters;
use process::Logger;
use sdb_cf_gen::cf_gen;
use sdb_cf_gen::enums::CashflowType;
use sdb_cf_gen::structs::AccData;
use sdb_cf_gen::structs::Cashflow as SdbCashflow;
use sdb_day_convention::Conventions;

pub fn generate_cfs(
    account: &AccountData,
    convention: Conventions,
    pp_rates: &Vec<f64>,
    config_params: &ConfigurationParameters,
    _logger: &Logger,
) -> (Vec<Cashflow>, f64, f64) {
    let mut cashflows: Vec<Cashflow> = Vec::new();
    let mut acc_prin_amt = 0.0;
    let mut acc_int_amt = 0.0;
    let new_acc = AccData {
        as_on_date: *config_params.as_on_date(),
        acc_start_date: account.acc_open_date,
        maturity_date: account.maturity_date,
        ost_bal: account.os_amount,
        int_rate: account.int_rate as f32,
        comp_freq: if config_params.cf_type() == "Compounding" {
            match &account.int_pay_freq[..] {
                "M" => Some(1),
                "Q" => Some(3),
                "H" => Some(6),
                "Y" => Some(12),
                _ => None,
            }
        } else {
            None
        },
        installment_amt: None,
        int_payout_freq: Some(account.payout_freq.to_string()),
        pre_payment_rates: pp_rates,
        convention: convention,
    };
    let cfs: Vec<SdbCashflow> = match cf_gen(new_acc, CashflowType::Simple) {
        Ok(cf) => cf,
        Err(error) => error.default_cfs,
    };
    for cf in &cfs {
        cashflows.push(new_cashflow(cf.int_amt, cf.prin_amt, cf.cf_date));
        acc_prin_amt = cf.prin_amt;
        acc_int_amt = cf.int_amt;
    }
    return (cashflows, acc_prin_amt, acc_int_amt);
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = d;

    cf
}
