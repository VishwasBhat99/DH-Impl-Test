use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use rbdate::timestamp;
use statics::DEFAULT_INT;

pub fn create_account_with_cashflows(
    acc: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();

    out_acc.deal_no = acc.deal_no;
    out_acc.bond_issuance = acc.bond_issuance;
    out_acc.isin = acc.isin;
    out_acc.issuance_dt = if let Some(dt) = acc.issuance_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.branch_entity = acc.branch_entity;
    out_acc.desk = acc.desk;
    out_acc.portfolio_type = acc.portfolio_type;
    out_acc.category = acc.category;
    out_acc.security_type = acc.security_type;
    out_acc.slrnon_slr = acc.slrnon_slr;
    out_acc.short_name = acc.short_name;
    out_acc.secured_unsecured = acc.secured_unsecured;
    out_acc.rt = acc.rt;
    out_acc.nxt_call_dt = if let Some(dt) = acc.nxt_call_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.nxt_put_dt = if let Some(dt) = acc.nxt_put_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.agency = acc.agency;
    out_acc.rating = acc.rating;
    out_acc.agency_of_current_rating = acc.agency_of_current_rating;
    out_acc.listed_unlisted = acc.listed_unlisted;
    out_acc.mat_dt = if let Some(dt) = acc.mat_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.conversion_rt_lcy = acc.conversion_rt_lcy;
    out_acc.ccy = acc.ccy;
    out_acc.bv_after_amortisation = acc.bv_after_amortisation;
    out_acc.wap = acc.wap;
    out_acc.laf_and_msf_ost_fv = acc.laf_and_msf_ost_fv;
    out_acc.laf_and_msf_ost_bv = acc.laf_and_msf_ost_bv;
    out_acc.reverse_laf_ost_fv = acc.reverse_laf_ost_fv;
    out_acc.reverse_repo_ost_fv = acc.reverse_repo_ost_fv;
    out_acc.collateral_placed_fv = acc.collateral_placed_fv;
    out_acc.encumbered_fv = acc.encumbered_fv;
    out_acc.encumbered_bv = acc.encumbered_bv;
    out_acc.ytm = acc.ytm;
    out_acc.basis = acc.basis;
    out_acc.issue_country = acc.issue_country;
    out_acc.domicile_country = acc.domicile_country;
    out_acc.category1 = acc.category1;
    out_acc.category2 = acc.category2;
    out_acc.category3 = acc.category3;
    out_acc.category4 = acc.category4;
    out_acc.industry_code = acc.industry_code;
    out_acc.taxability = acc.taxability;
    out_acc.air_till_dt = acc.air_till_dt;
    out_acc.modified_duration = acc.modified_duration;
    out_acc.int_coupontype = acc.int_coupontype;
    out_acc.nxt_rep_dt = if let Some(dt) = acc.nxt_rep_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.sec_grp = acc.sec_grp;
    out_acc.sec_typ = acc.sec_typ;
    out_acc.sec_issuer = acc.sec_issuer;
    out_acc.sec_guaranteed = acc.sec_guaranteed;
    out_acc.mrkt = acc.mrkt;
    out_acc.idx_label = acc.idx_label;
    out_acc.bd_cat = acc.bd_cat;
    out_acc.bd_typ = acc.bd_typ;
    out_acc.lstd = acc.lstd;
    out_acc.npa = acc.npa;
    out_acc.cf_dt = if let Some(dt) = acc.cf_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.alm_line = acc.alm_line;
    out_acc.ia_line = acc.ia_line;
    out_acc.cf_int_amt = acc.cf_int_amt;
    out_acc.cf_prin_amt = acc.cf_prin_amt;
    out_acc.crnt_rating = acc.crnt_rating;
    out_acc.lst_cpn_dt = if let Some(dt) = acc.lst_cpn_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.nxt_cpn_dt = if let Some(dt) = acc.nxt_cpn_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.conv_rt_inr = acc.conv_rt_inr;
    out_acc.reval_val = acc.reval_val;
    out_acc.reval_profit = acc.reval_profit;
    out_acc.reval_loss = acc.reval_loss;
    out_acc.issuer = acc.issuer;
    out_acc.affiliated_to = acc.affiliated_to;
    out_acc.rating_master_rating = acc.rating_master_rating;
    out_acc.concat = acc.concat;
    out_acc.tenor = acc.tenor;
    out_acc.blr_eligibility = acc.blr_eligibility;
    out_acc.level_1 = acc.level_1;
    out_acc.level_2a_pse = acc.level_2a_pse;
    out_acc.level_2a_corp_bonds = acc.level_2a_corp_bonds;
    out_acc.level_2a_corp_cps = acc.level_2a_corp_cps;
    out_acc.npa_yes_no = acc.npa_yes_no;
    out_acc.level_2b = acc.level_2b;
    out_acc.others = acc.others;
    out_acc.comp_freq = acc.comp_freq;
    out_acc.isin_flag = acc.isin_flag;
    out_acc.bucket = acc.bucket;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
