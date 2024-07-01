use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use rbdate::{timestamp, NaiveDate};
use statics::*;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut tot_int_amt = 0.0;
    let mut tot_prin_amt = 0.0;

    out_acc.holding_deal_id = account.holding_deal_id;
    out_acc.portfolio = account.portfolio;
    out_acc.instrument_type = account.instrument_type;
    out_acc.isdiscounted = account.isdiscounted;
    out_acc.slr_non_slr = account.slr_non_slr;
    out_acc.isin = account.isin;
    out_acc.issuer_id = account.issuer_id;
    out_acc.issuer_name = account.issuer_name;
    out_acc.security_id = account.security_id;
    out_acc.security_name = account.security_name;

    out_acc.deal_value_date = if let Some(dt) = account.deal_value_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.issue_date = if let Some(dt) = account.issue_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.maturity_date = if let Some(dt) = account.maturity_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };

    out_acc.coupon_rate = account.coupon_rate;
    out_acc.interest_calculation_basis = account.interest_calculation_basis;
    out_acc.face_value_per_units = account.face_value_per_units;
    out_acc.quantity = account.quantity;
    out_acc.face_value = if let Some(face_value) = account.face_value {
        face_value
    } else {
        DEFAULT_FLOAT
    };
    out_acc.currency = account.currency;
    out_acc.book_value = if let Some(book_value) = account.book_value {
        book_value
    } else {
        DEFAULT_FLOAT
    };
    out_acc.market_value = account.market_value;
    out_acc.mtm = account.mtm;
    out_acc.market_yield = account.market_yield;
    out_acc.interest_accruad_days = account.interest_accruad_days;
    out_acc.accrued_interest = if let Some(accrued_interest) = account.accrued_interest {
        accrued_interest
    } else {
        DEFAULT_FLOAT
    };
    out_acc.coupon_pay_day = account.coupon_pay_day;
    out_acc.coupon_type = account.coupon_type;
    out_acc.spread = account.spread;
    out_acc.benchmark = account.benchmark;
    out_acc.coupon_frequency = account.coupon_frequency;
    out_acc.last_coupon_date = if let Some(dt) = account.last_coupon_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.next_coupon_date = if let Some(dt) = account.next_coupon_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.last_repricing_date = if let Some(dt) = account.last_repricing_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.next_repricing_date = if let Some(dt) = account.next_repricing_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.put_date = if let Some(dt) = account.put_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.call_date = if let Some(dt) = account.call_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };

    out_acc.is_listed = account.is_listed;
    out_acc.issuer_country = account.issuer_country;
    out_acc.issuer_type = account.issuer_type;
    out_acc.isuser_classification_1 = account.isuser_classification_1;
    out_acc.isuser_classification_2 = account.isuser_classification_2;
    out_acc.isuser_classification_3 = account.isuser_classification_3;
    out_acc.gurantor_type = account.gurantor_type;
    out_acc.rating_agency = account.rating_agency;
    out_acc.external_rating = account.external_rating;
    out_acc.asset_type = account.asset_type;
    out_acc.asset_category = account.asset_category;
    out_acc.treaury_gl_code = account.treaury_gl_code;
    out_acc.m_npaclassification = account.m_npaclassification;
    out_acc.internal_rating = account.internal_rating;
    out_acc.customer_constitution_code = account.customer_constitution_code;
    out_acc.risk_weight = account.risk_weight;
    out_acc.crar_basel_classification = account.crar_basel_classification;
    out_acc.listing_status = account.listing_status;
    out_acc.listed_exchange = account.listed_exchange;
    out_acc.open_ended = account.open_ended;
    out_acc.haircut = account.haircut;
    out_acc.mtm_gl_code = account.mtm_gl_code;
    out_acc.mtm_double = account.mtm_double;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}
