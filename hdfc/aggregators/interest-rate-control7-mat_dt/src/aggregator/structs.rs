#[derive(Debug, Deserialize)]
pub struct OpLeadingFields {
    pub account_number: String,
    pub source_system: String,
    pub customer_id: String,
    pub customer_name: String,
    pub product_code: String,
    pub scheme_id: String,
    pub booking_date: String,
    pub value_date: String,
    pub maturity_dt_org: String,
    pub maturity_dt_new: String,
    pub mis1: String,
    pub mis2: String,
    pub mis3: String,
    pub ccy: String,
    pub original_amount: String,
    pub current_os_amount: String,
    pub old_bm: String,
    pub new_bm: String,
    pub old_rate_type: String,
    pub new_rate_type: String,
    pub old_bm_rate: String,
    pub new_bm_rate: String,
    pub int_rt_prev_mth: String,
    pub int_rt_cur_mth: String,
    pub spread_old: String,
    pub spread_new: String,
}

#[derive(Debug, Deserialize)]
pub struct OpDrilldownReport {
    pub res_tenor_old: i64,
    pub res_tenor_new: i64,
    pub res_tenor_impact: f64,
    pub present_val: f64,
}

#[derive(Debug, Deserialize)]
pub struct OpTrailingFields {
    pub alm_line: String,
    pub ia_line: String,
    pub concat: String,
    pub division: String,
    pub npa_type: String,
    pub raw_bm: String,
    pub final_bm: String,
    pub old_rt_flag: String,
    pub new_rt_flag: String,
}

impl OpLeadingFields {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|",
            self.account_number,
            self.source_system,
            self.customer_id,
            self.customer_name,
            self.product_code,
            self.scheme_id,
            self.booking_date,
            self.value_date,
            self.maturity_dt_org,
            self.maturity_dt_new,
            self.mis1,
            self.mis2,
            self.mis3,
            self.ccy,
            self.original_amount,
            self.current_os_amount,
            self.old_bm,
            self.new_bm,
            self.old_rate_type,
            self.new_rate_type,
            self.old_bm_rate,
            self.new_bm_rate,
            self.int_rt_prev_mth,
            self.int_rt_cur_mth,
            self.spread_old,
            self.spread_new
        )
    }
}

impl OpDrilldownReport {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|",
            self.res_tenor_old, self.res_tenor_new, self.res_tenor_impact, self.present_val,
        )
    }
}

impl OpTrailingFields {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.alm_line,
            self.ia_line,
            self.concat,
            self.division,
            self.npa_type,
            self.raw_bm,
            self.final_bm,
            self.old_rt_flag,
            self.new_rt_flag,
        )
    }
}
