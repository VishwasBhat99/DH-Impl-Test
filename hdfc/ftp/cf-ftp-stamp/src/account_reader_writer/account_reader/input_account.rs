use rbdate::{DateParser, NaiveDate};
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub account_number: String,
    pub cust_name: String,
    pub average_balance: f64,
    pub accr_int: f64,
    pub yld_to_call: String,
    pub interest_rate: f64,
    pub base_rate_1: f64,
    pub final_ftp_rate: f64,
    pub value_date: Option<NaiveDate>,
    pub maturity_date: Option<NaiveDate>,
    pub next_reprice_date: Option<NaiveDate>,
    pub last_reprice_date: Option<NaiveDate>,
    pub mis1: String,
    pub mis2: String,
    pub psl_code: String,
    pub prod_code_type: String,
    pub rate_flag: String,
    pub blank_1: String,
    pub source_file_name: String,
    pub currency: String,
    pub gl: i64,
    pub cust_id: String,
    pub final_ftp_amount: f64,
    pub alm_line: String,
    pub blank_2: String,
    pub initial_dep_amt_td: f64,
    pub current_outstanding_td: f64,
    pub base_rate_2: f64,
    pub adj1: f64,
    pub adj2: f64,
    pub adj3: f64,
    pub adj4: f64,
    pub adj5: f64,
    pub adj6: f64,
    pub input_benchmark: String,
    pub pdo: String,
    pub npa: String,
    pub ftp_method: String,
    pub ftp_rate_curve: String,
    pub org_tenor: i64,
    pub repricing_tenor: i64,
    pub fixed_spread: f64,
    pub variable_spread: f64,
    pub first_month_ftp: String,
    pub bc_as_on_rule: Option<NaiveDate>,
    pub tenor_start_date_rule: Option<NaiveDate>,
    pub tenor_end_date_rule: Option<NaiveDate>,
    pub bc_as_on_applied: Option<NaiveDate>,
    pub tenor_start_date_applied: Option<NaiveDate>,
    pub tenor_end_date_applied: Option<NaiveDate>,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            account_number: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `account_number`.");
                }
            },
            cust_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_name`.");
                }
            },
            average_balance: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `average_balance`.");
                }
            },
            accr_int: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `accr_int`.");
                }
            },
            yld_to_call: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `yld_to_call`.");
                }
            },
            interest_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `interest_rate`.");
                }
            },
            base_rate_1: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `base_rate_1`.");
                }
            },
            final_ftp_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `final_ftp_rate`.");
                }
            },
            value_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `value_date`.");
                }
            },
            maturity_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `maturity_date`.");
                }
            },
            next_reprice_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `next_reprice_date`.");
                }
            },
            last_reprice_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `last_reprice_date`.");
                }
            },
            mis1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `mis1`.");
                }
            },
            mis2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `mis2`.");
                }
            },
            psl_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `psl_code`.");
                }
            },
            prod_code_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `prod_code_type`.");
                }
            },
            rate_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rate_flag`.");
                }
            },
            blank_1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `blank_1`.");
                }
            },
            source_file_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `source_file_name`.");
                }
            },
            currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `currency`.");
                }
            },
            gl: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `gl`.");
                }
            },
            cust_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_id`.");
                }
            },
            final_ftp_amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `final_ftp_amount`.");
                }
            },
            alm_line: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `alm_line`.");
                }
            },
            blank_2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `blank_2`.");
                }
            },
            initial_dep_amt_td: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `initial_dep_amt_td`.");
                }
            },
            current_outstanding_td: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `current_outstanding_td`.");
                }
            },
            base_rate_2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `base_rate_2`.");
                }
            },
            adj1: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `adj1`.");
                }
            },
            adj2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `adj2`.");
                }
            },
            adj3: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `adj3`.");
                }
            },
            adj4: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `adj4`.");
                }
            },
            adj5: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `adj5`.");
                }
            },
            adj6: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `adj6`.");
                }
            },
            input_benchmark: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `input_benchmark`.");
                }
            },
            pdo: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pdo`.");
                }
            },
            npa: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `npa`.");
                }
            },
            ftp_method: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ftp_method`.");
                }
            },
            ftp_rate_curve: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ftp_rate_curve`.");
                }
            },
            org_tenor: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `org_tenor`.");
                }
            },
            repricing_tenor: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `repricing_tenor`.");
                }
            },
            fixed_spread: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `fixed_spread`.");
                }
            },
            variable_spread: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `variable_spread`.");
                }
            },
            first_month_ftp: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `first_month_ftp`.");
                }
            },
            bc_as_on_rule: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `bc_as_on_rule`.");
                }
            },
            tenor_start_date_rule: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `tenor_start_date_rule`.");
                }
            },
            tenor_end_date_rule: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `tenor_end_date_rule`.");
                }
            },
            bc_as_on_applied: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `bc_as_on_applied`.");
                }
            },
            tenor_start_date_applied: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `tenor_start_date_applied`.");
                }
            },
            tenor_end_date_applied: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `tenor_end_date_applied`.");
                }
            },
        };
        Ok(input_account)
    }
}
