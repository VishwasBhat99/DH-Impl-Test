#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct InputFields {
    pub col_id: String,
    pub acc_id: String,
    pub acc_type: String,
    pub cust_id: String,
    pub col_type_cd: String,
    pub col_type_desc: String,
    pub tot_val_of_col: String,
    pub ccy: String,
    pub tot_mk_val_of_col: f64,
    pub net_weight: String,
}

impl InputFields {
    pub fn print(&self, mat_dt: &str) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            self.col_id,
            self.acc_id,
            self.acc_type,
            self.cust_id,
            self.col_type_cd,
            self.col_type_desc,
            self.tot_val_of_col,
            self.ccy,
            self.tot_mk_val_of_col,
            mat_dt,
        )
    }
}
