#[derive(Clone, Debug, Default)]
pub struct GAMFields {
    pub branch: String,
    pub scheme_code: String,
    pub acid: String,
    pub customer_id: String,
    pub gl_code: String,
    pub entity_cre_flag: String,
    pub cif_id: String,
    pub schm_type: String,
    pub acct_ownership: String,
}

impl GAMFields {
    pub fn new() -> GAMFields {
        GAMFields {
            branch: "NA".to_string(),
            scheme_code: "NA".to_string(),
            acid: "NA".to_string(),
            customer_id: "NA".to_string(),
            gl_code: "NA".to_string(),
            entity_cre_flag: "NA".to_string(),
            cif_id: "NA".to_string(),
            schm_type: "NA".to_string(),
            acct_ownership: "NA".to_string(),
        }
    }
    pub fn get_gam_fields(fields: Vec<&str>) -> GAMFields {
        GAMFields {
            branch: fields[5].to_string(),
            scheme_code: fields[14].to_string(),
            acid: fields[0].to_string(),
            customer_id: fields[6].to_string(),
            gl_code: fields[13].to_string(),
            entity_cre_flag: fields[21].to_string(),
            cif_id: fields[31].to_string(),
            schm_type: fields[15].to_string(),
            acct_ownership: fields[7].to_string(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct NPAFields {
    pub npa_classification: String,
    pub provision_amt: f64,
    pub claim_amount: f64,
    pub writeoff: String,
    pub gnpa: String,
}

impl NPAFields {
    pub fn new() -> NPAFields {
        NPAFields {
            npa_classification: "NA".to_string(),
            provision_amt: 0.0,
            claim_amount: 0.0,
            writeoff: "NA".to_string(),
            gnpa: "NA".to_string(),
        }
    }
    pub fn get_npa_fields(fields: Vec<&str>) -> NPAFields {
        NPAFields {
            npa_classification: fields[2].to_string(),
            provision_amt: fields[3].parse::<f64>().unwrap_or(0.0),
            claim_amount: fields[4].parse::<f64>().unwrap_or(0.0),
            writeoff: fields[5].to_string(),
            gnpa: fields[6].to_string(),
        }
    }
}
