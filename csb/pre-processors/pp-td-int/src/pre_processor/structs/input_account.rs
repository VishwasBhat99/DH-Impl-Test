#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InputAccount {
    pub acc_no: String,
    pub br_cd: String,
    pub cust_id: String,
    pub ucic_id: String,
    pub ccy: String,
    pub prod_cd: String,
    pub gl_cd: String,
    pub gl_comp_portion: String,
    pub acc_open_dt: String,
    pub effc_dt: String,
    pub bal_os: String,
    pub bal_os_cly: String,
    pub int_comp_type: String,
    pub compo_int_amt: String,
    pub int_rt: String,
    pub mat_dt: String,
    pub dep_amt: String,
    pub dep_amt_lcy: String,
    pub int_amt: String,
    pub int_acc_amt: String,
    pub non_with_flag: String,
    pub notice_day: String,
    pub cust_const_code: String,
    pub const_desc: String,
    pub cntrct_num: String,
    pub as_on: String,
    pub pay_freq: String,
    pub comp_freq: String,
    pub over_dt: String,
    pub lst_int_acr_dt: String,
    pub int_pay_amt: String,
}

impl InputAccount {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|",
            self.acc_no,
            self.br_cd,
            self.cust_id,
            self.ucic_id,
            self.ccy,
            self.prod_cd,
            self.gl_cd,
            self.gl_comp_portion,
            self.acc_open_dt,
            self.effc_dt,
            self.bal_os,
            self.bal_os_cly,
            self.int_comp_type,
            self.compo_int_amt,
            self.int_rt,
            self.mat_dt,
            self.dep_amt,
            self.dep_amt_lcy,
            self.int_amt,
            self.int_acc_amt,
            self.non_with_flag,
            self.notice_day,
            self.cust_const_code,
            self.cntrct_num,
            self.as_on,
            self.comp_freq.to_uppercase(),
            self.pay_freq.to_uppercase(),
            self.over_dt,
            self.lst_int_acr_dt,
            self.int_pay_amt,
        )
    }
}
