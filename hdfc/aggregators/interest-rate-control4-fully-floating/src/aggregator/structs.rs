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
}

#[derive(Debug, Deserialize)]
pub struct SummaryOpLeadingFields {
    pub rate_type: String,
    pub benchmark: String,
    pub int_rt_cur_mth: String,
    pub bm_rate: String,
    pub bm_spread: String,
    pub last_reset_dt: String,
    pub next_reset_dt: String,
    pub original_amount: String,
    pub current_os_amount: String,
}

#[derive(Debug, Deserialize)]
pub struct DrilldownOpLeadingFields {
    pub old_int_rt: String,
    pub cur_int_rt: String,
    pub old_benchmark: String,
    pub benchmark: String,
    pub old_bm_rate: String,
    pub new_bm_rate: String,
    pub old_bm_spread: String,
    pub new_bm_spread: String,
    pub rate_type: String,
    pub last_reset_dt: String,
    pub next_reset_dt: String,
    pub org_amt: String,
    pub os_amt: String,
    pub int_rt_diff: String,
    pub spread_diff: String,
}

#[derive(Debug, Deserialize)]
pub struct OpDerivedFields {
    pub ftm_impact: String,
    pub residual_tenor: String,
    pub residual_tenor_impact: String,
    pub present_val: String,
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
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|",
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
        )
    }
}

impl SummaryOpLeadingFields {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|",
            self.rate_type,
            self.benchmark,
            self.int_rt_cur_mth,
            self.bm_rate,
            self.bm_spread,
            self.last_reset_dt,
            self.next_reset_dt,
            self.original_amount,
            self.current_os_amount,
        )
    }
}

impl DrilldownOpLeadingFields {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|",
            self.old_int_rt,
            self.cur_int_rt,
            self.old_benchmark,
            self.benchmark,
            self.old_bm_rate,
            self.new_bm_rate,
            self.old_bm_spread,
            self.new_bm_spread,
            self.rate_type,
            self.last_reset_dt,
            self.next_reset_dt,
            self.org_amt,
            self.os_amt,
            self.int_rt_diff,
            self.spread_diff,
        )
    }
}

impl OpDerivedFields {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|",
            self.ftm_impact, self.residual_tenor, self.residual_tenor_impact, self.present_val,
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
