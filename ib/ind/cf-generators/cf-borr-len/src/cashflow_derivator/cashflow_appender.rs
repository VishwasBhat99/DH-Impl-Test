use crate::configuration_parameters;
use crate::configuration_parameters::ConfigurationParameters;
use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use rbdate::{timestamp, NaiveDate};

pub fn create_account_with_cashflows(
    input_account: InputAccount,
    config_params: &ConfigurationParameters,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut op = AccountWithCashflows::new();
    op.deal_ref = input_account.deal_ref;
    op.intt_name = input_account.intt_name;
    op.cntpty_name = input_account.cntpty_name;
    op.deal_date = {
        if input_account.deal_date.is_some() {
            rbdate::timestamp(
                input_account
                    .deal_date
                    .unwrap_or(*config_params.as_on_date()),
            )
        } else {
            0
        }
    };
    op.value_date = {
        if input_account.value_date.is_some() {
            rbdate::timestamp(
                input_account
                    .value_date
                    .unwrap_or(*config_params.as_on_date()),
            )
        } else {
            0
        }
    };
    op.mat_date = {
        if input_account.mat_date.is_some() {
            rbdate::timestamp(
                input_account
                    .mat_date
                    .unwrap_or(*config_params.as_on_date()),
            )
        } else {
            0
        }
    };
    op.deal_amt_act = input_account.deal_amt_act;
    op.deal_amt_plc = input_account.deal_amt_plc;
    op.roi = input_account.roi;
    op.int_amt_fx_deal = input_account.int_amt_fx_deal;
    op.mat_amt_fx_deal = input_account.mat_amt_fx_deal;
    op.practice = input_account.practice;
    op.spread = input_account.spread;
    op.benchmark = input_account.benchmark;

    op.rate_sett_freq = input_account.rate_sett_freq;
    op.sett_freq = input_account.sett_freq;
    op.sett_typ = input_account.sett_typ;
    op.dealer = input_account.dealer;
    op.cntpty_id = input_account.cntpty_id;
    op.inv_curcy = input_account.inv_curcy;
    op.gl_code = input_account.gl_code;
    op.int_type = input_account.int_type;
    op.cgl = input_account.cgl;
    op.group = input_account.group;
    op.llg = input_account.llg;
    op.cf_type = input_account.cf_type;
    op.cf_currency = input_account.cf_currency;
    op.cf_amount = input_account.cf_amount;
    op.cf_date = {
        if input_account.cf_date.is_some() {
            rbdate::timestamp(input_account.cf_date.unwrap_or(*config_params.as_on_date()))
        } else {
            0
        }
    };
    op.cf_date_2 = {
        if input_account.cf_date_2.is_some() {
            rbdate::timestamp(
                input_account
                    .cf_date_2
                    .unwrap_or(*config_params.as_on_date()),
            )
        } else {
            0
        }
    };

    op.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    op
}
