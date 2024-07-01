use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub acid: String,
    pub foracid: String,
    pub sol_id: String,
    pub acct_opn_date: Option<NaiveDate>,
    pub gl_sub_head_code: String,
    pub schm_code: String,
    pub schm_type: String,
    pub acct_crncy_code: String,
    pub rep_shdl_num: i64,
    pub rep_shdl_date: Option<NaiveDate>,
    pub dis_shdl_num: i64,
    pub dis_shdl_date: Option<NaiveDate>,
    pub dis_amt: f64,
    pub clr_bal_amt: f64,
    pub sanct_lim: f64,
    pub rephasement_principal: f64,
    pub ei_perd_end_date: Option<NaiveDate>,
    pub cust_id: String,
    pub cust_name: String,
    pub ei_schm_flg: String,
    pub int_basis: String,
    pub ei_formula_flg: String,
    pub ei_intcalc_freq: String,
    pub ei_method: String,
    pub int_rate: f64,
    pub int_type: String,
    pub next_repricing_date: Option<NaiveDate>,
    pub last_repricing_date: Option<NaiveDate>,
    pub repricing_freq: String,
    pub float_rate_benchmark: String,
    pub spread: f64,
    pub npa_flg: String,
    pub npa_classification: String,
    pub npa_amt: f64,
    pub cust_country_cd: String,
    pub cust_credit_rating: String,
    pub cust_sector_cd: String,
    pub cust_industry_cd: String,
    pub exchangert: f64,
    pub custom1: String,
    pub custom2: String,
    pub custom3: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            acid: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acid`.");
                }
            },
            foracid: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `foracid`.");
                }
            },
            sol_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sol_id`.");
                }
            },
            acct_opn_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `acct_opn_date`.");
                }
            },
            gl_sub_head_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `gl_sub_head_code`.");
                }
            },
            schm_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `schm_code`.");
                }
            },
            schm_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `schm_type`.");
                }
            },
            acct_crncy_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acct_crncy_code`.");
                }
            },
            rep_shdl_num: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `rep_shdl_num`.");
                }
            },
            rep_shdl_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `rep_shdl_date`.");
                }
            },
            dis_shdl_num: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `dis_shdl_num`.");
                }
            },
            dis_shdl_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `dis_shdl_date`.");
                }
            },
            dis_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `dis_amt`.");
                }
            },
            clr_bal_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `clr_bal_amt`.");
                }
            },
            sanct_lim: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `sanct_lim`.");
                }
            },
            rephasement_principal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `rephasement_principal`.");
                }
            },
            ei_perd_end_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `ei_perd_end_date`.");
                }
            },
            cust_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_id`.");
                }
            },
            cust_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_name`.");
                }
            },
            ei_schm_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ei_schm_flg`.");
                }
            },
            int_basis: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `int_basis`.");
                }
            },
            ei_formula_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ei_formula_flg`.");
                }
            },
            ei_intcalc_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ei_intcalc_freq`.");
                }
            },
            ei_method: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ei_method`.");
                }
            },
            int_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_rate`.");
                }
            },
            int_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `int_type`.");
                }
            },
            next_repricing_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `next_repricing_date`.");
                }
            },
            last_repricing_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `last_repricing_date`.");
                }
            },
            repricing_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `repricing_freq`.");
                }
            },
            float_rate_benchmark: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `float_rate_benchmark`.");
                }
            },
            spread: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `spread`.");
                }
            },
            npa_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `npa_flg`.");
                }
            },
            npa_classification: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `npa_classification`.");
                }
            },
            npa_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `npa_amt`.");
                }
            },
            cust_country_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_country_cd`.");
                }
            },
            cust_credit_rating: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_credit_rating`.");
                }
            },
            cust_sector_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_sector_cd`.");
                }
            },
            cust_industry_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_industry_cd`.");
                }
            },
            exchangert: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `exchangert`.");
                }
            },
            custom1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `custom1`.");
                }
            },
            custom2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `custom2`.");
                }
            },
            custom3: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `custom3`.");
                }
            },
        };
        Ok(input_account)
    }
}
