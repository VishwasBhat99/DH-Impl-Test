#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct FBMData {
    pub bill_param_type: String,
    pub bill_b2k_id: String,
    pub bill_id: String,
    pub bill_amt: String,
    pub bill_amt_inr: String,
    pub bill_crncy_code: String,
    pub due_date: String,
    pub bp_acid: String,
    pub del_flg: String,
    pub cls_flg: String,
    pub reg_type: String,
    pub reg_sub_type: String,
    pub bp_liab: String,
    pub bp_liab_crncy: String,
    pub bill_liab_inr: String,
    pub bill_stat: String,
    pub bill_func_code: String,
    pub bill_liab: String,
    pub bill_liab_hc_eq: String,
    pub bill_liab_crncy: String,
}

impl FBMData {
    pub fn new(input_acc: Vec<&str>) -> FBMData {
        FBMData {
            bill_param_type: input_acc[0].trim().to_string(),
            bill_b2k_id: input_acc[1].trim().to_string(),
            bill_id: input_acc[2].trim().to_string(),
            bill_amt: input_acc[3].trim().to_string(),
            bill_amt_inr: input_acc[4].trim().to_string(),
            bill_crncy_code: input_acc[5].trim().to_string(),
            due_date: input_acc[6].trim().to_string(),
            bp_acid: input_acc[7].trim().to_string(),
            del_flg: input_acc[8].trim().to_string(),
            cls_flg: input_acc[9].trim().to_string(),
            reg_type: input_acc[10].trim().to_string(),
            reg_sub_type: input_acc[11].trim().to_string(),
            bp_liab: input_acc[12].trim().to_string(),
            bp_liab_crncy: input_acc[13].trim().to_string(),
            bill_liab_inr: input_acc[14].trim().to_string(),
            bill_stat: input_acc[15].trim().to_string(),
            bill_func_code: input_acc[16].trim().to_string(),
            bill_liab: input_acc[17].trim().to_string(),
            bill_liab_hc_eq: input_acc[18].trim().to_string(),
            bill_liab_crncy: input_acc[19].trim().to_string(),
        }
    }
}

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct NPAData {
    pub npa_classification: String,
    pub cust_hlth_code: String,
    pub cust_npa_class: String,
    pub final_npa_class: String,
    pub npa_amount: String,
}

impl NPAData {
    pub fn new(input_acc: Vec<&str>) -> NPAData {
        NPAData {
            npa_classification: input_acc[8].trim().to_string(),
            cust_hlth_code: input_acc[12].trim().to_string(),
            cust_npa_class: input_acc[17].trim().to_string(),
            final_npa_class: input_acc[18].trim().to_string(),
            npa_amount: input_acc[10].trim().to_string(),
        }
    }
}
