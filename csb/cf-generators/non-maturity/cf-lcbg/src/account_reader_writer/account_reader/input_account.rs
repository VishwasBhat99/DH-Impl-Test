use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub acc_num: String,
    pub br_code: String,
    pub client_id: String,
    pub lc_typ: String,
    pub ccy: String,
    pub gl_cd: String,
    pub acc_open_dt: Option<NaiveDate>,
    pub lc_amt: f64,
    pub bal_os: f64,
    pub lc_dt: Option<NaiveDate>,
    pub cancel_dt: Option<NaiveDate>,
    pub lst_dt_of_negotiation: Option<NaiveDate>,
    pub acc_typ_cd: String,
    pub acc_typ_desc: String,
    pub prod_code: String,
    pub as_on: Option<NaiveDate>,
    pub client_type: String,
    pub clients_name: String,
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
            acc_num: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acc_num`.");
                }
            },
            br_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `br_code`.");
                }
            },
            client_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `client_id`.");
                }
            },
            lc_typ: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `lc_typ`.");
                }
            },
            ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ccy`.");
                }
            },
            gl_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `gl_cd`.");
                }
            },
            acc_open_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `acc_open_dt`.");
                }
            },
            lc_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `lc_amt`.");
                }
            },
            bal_os: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `bal_os`.");
                }
            },

            lc_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `lc_dt`.");
                }
            },
            cancel_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `cancel_dt`.");
                }
            },
            lst_dt_of_negotiation: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `lst_dt_of_negotiation`.");
                }
            },
            acc_typ_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acc_typ_cd`.");
                }
            },
            acc_typ_desc: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acc_typ_desc`.");
                }
            },
            prod_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `prod_code`.");
                }
            },
            as_on: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `as_on`.");
                }
            },
            client_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `client_type`.");
                }
            },
            clients_name: match value_iterator.next() {
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
