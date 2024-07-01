#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InputAccount {
    pub portfolio_num: String,
    pub portfolio: String,
    pub security_name: String,
    pub maturity_dt: String,
    pub coupon: String,
    pub face_val_per_units: String,
    pub quantity: String,
    pub face_val: String,
    pub wap: String,
    pub book_val: String,
    pub market_val: String,
    pub mtm: String,
    pub yeild: String,
    pub appreciation: String,
    pub depreciation: String,
    pub net_appreciation_depreciation: String,
    pub amort_as_on_dt: String,
    pub accounted_amort: String,
    pub un_accounted_amort: String,
    pub accrued_int: String,
    pub no_ca_skipped: String,
    pub ca_int_not_receieved: String,
    pub total_int: String,
    pub instrument_id: String,
    pub instrument_typ: String,
    pub isin_code: String,
    pub int_freq: String,
    pub int_practice: String,
    pub category: String,
    pub sub_category: String,
    pub put_dt: String,
    pub call_dt: String,
    pub lst_coupon: String,
    pub nxt_coupon: String,
    pub issue_dt: String,
    pub place: String,
    pub country: String,
    pub booking_basis: String,
    pub residual_maturity: String,
    pub slr_non_slr: String,
    pub listed: String,
    pub issuer_name: String,
    pub rating_agency: String,
    pub rating: String,
    pub market: String,
    pub asset_classification: String,
    pub gurantor: String,
    pub industry: String,
    pub sub_industry: String,
    pub borrower_category: String,
    pub asset_typ: String,
    pub asset_category: String,
    pub curr: String,
    pub coupon_classification_1: String,
    pub lst_rep_dt: String,
    pub nxt_rep_dt: String,
    pub m_duration: String,
    pub trsy_gl_cd: String,
    pub cf_type: String,
    pub cf_amt: String,
    pub cf_ccy: String,
    pub cf_dt: String,
}

impl InputAccount {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|",
            self.portfolio_num,
            self.portfolio,
            self.security_name,
            self.maturity_dt,
            self.coupon,
            self.face_val_per_units,
            self.quantity,
            self.face_val,
            self.wap,
            self.book_val,
            self.market_val,
            self.mtm,
            self.yeild,
            self.appreciation,
            self.depreciation,
            self.net_appreciation_depreciation,
            self.amort_as_on_dt,
            self.accounted_amort,
            self.un_accounted_amort,
            self.accrued_int,
            self.no_ca_skipped,
            self.ca_int_not_receieved,
            self.total_int,
            self.instrument_id,
            self.instrument_typ,
            self.isin_code,
            self.int_freq,
            self.int_practice,
            self.category,
            self.sub_category,
            self.put_dt,
            self.call_dt,
            self.lst_coupon,
            self.nxt_coupon,
            self.issue_dt,
            self.place,
            self.country,
            self.booking_basis,
            self.residual_maturity,
            self.slr_non_slr,
            self.listed,
            self.issuer_name,
            self.rating_agency,
            self.rating,
            self.market,
            self.asset_classification,
            self.gurantor,
            self.industry,
            self.sub_industry,
            self.borrower_category,
            self.asset_typ,
            self.asset_category,
            self.curr,
            self.coupon_classification_1,
            self.lst_rep_dt,
            self.nxt_rep_dt,
            self.m_duration,
            self.trsy_gl_cd,
            self.cf_type,
            self.cf_amt,
            self.cf_ccy,
            self.cf_dt,
        )
    }
}
