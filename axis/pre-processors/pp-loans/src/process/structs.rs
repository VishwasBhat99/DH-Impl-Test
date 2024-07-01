use chrono::NaiveDate;
use rbdate::date_from_timestamp;

#[derive(Clone, Debug)]
pub struct LoanAccount {
    pub dis_amt: String,
    pub rephasement_principal: String,
    pub rep_shdl_num: i64,
    pub rep_shdl_date: NaiveDate,
    pub acid: String,
    pub bacid: String,
    pub clr_bal_amt: f64,
    pub out_bal_amt: f64,
    pub sanct_lim: f64,
    pub gl_sub_head_code: String,
    pub schm_code: String,
    pub schm_type: String,
    pub acct_crncy_code: String,
    pub acct_opn_date: NaiveDate,
    pub dis_shdl_num: String,
    pub dis_shdl_date: NaiveDate,
    pub sol_id: String,
    pub custname: String,
    pub interest_rate: f64,
    pub end_date: NaiveDate,
    pub pegged_flg: String,
    pub cust_id: String,
    pub foracid: String,
    pub ei_schm_flg: String,
    pub num_of_flows: f64,
    pub flow_start_date: NaiveDate,
    pub flow_amt: f64,
    pub lr_freq_type: String,
    pub num_of_dmds: f64,
    pub cashflow_code: String,
    pub exrate: f64,
    pub segment_code: String,
    pub npa_classification: String,
    pub floating_type: String,
    pub cust_hlth_code: String,
    pub cust_npa_class: String,
    pub final_npa_class: String,
    pub repricing_plan: String,
    pub next_repricing_date: NaiveDate,
}

impl LoanAccount {
    pub fn new() -> LoanAccount {
        LoanAccount {
            acid: "NA".to_string(),
            acct_crncy_code: "NA".to_string(),
            acct_opn_date: date_from_timestamp(0),
            bacid: "NA".to_string(),
            clr_bal_amt: 0.0,
            out_bal_amt: 0.0,
            cust_id: "NA".to_string(),
            custname: "NA".to_string(),
            foracid: "NA".to_string(),
            gl_sub_head_code: "NA".to_string(),
            pegged_flg: "NA".to_string(),
            sanct_lim: 0.0,
            schm_code: "NA".to_string(),
            schm_type: "NA".to_string(),
            sol_id: "NA".to_string(),
            dis_amt: "NA".to_string(),
            dis_shdl_date: date_from_timestamp(0),
            dis_shdl_num: "NA".to_string(),
            rep_shdl_date: date_from_timestamp(0),
            rep_shdl_num: 0,
            rephasement_principal: "NA".to_string(),
            ei_schm_flg: "NA".to_string(),
            interest_rate: 0.0,
            end_date: date_from_timestamp(0),
            num_of_flows: 0.0,
            flow_start_date: date_from_timestamp(0),
            flow_amt: 0.0,
            lr_freq_type: "NA".to_string(),
            num_of_dmds: 0.0,
            cashflow_code: "NA".to_string(),
            exrate: 0.0,
            segment_code: "NA".to_string(),
            npa_classification: "0".to_string(),
            floating_type: "0".to_string(),
            cust_hlth_code: "0".to_string(),
            cust_npa_class: "0".to_string(),
            final_npa_class: "0".to_string(),
            repricing_plan: "NA".to_string(),
            next_repricing_date: date_from_timestamp(0),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LamData {
    pub dis_amt: f64,
    pub dis_shdl_date: NaiveDate,
    pub dis_shdl_num: String,
    pub rep_shdl_date: NaiveDate,
    pub rep_shdl_num: i64,
    pub rephasement_principal: String,
    pub ei_schm_flg: String,
}

#[derive(Clone)]
pub struct IntRateData {
    pub interest_rate: f64,
    pub end_date: NaiveDate,
    pub pegged_flg: String,
    pub int_tbl_code: String,
}

#[derive(Clone, Debug)]
pub struct LrsData {
    pub num_of_flows: f64,
    pub flow_start_date: NaiveDate,
    pub flow_amt: f64,
    pub lr_freq_type: String,
    pub num_of_dmds: f64,
    pub cashflow_code: String,
}

#[derive(Clone)]
pub struct BenchmarkData {
    pub floating_type: String,
    pub repricing_plan: String,
    pub peg_review_date: NaiveDate,
}

#[derive(Clone)]
pub struct NpaData {
    pub npa_amount: String,
    pub npa_classification: String,
    pub cust_hlth_code: String,
    pub cust_npa_class: String,
    pub final_npa_class: String,
}
