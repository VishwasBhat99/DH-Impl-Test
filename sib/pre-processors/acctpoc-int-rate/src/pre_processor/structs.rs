#[derive(Debug, Clone, PartialEq)]
pub struct ItcData {
    pub int_tbl_code: String,
    pub int_tbl_ver_num: i64,
    pub id_cr_pref_pcnt: f64,
    pub id_dr_pref_pcnt: f64,
    pub cust_dr_pref_pcnt: f64,
    pub int_tbl_code_srl_num: String,
    pub min_int_pcnt_dr: f64,
    pub max_int_pcnt_dr: f64,
}

impl Default for ItcData {
    fn default() -> ItcData {
        ItcData {
            int_tbl_code: "".to_string(),
            int_tbl_ver_num: 0,
            id_cr_pref_pcnt: 0.0,
            id_dr_pref_pcnt: 0.0,
            cust_dr_pref_pcnt: 0.0,
            int_tbl_code_srl_num: "".to_string(),
            min_int_pcnt_dr: 0.0,
            max_int_pcnt_dr: 0.0,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct IvsKey {
    pub int_tbl_code: String,
    pub crncy_code: String,
    pub int_tbl_ver_num: i64,
    pub int_slab_dr_cr_flg: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IvsVal {
    pub nrml_int_pcnt: f64,
    pub end_slab_amt: f64,
    pub int_slab_srl_num: i64,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct IcvKey {
    pub int_tbl_code: String,
    pub crncy_code: String,
    //pub int_tbl_ver_num: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IcvValue {
    pub int_version: String,
    pub base_int_tbl_code: String,
    pub base_pcnt_dr: f64,
    pub base_pcnt_cr: f64,
    pub lchg_time: String,
    pub start_date: String,
    pub end_time: String,
    pub int_tbl_ver_num: i64,
}

impl Default for IcvValue {
    fn default() -> IcvValue {
        IcvValue {
            int_version: "".to_string(),
            base_int_tbl_code: "".to_string(),
            base_pcnt_dr: 0.0,
            base_pcnt_cr: 0.0,
            lchg_time: "".to_string(),
            start_date: "".to_string(),
            end_time: "".to_string(),
            int_tbl_ver_num: 0,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct IvsLavsMinKey {
    pub int_tbl_code: String,
    pub crncy_code: String,
    pub int_tbl_ver_num: i64,
    pub int_slab_dr_cr_flg: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IvsLavsMinVal {
    pub end_slab_amt: f64,
    pub int_slab_srl_num: i64,
    pub nrml_int_pcnt: f64,
}

impl Default for IvsLavsMinVal {
    fn default() -> Self {
        IvsLavsMinVal {
            end_slab_amt: 0.0,
            int_slab_srl_num: 0,
            nrml_int_pcnt: 0.0,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PCAData {
    pub disb_id: String,
    pub ost_amt: f64,
}

impl Default for PCAData {
    fn default() -> Self {
        PCAData {
            disb_id: "".to_string(),
            ost_amt: 0.0,
        }
    }
}
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct IvsLavsKey {
    pub int_tbl_code: String,
    pub crncy_code: String,
    pub int_tbl_ver_num: i64,
    pub int_slab_dr_cr_flg: String,
    pub int_slab_srl_num: String,
}

#[derive(Debug)]
pub struct IvsLavsVal {
    pub nrml_int_pcnt: f64,
}

impl Default for IvsLavsVal {
    fn default() -> Self {
        IvsLavsVal { nrml_int_pcnt: 0.0 }
    }
}
