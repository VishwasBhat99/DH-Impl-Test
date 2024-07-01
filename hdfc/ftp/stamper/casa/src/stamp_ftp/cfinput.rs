#[derive(Debug, Clone)]
pub struct AccFieldNames {
    pub seg_1: String,
    pub account_no: String,
    pub seg_5: String,
    pub seg_6: String,
    pub seg_3: String,
    pub seg_8: String,
    pub seg_4: String,
    pub dr_bal: String,
    pub cr_bal: String,
    pub bal_total: String,
    pub concat: String,
    pub alm_line: String,
    pub int_rate: String,
    pub cf_type: String,
    pub gl_prefix: String,
    pub is_acct_gl: String,
    pub ia_line: String,
}

impl AccFieldNames {
    pub fn get_input_fields_names() -> AccFieldNames {
        AccFieldNames {
            seg_1: "seg_1".to_string(),
            account_no: "account_no".to_string(),
            seg_5: "seg_5".to_string(),
            seg_6: "seg_6".to_string(),
            seg_3: "seg_3".to_string(),
            seg_8: "seg_8".to_string(),
            seg_4: "seg_4".to_string(),
            dr_bal: "dr_bal".to_string(),
            cr_bal: "cr_bal".to_string(),
            bal_total: "bal_total".to_string(),
            concat: "concat".to_string(),
            alm_line: "alm_line".to_string(),
            int_rate: "int_rate".to_string(),
            cf_type: "cf_type".to_string(),
            gl_prefix: "gl_prefix".to_string(),
            is_acct_gl: "is_acct_gl".to_string(),
            ia_line: "ia_line".to_string(),
        }
    }
}
