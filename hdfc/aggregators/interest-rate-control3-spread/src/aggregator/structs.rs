#[derive(Debug, Deserialize)]
pub struct OpLeadingFields {
    pub account_number: String,
    pub source_system: String,
    pub customer_id: String,
    pub customer_name: String,
    pub product_code: String,
    pub scheme_id: String,
    pub booking_date: String,
    pub validity_date: String,
    pub maturity_date: String,
    pub ccy: String,
    pub mis1: String,
    pub mis2: String,
    pub mis3: String,
    pub source_gl: String,
    pub int_rt_prev_mth: String,
    pub int_rt_cur_mth: String,
    pub old_benchmark: String,
    pub new_benchmark: String,
    pub old_bm_rate: String,
    pub new_bm_rate: String,
    pub old_spread_bm: String,
    pub new_spread_bm: String,
    pub original_amount: String,
    pub current_os_amount: String,
    pub old_effective_int_rt: String,
    pub new_effective_int_rt: String,
    pub int_rate_diff: String,
    pub spread_diff: String,
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
#[derive(Debug, Deserialize)]
pub struct OpDrilldownReport {
    pub ftm_impact: f64,
    pub res_tenor: i64,
    pub res_ten_impact: f64,
    pub present_val: String,
}

impl OpLeadingFields {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|",
            self.account_number,
            self.source_system,
            self.customer_id,
            self.customer_name,
            self.product_code,
            self.scheme_id,
            self.booking_date,
            self.validity_date,
            self.maturity_date,
            self.ccy,
            self.mis1,
            self.mis2,
            self.mis3,
            self.source_gl,
            self.int_rt_prev_mth,
            self.int_rt_cur_mth,
            self.old_benchmark,
            self.new_benchmark,
            self.old_bm_rate,
            self.new_bm_rate,
            self.old_spread_bm,
            self.new_spread_bm,
            self.original_amount,
            self.current_os_amount,
            self.old_effective_int_rt,
            self.new_effective_int_rt,
            self.int_rate_diff,
            self.spread_diff,
        )
    }
}

impl OpDrilldownReport {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|",
            self.ftm_impact.to_string(),
            self.res_tenor.to_string(),
            self.res_ten_impact.to_string(),
            self.present_val.to_string(),
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
