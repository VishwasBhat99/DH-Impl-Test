#[derive(Debug, Deserialize)]
pub struct OpLeadingFields {
    pub cust_id: String,
    pub pdt_code: String,
    pub ccy: String,
}

#[derive(Debug, Deserialize)]
pub struct SummaryOpTrailingFields {
    pub cust_name: String,
    pub value_dt: String,
    pub agg_booking: String,
    pub mat_dt: String,
    pub mis1: String,
    pub division: String,
    pub source_gl: String,
    pub org_tenor: String,
    pub bucket: String,
    pub effective_rate: String,
    pub alm_line: String,
    pub ia_line: String,
    pub concat: String,
    pub count_td: String,
    pub lcr_cat: String,
}
#[derive(Debug, Deserialize)]
pub struct DrilldownOpTrailingFields {
    pub acc_no: String,
    pub cust_name: String,
    pub value_dt: String,
    pub os_amt: String,
    pub mat_dt: String,
    pub mis1: String,
    pub division: String,
    pub source_gl: String,
    pub org_tenor: String,
    pub bucket: String,
    pub effective_rate: String,
    pub alm_line: String,
    pub ia_line: String,
    pub concat: String,
    pub lcr_cat: String,
}

impl OpLeadingFields {
    pub fn print(&self) -> String {
        format!("{}|{}|{}|", self.cust_id, self.pdt_code, self.ccy,)
    }
}

impl SummaryOpTrailingFields {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            self.cust_name,
            self.value_dt,
            self.agg_booking,
            self.mat_dt,
            self.mis1,
            self.division,
            self.source_gl,
            self.org_tenor,
            self.bucket,
            self.effective_rate,
            self.alm_line,
            self.ia_line,
            self.concat,
            self.count_td,
            self.lcr_cat,
        )
    }
}

impl DrilldownOpTrailingFields {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            self.acc_no,
            self.cust_name,
            self.value_dt,
            self.os_amt,
            self.mat_dt,
            self.mis1,
            self.division,
            self.source_gl,
            self.org_tenor,
            self.bucket,
            self.effective_rate,
            self.alm_line,
            self.ia_line,
            self.concat,
            self.lcr_cat,
        )
    }
}
