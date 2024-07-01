#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InputAccount {
    pub deal_no: String,
    pub instr_id: String,
    pub short_name: String,
    pub intr_typ: String,
    pub intr_app_freq: String,
    pub comp_freq: String,
    pub intr_prac: String,
    pub coup: String,
    pub lst_intr_dt: String,
    pub nxt_intr_dt: String,
    pub nxt_rep_dt: String,
    pub rt_spread: String,
    pub rating: String,
    pub mat_dt: String,
    pub call_dt: String,
    pub put_dt: String,
    pub tax_status: String,
    pub prod: String,
    pub prod_desc: String,
    pub gl_cd: String,
    pub deal_dt: String,
    pub days_trade_dt_wise: String,
    pub master_val_dt: String,
    pub days_val_dt_wise: String,
    pub portfolio: String,
    pub portfolio_type: String,
    pub deal_ytm: String,
    pub deal_rt: String,
    pub avg_os_vd: String,
    pub org_fv: String,
    pub os_fv: String,
    pub org_cv: String,
    pub os_cv_before_amort: String,
    pub accr_int: String,
    pub int_income: String,
    pub wap_igaap: String,
    pub amort_till_dt: String,
    pub period_amort: String,
    pub os_cv_after_amort: String,
    pub book_yield: String,
    pub yield_on_avg_os_vd: String,
    pub mrkt_val: String,
    pub sec_grp: String,
    pub sec_type: String,
    pub sec_issuer: String,
    pub sec_guaranteed: String,
    pub mrkt: String,
    pub index_label: String,
    pub bd_categ: String,
    pub bd_type: String,
    pub listed: String,
    pub npa_class: String,
    pub entity: String,
    pub desk: String,
    pub acc_sec_igaap: String,
    pub vw_val_dt: String,
    pub irr_dt: String,
    pub flow_amt: String,
    pub flow_type: String,
    pub isin: String,
    pub currency: String,
    pub slr_nslr: String,
    pub contract_no: String,
    pub parent_code: String,
    pub issuer_name: String,
    pub sec_issuance_date: String,
}

impl InputAccount {
    pub fn new() -> InputAccount {
        InputAccount {
            ..Default::default()
        }
    }
}
