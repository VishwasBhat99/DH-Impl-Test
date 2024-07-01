use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use rbdate::timestamp;
use statics::*;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
    ccy: String,
    cf_type: &str,
) -> AccountWithCashflows {
    let mut op_acc = AccountWithCashflows::new();
    let mut tot_int_amt = DEFAULT_FLOAT;
    let mut tot_prin_amt = DEFAULT_FLOAT;

    op_acc.entity = account.entity;
    op_acc.trade_id = account.trade_id;
    op_acc.contract_typology = account.contract_typology;
    op_acc.trade_typology = account.trade_typology;
    op_acc.usage = if account.usage.is_empty() {
        String::from("NA")
    } else {
        account.usage
    };
    op_acc.desk = account.desk;
    op_acc.book = account.book;
    op_acc.folder = account.folder;
    op_acc.trading_banking = account.trading_banking;
    op_acc.internal_external = account.internal_external;
    op_acc.inter_entity = account.inter_entity;
    op_acc.counterparty_long_name = if account.counterparty_long_name.is_empty() {
        String::from("NA")
    } else {
        account.counterparty_long_name
    };
    op_acc.cp_category_level_1 = if account.cp_category_level_1.is_empty() {
        String::from("NA")
    } else {
        account.cp_category_level_1
    };
    op_acc.cp_category_level_2 = if account.cp_category_level_2.is_empty() {
        String::from("NA")
    } else {
        account.cp_category_level_2
    };
    op_acc.cp_category_level_3 = if account.cp_category_level_3.is_empty() {
        String::from("NA")
    } else {
        account.cp_category_level_3
    };
    op_acc.near_leg_far_leg = account.near_leg_far_leg;
    op_acc.option_start_date = if let Some(dt) = account.option_start_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    op_acc.option_end_date = if let Some(dt) = account.option_end_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    op_acc.effective_date = if let Some(dt) = account.effective_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    op_acc.mat_date_of_contract = if let Some(dt) = account.mat_date_of_contract {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    op_acc.ccil_guranteed = account.ccil_guranteed;
    op_acc.buy_currency = account.buy_currency;
    op_acc.buy_current_notional = account.buy_current_notional;
    op_acc.sell_currency = account.sell_currency;
    op_acc.sell_current_notional = account.sell_current_notional;
    op_acc.forward_rate = account.forward_rate;
    op_acc.forward_mtm = account.forward_mtm;
    op_acc.mtm_in_inr_forward = account.mtm_in_inr_forward;
    op_acc.mtm_in_usd_forward = account.mtm_in_usd_forward;
    op_acc.pv_mtm_in_inr = account.pv_mtm_in_inr;
    op_acc.pv_mtm_in_usd = account.pv_mtm_in_usd;
    op_acc.pv01_in_inr = account.pv01_in_inr;
    op_acc.mduriation_ccy1_inr = account.mduriation_ccy1_inr;
    op_acc.mduriation_ccy2_inr = account.mduriation_ccy2_inr;
    op_acc.contingent_notional_in_inr = account.contingent_notional_in_inr;
    op_acc.npa = account.npa;
    op_acc.bank_nonbank = account.bank_nonbank;
    op_acc.original_tenor = account.original_tenor;
    op_acc.residual_tenor = account.residual_tenor;
    for cf in &cashflows {
        tot_int_amt += cf.int_amt;
        tot_prin_amt += cf.prin_amt;
    }
    op_acc.tot_int_amt = tot_int_amt;
    op_acc.tot_prin_amt = tot_prin_amt;
    op_acc.cntry_resid = if account.country_of_residence.is_empty() {
        String::from("NA")
    } else {
        account.country_of_residence
    };
    op_acc.cntry_ultim_risk = if account.country_of_ultimate_risk.is_empty() {
        String::from("NA")
    } else {
        account.country_of_ultimate_risk
    };
    op_acc.counterparty_child_code = if account.counterparty_child_code.is_empty() {
        String::from("NA")
    } else {
        account.counterparty_child_code
    };
    op_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    op_acc.currency = ccy;
    op_acc.cf_type = cf_type.to_string();

    op_acc
}
