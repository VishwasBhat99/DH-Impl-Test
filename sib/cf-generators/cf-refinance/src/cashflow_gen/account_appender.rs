use crate::configuration_parameters::ConfigurationParameters;
use cashflow_gen::account_reader::input_account::InputAccount;
use cashflow_gen::account_with_cashflows::AccountWithCashflows;
use rbdate::timestamp;
use statics::*;

use super::account_with_cashflows::Cashflow;
// use super::gen_cashflows::Derive_additional_fields;

pub fn create_account_with_cashflows(
    acc: InputAccount,
    total_inst_amt: f64,
    cashflows: Vec<Cashflow>,
    config_params: &ConfigurationParameters,
) -> (AccountWithCashflows, f64, f64, usize) {
    let mut out_acc = AccountWithCashflows::new();
    out_acc.date_of_availment = if let Some(dt) = acc.date_of_availment {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.source = acc.source;
    out_acc.amount = acc.amount;
    out_acc.roi = acc.roi;
    out_acc.mat_date = if let Some(dt) = acc.mat_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.repayment_sch = acc.repayment_sch;
    out_acc.res_mat = acc.res_mat;
    out_acc.frequency = acc.frequency;
    out_acc.pmt_st_dt = if let Some(dt) = acc.pmt_st_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.crncy_code = config_params.currency().to_string();
    out_acc.remaining_prin = acc.remaining_prin;
    out_acc.p_installment = acc.p_installment;
    out_acc.no_of_installment = acc.no_of_installment;
    out_acc.total_inst_amt = total_inst_amt;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows.clone());
    let (mut prin_amt, mut int_amt) = (0.0, 0.0);

    for cf in out_acc.cashflows.iter() {
        int_amt += cf.get_int_amt();
        prin_amt += cf.get_prin_amt();
    }
    (out_acc, int_amt, prin_amt, cashflows.len())
}
