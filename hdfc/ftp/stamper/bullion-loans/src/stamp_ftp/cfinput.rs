#[derive(Debug, Clone)]
pub struct AccFieldNames {
    pub acc_no: String,
    pub counter_party: String,
    pub ccy: String,
    pub gl_no: String,
    pub amt: String,
    pub int_rt: String,
    pub st_dt: String,
    pub mat_dt: String,
    pub alm_line: String,
    pub div: String,
    pub cust_typ: String,
    pub as_on_dt: String,
    pub nxt_rep_dt: String,
    pub tot_int_amt: String,
    pub tot_prin_amt: String,
    pub compmis1: String,
    pub compmis2: String,
    pub cashflows: String,
}

impl AccFieldNames {
    pub fn get_input_fields_names() -> AccFieldNames {
        AccFieldNames {
            acc_no: "acc_no".to_string(),
            counter_party: "counter_party".to_string(),
            ccy: "ccy".to_string(),
            gl_no: "gl_no".to_string(),
            amt: "amt".to_string(),
            int_rt: "int_rt".to_string(),
            st_dt: "st_dt".to_string(),
            mat_dt: "mat_dt".to_string(),
            alm_line: "alm_line".to_string(),
            div: "div".to_string(),
            cust_typ: "cust_typ".to_string(),
            as_on_dt: "as_on_dt".to_string(),
            nxt_rep_dt: "nxt_rep_dt".to_string(),
            tot_int_amt: "tot_int_amt".to_string(),
            tot_prin_amt: "tot_prin_amt".to_string(),
            compmis1: "compmis3".to_string(),
            compmis2: "compmis2".to_string(),
            cashflows: "cashflows".to_string(),
        }
    }
}
