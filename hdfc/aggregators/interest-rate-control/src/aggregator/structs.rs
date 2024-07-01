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
    pub rate_type: String,
    pub original_amount: String,
    pub current_os_amount: String,
    pub int_rt_prev_mth: String,
    pub int_rt_cur_mth: String,
    pub int_diff: String,
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
    pub opening_bal: f64,
    pub principal: f64,
    pub interest_amt: f64,
    pub reducing_principal: f64,
    pub ftm_impact: f64,
    pub residual_tenor: i64,
    pub residual_tenor_yrs: f64,
    pub residual_tenor_impact: f64,
    pub discount_factor: f64,
    pub present_value: f64,
}

impl OpLeadingFields {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|",
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
            self.rate_type,
            self.original_amount,
            self.current_os_amount,
            self.int_rt_prev_mth,
            self.int_rt_cur_mth,
            self.int_diff,
        )
    }
}

impl OpDrilldownReport {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|",
            self.opening_bal.to_string(),
            self.principal.to_string(),
            self.interest_amt.to_string(),
            self.reducing_principal.to_string(),
            self.ftm_impact.to_string(),
            self.residual_tenor.to_string(),
            self.residual_tenor_yrs.to_string(),
            self.residual_tenor_impact.to_string(),
            self.discount_factor.to_string(),
            self.present_value.to_string(),
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
