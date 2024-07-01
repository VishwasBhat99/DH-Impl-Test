use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub entity: String,
    pub trade_id: String,
    pub contract_id: i64,
    pub folder: String,
    pub trading_banking: String,
    pub intrnl_extrnl: String,
    pub cntr_prty_name: String,
    pub trade_dt: Option<NaiveDate>,
    pub st_dt: Option<NaiveDate>,
    pub ed_dt: Option<NaiveDate>,
    pub deal_ccy: String,
    pub org_notional: f64,
    pub org_notional_inr: f64,
    pub rec_crnt_notional: f64,
    pub rec_crnt_notional_inr: f64,
    pub pay_crnt_notional: f64,
    pub pay_crnt_notional_inr: f64,
    pub contignent_notional: f64,
    pub deal_side: String,
    pub pay_leg_idx: String,
    pub pay_int_rt: f64,
    pub spread_pay_leg: f64,
    pub rec_leg_idx: String,
    pub rec_int_rt: f64,
    pub spread_rec_leg: f64,
    pub pay_side_acrl: f64,
    pub pay_side_mtm: f64,
    pub pay_side_gmtm: f64,
    pub rec_side_acrl: f64,
    pub net_acrl_inr: f64,
    pub net_acrl_usd: f64,
    pub future_cash_proceeds_ccy: String,
    pub future_cash_proceeds: f64,
    pub future_cash_proceeds_inr: f64,
    pub net_mtm: f64,
    pub net_mtm_inr: f64,
    pub net_mtm_usd: f64,
    pub net_gmtm_inr: f64,
    pub net_gmtm_usd: f64,
    pub net_bcva_adjstd_gmtm_inr: f64,
    pub pay_side_pv01_inr: f64,
    pub rec_side_pv01_inr: f64,
    pub net_pv01_inr: f64,
    pub pay_side_modified_duration: f64,
    pub exchange_rt: f64,
    pub pay_reset_dt: Option<NaiveDate>,
    pub rec_reset_dt: Option<NaiveDate>,
    pub pay_payment_dt: Option<NaiveDate>,
    pub rec_payment_dt: Option<NaiveDate>,
    pub org_tenor: String,
    pub residual_tenor: String,
    pub pay_reset_freq: String,
    pub rec_reset_freq: String,
    pub pay_payment_freq: String,
    pub rec_payment_freq: String,
    pub deal_status: String,
    pub inp_id: String,
    pub auth_id: String,
    pub trad_bank: String,
    pub m_bank_b: String,
    pub flowtype: String,
    pub flowtype1: String,
    pub flowtype2: String,
    pub flowtype3: String,
    pub flowtype4: String,
    pub flow_amt: f64,
    pub cf_dt: Option<NaiveDate>,
    pub flow_ccy: String,
    pub hkd_rt: f64,
    pub hkd_amt: f64,
    pub m_h_rep_dt2: Option<NaiveDate>,
    pub inr_amt: f64,
    pub inr_rt: f64,
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
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `trade_id`.");
                }
            },
            contract_id: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `contract_id`.");
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
            intrnl_extrnl: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `intrnl_extrnl`.");
                }
            },
            cntr_prty_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cntr_prty_name`.");
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
            ed_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `ed_dt`.");
                }
            },
            deal_ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `deal_ccy`.");
                }
            },
            org_notional: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `org_notional`.");
                }
            },
            org_notional_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `org_notional_leg`.");
                }
            },
            rec_crnt_notional: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `rec_crnt_notional`.");
                }
            },
            rec_crnt_notional_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `rec_crnt_notional_inr`.");
                }
            },
            pay_crnt_notional: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `pay_crnt_notional`.");
                }
            },
            pay_crnt_notional_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `pay_crnt_notional_inr`.");
                }
            },
            contignent_notional: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `contignent_notional`.");
                }
            },
            deal_side: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `deal_side`.");
                }
            },
            pay_leg_idx: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pay_leg_idx`.");
                }
            },
            pay_int_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `pay_int_rt`.");
                }
            },
            spread_pay_leg: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `spread_pay_leg`.");
                }
            },
            rec_leg_idx: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rec_leg_idx`.");
                }
            },
            rec_int_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `rec_int_rt`.");
                }
            },
            spread_rec_leg: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `spread_rec_leg`.");
                }
            },
            pay_side_acrl: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `pay_side_acrl`.");
                }
            },
            pay_side_mtm: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `pay_side_mtm`.");
                }
            },
            pay_side_gmtm: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `pay_side_gmtm`.");
                }
            },
            rec_side_acrl: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `rec_side_acrl`.");
                }
            },
            net_acrl_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `net_acrl_inr`.");
                }
            },
            net_acrl_usd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `net_acrl_usd`.");
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
            net_mtm: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `net_mtm`.");
                }
            },
            net_mtm_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `net_mtm_inr`.");
                }
            },
            net_mtm_usd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `net_mtm_usd`.");
                }
            },
            net_gmtm_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `net_gmtm_inr`.");
                }
            },
            net_gmtm_usd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `net_gmtm_usd`.");
                }
            },
            net_bcva_adjstd_gmtm_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `net_bcva_adjstd_gmtm_inr`.");
                }
            },
            pay_side_pv01_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `pay_side_pv01_inr`.");
                }
            },
            rec_side_pv01_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `rec_side_pv01_inr`.");
                }
            },
            net_pv01_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `net_pv01_inr`.");
                }
            },
            pay_side_modified_duration: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `pay_side_modified_duration`.");
                }
            },
            exchange_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `exchange_rt`.");
                }
            },
            pay_reset_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `pay_reset_dt`.");
                }
            },
            rec_reset_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `rec_reset_dt`.");
                }
            },
            pay_payment_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `pay_payment_dt`.");
                }
            },
            rec_payment_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `rec_payment_dt`.");
                }
            },
            org_tenor: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `org_tenor`.");
                }
            },
            residual_tenor: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `residual_tenor`.");
                }
            },
            pay_reset_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pay_reset_freq`.");
                }
            },
            rec_reset_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rec_reset_freq`.");
                }
            },
            pay_payment_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pay_payment_freq`.");
                }
            },
            rec_payment_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rec_payment_freq`.");
                }
            },
            deal_status: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `deal_status`.");
                }
            },
            inp_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `inp_id`.");
                }
            },
            auth_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `auth_id`.");
                }
            },
            trad_bank: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `trad_bank`.");
                }
            },
            m_bank_b: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `m_bank_b`.");
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
            flow_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `flow_amt`.");
                }
            },
            cf_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `cf_dt`.");
                }
            },
            flow_ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `flow_ccy`.");
                }
            },
            hkd_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `hkd_rt`.");
                }
            },
            hkd_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `hkd_amt`.");
                }
            },
            m_h_rep_dt2: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `m_h_rep_dt2`.");
                }
            },
            inr_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `inr_amt`.");
                }
            },
            inr_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `inr_rt`.");
                }
            },
        };
        Ok(input_account)
    }
}
