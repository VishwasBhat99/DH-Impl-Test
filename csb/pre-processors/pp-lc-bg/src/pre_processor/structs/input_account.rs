#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InputAccount {
    pub acc_num: String,
    pub br_code: String,
    pub client_id: String,
    pub lc_typ: String,
    pub ccy: String,
    pub gl_cd: String,
    pub acc_open_dt: String,
    pub bal_os: String,
    pub lc_dt: String,
    pub cancel_dt: String,
    pub lst_dt_of_negotiation: String,
    pub acc_typ_cd: String,
    pub acc_typ_desc: String,
    pub prod_code: String,
    pub as_on: String,
}

impl InputAccount {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|",
            self.acc_num,
            self.br_code,
            self.client_id,
            self.lc_typ,
            self.ccy,
            self.gl_cd,
            self.acc_open_dt,
            self.bal_os,
            self.bal_os,
            self.lc_dt,
            self.cancel_dt,
            self.lst_dt_of_negotiation,
            self.acc_typ_cd,
            self.acc_typ_desc,
            self.prod_code,
            self.as_on,
        )
    }
}
