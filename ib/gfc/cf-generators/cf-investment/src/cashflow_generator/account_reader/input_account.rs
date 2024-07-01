use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;

use crate::cashflow_generator::DEFAULT_INT;

#[derive(Debug)]
pub struct InputAccount {
    pub deal_no: String,
    pub portfolio: String,
    pub instrument_id: String,
    pub instrument_type: String,
    pub isin_code: String,
    pub security_name: String,
    pub face_value: f64,
    pub book_value: f64,
    pub market_value: f64,
    pub currency: String,
    pub mtm: i64,
    pub yeild: f64,
    pub maturity_date: NaiveDate,
    pub coupon_classification_1: String,
    pub coupon_rate: f64,
    pub face_value_perunits: f64,
    pub quantity: f64,
    pub appreciation: String,
    pub depreciation: String,
    pub net_appreciation_depreciation: String,
    pub amortisation_asondate: String,
    pub accounted_amortisation: i64,
    pub unaccounted_amortisation: i64,
    pub accured_interest: f64,
    pub coupon_frequency: String,
    pub coupon_basis: String,
    pub category: String,
    pub sub_category: String,
    pub put_date: String,
    pub call_date: String,
    pub last_coupon: String,
    pub next_coupon: NaiveDate,
    pub issue_date: NaiveDate,
    pub last_repricing_date: String,
    pub next_repricing_date: String,
    pub place: String,
    pub country: String,
    pub slr_nonslr: String,
    pub listed: String,
    pub issuer_id: String,
    pub issuer_name: String,
    pub external_rating_agency: String,
    pub external_rating: String,
    pub market: String,
    pub asset_classification: String,
    pub guarantor: String,
    pub industry: String,
    pub sub_industry: String,
    pub npa_classification: String,
    pub deal_value_date: String,
    pub m_duration: String,
    pub treasury_gl_code: String,
}

impl InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, String> {
        let mut value_iterator = line.split('|');

        let input_account = InputAccount {
            deal_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `deal number`.".to_string());
                }
            },
            portfolio: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `portfolio`.".to_string());
                }
            },
            instrument_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `instrument id`.".to_string());
                }
            },
            instrument_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `instrument type`.".to_string());
                }
            },
            isin_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `is in code`.".to_string());
                }
            },
            security_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `security name`.".to_string());
                }
            },
            face_value: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `face value`.".to_string());
                }
            },
            book_value: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `book value`.".to_string());
                }
            },
            market_value: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `market value`.".to_string());
                }
            },
            currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `currency`.".to_string());
                }
            },
            mtm: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `face value`.".to_string());
                }
            },
            yeild: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `market value`.".to_string());
                }
            },
            maturity_date: match value_iterator.next() {
                Some(val) => {
                    let formated_date = val.replace(".", "-").replace("/", "-");
                    let mat_dt = dmy_date_parser.parse_opt(&formated_date);
                    if mat_dt.is_none() {
                        return Err("Could not parse property `maturity date`.".to_string());
                    }
                    mat_dt.unwrap()
                }
                None => {
                    return Err("Could not read property `maturity_date`.".to_string());
                }
            },
            coupon_classification_1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `coupon_classification_1`.".to_string());
                }
            },
            coupon_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `coupon rate`.".to_string());
                }
            },
            face_value_perunits: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `face_value_perunits`.".to_string());
                }
            },
            quantity: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `quantity`.".to_string());
                }
            },
            appreciation: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `appreciation`.".to_string());
                }
            },
            depreciation: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `depreciation`.".to_string());
                }
            },
            net_appreciation_depreciation: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err(
                        "Could not read property `net_appreciation_depreciation`.".to_string()
                    );
                }
            },
            amortisation_asondate: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `amortisation_asondate`.".to_string());
                }
            },
            accounted_amortisation: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `accounted_amortisation`.".to_string());
                }
            },
            unaccounted_amortisation: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `face value`.".to_string());
                }
            },
            accured_interest: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `accured_interest`.".to_string());
                }
            },
            coupon_frequency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `coupon_frequency`.".to_string());
                }
            },
            coupon_basis: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `coupon_basis`.".to_string());
                }
            },
            category: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `category`.".to_string());
                }
            },
            sub_category: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `sub_category`.".to_string());
                }
            },
            put_date: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `put_date`.".to_string());
                }
            },
            call_date: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `call_date`.".to_string());
                }
            },
            last_coupon: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `last coupon`.".to_string());
                }
            },
            next_coupon: match value_iterator.next() {
                Some(val) => {
                    let formated_date = val.replace(".", "-").replace("/", "-");
                    let next_coupon = dmy_date_parser.parse_opt(&formated_date);
                    if next_coupon.is_none() {
                        return Err("Could not parse property `next_coupon`.".to_string());
                    }
                    next_coupon.unwrap()
                }
                None => {
                    return Err("Could not read property `next_coupon`.".to_string());
                }
            },
            issue_date: match value_iterator.next() {
                Some(val) => {
                    let formated_date = val.replace(".", "-").replace("/", "-");
                    let issue_dt = dmy_date_parser.parse_opt(&formated_date);
                    if issue_dt.is_none() {
                        return Err("Could not parse property `issue_date`.".to_string());
                    }
                    issue_dt.unwrap()
                }
                None => {
                    return Err("Could not read property `issue_date`.".to_string());
                }
            },
            last_repricing_date: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `last repricing date`.".to_string());
                }
            },
            next_repricing_date: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `next repricing date`.".to_string());
                }
            },

            place: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `place`.".to_string());
                }
            },
            country: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `country`.".to_string());
                }
            },
            slr_nonslr: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `slr_nonslr`.".to_string());
                }
            },
            listed: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `listed`.".to_string());
                }
            },
            issuer_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `issuer id`.".to_string());
                }
            },
            issuer_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `issuer name`.".to_string());
                }
            },
            external_rating_agency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `external rating agency`.".to_string());
                }
            },
            external_rating: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `external rating`.".to_string());
                }
            },
            market: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `market`.".to_string());
                }
            },
            asset_classification: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `asset classification`.".to_string());
                }
            },
            guarantor: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `guarantor`.".to_string());
                }
            },
            industry: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `industry`.".to_string());
                }
            },
            sub_industry: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `sub_industry`.".to_string());
                }
            },
            npa_classification: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `npa_classification`.".to_string());
                }
            },
            deal_value_date: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `deal_value_date`.".to_string());
                }
            },
            m_duration: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `m_duration`.".to_string());
                }
            },
            treasury_gl_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `treasury gl code`.".to_string());
                }
            },
        };

        Ok(input_account)
    }
}
