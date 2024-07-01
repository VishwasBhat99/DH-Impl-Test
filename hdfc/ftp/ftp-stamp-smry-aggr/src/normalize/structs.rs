use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, Default, Copy)]
pub struct AggregatedValue {
    pub ttl_crnt_ost_td: f64,
    pub ttl_avg_bal: f64,
    pub ttl_accr_int: f64,
    pub ttl_base_tpr: f64,
    pub ttl_adjs: f64,
    pub ttl_fin_tpr: f64,
    pub ttl_margin: f64,
    pub ttl_spread: f64,
    pub ttl_psl_amt: f64,
    pub ttl_ftp_amt: f64,
    pub ttl_ftp_without_psl: f64,
    pub ttl_additional_smf: f64,
    pub gr_ofs_gl_amt: f64,
    pub ui_ofs_gl_amt: f64,
    pub re_ofs_gl_amt: f64,
    pub is_ofs_gl_amt: f64,
    pub int_income_gl_amt: f64,
    pub overdue_int_gl_amt: f64,
    pub int_on_cancellation_gl_amt: f64,
    pub woff_gl_amt: f64,
    pub count: usize,
}

impl AggregatedValue {
    pub fn add(&mut self, aggr_value: AggregatedValue) {
        self.ttl_crnt_ost_td += aggr_value.ttl_crnt_ost_td;
        self.ttl_avg_bal += aggr_value.ttl_avg_bal;
        self.ttl_accr_int += aggr_value.ttl_accr_int;
        self.ttl_base_tpr += aggr_value.ttl_base_tpr;
        self.ttl_adjs += aggr_value.ttl_adjs;
        self.ttl_fin_tpr += aggr_value.ttl_fin_tpr;
        self.ttl_margin += aggr_value.ttl_margin;
        self.ttl_spread += aggr_value.ttl_spread;
        self.ttl_psl_amt += aggr_value.ttl_psl_amt;
        self.ttl_ftp_amt += aggr_value.ttl_ftp_amt;
        self.ttl_ftp_without_psl += aggr_value.ttl_ftp_without_psl;
        self.ttl_additional_smf += aggr_value.ttl_additional_smf;
        self.gr_ofs_gl_amt += aggr_value.gr_ofs_gl_amt;
        self.ui_ofs_gl_amt += aggr_value.ui_ofs_gl_amt;
        self.re_ofs_gl_amt += aggr_value.re_ofs_gl_amt;
        self.is_ofs_gl_amt += aggr_value.is_ofs_gl_amt;
        self.int_income_gl_amt += aggr_value.int_income_gl_amt;
        self.overdue_int_gl_amt += aggr_value.overdue_int_gl_amt;
        self.int_on_cancellation_gl_amt += aggr_value.int_on_cancellation_gl_amt;
        self.woff_gl_amt += aggr_value.woff_gl_amt;
        self.count += aggr_value.count;
    }
}


impl Display for AggregatedValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.gr_ofs_gl_amt,
            self.ui_ofs_gl_amt,
            self.re_ofs_gl_amt,
            self.is_ofs_gl_amt,
            self.int_income_gl_amt,
            self.overdue_int_gl_amt,
            self.int_on_cancellation_gl_amt,
            self.woff_gl_amt,
            self.ttl_avg_bal,
            self.ttl_accr_int,
            self.ttl_psl_amt,
            self.ttl_ftp_amt,
            self.ttl_ftp_without_psl,
            self.ttl_additional_smf,
            self.ttl_crnt_ost_td,
            self.ttl_base_tpr,
            self.ttl_adjs,
            self.ttl_margin
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct AggregatedValue_Report2 {
    pub dep_type: String,
    pub mis2: String,
    pub rate_flag: String,
    pub ttl_avg_bal: f64,
    pub ttl_accr_int: f64,
    pub ttl_base_tpr: f64,
    pub ttl_margin: f64,
    pub ttl_psl: f64,
    pub ttl_fin_tpr: f64,
    pub days_in_month: f64,
    pub days_in_year: f64,
    pub weighted_yield: f64,
    pub weighted_base_ftp_rate: f64,
    pub weighted_psl_rate: f64,
    pub weighted_final_ftp_rate: f64,
    pub weighted_total_spread: f64,
}

impl AggregatedValue_Report2 {
    pub fn add(&mut self, aggr_value: AggregatedValue_Report2) {
        self.dep_type = aggr_value.dep_type;
        self.mis2 = aggr_value.mis2;
        self.rate_flag = aggr_value.rate_flag;
        self.ttl_avg_bal += aggr_value.ttl_avg_bal;
        self.ttl_accr_int += aggr_value.ttl_accr_int;
        self.ttl_base_tpr += aggr_value.ttl_base_tpr;
        self.ttl_margin += aggr_value.ttl_margin;
        self.ttl_psl += aggr_value.ttl_psl;
        self.ttl_fin_tpr += aggr_value.ttl_fin_tpr;
        self.days_in_month = aggr_value.days_in_month;
        self.days_in_year = aggr_value.days_in_year;
        self.weighted_yield = get_div_val(self.ttl_accr_int,self.ttl_avg_bal)*get_div_val(self.days_in_year,self.days_in_month);
        self.weighted_base_ftp_rate = get_div_val(self.ttl_base_tpr,self.ttl_avg_bal)*100.0;
        self.weighted_psl_rate = get_div_val(self.ttl_psl,self.ttl_avg_bal)*100.0;
        self.weighted_final_ftp_rate = get_div_val(self.ttl_fin_tpr,self.ttl_avg_bal)*100.0;
        self.weighted_total_spread = self.weighted_yield - self.weighted_final_ftp_rate;
    }
}

impl Display for AggregatedValue_Report2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:.4}|{:.4}|{:.4}|{:.4}|{:.4}",
            self.weighted_yield,
            self.weighted_base_ftp_rate,
            self.weighted_psl_rate,
            self.weighted_final_ftp_rate,
            self.weighted_total_spread
        )
    }
}

fn get_div_val(num: f64, den: f64) -> f64 {
    if den == 0.0 {
        0.0
    } else {
        num / den
    }
}
