use rbdate::DateParser;
use rbdate::NaiveDate;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub id: String,
    pub account_number: String,
    pub cust_id: String,
    pub deal_name: String,
    pub principal_os: f64,
    pub deal_start_date: NaiveDate,
    pub cf_start_date: NaiveDate,
    pub cf_end_date: NaiveDate,
    pub accrued_interest: f64,
    pub deal_value: f64,
    pub gl_code: String,
    pub system_value: String,
    pub current_nominal_interest_rate: f64,
    pub product_type: String,
    pub originator_name: String,
    pub contract_yield: f64,
    pub payment_frequency: i64,
    pub loan_reset_frequency: String,
    pub interest_rate_type: String,
    pub next_reset_date: NaiveDate,
    pub borrower_constitution: String,
    pub pan: String,
    pub voter_id: String,
    pub external_benchmark: String,
    pub dpd_in_days: String,
    pub daily_dpd_reported_date: NaiveDate,
    pub due_from_customer: f64,
    pub cmonth_emi_due: f64,
    pub actual_amount_paid: f64,
    pub principal_due_cmonth: f64,
    pub principal_rcvd_cmonth: f64,
    pub interest_method_code: String,
    pub bank_share: f64,
    pub originator_share: f64,
    pub customer_od_bank_share: f64,
    pub customer_od_originator_share: f64,
    pub od_interest_bank_share: f64,
    pub od_interest_originator_share: f64,
    pub maturity_date: NaiveDate,
    pub exposure_unique_id: String,
    pub fic_mis_date: NaiveDate,
    pub system_date: NaiveDate,
    pub balm_control_status_id: String,
    pub derived_principal: f64,
    pub derived_cmonth_emi_due: f64,
    pub npa_classification: String,
    pub cust_hlth_code: String,
    pub cust_npa_class: String,
    pub final_npa_class: String,
    pub currency: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `id`.");
                }
            },
            account_number: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `account_number`.");
                }
            },
            cust_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_id`.");
                }
            },
            deal_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `deal_name`.");
                }
            },
            principal_os: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `principal_os`.");
                }
            },
            deal_start_date: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch deal start date from input."),
                },
                None => {
                    return Err("Could not parse property `deal_start_date`.");
                }
            },
            cf_start_date: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch cf start date from input."),
                },
                None => {
                    return Err("Could not parse property `cf_start_date`.");
                }
            },
            cf_end_date: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch cf end date from input."),
                },
                None => {
                    return Err("Could not parse property `cf_end_date`.");
                }
            },
            accrued_interest: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `accrued_interest`.");
                }
            },
            deal_value: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `deal_value`.");
                }
            },
            gl_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `gl_code`.");
                }
            },
            system_value: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `system_value`.");
                }
            },
            current_nominal_interest_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `current_nominal_interest_rate`.");
                }
            },
            product_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `product_type`.");
                }
            },
            originator_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `originator_name`.");
                }
            },
            contract_yield: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `contract_yield`.");
                }
            },
            payment_frequency: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return Err("Could not parse property `payment_frequency`.");
                }
            },
            loan_reset_frequency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `loan_reset_frequency`.");
                }
            },
            interest_rate_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `interest_rate_type`.");
                }
            },
            next_reset_date: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch next reset date from input."),
                },
                None => {
                    return Err("Could not parse property `next_reset_date`.");
                }
            },
            borrower_constitution: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `borrower_constitution`.");
                }
            },
            pan: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pan`.");
                }
            },
            voter_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `voter_id`.");
                }
            },
            external_benchmark: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `external_benchmark`.");
                }
            },
            dpd_in_days: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `dpd_in_days`.");
                }
            },
            daily_dpd_reported_date: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch next reset date from input."),
                },
                None => {
                    return Err("Could not parse property `daily_dpd_reported_date`.");
                }
            },
            due_from_customer: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `due_from_customer`.");
                }
            },
            cmonth_emi_due: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `cmonth_emi_due`.");
                }
            },
            actual_amount_paid: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `actual_amount_paid`.");
                }
            },
            principal_due_cmonth: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `principal_due_cmonth`.");
                }
            },
            principal_rcvd_cmonth: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `principal_rcvd_cmonth`.");
                }
            },
            interest_method_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `interest_method_code`.");
                }
            },
            bank_share: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `bank_share`.");
                }
            },
            originator_share: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `originator_share`.");
                }
            },
            customer_od_bank_share: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `customer_od_bank_share`.");
                }
            },
            customer_od_originator_share: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `customer_od_originator_share`.");
                }
            },
            od_interest_bank_share: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `od_interest_bank_share`.");
                }
            },
            od_interest_originator_share: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `od_interest_originator_share`.");
                }
            },
            maturity_date: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch next reset date from input."),
                },
                None => {
                    return Err("Could not parse property `maturity_date`.");
                }
            },
            exposure_unique_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `exposure_unique_id`.");
                }
            },
            fic_mis_date: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch next reset date from input."),
                },
                None => {
                    return Err("Could not parse property `fic_mis_date`.");
                }
            },
            system_date: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch next reset date from input."),
                },
                None => {
                    return Err("Could not parse property `system_date`.");
                }
            },
            balm_control_status_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `balm_control_status_id`.");
                }
            },
            derived_principal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `derived_principal`.");
                }
            },
            derived_cmonth_emi_due: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `derived_cmonth_emi_due`.");
                }
            },
            npa_classification: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `npa_classification`.");
                }
            },
            cust_hlth_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_hlth_code`.");
                }
            },
            cust_npa_class: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_npa_class`.");
                }
            },
            final_npa_class: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `final_npa_class`.");
                }
            },
            currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `currency`.");
                }
            },
        };
        Ok(input_account)
    }
}
