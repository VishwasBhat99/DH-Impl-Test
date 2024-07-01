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
    pub mis1: String,
    pub org_mis2: String,
    pub new_mis2: String,
    pub mis3: String,
    pub ccy: String,
    pub org_psl_mapping: String,
    pub new_psl_mapping: String,
    pub source_gl: String,
    pub original_amount: String,
    pub previous_os_amount: String,
    pub current_os_amount: String,
    pub old_rate_type: String,
    pub new_rate_type: String,
    pub old_bm: String,
    pub new_bm: String,
    pub old_bm_rate: String,
    pub new_bm_rate: String,
    pub old_int_spread: String,
    pub new_int_spread: String,
    pub int_rt_prev_mth: String,
    pub int_rt_cur_mth: String,
    pub old_psl_rate: String,
    pub new_psl_rate: String,
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
    pub ftm_impact: f64,
    pub residual_tenor: i64,
    pub residual_tenor_impact: f64,
    pub present_value: f64,
}

impl OpLeadingFields {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.account_number,
            self.source_system,
            self.customer_id,
            self.customer_name,
            self.product_code,
            self.scheme_id,
            self.booking_date,
            self.validity_date,
            self.maturity_date,
            self.mis1,
            self.org_mis2,
            self.new_mis2,
            self.mis3,
            self.ccy,
            self.org_psl_mapping,
            self.new_psl_mapping,
            self.source_gl,
            self.original_amount,
            self.previous_os_amount,
            self.current_os_amount,
            self.old_rate_type,
            self.new_rate_type,
            self.old_bm,
            self.new_bm,
            self.old_bm_rate,
            self.new_bm_rate,
            self.old_int_spread,
            self.new_int_spread,
            self.int_rt_prev_mth,
            self.int_rt_cur_mth,
            self.old_psl_rate,
            self.new_psl_rate,
            self.int_diff,
        )
    }
}

impl OpDrilldownReport {
    pub fn print(&self) -> String {
        format!(
            "|{}|{}|{}|{}|",
            self.ftm_impact, self.residual_tenor, self.residual_tenor_impact, self.present_value,
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
