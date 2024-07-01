use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::cap_floor;
use cashflow_derivator::account_with_cashflows::Cashflow;
use rbdate::timestamp;
use statics::*;

pub fn create_account_with_cashflows(account: InputAccount, cashflows: Vec<Cashflow>) -> cap_floor {
    let mut out_acc = cap_floor::new();
    let mut tot_int_amt = DEFAULT_FLOAT;
    let mut tot_prin_amt = DEFAULT_FLOAT;

    out_acc.entity = account.entity;
    out_acc.trade_id = account.trade_id;
    out_acc.structure_id_link = account.structureid_link;
    out_acc.component_typo = account.component_typo;
    out_acc.contract_type = account.contract_type;
    out_acc.package_typo = account.package_typo;
    out_acc.desk = account.desk;
    out_acc.book = account.book;
    out_acc.folder = account.folder;
    out_acc.trading_banking = account.trading_banking;
    out_acc.internal_external = account.internal_external;
    out_acc.counterparty_group_code = account.counterparty_group_code;
    out_acc.counterparty_parent_code = account.counterparty_parent_code;
    out_acc.counterparty_child_code = account.counterparty_child_code;
    out_acc.bank_non_bank = account.bank_non_bank;
    out_acc.trade_date = if let Some(dt) = account.trade_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.maturity_date = if let Some(dt) = account.maturity_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.buy_sale = account.buy_sale;
    out_acc.underlying_index = account.underlying_index;
    out_acc.notional_currency = account.notional_currency;
    out_acc.original_notional_amount = account.original_notional_amount;
    out_acc.mtm_in_inr = account.mtm_in_inr;
    out_acc.net_pv01_in_inr = account.net_pv01_in_inr;
    out_acc.modified_duration_of_the_deal = account.modified_duration_of_the_deal;
    out_acc.reset_frequency = account.reset_frequency;
    out_acc.next_reset_date = if let Some(dt) = account.next_reset_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.underlying_pp = account.underlying_pp;
    out_acc.deal_status = account.deal_status;
    out_acc.counterparty_category1 = account.counterparty_category1;
    out_acc.counterparty_category2 = account.counterparty_category2;
    out_acc.counterparty_category3 = account.counterparty_category3;
    out_acc.accounting_section = account.accounting_section;
    out_acc.flowtype4 = account.flowtype4;
    out_acc.flow_amount = account.flow_amount;
    out_acc.cashflow_date = if let Some(dt) = account.cashflow_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.flow_currency = account.flow_currency;
    for cf in &cashflows {
        tot_int_amt += cf.interest_amount;
        tot_prin_amt += cf.principal_amount;
    }
    out_acc.tot_int_amt = tot_int_amt;
    out_acc.tot_prin_amt = tot_prin_amt;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
