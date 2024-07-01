use chrono::NaiveDate;
use rbdate::date_from_timestamp;

#[derive(Clone, Debug)]
pub struct LoanAccount {
    pub accid: String,
    pub sanct_limit: String,
    pub pout_bal: f64,
    pub int_rate: String,
    pub ei_or_nonei: String,
    pub ccy: String,
    pub sdate: NaiveDate,
    pub scheme_code: String,
    pub cbs_gl_code: String,
    pub int_type: String,
    pub floating_type: String,
    pub c_date: NaiveDate,
    pub dis_amt: f64,
    pub dis_date: NaiveDate,
    pub npa_classification: String,
    pub npa_amount: f64,
    pub npa: String,
    pub segment_code: String,
    pub branch_id: String,
    pub repricing_date: NaiveDate,
    pub cust_id: String,
    pub total_num_inst: i64,
    pub num_inst_paid: i64,
    pub inst_start_date: NaiveDate,
    pub num_inst: i64,
    pub p_frequency_code: String,
    pub i_frequency_code: String,
    pub freq_period: i64,
    pub cfp_amt: f64,
    pub last_installment_date: NaiveDate,
    pub freq: i64,
    pub pri_inst_start_date: NaiveDate,
    pub int_inst_start_date: NaiveDate,
    pub exrate: f64,
    pub frequency_type: String,
    pub final_npa_class: String,
}

impl LoanAccount {
    pub fn new() -> LoanAccount {
        LoanAccount {
            accid: "NA".to_string(),
            sanct_limit: "0.0".to_string(),
            pout_bal: 0.0,
            int_rate: "NA".to_string(),
            ei_or_nonei: "NA".to_string(),
            ccy: "NA".to_string(),
            sdate: date_from_timestamp(0),
            scheme_code: "NA".to_string(),
            cbs_gl_code: "NA".to_string(),
            int_type: "NA".to_string(),
            floating_type: "NA".to_string(),
            c_date: NaiveDate::parse_from_str("01-01-2099", "%d-%m-%Y")
                .unwrap_or_else(|_| date_from_timestamp(0)),
            dis_amt: 0.0,
            dis_date: date_from_timestamp(0),
            npa_classification: "0".to_string(),
            npa_amount: 0.0,
            npa: "N".to_string(),
            segment_code: "060".to_string(),
            branch_id: "Axis".to_string(),
            repricing_date: NaiveDate::parse_from_str("01-01-2099", "%d-%m-%Y")
                .unwrap_or_else(|_| date_from_timestamp(0)),
            cust_id: "NA".to_string(),
            total_num_inst: 0,
            num_inst_paid: 0,
            inst_start_date: date_from_timestamp(0),
            num_inst: 0,
            p_frequency_code: "NA".to_string(),
            i_frequency_code: "NA".to_string(),
            freq_period: 0,
            cfp_amt: 0.0,
            last_installment_date: date_from_timestamp(0),
            freq: 0,
            pri_inst_start_date: date_from_timestamp(0),
            int_inst_start_date: date_from_timestamp(0),
            exrate: 1.0,
            frequency_type: "NA".to_string(),
            final_npa_class: "NA".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct LoanRepStr {
    pub total_num_inst: i64,
    pub num_inst_paid: i64,
    pub inst_start_date: NaiveDate,
    pub freq_type: String,
    pub freq_period: i64,
    pub inst_amt: f64,
}
#[derive(Clone)]
pub struct NpaData {
    pub npa_classification: String,
    pub final_npa_class: String,
    pub cif_no: String,
    pub npa_amount: String,
}
