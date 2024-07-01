use crate::cashflow_derivator::account::OutputAccount;
use crate::cashflow_derivator::{Data, Key};
use crate::configuration_parameters::ConfigurationParameters;

pub fn create_account(
    config_params: &ConfigurationParameters,
    acc_no: &Key,
    data: &Data,
) -> OutputAccount {
    let mut out_acc = OutputAccount::new();

    out_acc.as_on_date = config_params.as_on_date().format("%d-%m-%Y").to_string();
    out_acc.acc_no = acc_no.acc_no.parse::<f64>().unwrap_or(0.0);
    out_acc.custid = data.custid;
    out_acc.classid = data.classid;
    out_acc.curr = data.curr.to_string();
    out_acc.mat_date = data.mat_date.format("%d-%m-%Y").to_string();
    out_acc.tot_amt = data.tot_amt;
    out_acc.tot_nwd_amt = data.tot_nwd_amt;

    out_acc
}
