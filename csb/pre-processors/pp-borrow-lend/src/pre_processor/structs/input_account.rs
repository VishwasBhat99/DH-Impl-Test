#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InputAccount {
    pub deal_num: String,
    pub os_bal: String,
    pub ccy: String,
    pub instrument: String,
    pub counter_party_id: String,
    pub counter_party_name: String,
    pub counter_party_type: String,
    pub borrowing_dt: String,
    pub maturity_dt: String,
    pub int_rt: String,
    pub int_rate_classification: String,
    pub next_reprice_dt: String,
    pub coupan_pay_strt_dt: String,
    pub coupan_pay_freq: String,
    pub spread: String,
    pub treasury_gl_code: String,
}

impl InputAccount {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|",
            self.deal_num,
            self.os_bal,
            self.ccy,
            self.instrument,
            self.counter_party_id,
            self.counter_party_name,
            self.counter_party_type,
            self.borrowing_dt,
            self.maturity_dt,
            self.int_rt,
            self.int_rate_classification,
            self.next_reprice_dt,
            self.coupan_pay_strt_dt,
            self.coupan_pay_freq,
            self.spread,
            self.treasury_gl_code
        )
    }
}
