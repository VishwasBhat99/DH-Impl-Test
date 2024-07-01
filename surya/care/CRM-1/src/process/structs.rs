#[derive(PartialEq, Clone, Default)]
pub struct CRMData {
    pub col_id: i64,
    pub exp_acc_no: ::std::string::String,
    pub cust_no: i64,
    pub col_type: ::std::string::String,
    pub tot_col_val_lcy: f64,
    pub col_ccy: ::std::string::String,
    pub current_val_col_lcy: f64,
    pub mat_date_col: String,
    pub is_eligible: ::std::string::String,
    pub haircut_prcnt: f64,
    pub haircut_amt: f64,
    pub col_val_aftr_haircut: f64,
}

impl CRMData {
    pub fn new() -> CRMData {
        CRMData {
            col_id: 0,
            exp_acc_no: "".to_string(),
            cust_no: 0,
            col_type: "".to_string(),
            tot_col_val_lcy: 0.0,
            col_ccy: "".to_string(),
            current_val_col_lcy: 0.0,
            mat_date_col: "01-01-1970".to_string(),
            is_eligible: "".to_string(),
            haircut_prcnt: 0.0,
            haircut_amt: 0.0,
            col_val_aftr_haircut: 0.0,
        }
    }

    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            self.col_id,
            self.exp_acc_no,
            self.cust_no,
            self.col_type,
            self.tot_col_val_lcy,
            self.col_ccy,
            self.current_val_col_lcy,
            self.mat_date_col,
            self.is_eligible,
            self.haircut_prcnt,
            self.haircut_amt,
            self.col_val_aftr_haircut,
        )
    }
}
