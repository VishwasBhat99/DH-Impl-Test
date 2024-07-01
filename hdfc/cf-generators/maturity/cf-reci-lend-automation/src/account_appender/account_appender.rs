use account::Account;
use process::structs::OPFields;

pub fn create_acc_wt_cfs(new_acc: &OPFields) -> Account {
    let mut account = Account::new();
    account.set_entity(new_acc.entity.to_owned());
    account.set_source(new_acc.source.to_owned());
    account.set_in_out(new_acc.in_out.to_owned());
    account.set_sub_type(new_acc.sub_type.to_owned());
    account.set_counter_party(new_acc.counter_party.to_owned());
    account.set_currency(new_acc.currency.to_owned());
    account.set_avaliabile_limit(new_acc.avaliabile_limit.to_owned());
    account.set_deal_amount_lcy(new_acc.deal_amount_lcy.to_owned());
    account.set_cf_date(new_acc.cf_date.to_owned());
    account.set_cp_parent_id(new_acc.cp_parent_id.to_owned());
    account.set_cashflows(new_acc.cashflows.to_owned());
    account
}
