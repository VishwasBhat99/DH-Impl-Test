use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use rbdate::timestamp;
use statics::*;

pub fn create_account_with_cashflows(
    acc: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut tot_int_amt = DEFAULT_FLOAT;
    let mut tot_prin_amt = DEFAULT_FLOAT;
    out_acc.portfolio_num = acc.portfolio_num;
    out_acc.portfolio = acc.portfolio;
    out_acc.security_name = acc.security_name;
    out_acc.maturity_dt = if let Some(dt) = acc.maturity_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.coupon = acc.coupon;
    out_acc.face_val_per_units = acc.face_val_per_units;
    out_acc.quantity = acc.quantity;
    out_acc.face_val = acc.face_val;
    out_acc.wap = acc.wap;
    out_acc.book_val = acc.book_val;
    out_acc.market_val = acc.market_val;
    out_acc.mtm = acc.mtm;
    out_acc.yeild = acc.yeild;
    out_acc.appreciation = acc.appreciation;
    out_acc.depreciation = acc.depreciation;
    out_acc.net_appreciation_depreciation = acc.net_appreciation_depreciation;
    out_acc.amort_as_on_dt = acc.amort_as_on_dt;
    out_acc.accounted_amort = acc.accounted_amort;
    out_acc.un_accounted_amort = acc.un_accounted_amort;
    out_acc.accrued_int = acc.accrued_int;
    out_acc.no_ca_skipped = acc.no_ca_skipped;
    out_acc.ca_int_not_receieved = acc.ca_int_not_receieved;
    out_acc.total_int = acc.total_int;
    out_acc.inst_id = acc.inst_id;
    out_acc.inst_typ = acc.inst_typ;
    out_acc.isin_code = acc.isin_code;
    out_acc.int_freq = acc.int_freq;
    out_acc.int_practice = acc.int_practice;
    out_acc.category = acc.category;
    out_acc.sub_category = acc.sub_category;
    out_acc.put_dt = if let Some(dt) = acc.put_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.call_dt = if let Some(dt) = acc.call_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.lst_coupon = if let Some(dt) = acc.lst_coupon {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.nxt_coupon = if let Some(dt) = acc.nxt_coupon {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.issue_dt = if let Some(dt) = acc.issue_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.place = acc.place;
    out_acc.country = acc.country;
    out_acc.booking_basis = acc.booking_basis;
    out_acc.residual_maturity = acc.residual_maturity;
    out_acc.slr_non_slr = acc.slr_non_slr;
    out_acc.listed = acc.listed;
    out_acc.issuer_name = acc.issuer_name;
    out_acc.rating_agency = acc.rating_agency;
    out_acc.rating = acc.rating;
    out_acc.market = acc.market;
    out_acc.asset_classification = acc.asset_classification;
    out_acc.gurantor = acc.gurantor;
    out_acc.industry = acc.industry;
    out_acc.sub_industry = acc.sub_industry;
    out_acc.borrower_category = acc.borrower_category;
    out_acc.asset_typ = acc.asset_typ;
    out_acc.asset_category = acc.asset_category;
    out_acc.curr = acc.curr;
    out_acc.coupon_classification_1 = acc.coupon_classification_1;
    out_acc.lst_rep_dt = if let Some(dt) = acc.lst_rep_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.nxt_rep_dt = if let Some(dt) = acc.nxt_rep_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.m_duration = acc.m_duration;
    out_acc.trsy_gl_cd = acc.trsy_gl_cd;
    out_acc.cf_type = acc.cf_type;
    out_acc.cf_amt = acc.cf_amt;
    out_acc.cf_ccy = acc.cf_ccy;
    out_acc.cf_dt = if let Some(dt) = acc.cf_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.prin_amt = acc.prin_amt;
    out_acc.int_amt = acc.int_amt;
    out_acc.cbs_gl_cd = acc.cbs_gl_cd;
    out_acc.w4b_cd = acc.w4b_cd;
    out_acc.balm_llg = acc.balm_llg;
    out_acc.care_llg = acc.care_llg;
    out_acc.ba_llg = acc.ba_llg;

    for cf in &cashflows {
        tot_int_amt += cf.int_amt;
        tot_prin_amt += cf.prin_amt;
    }
    out_acc.tot_int_amt = tot_int_amt;
    out_acc.tot_prin_amt = tot_prin_amt;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
