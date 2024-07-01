use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    // Standard Fields
    pub account_id: String,
    pub currency: String,
    pub int_rate: f64,
    pub outstanding_bal: Option<f64>,
    pub field_type: String,
    pub cf_principal_amount: f64,
    pub cf_date: Option<NaiveDate>,
    pub gl: String,
    pub start_date: Option<NaiveDate>,
    pub maturity_date: Option<NaiveDate>,
    pub rate_flag: String,
    pub branch: String,
    pub customer_id: String,
    pub customer_type: String,
    pub product_code: String,
    // Standard Passthrough
    pub group: String,
    pub acc_branch: String,
    pub acc_number: String,
    pub acc_suffix: String,
    pub acc_type: String,
    pub deal_type: String,
    pub repricing_frequency: String,
    pub last_repr_date: Option<NaiveDate>,
    pub next_repr_date: Option<NaiveDate>,
    pub int_compounding_frequency: String,
    pub int_repayment_frequency: String,
    pub margin_rate: String,
    pub cpas: String,
    pub cust_constitution_code: String,
    pub customer_rating: String,
    pub p2: String,
    pub analysis_code: String,
    pub sundry_analysis_code: String,
    pub numeric_analysis_code: String,
    pub base_rate_code: String,
    pub differential_rate_code: String,
    pub accrued_int_amt: f64,
    // Additional Passthrough
    pub next_rollover_date: Option<NaiveDate>,
    pub npa_flag: String,
    pub npa_type: String,
    pub rm: String,
    pub customer_name: String,
    pub monthly_avg_bal: f64,
    pub pension_account_flag: String,
    pub waiver_flag: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            // Standard Fields
            account_id: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `account_id`.");
                }
            },
            currency: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `currency`.");
                }
            },
            int_rate: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_rate`.");
                }
            },
            outstanding_bal: match value_iterator.next() {
                Some(val) => val.trim().parse().ok(),
                None => {
                    return Err("Could not parse property `outstanding_bal`.");
                }
            },
            field_type: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `field_type`.");
                }
            },
            cf_principal_amount: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `cf_principal_amount`.");
                }
            },
            cf_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `cf_date`.");
                }
            },
            gl: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `gl`.");
                }
            },
            start_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `start_date`.");
                }
            },
            maturity_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `maturity_date`.");
                }
            },
            rate_flag: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `rate_flag`.");
                }
            },
            branch: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `branch`.");
                }
            },
            customer_id: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `customer_id`.");
                }
            },
            customer_type: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `customer_type`.");
                }
            },
            product_code: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `product_code`.");
                }
            },
            // Standard Passthrough
            group: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `group`.");
                }
            },
            acc_branch: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `acc_branch`.");
                }
            },
            acc_number: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `acc_number`.");
                }
            },
            acc_suffix: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `acc_suffix`.");
                }
            },
            acc_type: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `acc_type`.");
                }
            },
            deal_type: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `deal_type`.");
                }
            },
            repricing_frequency: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `repricing_frequency`.");
                }
            },
            last_repr_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `last_repr_date`.");
                }
            },
            next_repr_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `next_repr_date`.");
                }
            },
            int_compounding_frequency: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `int_compounding_frequency`.");
                }
            },
            int_repayment_frequency: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `int_repayment_frequency`.");
                }
            },
            margin_rate: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `margin_rate`.");
                }
            },
            cpas: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `cpas`.");
                }
            },
            cust_constitution_code: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `cust_constitution_code`.");
                }
            },
            customer_rating: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `customer_rating`.");
                }
            },
            p2: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `p2`.");
                }
            },
            analysis_code: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `analysis_code`.");
                }
            },
            sundry_analysis_code: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `sundry_analysis_code`.");
                }
            },
            numeric_analysis_code: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `numeric_analysis_code`.");
                }
            },
            base_rate_code: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `base_rate_code`.");
                }
            },
            differential_rate_code: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `differential_rate_code`.");
                }
            },
            accrued_int_amt: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `accrued_int_amt`.");
                }
            },
            // Additional Passthrough
            next_rollover_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `next_rollover_date`.");
                }
            },
            npa_flag: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `npa_flag`.");
                }
            },
            npa_type: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `npa_type`.");
                }
            },
            rm: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `rm`.");
                }
            },
            customer_name: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `customer_name`.");
                }
            },
            monthly_avg_bal: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `monthly_avg_bal`.");
                }
            },
            pension_account_flag: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `pension_account_flag`.");
                }
            },
            waiver_flag: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `waiver_flag`.");
                }
            },
        };
        Ok(input_account)
    }
}
