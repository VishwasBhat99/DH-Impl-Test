use rbdate::{DateParser, NaiveDate};
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub entity: String,
    pub trade_id: i64,
    pub extrnl_id: i64,
    pub struct_id: i64,
    pub struct_id_link_id: i64,
    pub comp_typology: String,
    pub cntrct_typology: String,
    pub pkq_typology: String,
    pub cntrct_usage: String,
    pub desk: String,
    pub book: String,
    pub folder: String,
    pub trading_banking: String,
    pub cntr_prty_grp_cd: String,
    pub cntr_prty_prnt_cd: String,
    pub cntr_prty_chld_cd: String,
    pub cntr_prty_name: String,
    pub intrnl_extrnl: String,
    pub trade_dt: Option<NaiveDate>,
    pub st_dt: Option<NaiveDate>,
    pub ex_dt: Option<NaiveDate>,
    pub del_dt: Option<NaiveDate>,
    pub buy_sell: String,
    pub put_call: String,
    pub call_ccy: String,
    pub call_amt: f64,
    pub put_ccy: String,
    pub put_amt: f64,
    pub notional_inr: f64,
    pub strike_rt: f64,
    pub bank_non_bank: String,
    pub ccy_pair: String,
    pub cnrt_spot: f64,
    pub prem_ccy: String,
    pub prem_amt: f64,
    pub setld_prem_amt: f64,
    pub unsetld_prem_amt: f64,
    pub prem_setld_dt: Option<NaiveDate>,
    pub future_cash_proceeds_ccy: String,
    pub future_cash_proceeds: f64,
    pub future_cash_proceeds_inr: f64,
    pub mrkt_val_finance: f64,
    pub mtm_excld_prem_inr: f64,
    pub mtm_unsetld_prem_inr: f64,
    pub cva: f64,
    pub dva: f64,
    pub bcva: f64,
    pub net_bcva_adjstd_gmtm_inr: f64,
    pub position_ccy: String,
    pub forward_delta_ccy_1_amt: f64,
    pub spot_delta_ccy_1_amt: f64,
    pub pl_ccy: String,
    pub forward_delta_ccy_2_amt: f64,
    pub spot_delta_ccy_2_amt: f64,
    pub gamma_ccy: String,
    pub gamma_amt: f64,
    pub vega_inr: f64,
    pub theta_inr: f64,
    pub rho_inr: f64,
    pub underlyng_ccy: String,
    pub phi_inr: f64,
    pub vanna_inr: f64,
    pub volga_inr: f64,
    pub modified_duration: f64,
    pub underlying_notional: f64,
    pub underlying_pp: String,
    pub original_tenor: i64,
    pub residual_tenor: i64,
    pub deal_status: String,
    pub input_id: String,
    pub m_h_fwd_rate: f64,
    pub flowtype: String,
    pub flowtype1: String,
    pub flowtype2: String,
    pub flowtype3: String,
    pub flowtype4: String,
    pub flowamount: f64,
    pub ccy: String,
    pub cf_dt: Option<NaiveDate>,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            entity: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `entity`.");
                }
            },
            trade_id: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `trade_id`.");
                }
            },
            extrnl_id: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `extrnl_id`.");
                }
            },
            struct_id: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `struct_id`.");
                }
            },
            struct_id_link_id: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `struct_id_link_id`.");
                }
            },
            comp_typology: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `comp_typology`.");
                }
            },
            cntrct_typology: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cntrct_typology`.");
                }
            },
            pkq_typology: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pkq_typology`.");
                }
            },
            cntrct_usage: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cntrct_usage`.");
                }
            },
            desk: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `desk`.");
                }
            },
            book: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `book`.");
                }
            },
            folder: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `folder`.");
                }
            },
            trading_banking: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `trading_banking`.");
                }
            },
            cntr_prty_grp_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cntr_prty_grp_cd`.");
                }
            },
            cntr_prty_prnt_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cntr_prty_prnt_cd`.");
                }
            },
            cntr_prty_chld_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cntr_prty_chld_cd`.");
                }
            },
            cntr_prty_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cntr_prty_name`.");
                }
            },
            intrnl_extrnl: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `intrnl_extrnl`.");
                }
            },
            trade_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `trade_dt`.");
                }
            },
            st_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `st_dt`.");
                }
            },
            ex_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `ex_dt`.");
                }
            },
            del_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `del_dt`.");
                }
            },
            buy_sell: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `buy_sell`.");
                }
            },
            put_call: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `put_call`.");
                }
            },
            call_ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `call_ccy`.");
                }
            },
            call_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `call_amt`.");
                }
            },
            put_ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `put_ccy`.");
                }
            },
            put_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `put_amt`.");
                }
            },
            notional_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `notional_inr`.");
                }
            },
            strike_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `strike_rt`.");
                }
            },
            bank_non_bank: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bank_non_bank`.");
                }
            },
            ccy_pair: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ccy_pair`.");
                }
            },
            cnrt_spot: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `cnrt_spot`.");
                }
            },
            prem_ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `prem_ccy`.");
                }
            },
            prem_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prem_amt`.");
                }
            },
            setld_prem_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `setld_prem_amt`.");
                }
            },
            unsetld_prem_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `unsetld_prem_amt`.");
                }
            },
            prem_setld_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `prem_setld_dt`.");
                }
            },
            future_cash_proceeds_ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `future_cash_proceeds_ccy`.");
                }
            },
            future_cash_proceeds: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `future_cash_proceeds`.");
                }
            },
            future_cash_proceeds_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `future_cash_proceeds_inr`.");
                }
            },
            mrkt_val_finance: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `mrkt_val_finance`.");
                }
            },
            mtm_excld_prem_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `mtm_excld_prem_inr`.");
                }
            },
            mtm_unsetld_prem_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `mtm_unsetld_prem_inr`.");
                }
            },
            cva: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `cva`.");
                }
            },
            dva: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `dva`.");
                }
            },
            bcva: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `bcva`.");
                }
            },
            net_bcva_adjstd_gmtm_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `net_bcva_adjstd_gmtm_inr`.");
                }
            },
            position_ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `position_ccy`.");
                }
            },
            forward_delta_ccy_1_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `forward_delta_ccy_1_amt`.");
                }
            },
            spot_delta_ccy_1_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `spot_delta_ccy_1_amt`.");
                }
            },
            pl_ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pl_ccy`.");
                }
            },
            forward_delta_ccy_2_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `forward_delta_ccy_2_amt`.");
                }
            },
            spot_delta_ccy_2_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `spot_delta_ccy_2_amt`.");
                }
            },
            gamma_ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `gamma_ccy`.");
                }
            },
            gamma_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `gamma_amt`.");
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
            underlyng_ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `underlyng_ccy`.");
                }
            },
            phi_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `phi_inr`.");
                }
            },
            vanna_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `vanna_inr`.");
                }
            },
            volga_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `volga_inr`.");
                }
            },
            modified_duration: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `modified_duration`.");
                }
            },
            underlying_notional: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `underlying_notional`.");
                }
            },
            underlying_pp: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `underlying_pp`.");
                }
            },
            original_tenor: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `original_tenor`.");
                }
            },
            residual_tenor: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `residual_tenor`.");
                }
            },
            deal_status: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `deal_status`.");
                }
            },
            input_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `input_id`.");
                }
            },
            m_h_fwd_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `m_h_fwd_rate`.");
                }
            },
            flowtype: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `flowtype`.");
                }
            },
            flowtype1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `flowtype1`.");
                }
            },
            flowtype2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `flowtype2`.");
                }
            },
            flowtype3: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `flowtype3`.");
                }
            },
            flowtype4: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `flowtype4`.");
                }
            },
            flowamount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `flowamount`.");
                }
            },
            ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ccy`.");
                }
            },
            cf_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `cf_dt`.");
                }
            },
        };
        Ok(input_account)
    }
}
