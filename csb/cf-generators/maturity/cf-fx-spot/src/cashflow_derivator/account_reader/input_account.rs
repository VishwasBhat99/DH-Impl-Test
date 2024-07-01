use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub reval_dt: Option<NaiveDate>,
    pub deal_type: String,
    pub deal_ref: String,
    pub leg_id: String,
    pub portfolio: String,
    pub product: String,
    pub counter_party: String,
    pub buy_sell: String,
    pub deal_dt: Option<NaiveDate>,
    pub val_dt: Option<NaiveDate>,
    pub ccy_1: String,
    pub ccy_2: String,
    pub deal_rt: f64,
    pub ccy1_amt: f64,
    pub crnct2_amt: f64,
    pub reval_rt: f64,
    pub reval_eqv: f64,
    pub actual_pl: f64,
    pub pnl_conversion_rt: f64,
    pub int_rt: f64,
    pub discounted_factor: f64,
    pub present_val_pl: f64,
    pub ccy1_spot_rt: f64,
    pub ccy2_spot_rt: f64,
    pub inr_eq_ccy1_spot_rt: f64,
    pub inr_eq_ccy2_spot_rt: f64,
    pub branch_code: String,
    pub trsy_gl: String,
    pub cf_dt: Option<NaiveDate>,
    pub cf_ccy: String,
    pub cf_amt: f64,
    pub cf_typ: String,
    pub prin_amt: f64,
    pub int_amt: f64,
    pub flow_typ: String,
    pub abs_cf_amt: f64,
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
            reval_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property 'reval_dt' .");
                }
            },
            deal_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 'deal_type' .");
                }
            },
            deal_ref: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 'deal_ref' .");
                }
            },
            leg_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 'leg_id' .");
                }
            },
            portfolio: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 'portfolio' .");
                }
            },
            product: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 'product' .");
                }
            },
            counter_party: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 'counter_party' .");
                }
            },
            buy_sell: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 'buy_sell' .");
                }
            },
            deal_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property 'deal_dt' .");
                }
            },
            val_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property 'val_dt' .");
                }
            },
            ccy_1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 'ccy_1' .");
                }
            },
            ccy_2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 'ccy_2' .");
                }
            },
            deal_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 'deal_rt' .");
                }
            },
            ccy1_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 'ccy1_amt' .");
                }
            },
            crnct2_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 'crnct2_amt' .");
                }
            },
            reval_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 'reval_rt' .");
                }
            },
            reval_eqv: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 'reval_eqv' .");
                }
            },
            actual_pl: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 'actual_pl' .");
                }
            },
            pnl_conversion_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 'pnl_conversion_rt' .");
                }
            },
            int_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 'int_rt' .");
                }
            },
            discounted_factor: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 'discounted_factor' .");
                }
            },
            present_val_pl: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 'present_val_pl' .");
                }
            },
            ccy1_spot_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 'ccy1_spot_rt' .");
                }
            },
            ccy2_spot_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 'ccy2_spot_rt' .");
                }
            },
            inr_eq_ccy1_spot_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 'inr_eq_ccy1_spot_rt' .");
                }
            },
            inr_eq_ccy2_spot_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 'inr_eq_ccy2_spot_rt' .");
                }
            },
            branch_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 'branch_code' .");
                }
            },
            trsy_gl: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 'trsy_gl' .");
                }
            },
            cf_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property 'cf_dt' .");
                }
            },
            cf_ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 'cf_ccy' .");
                }
            },
            cf_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 'cf_amt' .");
                }
            },
            cf_typ: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 'cf_typ' .");
                }
            },
            prin_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 'prin_amt' .");
                }
            },
            int_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 'int_amt' .");
                }
            },
            flow_typ: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 'flow_typ' .");
                }
            },
            abs_cf_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 'abs_cf_amt' .");
                }
            },
            cbs_gl_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 'cbs_gl_cd' .");
                }
            },
            w4b_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 'w4b_cd' .");
                }
            },
            balm_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 'balm_llg' .");
                }
            },
            care_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 'care_llg' .");
                }
            },
            ba_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 'ba_llg' .");
                }
            },
        };
        Ok(input_account)
    }
}
