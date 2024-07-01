use crate::cashflow_derivator::account::OutputAccount;
use crate::cashflow_derivator::AccsData;
use crate::cashflow_derivator::KeyID;
use crate::configuration_parameters::ConfigurationParameters;

pub fn create_account(
    config_params: &ConfigurationParameters,
    keypair: &KeyID,
    data: &AccsData,
) -> OutputAccount {
    let mut out_acc = OutputAccount::new();

    out_acc.as_on_date = config_params.as_on_date().format("%d-%m-%Y").to_string();
    out_acc.custid = keypair.custid;
    out_acc.classid = keypair.classid;
    out_acc.curr = data.curr.to_string();
    out_acc.tot_amt = data.tot_amt;
    out_acc.tot_nwd_amt = data.tot_nwd_amt;
    out_acc.tot_accs = data.tot_accs;
    out_acc.tot_nwd_accs = data.tot_nwd_accs;
    out_acc.ca_accs = data.ca_accs;
    out_acc.sa_accs = data.sa_accs;
    out_acc.td_accs = data.td_accs;
    out_acc.rd_accs = data.rd_accs;
    out_acc.ca_nwd_accs_op = data.ca_nwd_accs_op;
    out_acc.ca_nwd_accs_nonop = data.ca_nwd_accs_nonop;
    out_acc.sa_nwd_accs = data.sa_nwd_accs;
    out_acc.td_nwd_accs = data.td_nwd_accs;
    out_acc.rd_nwd_accs = data.rd_nwd_accs;
    out_acc.td_amt = data.td_amt;
    out_acc.rd_amt = data.rd_amt;
    out_acc.sa_amt = data.sa_amt;
    out_acc.ca_amt = data.ca_amt;

    out_acc
}
