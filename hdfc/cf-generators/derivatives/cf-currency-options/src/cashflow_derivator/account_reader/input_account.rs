use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub generated_pk: i64,
    pub trade_id: String,
    pub book: String,
    pub trade_dt: Option<NaiveDate>,
    pub st_dt: Option<NaiveDate>,
    pub opt_sell_buy: String,
    pub put_call: String,
    pub call_ccy: String,
    pub call_amt: f64,
    pub put_ccy: String,
    pub put_amt: f64,
    pub ex_rt: f64,
    pub notional_inr: String,
    pub strike_rt: f64,
    pub parent_id: String,
    pub cust_id: String,
    pub cust_name: String,
    pub expiry_dt: Option<NaiveDate>,
    pub delivery_dt: Option<NaiveDate>,
    pub bank_vs_non_bank: String,
    pub premium_ccy: f64,
    pub premium_amt: f64,
    pub premium_amt_inr: f64,
    pub extrnl_id: i64,
    pub ccy_pair: String,
    pub org_spot: f64,
    pub crnt_spot: f64,
    pub crnt_price_mtm_inr: f64,
    pub tag_premium: String,
    pub mtm_gain_loss_inr: f64,
    pub cnvrsn_rt: f64,
    pub spot_delta_ccy: String,
    pub spot_delta: f64,
    pub frwrd_delta_ccy: String,
    pub frwrd_delta: Option<f64>,
    pub gamma_ccy: String,
    pub gamma: f64,
    pub vega_inr: f64,
    pub theta_inr: f64,
    pub rho_inr: f64,
    pub volatility: f64,
    pub cntry_name: String,
    pub deal_stats: String,
    pub knock_in_rt: f64,
    pub knock_in_dt: Option<NaiveDate>,
    pub knock_out_rt: f64,
    pub struct_id: i64,
    pub inp_usr: String,
    pub authorised_usr: String,
    pub knock_out_dt: String,
    pub desk: String,
    pub folder: String,
    pub underlying: String,
    pub prod_grp: String,
    pub cust_typ: String,
    pub trader_id: String,
    pub mrktr_id: String,
    pub inr_call_notational: f64,
    pub inr_put_notional: f64,
    pub usd_call_notional: f64,
    pub usd_put_notional: f64,
    pub job_strm_id: i64,
    pub job_run_id: i64,
    pub business_dt: Option<NaiveDate>,
    pub load_ts: Option<NaiveDate>,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        ymd_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split(',');
        let input_account = InputAccount {
            generated_pk: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `account_id`.");
                }
            },
            trade_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `accural_basis`.");
                }
            },
            book: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `portfolio`.");
                }
            },
            trade_dt: match value_iterator.next() {
                Some(val) => ymd_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `gl`.");
                }
            },
            st_dt: match value_iterator.next() {
                Some(val) => ymd_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `couprt`.");
                }
            },
            opt_sell_buy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `dealdt`.");
                }
            },
            put_call: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `valudt`.");
                }
            },
            call_ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `matudt`.");
                }
            },
            call_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `currcd`.");
                }
            },
            put_ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `orgballcy`.");
                }
            },
            put_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `orgbaltcy`.");
                }
            },
            ex_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `avgbaldlcy`.");
                }
            },
            notional_inr: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `avgbalvdtcy`.");
                }
            },
            strike_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `oscostlcy`.");
                }
            },
            parent_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `oscosttcy`.");
                }
            },
            cust_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `finallcy`.");
                }
            },
            cust_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `finaltcy`.");
                }
            },
            expiry_dt: match value_iterator.next() {
                Some(val) => ymd_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `int_amt`.");
                }
            },
            delivery_dt: match value_iterator.next() {
                Some(val) => ymd_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `lcyinterestamount`.");
                }
            },
            bank_vs_non_bank: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bank_vs_non_bank`.");
                }
            },
            premium_ccy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `premium_ccy`.");
                }
            },
            premium_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `premium_amt`.");
                }
            },
            premium_amt_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `premium_amt_inr`.");
                }
            },
            extrnl_id: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `extrnl_id`.");
                }
            },
            ccy_pair: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ccy_pair`.");
                }
            },
            org_spot: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `org_spot`.");
                }
            },
            crnt_spot: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `crnt_spot`.");
                }
            },
            crnt_price_mtm_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `crnt_price_mtm_inr`.");
                }
            },
            tag_premium: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `tag_premium`.");
                }
            },
            mtm_gain_loss_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `mtm_gain_loss_inr`.");
                }
            },
            cnvrsn_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `cnvrsn_rt`.");
                }
            },
            spot_delta_ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `spot_delta_ccy`.");
                }
            },
            spot_delta: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `spot_delta`.");
                }
            },
            frwrd_delta_ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `frwrd_delta_ccy`.");
                }
            },
            frwrd_delta: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `frwrd_delta`.");
                }
            },
            gamma_ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `gamma_ccy`.");
                }
            },
            gamma: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `gamma`.");
                }
            },
            vega_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `vega_inr`.");
                }
            },
            theta_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `theta_inr`.");
                }
            },
            rho_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `rho_inr`.");
                }
            },
            volatility: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `volatility`.");
                }
            },
            cntry_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cntry_name`.");
                }
            },
            deal_stats: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `deal_stats`.");
                }
            },
            knock_in_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `knock_in_rt`.");
                }
            },
            knock_in_dt: match value_iterator.next() {
                Some(val) => ymd_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `knock_in_dt`.");
                }
            },
            knock_out_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `knock_out_rt`.");
                }
            },
            struct_id: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `struct_id`.");
                }
            },
            inp_usr: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `inp_usr`.");
                }
            },
            authorised_usr: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `authorised_usr`.");
                }
            },
            knock_out_dt: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `knock_out_dt`.");
                }
            },
            desk: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `desk`.");
                }
            },
            folder: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `folder`.");
                }
            },
            underlying: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `underlying`.");
                }
            },
            prod_grp: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `prod_grp`.");
                }
            },
            cust_typ: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_typ`.");
                }
            },
            trader_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `trader_id`.");
                }
            },
            mrktr_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `mrktr_id`.");
                }
            },
            inr_call_notational: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `inr_call_notational`.");
                }
            },
            inr_put_notional: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `inr_put_notional`.");
                }
            },
            usd_call_notional: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `usd_call_notional`.");
                }
            },
            usd_put_notional: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `usd_put_notional`.");
                }
            },
            job_strm_id: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `job_strm_id`.");
                }
            },
            job_run_id: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `job_run_id`.");
                }
            },
            business_dt: match value_iterator.next() {
                Some(val) => ymd_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `business_dt`.");
                }
            },
            load_ts: match value_iterator.next() {
                Some(val) => ymd_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `load_ts`.");
                }
            },
        };
        Ok(input_account)
    }
}
