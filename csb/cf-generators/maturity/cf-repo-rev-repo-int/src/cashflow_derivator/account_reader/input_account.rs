use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub deal_no: String,
    pub book_value: f64,
    pub ccy: String,
    pub cntr_party_id: String,
    pub cntr_party_name: String,
    pub cntr_party_type: String,
    pub repo_dt: Option<NaiveDate>,
    pub repo_mat_dt: Option<NaiveDate>,
    pub int_rate: f64,
    pub int_amt: f64,
    pub treas_gl_cd: i64,
    pub client_type: String,
    pub clients_name1: String,
    pub clients_bsr_type_flg: String,
    pub clients_busdivn_code: String,
    pub clients_const_code: String,
    pub clients_pan_gir_num: String,
    pub clients_risk_categorization: String,
    pub clients_risk_cntry: String,
    pub clients_segment_code: String,
    pub corpcl_orgn_qualifier: String,
    pub corpcl_indus_code: String,
    pub corpcl_nature_of_bus1: String,
    pub corpcl_central_state_flg: String,
    pub corpcl_public_sector_flg: String,
    pub corpcl_primary_dlr_flg: String,
    pub corpcl_multilateral_bank: String,
    pub corpcl_connp_inv_num: String,
    pub corpcl_bc_gross_turnover: f64,
    pub w4b_cd: String,
    pub balm_llg: String,
    pub care_llg: String,
    pub ba_llg: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            deal_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `account_id`.");
                }
            },
            book_value: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `book_value`.");
                }
            },
            ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ccy`.");
                }
            },
            cntr_party_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cntr_party_id`.");
                }
            },
            cntr_party_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cntr_party_name`.");
                }
            },
            cntr_party_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cntr_party_type`.");
                }
            },
            repo_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `repo_dt`.");
                }
            },
            repo_mat_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `repo_mat_dt`.");
                }
            },
            int_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_rate`.");
                }
            },
            int_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt`.");
                }
            },
            treas_gl_cd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `treas_gl_cd`.");
                }
            },
            client_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `client_type`.");
                }
            },
            clients_name1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `clients_name`.");
                }
            },
            clients_bsr_type_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `clients_bsr_type_flg`.");
                }
            },
            clients_busdivn_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `clients_busdivn_code`.");
                }
            },
            clients_const_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `clients_const_code`.");
                }
            },
            clients_pan_gir_num: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `clients_pan_gir_num`.");
                }
            },
            clients_risk_categorization: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `clients_risk_categorization`.");
                }
            },
            clients_risk_cntry: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `clients_risk_cntry`.");
                }
            },
            clients_segment_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `clients_segment_code`.");
                }
            },
            corpcl_orgn_qualifier: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `corpcl_orgn_qualifier`.");
                }
            },
            corpcl_indus_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `corpcl_indus_code`.");
                }
            },
            corpcl_nature_of_bus1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `corpcl_nature_of_bus1`.");
                }
            },
            corpcl_central_state_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `corpcl_central_state_flg`.");
                }
            },
            corpcl_public_sector_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `corpcl_public_sector_flg`.");
                }
            },
            corpcl_primary_dlr_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `corpcl_primary_dlr_flg`.");
                }
            },
            corpcl_multilateral_bank: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `corpcl_multilateral_bank`.");
                }
            },
            corpcl_connp_inv_num: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `corpcl_connp_inv_num`.");
                }
            },
            corpcl_bc_gross_turnover: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `corpcl_bc_gross_turnover`.");
                }
            },
            w4b_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `w4b_cd`.");
                }
            },
            balm_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `balm_llg`.");
                }
            },
            care_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `care_llg`.");
                }
            },
            ba_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ba_llg`.");
                }
            },
        };
        Ok(input_account)
    }
}
