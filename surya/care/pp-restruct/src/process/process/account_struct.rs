pub struct ExtraFieldData {
    pub acc_id: String,
    pub sanc_dt: String,
    pub occp_cd: String,
    pub sens_sec: String,
    pub prior_subtype: String,
    pub restruct_flag: String,
    pub restruct_dt: String,
    pub mor_prd: String,
    pub rating: String,
    pub consitin: String,
    pub pan: String,
    pub limit_amt: String,
    pub gross_adv: String,
    pub exp_amt: String,
    pub unvail_amt: String,
    pub gold_gram: String,
    pub fund_flag: String,
}
impl ExtraFieldData {
    pub fn to_string(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.acc_id,
            self.sanc_dt,
            self.occp_cd,
            self.sens_sec,
            self.prior_subtype,
            self.restruct_flag,
            self.restruct_dt,
            self.mor_prd,
            self.rating,
            self.consitin,
            self.pan,
            self.limit_amt,
            self.gross_adv,
            self.exp_amt,
            self.unvail_amt,
            self.gold_gram,
            self.fund_flag
        )
    }
}
