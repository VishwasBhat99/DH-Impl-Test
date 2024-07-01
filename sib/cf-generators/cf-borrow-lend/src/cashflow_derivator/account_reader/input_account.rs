use rbdate::DateParser;
use rbdate::NaiveDate;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub as_on_date: NaiveDate,
    pub deal_number: String,
    pub instrument_type: String,
    pub branch: i64,
    pub outstanding_amount: f64,
    pub currency: String,
    pub instrument_classification: String,
    pub counter_party_id: i64,
    pub counter_party_name: String,
    pub counter_party_type: String,
    pub borrowing_date: NaiveDate,
    pub borrowing_maturity_date: NaiveDate,
    pub interest_rate: f64,
    pub interest_rate_classification: String,
    pub frequency: f64,
    pub basis: f64,
    pub next_repricing_date: NaiveDate,
    pub last_repricing_date: NaiveDate,
    pub repricing_frequency: String,
    pub coupon_payment_start_date: NaiveDate,
    pub coupon_payment_frequency: String,
    pub benchmark: String,
    pub spread: String,
    pub isin_code: String,
    pub mduration: String,
    pub treasury_gl_code: String,
    pub accrued_interest: f64,
    pub accrued_gl: String,
    pub deal_date: NaiveDate,
    pub value_date: NaiveDate,
    pub avg_mon_balance: f64,
    pub cdr_flg: String,
    pub cf_type: String,
    pub cf_sub_type: String,
    pub cf_amount: f64,
    pub cf_date: NaiveDate,
    pub cf_currency: String,
    pub outstanding_diff_amount: f64,
    pub group: String,
    pub llg: String,
    pub other_llg_classification: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            as_on_date: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch deal start date from input."),
                },
                None => {
                    return Err("Could not parse property `as_on_date`.");
                }
            },
            deal_number: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `deal_number`.");
                }
            },
            instrument_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `instrument_type`.");
                }
            },
            branch: match value_iterator.next() {
                Some(val) => val.parse::<i64>().unwrap_or(0),
                None => {
                    return Err("Could not parse property `branch`.");
                }
            },
            outstanding_amount: match value_iterator.next() {
                Some(val) => val.parse::<f64>().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `outstanding_amount`.");
                }
            },
            currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `currency`.");
                }
            },
            instrument_classification: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `instrument_classification`.");
                }
            },
            counter_party_id: match value_iterator.next() {
                Some(val) => val.parse::<i64>().unwrap_or(0),
                None => {
                    return Err("Could not parse property `counter_party_id`.");
                }
            },
            counter_party_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `counter_party_name`.");
                }
            },
            counter_party_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `counter_party_type`.");
                }
            },
            borrowing_date: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch borrowing_date from input."),
                },
                None => {
                    return Err("Could not parse property `borrowing_date`.");
                }
            },
            borrowing_maturity_date: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch borrowing_maturity_date from input."),
                },
                None => {
                    return Err("Could not parse property `borrowing_maturity_date`.");
                }
            },
            interest_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `interest_rate`.");
                }
            },
            interest_rate_classification: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `interest_rate_classification`.");
                }
            },
            frequency: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `frequency`.");
                }
            },
            basis: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `basis`.");
                }
            },
            next_repricing_date: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch next_repricing_date from input."),
                },
                None => {
                    return Err("Could not parse property `next_repricing_date`.");
                }
            },
            last_repricing_date: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch last_repricing_date from input."),
                },
                None => {
                    return Err("Could not parse property `last_repricing_date`.");
                }
            },
            repricing_frequency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `repricing_frequency`.");
                }
            },
            coupon_payment_start_date: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch coupon_payment_start_date from input."),
                },
                None => {
                    return Err("Could not parse property `coupon_payment_start_date`.");
                }
            },
            coupon_payment_frequency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `coupon_payment_frequency`.");
                }
            },
            benchmark: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `benchmark`.");
                }
            },
            spread: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `spread`.");
                }
            },
            isin_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `isin_code`.");
                }
            },
            mduration: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `mduration`.");
                }
            },
            treasury_gl_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `treasury_gl_code`.");
                }
            },
            accrued_interest: match value_iterator.next() {
                Some(val) => val.parse::<f64>().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `accrued_interest`.");
                }
            },
            accrued_gl: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `accrued_gl`.");
                }
            },
            deal_date: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch deal_date from input."),
                },
                None => {
                    return Err("Could not parse property `deal_date`.");
                }
            },
            value_date: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch value_date from input."),
                },
                None => {
                    return Err("Could not parse property `value_date`.");
                }
            },
            avg_mon_balance: match value_iterator.next() {
                Some(val) => val.parse::<f64>().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `avg_mon_balance`.");
                }
            },
            cdr_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cdr_flg`.");
                }
            },
            cf_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cf_type`.");
                }
            },
            cf_sub_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cf_sub_type`.");
                }
            },
            cf_amount: match value_iterator.next() {
                Some(val) => val.parse::<f64>().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `cf_amount`.");
                }
            },
            cf_date: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch cf_date from input."),
                },
                None => {
                    return Err("Could not parse property `cf_date`.");
                }
            },
            cf_currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cf_currency`.");
                }
            },
            outstanding_diff_amount: match value_iterator.next() {
                Some(val) => val.parse::<f64>().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `outstanding_diff_amount`.");
                }
            },
            group: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `group`.");
                }
            },
            llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `llg`.");
                }
            },
            other_llg_classification: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `other_llg_classification`.");
                }
            },
        };
        Ok(input_account)
    }
}
