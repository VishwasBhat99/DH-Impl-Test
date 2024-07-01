use super::account_as_cashflows::Cashflow;
use super::account_as_cashflows::LienAccount;
use configuration_parameters::ConfigurationParameters;

pub fn append_data<'a>(
    acc_id: String,
    ccy: String,
    amt: f64,
    tenor: i64,
    cust_type: String,
    config_params: &ConfigurationParameters,
) -> LienAccount {
    let mut out_acc = LienAccount::new();

    // as lien data should be written as negative value
    let final_amt = amt * -1.0;
    let mut cashflows = Vec::new();
    let mut cf = Cashflow::new();
    cf.interest_amount = final_amt;
    cf.principal_amount = final_amt;
    cf.date = rbdate::timestamp(*config_params.as_on_date());
    cashflows.push(cf);

    out_acc.acc_id = acc_id;
    out_acc.ccy = ccy;
    out_acc.tenor = tenor.to_string();
    out_acc.cust_type = cust_type;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
