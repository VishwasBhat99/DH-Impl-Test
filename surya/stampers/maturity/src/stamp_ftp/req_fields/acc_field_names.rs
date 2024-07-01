use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AccFieldNames {
    pub account_id: String,
    pub currency: String,
    pub eop_balance_ccy: String,
    pub eop_balance_hcy: String,
    pub int_rate: String,
    pub rate_flag: String,
    pub val_dt: String,
    pub open_dt: String,
    pub mat_dt: String,
    pub lst_repricing_dt: String,
    pub rep_freq: String,
    pub cust_agg_bal: String,
    pub day_count_basis: String,
    pub a_or_l: String,
    pub dim1: String,
    pub dim2: String,
    pub dim3: String,
    pub dim4: String,
    pub customer_id: String,
    pub rl1: String,
    pub rl2: String,
    pub rl3: String,
    pub gl_code: String,
    pub prod_code: String,
    pub div_code: String,
    pub mis_code_1: String,
    pub mis_code_2: String,
    pub mis_code_3: String,
    pub cashflows: String,
}

impl AccFieldNames {
    pub fn new_from_path(path: &str) -> AccFieldNames {
        let mut file = open_file(path);
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string");
        let req_fields: AccFieldNames = serde_json::from_str(&buf[..])
            .expect("Account metadata json file was not well-formatted.");
        req_fields
    }
}
