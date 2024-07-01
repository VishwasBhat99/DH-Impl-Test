use std::collections::HashMap;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecDealData {
    pub portfolio: String,
    pub port_folio: String,
    pub sec_name: String,
    pub mat_date: String,
    pub coupon: String,
    pub face_val_units: String,
    pub quantity: String,
    pub face_value: String,
    pub wap: String,
    pub book_val: String,
    pub mark_val: String,
    pub mtm: String,
    pub yld: String,
    pub appreciation: String,
    pub depreciation: String,
    pub net_app_dep: String,
    pub amort_ason: String,
    pub accounted_amort: String,
    pub unaccounted_amort: String,
    pub accrued_int: String,
    pub no_ca_skip: String,
    pub ca_int_not_rec: String,
    pub tot_int: String,
    pub instr_id: String,
    pub instr_type: String,
    pub isin_cd: String,
    pub int_freq: String,
    pub int_prac: String,
    pub catg: String,
    pub sub_catg: String,
    pub put_date: String,
    pub call_date: String,
    pub last_cou: String,
    pub nxt_cou: String,
    pub issue_date: String,
    pub place: String,
    pub country: String,
    pub booking_basis: String,
    pub res_mat: String,
    pub slr_nonslr: String,
    pub listed: String,
    pub issuer_name: String,
    pub rating_agcy: String,
    pub rating: String,
    pub market: String,
    pub asset_clas: String,
    pub gurantor: String,
    pub industry: String,
    pub sub_ind: String,
    pub borr_catg: String,
    pub asset_type: String,
    pub asset_catg: String,
    pub currency: String,
    pub cou_class: String,
    pub last_repr_date: String,
    pub next_rep_date: String,
    pub m_dur: String,
    pub treasury_gl_cd: String,
}

impl Default for SecDealData {
    fn default() -> Self {
        SecDealData {
            portfolio: String::from("NA"),
            port_folio: String::from("NA"),
            sec_name: String::from("NA"),
            mat_date: String::from("NA"),
            coupon: String::from("NA"),
            face_val_units: String::from("NA"),
            quantity: String::from("NA"),
            face_value: String::from("NA"),
            wap: String::from("NA"),
            book_val: String::from("NA"),
            mark_val: String::from("NA"),
            mtm: String::from("NA"),
            yld: String::from("NA"),
            appreciation: String::from("NA"),
            depreciation: String::from("NA"),
            net_app_dep: String::from("NA"),
            amort_ason: String::from("NA"),
            accounted_amort: String::from("NA"),
            unaccounted_amort: String::from("NA"),
            accrued_int: String::from("NA"),
            no_ca_skip: String::from("NA"),
            ca_int_not_rec: String::from("NA"),
            tot_int: String::from("NA"),
            instr_id: String::from("NA"),
            instr_type: String::from("NA"),
            isin_cd: String::from("NA"),
            int_freq: String::from("NA"),
            int_prac: String::from("NA"),
            catg: String::from("NA"),
            sub_catg: String::from("NA"),
            put_date: String::from("NA"),
            call_date: String::from("NA"),
            last_cou: String::from("NA"),
            nxt_cou: String::from("NA"),
            issue_date: String::from("NA"),
            place: String::from("NA"),
            country: String::from("NA"),
            booking_basis: String::from("NA"),
            res_mat: String::from("NA"),
            slr_nonslr: String::from("NA"),
            listed: String::from("NA"),
            issuer_name: String::from("NA"),
            rating_agcy: String::from("NA"),
            rating: String::from("NA"),
            market: String::from("NA"),
            asset_clas: String::from("NA"),
            gurantor: String::from("NA"),
            industry: String::from("NA"),
            sub_ind: String::from("NA"),
            borr_catg: String::from("NA"),
            asset_type: String::from("NA"),
            asset_catg: String::from("NA"),
            currency: String::from("NA"),
            cou_class: String::from("NA"),
            last_repr_date: String::from("NA"),
            next_rep_date: String::from("NA"),
            m_dur: String::from("NA"),
            treasury_gl_cd: String::from("NA"),
        }
    }
}

impl SecDealData {
    pub fn new() -> Self {
        SecDealData {
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct SecDealMap {
    pub store: HashMap<String, SecDealData>,
}

impl SecDealMap {
    pub fn new() -> Self {
        SecDealMap {
            store: HashMap::new(),
        }
    }
}
