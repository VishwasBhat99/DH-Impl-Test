use rbdate::DateParser;
use rbdate::NaiveDate;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub flow_id: String,
    pub group_id: i64,
    pub sub_group_id: i64,
    pub amount: f64,
    pub currency: String,
    pub int_rate: f64,
    pub repr_freq: String,
    pub early_date: Option<NaiveDate>,
    pub maturity_date: Option<NaiveDate>,
    pub account_id: String,
    pub start_date: Option<NaiveDate>,
    pub int_freq: String,
    pub is_floating_rate: String,
    pub floating_bnchmrk: String,
    pub business_unit_id: String,
    pub cust_id: String,
    pub cust_name: String,
    pub spread: String,
    pub scheme_code: String,
    pub min_ir: f64,
    pub max_ir: f64,
    pub principal_amount: f64,
    pub maturity_value: f64,
    pub ccy_conv_rate: f64,
    pub cust_cnrty_code: String,
    pub cust_crd_rating: String,
    pub cust_sec_code: String,
    pub cust_indt_code: String,
    pub custom1: String,
    pub custom2: String,
    pub waiver: String,
    pub maturity_modify: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            flow_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `flow_id`.");
                }
            },
            group_id: match value_iterator.next() {
                Some(val) => val.parse::<i64>().unwrap_or(0),
                None => {
                    return Err("Could not parse property `group_id`.");
                }
            },
            sub_group_id: match value_iterator.next() {
                Some(val) => val.parse::<i64>().unwrap_or(0),
                None => {
                    return Err("Could not parse property `sub_group_id`.");
                }
            },
            amount: match value_iterator.next() {
                Some(val) => val.parse::<f64>().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `amount`.");
                }
            },
            currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `currency`.");
                }
            },
            int_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `int_rate`.");
                }
            },
            repr_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `repr_freq`.");
                }
            },
            early_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `early_date`.");
                }
            },
            maturity_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `maturity_date`.");
                }
            },
            account_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `account_id`.");
                }
            },
            start_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `start_date`.");
                }
            },
            int_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `int_freq`.");
                }
            },
            is_floating_rate: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `is_floating_rate`.");
                }
            },
            floating_bnchmrk: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `floating_bnchmrk`.");
                }
            },
            business_unit_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `business_unit_id`.");
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
            spread: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `spread`.");
                }
            },
            scheme_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `scheme_code`.");
                }
            },
            min_ir: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `min_ir`.");
                }
            },
            max_ir: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `max_ir`.");
                }
            },
            principal_amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `principal_amount`.");
                }
            },
            maturity_value: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `maturity_value`.");
                }
            },
            ccy_conv_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `ccy_conv_rate`.");
                }
            },
            cust_cnrty_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_cnrty_code`.");
                }
            },
            cust_crd_rating: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_crd_rating`.");
                }
            },
            cust_sec_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_sec_code`.");
                }
            },
            cust_indt_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_indt_code`.");
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
            waiver: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `waiver`.");
                }
            },
            maturity_modify: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `maturity_modify`.");
                }
            },
        };
        Ok(input_account)
    }
}
