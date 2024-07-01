use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use rbdate::timestamp;
use statics::DEFAULT_INT;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();

    out_acc.deal_no = account.deal_no;
    out_acc.acc_open_dt = if let Some(dt) = account.acc_open_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.nature = account.nature;
    out_acc.deal_stat = account.deal_stat;
    out_acc.deal_type = account.deal_type;
    out_acc.slr_typ = account.slr_typ;
    out_acc.security = account.security;
    out_acc.category = account.category;
    out_acc.sub_category = account.sub_category;
    out_acc.desk = account.desk;
    out_acc.portfolio = account.portfolio;
    out_acc.accounting_section = account.accounting_section;
    out_acc.counterparty = account.counterparty;
    out_acc.counterparty_full_name = account.counterparty_full_name;
    out_acc.currency = account.currency;
    out_acc.repo_rate = account.repo_rate;
    out_acc.ytm = account.ytm;
    out_acc.value_dt = if let Some(dt) = account.value_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.price = account.price;
    out_acc.settle_amt_1st_leg = account.settle_amt_1st_leg;
    out_acc.accrued_interest = account.accrued_interest;
    out_acc.repo_int = account.repo_int;
    out_acc.settle_amt_2nd_leg = account.settle_amt_2nd_leg;
    out_acc.entity = account.entity;
    out_acc.bank_non_bank = account.bank_non_bank;
    out_acc.air_aip = account.air_aip;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
