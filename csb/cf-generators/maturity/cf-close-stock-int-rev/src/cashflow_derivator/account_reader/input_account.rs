use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub portfolio_num: String,
    pub portfolio: String,
    pub security_name: String,
    pub maturity_dt: Option<NaiveDate>,
    pub coupon: f64,
    pub face_val_per_units: f64,
    pub quantity: f64,
    pub face_val: f64,
    pub wap: f64,
    pub book_val: f64,
    pub market_val: f64,
    pub mtm: f64,
    pub yeild: f64,
    pub appreciation: f64,
    pub depreciation: f64,
    pub net_appreciation_depreciation: f64,
    pub amort_as_on_dt: f64,
    pub accounted_amort: f64,
    pub un_accounted_amort: f64,
    pub accrued_int: f64,
    pub no_ca_skipped: f64,
    pub ca_int_not_receieved: f64,
    pub total_int: f64,
    pub inst_id: String,
    pub inst_typ: String,
    pub isin_code: String,
    pub int_freq: String,
    pub int_practice: String,
    pub category: String,
    pub sub_category: String,
    pub put_dt: Option<NaiveDate>,
    pub call_dt: Option<NaiveDate>,
    pub lst_coupon: Option<NaiveDate>,
    pub nxt_coupon: Option<NaiveDate>,
    pub issue_dt: Option<NaiveDate>,
    pub place: String,
    pub country: String,
    pub booking_basis: String,
    pub residual_maturity: String,
    pub slr_non_slr: String,
    pub listed: String,
    pub issuer_name: String,
    pub rating_agency: String,
    pub rating: String,
    pub market: String,
    pub asset_classification: String,
    pub gurantor: String,
    pub industry: String,
    pub sub_industry: String,
    pub borrower_category: String,
    pub asset_typ: String,
    pub asset_category: String,
    pub curr: String,
    pub coupon_classification_1: String,
    pub lst_rep_dt: Option<NaiveDate>,
    pub nxt_rep_dt: Option<NaiveDate>,
    pub m_duration: f64,
    pub trsy_gl_cd: String,
    pub cf_type: String,
    pub cf_amt: String,
    pub cf_ccy: String,
    pub cf_dt: Option<NaiveDate>,
    pub prin_amt: f64,
    pub int_amt: f64,
    pub cbs_gl_cd: String,
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
            portfolio_num: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	portfolio_num	.");
                }
            },
            portfolio: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	portfolio	.");
                }
            },
            security_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	security_name	.");
                }
            },
            maturity_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property 	maturity_dt	.");
                }
            },
            coupon: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 	coupon	.");
                }
            },
            face_val_per_units: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 	face_val_per_units	.");
                }
            },
            quantity: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 	quantity	.");
                }
            },
            face_val: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 	face_val	.");
                }
            },
            wap: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 	wap	.");
                }
            },
            book_val: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 	book_val	.");
                }
            },
            market_val: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 	market_val	.");
                }
            },
            mtm: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 	mtm	.");
                }
            },
            yeild: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 	yeild	.");
                }
            },
            appreciation: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 	appreciation	.");
                }
            },
            depreciation: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 	depreciation	.");
                }
            },
            net_appreciation_depreciation: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 	net_appreciation_depreciation	.");
                }
            },
            amort_as_on_dt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 	amort_as_on_dt	.");
                }
            },
            accounted_amort: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 	accounted_amort	.");
                }
            },
            un_accounted_amort: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 	un_accounted_amort	.");
                }
            },
            accrued_int: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 	accrued_int	.");
                }
            },
            no_ca_skipped: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 	no_ca_skipped	.");
                }
            },
            ca_int_not_receieved: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 	ca_int_not_receieved	.");
                }
            },
            total_int: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 	total_int	.");
                }
            },
            inst_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	inst_id	.");
                }
            },
            inst_typ: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	inst_typ	.");
                }
            },
            isin_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	isin_code	.");
                }
            },
            int_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	int_freq	.");
                }
            },
            int_practice: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	int_practice	.");
                }
            },
            category: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	category	.");
                }
            },
            sub_category: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	sub_category	.");
                }
            },
            put_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property 	put_dt	.");
                }
            },
            call_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property 	call_dt	.");
                }
            },
            lst_coupon: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property 	lst_coupon	.");
                }
            },
            nxt_coupon: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property 	nxt_coupon	.");
                }
            },
            issue_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property 	issue_dt	.");
                }
            },
            place: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	place	.");
                }
            },
            country: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	country	.");
                }
            },
            booking_basis: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	booking_basis	.");
                }
            },
            residual_maturity: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	residual_maturity	.");
                }
            },
            slr_non_slr: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	slr_non_slr	.");
                }
            },
            listed: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	listed	.");
                }
            },
            issuer_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	issuer_name	.");
                }
            },
            rating_agency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	rating_agency	.");
                }
            },
            rating: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	rating	.");
                }
            },
            market: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	market	.");
                }
            },
            asset_classification: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	asset_classification	.");
                }
            },
            gurantor: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	gurantor	.");
                }
            },
            industry: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	industry	.");
                }
            },
            sub_industry: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	sub_industry	.");
                }
            },
            borrower_category: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	borrower_category	.");
                }
            },
            asset_typ: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	asset_typ	.");
                }
            },
            asset_category: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	asset_category	.");
                }
            },
            curr: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	curr	.");
                }
            },
            coupon_classification_1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	coupon_classification_1	.");
                }
            },
            lst_rep_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property 	lst_rep_dt	.");
                }
            },
            nxt_rep_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property 	nxt_rep_dt	.");
                }
            },
            m_duration: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 	m_duration	.");
                }
            },
            trsy_gl_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	trsy_gl_cd	.");
                }
            },
            cf_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	cf_type	.");
                }
            },
            cf_amt: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	cf_amt	.");
                }
            },
            cf_ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	cf_ccy	.");
                }
            },
            cf_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property 	cf_dt	.");
                }
            },
            prin_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 	prin_amt	.");
                }
            },
            int_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 	int_amt	.");
                }
            },
            cbs_gl_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	cbs_gl_cd	.");
                }
            },
            w4b_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	w4b_cd	.");
                }
            },
            balm_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	balm_llg	.");
                }
            },
            care_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	care_llg	.");
                }
            },
            ba_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 	ba_llg	.");
                }
            },
        };
        Ok(input_account)
    }
}
