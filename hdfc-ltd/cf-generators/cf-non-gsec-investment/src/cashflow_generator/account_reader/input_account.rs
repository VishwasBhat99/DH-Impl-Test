use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub holding_deal_id: String,
    pub portfolio: String,
    pub instrument_type: String,
    pub isdiscounted: String,
    pub slr_non_slr: String,
    pub isin: String,
    pub issuer_id: String,
    pub issuer_name: String,
    pub security_id: String,
    pub security_name: String,
    pub deal_value_date: Option<NaiveDate>,
    pub issue_date: Option<NaiveDate>,
    pub maturity_date: Option<NaiveDate>,
    pub coupon_rate: f64,
    pub interest_calculation_basis: String,
    pub face_value_per_units: f64,
    pub quantity: i64,
    pub face_value: Option<f64>,
    pub currency: String,
    pub book_value: Option<f64>,
    pub market_value: f64,
    pub mtm: i64,
    pub market_yield: f64,
    pub interest_accruad_days: i64,
    pub accrued_interest: Option<f64>,
    pub coupon_pay_day: i64,
    pub coupon_type: String,
    pub spread: String,
    pub benchmark: String,
    pub coupon_frequency: String,
    pub last_coupon_date: Option<NaiveDate>,
    pub next_coupon_date: Option<NaiveDate>,
    pub last_repricing_date: Option<NaiveDate>,
    pub next_repricing_date: Option<NaiveDate>,
    pub put_date: Option<NaiveDate>,
    pub call_date: Option<NaiveDate>,
    pub is_listed: String,
    pub issuer_country: String,
    pub issuer_type: String,
    pub isuser_classification_1: String,
    pub isuser_classification_2: String,
    pub isuser_classification_3: String,
    pub gurantor_type: String,
    pub rating_agency: String,
    pub external_rating: String,
    pub asset_type: String,
    pub asset_category: String,
    pub treaury_gl_code: i64,
    pub m_npaclassification: String,
    pub internal_rating: String,
    pub customer_constitution_code: String,
    pub risk_weight: String,
    pub crar_basel_classification: String,
    pub listing_status: String,
    pub listed_exchange: String,
    pub open_ended: String,
    pub haircut: String,
    pub mtm_gl_code: String,
    pub mtm_double: f64,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            holding_deal_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `holding_deal_id`.");
                }
            },
            portfolio: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `portfolio`.");
                }
            },
            instrument_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `instrument_type`.");
                }
            },
            isdiscounted: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `isdiscounted`.");
                }
            },
            slr_non_slr: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `slr_non_slr`.");
                }
            },
            isin: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `isin`.");
                }
            },
            issuer_id: match value_iterator.next() {
                Some(val) => val.to_string(),
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
            security_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `security_id`.");
                }
            },
            security_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `security_name`.");
                }
            },
            deal_value_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `deal_value_date`.");
                }
            },
            issue_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `issue_date`.");
                }
            },
            maturity_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `maturity_date`.");
                }
            },
            coupon_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `coupon_rate`.");
                }
            },
            interest_calculation_basis: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `interest_calculation_basis`.");
                }
            },
            face_value_per_units: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `face_value_per_units`.");
                }
            },
            quantity: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `quantity`.");
                }
            },
            face_value: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `face_value`.");
                }
            },
            currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `currency`.");
                }
            },
            book_value: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `book_value`.");
                }
            },
            market_value: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `market_value`.");
                }
            },
            mtm: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `mtm`.");
                }
            },
            market_yield: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `market_yield`.");
                }
            },
            interest_accruad_days: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `interest_accruad_days`.");
                }
            },
            accrued_interest: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `accrued_interest`.");
                }
            },
            coupon_pay_day: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `coupon_pay_day`.");
                }
            },
            coupon_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `coupon_type`.");
                }
            },
            spread: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `spread`.");
                }
            },
            benchmark: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `benchmark`.");
                }
            },
            coupon_frequency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `coupon_frequency`.");
                }
            },
            last_coupon_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `last_coupon_date`.");
                }
            },
            next_coupon_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `next_coupon_date`.");
                }
            },
            last_repricing_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `last_repricing_date`.");
                }
            },
            next_repricing_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `next_repricing_date`.");
                }
            },
            put_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `put_date`.");
                }
            },
            call_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `call_date`.");
                }
            },
            is_listed: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `is_listed`.");
                }
            },
            issuer_country: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `issuer_country`.");
                }
            },
            issuer_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `issuer_type`.");
                }
            },
            isuser_classification_1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `isuser_classification_1`.");
                }
            },
            isuser_classification_2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `isuser_classification_2`.");
                }
            },
            isuser_classification_3: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `isuser_classification_3`.");
                }
            },
            gurantor_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `gurantor_type`.");
                }
            },
            rating_agency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rating_agency`.");
                }
            },
            external_rating: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `external_rating`.");
                }
            },
            asset_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `asset_type`.");
                }
            },
            asset_category: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `asset_category`.");
                }
            },
            treaury_gl_code: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `treaury_gl_code`.");
                }
            },
            m_npaclassification: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `m_npaclassification`.");
                }
            },
            internal_rating: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `internal_rating`.");
                }
            },
            customer_constitution_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `customer_constitution_code`.");
                }
            },
            risk_weight: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `risk_weight`.");
                }
            },
            crar_basel_classification: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `crar_basel_classification`.");
                }
            },
            listing_status: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `listing_status`.");
                }
            },
            listed_exchange: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `listed_exchange`.");
                }
            },
            open_ended: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `open_ended`.");
                }
            },
            haircut: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `haircut`.");
                }
            },
            mtm_gl_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `mtm_gl_code`.");
                }
            },
            mtm_double: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `mtm`.");
                }
            },
        };
        Ok(input_account)
    }
}
