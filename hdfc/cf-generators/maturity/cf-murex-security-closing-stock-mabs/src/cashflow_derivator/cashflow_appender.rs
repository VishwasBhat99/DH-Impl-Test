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
    out_acc.bond_issuance = account.bond_issuance;
    out_acc.isin = account.isin;
    out_acc.issuance_dt = if let Some(dt) = account.issuance_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.branch_entity = account.branch_entity;
    out_acc.desk = account.desk;
    out_acc.portfolio_type = account.portfolio_type;
    out_acc.category = account.category;
    out_acc.security_type = account.security_type;
    out_acc.slrnon_slr = account.slrnon_slr;
    out_acc.short_name = account.short_name;
    out_acc.secured_unsecured = account.secured_unsecured;
    out_acc.rt = account.rt;
    out_acc.nxt_call_dt = if let Some(dt) = account.nxt_call_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.nxt_put_dt = if let Some(dt) = account.nxt_put_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.agency = account.agency;
    out_acc.rating = account.rating;
    out_acc.agency_of_current_rating = account.agency_of_current_rating;
    out_acc.listed_unlisted = account.listed_unlisted;
    out_acc.mat_dt = if let Some(dt) = account.mat_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.conversion_rt_lcy = account.conversion_rt_lcy;
    out_acc.ccy = account.ccy;
    out_acc.bv_after_amortisation = account.bv_after_amortisation;
    out_acc.wap = account.wap;
    out_acc.laf_and_msf_ost_fv = account.laf_and_msf_ost_fv;
    out_acc.laf_and_msf_ost_bv = account.laf_and_msf_ost_bv;
    out_acc.reverse_laf_ost_fv = account.reverse_laf_ost_fv;
    out_acc.reverse_repo_ost_fv = account.reverse_repo_ost_fv;
    out_acc.collateral_placed_fv = account.collateral_placed_fv;
    out_acc.encumbered_fv = account.encumbered_fv;
    out_acc.encumbered_bv = account.encumbered_bv;
    out_acc.ytm = account.ytm;
    out_acc.basis = account.basis;
    out_acc.issue_country = account.issue_country;
    out_acc.domicile_country = account.domicile_country;
    out_acc.category1 = account.category1;
    out_acc.category2 = account.category2;
    out_acc.category3 = account.category3;
    out_acc.category4 = account.category4;
    out_acc.industry_code = account.industry_code;
    out_acc.taxability = account.taxability;
    out_acc.air_till_dt = account.air_till_dt;
    out_acc.modified_duration = account.modified_duration;
    out_acc.int_coupontype = account.int_coupontype;
    out_acc.nxt_rep_dt = if let Some(dt) = account.nxt_rep_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.sec_grp = account.sec_grp;
    out_acc.sec_typ = account.sec_typ;
    out_acc.sec_issuer = account.sec_issuer;
    out_acc.sec_guaranteed = account.sec_guaranteed;
    out_acc.mrkt = account.mrkt;
    out_acc.idx_label = account.idx_label;
    out_acc.bd_cat = account.bd_cat;
    out_acc.bd_typ = account.bd_typ;
    out_acc.lstd = account.lstd;
    out_acc.npa = account.npa;
    out_acc.cf_dt = if let Some(dt) = account.cf_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.alm_line = account.alm_line;
    out_acc.ia_line = account.ia_line;
    out_acc.face_val = account.face_val;
    out_acc.book_val = account.book_val;
    out_acc.market_val = account.market_val;
    out_acc.mtm_amt = account.mtm_amt;
    out_acc.cf_int_amt = account.cf_int_amt;
    out_acc.cf_prin_amt = account.cf_prin_amt;

    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
