use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use rbdate::timestamp;
use statics::*;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut tot_int_amt = DEFAULT_FLOAT;
    let mut tot_prin_amt = DEFAULT_FLOAT;

    out_acc.deal_no = account.deal_no;
    out_acc.portfolio = account.portfolio;
    out_acc.instrument_id = account.instrument_id;
    out_acc.instrument_type = account.instrument_type;
    out_acc.isin_code = account.isin_code;
    out_acc.security_name = account.security_name;
    out_acc.face_value = account.face_value;
    out_acc.book_value = account.book_value;
    out_acc.market_value = account.market_value;
    out_acc.currency = account.currency;
    out_acc.mtm = account.mtm;
    out_acc.yeild = account.yeild;
    out_acc.maturity_date = timestamp(account.maturity_date);
    out_acc.coupon_classification_1 = account.coupon_classification_1;
    out_acc.coupon_rate = account.coupon_rate;
    out_acc.face_value_perunits = account.face_value_perunits;
    out_acc.quantity = account.quantity;
    out_acc.appreciation = account.appreciation;
    out_acc.depreciation = account.depreciation;
    out_acc.net_appreciation_depreciation = account.net_appreciation_depreciation;
    out_acc.amortisation_asondate = account.amortisation_asondate;
    out_acc.accounted_amortisation = account.accounted_amortisation;
    out_acc.unaccounted_amortisation = account.unaccounted_amortisation;
    out_acc.accured_interest = account.accured_interest;
    out_acc.coupon_frequency = account.coupon_frequency;
    out_acc.coupon_basis = account.coupon_basis;
    out_acc.category = account.category;
    out_acc.sub_category = account.sub_category;
    out_acc.put_date = account.put_date;
    out_acc.call_date = account.call_date;
    out_acc.last_coupon = account.last_coupon;
    out_acc.next_coupon = timestamp(account.next_coupon);
    out_acc.issue_date = timestamp(account.issue_date);
    out_acc.last_repricing_date = account.last_repricing_date;
    out_acc.next_repricing_date = account.next_repricing_date;
    out_acc.place = account.place;
    out_acc.country = account.country;
    out_acc.slr_nonslr = account.slr_nonslr;
    out_acc.listed = account.listed;
    out_acc.issuer_id = account.issuer_id;
    out_acc.issuer_name = account.issuer_name;
    out_acc.external_rating_agency = account.external_rating_agency;
    out_acc.external_rating = account.external_rating;
    out_acc.market = account.market;
    out_acc.asset_classification = account.asset_classification;
    out_acc.guarantor = account.guarantor;
    out_acc.industry = account.industry;
    out_acc.sub_industry = account.sub_industry;
    out_acc.npa_classification = account.npa_classification;
    out_acc.deal_value_date = account.deal_value_date;
    out_acc.m_duration = account.m_duration;
    out_acc.treasury_gl_code = account.treasury_gl_code;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
