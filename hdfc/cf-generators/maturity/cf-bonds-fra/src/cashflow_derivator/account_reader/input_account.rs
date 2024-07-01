use rbdate::{DateParser, NaiveDate};
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub entity: String,
    pub trade_id: String,
    pub cntrct_id: i64,
    pub struct_id: i64,
    pub comp_typology: String,
    pub pkg_typology: String,
    pub cntrct_typology: String,
    pub cntrct_usage: String,
    pub desk: String,
    pub trading_banking: String,
    pub cntr_prty_grp_cd: String,
    pub cntr_prty_prnt_cd: String,
    pub cntr_prty_child_cd: String,
    pub cntr_prty_name: String,
    pub intrnl_extrnl: String,
    pub trade_dt: Option<NaiveDate>,
    pub fixing_dt: Option<NaiveDate>,
    pub settlement_dt: Option<NaiveDate>,
    pub maturity_dt: Option<NaiveDate>,
    pub deal_ccy: String,
    pub sec_cd: String,
    pub undrlying_sec: String,
    pub undrlying_sec_maturity: Option<NaiveDate>,
    pub notional_amt: i64,
    pub org_notional_in_inr: i64,
    pub ost_notional_in_inr: f64,
    pub cont_notional: i64,
    pub buy_sell: String,
    pub fut_cash_proceeds_ccy: String,
    pub fut_cash_proceeds: i64,
    pub fut_cash_proceeds_in_inr: f64,
    pub mtm: f64,
    pub mtm_in_inr: f64,
    pub fwdmtm_in_inr: f64,
    pub net_bcva_adj_gmtm_in_inr: f64,
    pub cva: f64,
    pub dva: f64,
    pub bcva: f64,
    pub netpv01: f64,
    pub bank_or_nonbank: String,
    pub org_tenor: i64,
    pub res_tenor: i64,
    pub udrlying: String,
    pub deal_status: String,
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
            cntrct_id: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `cntrct_id`.");
                }
            },
            struct_id: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `struct_id`.");
                }
            },
            comp_typology: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `comp_typology`.");
                }
            },
            pkg_typology: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pkg_typology`.");
                }
            },
            cntrct_typology: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cntrct_typology`.");
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
            cntr_prty_child_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cntr_prty_child_cd`.");
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
            fixing_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `fixing_dt`.");
                }
            },
            settlement_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `settlement_dt`.");
                }
            },
            maturity_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `maturity_dt`.");
                }
            },
            deal_ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `deal_ccy`.");
                }
            },
            sec_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sec_cd`.");
                }
            },
            undrlying_sec: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `undrlying_sec`.");
                }
            },
            undrlying_sec_maturity: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `undrlying_sec_maturity`.");
                }
            },
            notional_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `notional_amt`.");
                }
            },
            org_notional_in_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `org_notional_in_inr`.");
                }
            },
            ost_notional_in_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `ost_notional_in_inr`.");
                }
            },
            cont_notional: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `cont_notional`.");
                }
            },
            buy_sell: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `buy_sell`.");
                }
            },
            fut_cash_proceeds_ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `fut_cash_proceeds_ccy`.");
                }
            },
            fut_cash_proceeds: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `fut_cash_proceeds`.");
                }
            },
            fut_cash_proceeds_in_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `fut_cash_proceeds_in_inr`.");
                }
            },
            mtm: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `mtm`.");
                }
            },
            mtm_in_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `mtm_in_inr`.");
                }
            },
            fwdmtm_in_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `fwdmtm_in_inr`.");
                }
            },
            net_bcva_adj_gmtm_in_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `net_bcva_adj_gmtm_in_inr`.");
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
            netpv01: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `netpv01`.");
                }
            },
            bank_or_nonbank: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bank_or_nonbank`.");
                }
            },
            org_tenor: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `org_tenor`.");
                }
            },
            res_tenor: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `res_tenor`.");
                }
            },
            udrlying: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `udrlying`.");
                }
            },
            deal_status: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `deal_status`.");
                }
            },
        };
        Ok(input_account)
    }
}
