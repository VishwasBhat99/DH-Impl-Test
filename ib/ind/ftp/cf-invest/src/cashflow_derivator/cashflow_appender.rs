use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use rbdate::date_from_timestamp;
use rbdate::timestamp;
use statics::*;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut tot_int_amt = DEFAULT_FLOAT;
    let mut tot_prin_amt = DEFAULT_FLOAT;

    out_acc.portfolio = account.portfolio;
    out_acc.security_name = account.security_name;
    out_acc.instrument_id = account.instrument_id;
    out_acc.instrument_type = account.instrument_type;
    out_acc.isin_code = account.isin_code;
    out_acc.slr_nslr = account.slr_nslr;
    out_acc.category = account.category;
    out_acc.category_grp = account.category_grp;
    out_acc.sub_category = account.sub_category;
    out_acc.fa_classification = account.fa_classification;
    out_acc.maturity_date = if let Some(dt) = account.maturity_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.coupon = account.coupon;
    out_acc.discount_intr_rate = account.discount_intr_rate;
    out_acc.face_val_per_units = account.face_val_per_units;
    out_acc.quantity = account.quantity;
    out_acc.face_value = account.face_value;
    out_acc.wap = account.wap;
    out_acc.yield_value = account.yield_value;
    out_acc.book_value = account.book_value;
    out_acc.mvmnt_amount = account.mvmnt_amount;
    out_acc.market_price = account.market_price;
    out_acc.market_yield = account.market_yield;
    out_acc.market_value = account.market_value;
    out_acc.duration = account.duration;
    out_acc.m_duration = account.m_duration;
    out_acc.appreciation = account.appreciation;
    out_acc.depreciation = account.depreciation;
    out_acc.net_app_dep = account.net_app_dep;
    out_acc.convexity = account.convexity;
    out_acc.pvbp = account.pvbp;
    out_acc.absolute_pvbp = account.absolute_pvbp;
    out_acc.amortization_asondate = account.amortization_asondate;
    out_acc.accounted_amortization = account.accounted_amortization;
    out_acc.unaccounted_amortization = account.unaccounted_amortization;
    out_acc.accrued_interest = account.accrued_interest;
    out_acc.no_of_ca_skipped = account.no_of_ca_skipped;
    out_acc.ca_interest_not_receieved = account.ca_interest_not_receieved;
    out_acc.total_interest = account.total_interest;
    out_acc.encumbered_since_repo = account.encumbered_since_repo;
    out_acc.amount_repo = account.amount_repo;
    out_acc.encumbered_since_ccil = account.encumbered_since_ccil;
    out_acc.amount_ccil = account.amount_ccil;
    out_acc.encumbered_since_treps = account.encumbered_since_treps;
    out_acc.amount_treps = account.amount_treps;
    out_acc.encumbered_since_mcxs = account.encumbered_since_mcxs;
    out_acc.amount_mcxs = account.amount_mcxs;
    out_acc.encumbered_since_others = account.encumbered_since_others;
    out_acc.amount_others = account.amount_others;
    out_acc.custody_pos_number = account.custody_pos_number;
    out_acc.custody_pos_type = account.custody_pos_type;
    out_acc.interest_frequency = account.interest_frequency;
    out_acc.interest_practice = account.interest_practice;
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
    out_acc.issue_date = if let Some(dt) = account.issue_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.place = account.place;
    out_acc.country = account.country;
    out_acc.booking_basis = account.booking_basis;
    out_acc.residual_maturity = account.residual_maturity;
    out_acc.issuer_name = account.issuer_name;
    out_acc.market = account.market;
    out_acc.gurantor = account.gurantor;
    out_acc.industry = account.industry;
    out_acc.sub_industry = account.sub_industry;
    out_acc.borrower_category = account.borrower_category;
    out_acc.asset_classification = account.asset_classification;
    out_acc.asset_type = account.asset_type;
    out_acc.asset_category = account.asset_category;
    out_acc.old_security_id = account.old_security_id;
    out_acc.curve1 = account.curve1;
    out_acc.listed = account.listed;
    out_acc.secured = account.secured;
    out_acc.quoted = account.quoted;
    out_acc.borrower = account.borrower;
    out_acc.extbank_ref = account.extbank_ref;
    out_acc.pan = account.pan;
    out_acc.intr_rating_agency = account.intr_rating_agency;
    out_acc.internal_rating = account.internal_rating;
    out_acc.intr_rating_valid_from = if let Some(dt) = account.intr_rating_valid_from {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.intr_rating_valid_till = if let Some(dt) = account.intr_rating_valid_till {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.extrn_rating_agency = account.extrn_rating_agency;
    out_acc.external_rating = account.external_rating;
    out_acc.extrn_rating_valid_from = if let Some(dt) = account.extrn_rating_valid_from {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.extrn_rating_valid_till = if let Some(dt) = account.extrn_rating_valid_till {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.liquid_status = account.liquid_status;
    out_acc.asset_sub_class = account.asset_sub_class;
    out_acc.hurdle_rating = account.hurdle_rating;
    out_acc.external_rating_vs_hurdle = account.external_rating_vs_hurdle;
    out_acc.internal_rating_vs_hurdle = account.internal_rating_vs_hurdle;
    out_acc.fsu = account.fsu;
    out_acc.equity_seg = account.equity_seg;
    out_acc.issuer_segr = account.issuer_segr;
    out_acc.restructuring = account.restructuring;
    out_acc.paid_up_share_captial = account.paid_up_share_captial;
    out_acc.exempted_amount = account.exempted_amount;
    out_acc.issuer_group = account.issuer_group;
    out_acc.murram_market_value = account.murram_market_value;
    out_acc.murram_depr = account.murram_depr;
    out_acc.var_settled_bv = account.var_settled_bv;
    out_acc.var_unsettled_bv = account.var_unsettled_bv;
    out_acc.var_settled_amount = account.var_settled_amount;
    out_acc.var_unsettled_amount = account.var_unsettled_amount;
    out_acc.kri_settled_qtd_fv = account.kri_settled_qtd_fv;
    out_acc.basel_group = account.basel_group;
    out_acc.basel_sub_group = account.basel_sub_group;
    out_acc.time_band = account.time_band;
    out_acc.capital_charge_market_risk_rate = account.capital_charge_market_risk_rate;
    out_acc.capital_charge_market_risk_amount = account.capital_charge_market_risk_amount;
    out_acc.trading_specif_risk_rate = account.trading_specif_risk_rate;
    out_acc.banking_specif_risk_rate = account.banking_specif_risk_rate;
    out_acc.trading_specif_risk_captial_charge = account.trading_specif_risk_captial_charge;
    out_acc.banking_specif_risk_captial_charge = account.banking_specif_risk_captial_charge;
    out_acc.mode_of_holding = account.mode_of_holding;
    out_acc.issuer_rating_agency = account.issuer_rating_agency;
    out_acc.issuer_rating = account.issuer_rating;
    out_acc.issuer_rating_valid_from = if let Some(dt) = account.issuer_rating_valid_from {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.issuer_rating_valid_till = if let Some(dt) = account.issuer_rating_valid_till {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.issuer_sub_industry = account.issuer_sub_industry;
    out_acc.gl_code = account.gl_code;
    out_acc.interest_type = account.interest_type;
    out_acc.computed_mat_date = if let Some(dt) = account.computed_mat_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.cgl = account.cgl;
    out_acc.group = account.group;
    out_acc.llg = account.llg;
    out_acc.currency = account.currency;
    out_acc.mat_date_flag = account.mat_date_flag;
    out_acc.concat_deal_id = account.concat_deal_id.clone();
    out_acc.concat_inst_id = account.concat_inst_id;
    out_acc.concat_deal_n_slr_id = account.concat_deal_n_slr_id;
    out_acc.ftp_coupon_rate = account.ftp_coupon_rate;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}
