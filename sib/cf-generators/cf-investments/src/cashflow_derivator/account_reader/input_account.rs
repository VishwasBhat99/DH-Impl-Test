use rbdate::DateParser;
use rbdate::NaiveDate;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub as_on_date: NaiveDate,
    pub deal_number: String,
    pub portfolio: String,
    pub instrument_id: String,
    pub concat_id: String,
    pub instrument_type: String,
    pub branch_code: String,
    pub category: String,
    pub isin_code: String,
    pub security_name: String,
    pub face_value: f64,
    pub book_value: f64,
    pub appreciation_value: f64,
    pub depreciation_value: f64,
    pub wap: f64,
    pub market_value: f64,
    pub currency: String,
    pub acc_yield: f64,
    pub maturity_date: NaiveDate,
    pub coupon_classification: String,
    pub coupon_rate: f64,
    pub face_value_per_unit: f64,
    pub outstanding_quantity: f64,
    pub accrued_interest: f64,
    pub coupon_frequency: String,
    pub coupon_basis: String,
    pub put_date: NaiveDate,
    pub call_date: NaiveDate,
    pub last_coupon: NaiveDate,
    pub next_coupon: NaiveDate,
    pub issue_date: NaiveDate,
    pub last_repricing_date: NaiveDate,
    pub next_repricing_date: NaiveDate,
    pub place: String,
    pub country: String,
    pub slr_non_slr: String,
    pub listed_unlisted: String,
    pub secured_unsecured: String,
    pub issuer_id: i64,
    pub issuer_name: String,
    pub issuer_type: String,
    pub sub_issuer_type: String,
    pub external_rating_agency: String,
    pub rating: String,
    pub issuer_guaranteed_by: String,
    pub industry: String,
    pub sub_industry: String,
    pub npa_classification: String,
    pub deal_value_date: NaiveDate,
    pub duration: f64,
    pub mduration: f64,
    pub benchmark_mark: f64,
    pub spread_rate: f64,
    pub treasury_glcode: f64,
    pub avg_mon_balance: f64,
    pub deal_date: NaiveDate,
    pub cdr_flg: String,
    pub gl_sub_head_code: i64,
    pub group: String,
    pub llg: String,
    pub other_llg_classification: String,
    pub cashflow_type: String,
    pub cashflow_amount: f64,
    pub cashflow_currency: String,
    pub cashflow_date: NaiveDate,
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
            portfolio: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `portfolio`.");
                }
            },
            instrument_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `instrument_id`.");
                }
            },
            concat_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `concat_id`.");
                }
            },
            instrument_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `instrument_type`.");
                }
            },
            branch_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `branch_code`.");
                }
            },
            category: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `category`.");
                }
            },
            isin_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `isin_code`.");
                }
            },
            security_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `security_name`.");
                }
            },
            face_value: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `face_value`.");
                }
            },
            book_value: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `book_value`.");
                }
            },
            appreciation_value: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `appreciation_value`.");
                }
            },
            depreciation_value: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `depreciation_value`.");
                }
            },
            wap: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `wap`.");
                }
            },
            market_value: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `market_value`.");
                }
            },
            currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `currency`.");
                }
            },
            acc_yield: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `acc_yield`.");
                }
            },
            maturity_date: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch cf start date from input."),
                },
                None => {
                    return Err("Could not parse property `maturity_date`.");
                }
            },
            coupon_classification: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `coupon_classification`.");
                }
            },
            coupon_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `coupon_rate`.");
                }
            },
            face_value_per_unit: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `face_value_per_unit`.");
                }
            },
            outstanding_quantity: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `outstanding_quantity`.");
                }
            },
            accrued_interest: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `accrued_interest`.");
                }
            },
            coupon_frequency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `coupon_frequency`.");
                }
            },
            coupon_basis: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `coupon_basis`.");
                }
            },
            put_date: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch cf end date from input."),
                },
                None => {
                    return Err("Could not parse property `put_date`.");
                }
            },
            call_date: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch cf end date from input."),
                },
                None => {
                    return Err("Could not parse property `call_date`.");
                }
            },
            last_coupon: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch cf end date from input."),
                },
                None => {
                    return Err("Could not parse property `last_coupon`.");
                }
            },
            next_coupon: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch cf end date from input."),
                },
                None => {
                    return Err("Could not parse property `next_coupon`.");
                }
            },
            issue_date: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch cf end date from input."),
                },
                None => {
                    return Err("Could not parse property `issue_date`.");
                }
            },
            last_repricing_date: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch cf end date from input."),
                },
                None => {
                    return Err("Could not parse property `last_repricing_date`.");
                }
            },
            next_repricing_date: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch cf end date from input."),
                },
                None => {
                    return Err("Could not parse property `next_repricing_date`.");
                }
            },
            place: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `place`.");
                }
            },
            country: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `country`.");
                }
            },
            slr_non_slr: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `slr_non_slr`.");
                }
            },
            listed_unlisted: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `listed_unlisted`.");
                }
            },
            secured_unsecured: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `secured_unsecured`.");
                }
            },
            issuer_id: match value_iterator.next() {
                Some(val) => val.parse::<i64>().unwrap_or(0),
                None => {
                    return Err("Could not parse property `issuer_id`.");
                }
            },
            issuer_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `issuer_name`.");
                }
            },
            issuer_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `issuer_type`.");
                }
            },
            sub_issuer_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sub_issuer_type`.");
                }
            },
            external_rating_agency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `external_rating_agency`.");
                }
            },
            rating: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rating`.");
                }
            },
            issuer_guaranteed_by: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `issuer_guaranteed_by`.");
                }
            },
            industry: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `industry`.");
                }
            },
            sub_industry: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sub_industry`.");
                }
            },
            npa_classification: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `npa_classification`.");
                }
            },
            deal_value_date: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch next reset date from input."),
                },
                None => {
                    return Err("Could not parse property `deal_value_date`.");
                }
            },
            duration: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `duration`.");
                }
            },
            mduration: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `mduration`.");
                }
            },
            benchmark_mark: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `benchmark_mark`.");
                }
            },
            spread_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `spread_rate`.");
                }
            },
            treasury_glcode: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `treasury_glcode`.");
                }
            },
            avg_mon_balance: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `avg_mon_balance`.");
                }
            },
            deal_date: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch next reset date from input."),
                },
                None => {
                    return Err("Could not parse property `deal_date`.");
                }
            },
            cdr_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cdr_flg`.");
                }
            },
            gl_sub_head_code: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return Err("Could not parse property `gl_sub_head_code`.");
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
            cashflow_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cashflow_type`.");
                }
            },
            cashflow_amount: match value_iterator.next() {
                Some(val) => val.parse::<f64>().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `cashflow_amount`.");
                }
            },
            cashflow_currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cashflow_currency`.");
                }
            },
            cashflow_date: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch next reset date from input."),
                },
                None => {
                    return Err("Could not parse property `cashflow_date`.");
                }
            },
        };
        Ok(input_account)
    }
}
