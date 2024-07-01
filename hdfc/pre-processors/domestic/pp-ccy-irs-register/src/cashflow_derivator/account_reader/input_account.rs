use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub entity: String,
    pub trade_id: String,
    pub contract_id: i64,
    pub struct_id_link_id: i64,
    pub comp_typology: String,
    pub pkg_typology: String,
    pub cntrct_typology: String,
    pub desk: String,
    pub book: String,
    pub folder: String,
    pub trading_banking: String,
    pub cntr_prty_grp_cd: String,
    pub cntr_prty_chld_cd: String,
    pub cntr_prty_name: String,
    pub intrnl_extrnl: String,
    pub trade_dt: Option<NaiveDate>,
    pub st_dt: Option<NaiveDate>,
    pub ed_dt: Option<NaiveDate>,
    pub ccy_pair: String,
    pub rec_leg_ccy: String,
    pub org_notional_rec_leg: f64,
    pub org_notional_rec_leg_inr: f64,
    pub ost_notional_rec_leg: f64,
    pub ost_notional_rec_leg_inr: f64,
    pub pay_leg_ccy: String,
    pub org_notional_pay_leg: f64,
    pub org_notional_pay_leg_inr: f64,
    pub ost_notional_pay_leg: f64,
    pub ost_notional_pay_leg_inr: f64,
    pub deal_side: String,
    pub pay_leg_idx: String,
    pub pay_int_rt: f64,
    pub spread_pay_leg: f64,
    pub rec_leg_idx: String,
    pub rec_int_rt: f64,
    pub spread_rec_leg: f64,
    pub rec_side_acrl_inr: f64,
    pub rec_side_mtm_inr: f64,
    pub future_cash_proceeds_ccy: String,
    pub future_cash_proceeds_inr: f64,
    pub mrkt_val_financed: f64,
    pub net_mtm_usd: f64,
    pub net_mtm_inr: f64,
    pub pay_side_pv01_inr: f64,
    pub rec_side_pv01_inr: f64,
    pub net_pv01_inr: f64,
    pub pay_side_modified_duration: f64,
    pub receive_side_modified_duration: f64,
    pub modified_duration_deal: f64,
    pub pay_leg_exchange_rt: f64,
    pub rec_leg_exchange_rt: f64,
    pub pay_reset_dt: Option<NaiveDate>,
    pub rec_reset_dt: Option<NaiveDate>,
    pub pay_payment_dt: Option<NaiveDate>,
    pub rec_payment_dt: Option<NaiveDate>,
    pub index_rec_leg: String,
    pub index_pay_leg: String,
    pub day_count_cnvntn_rec_leg: String,
    pub day_count_cnvntn_pay_leg: String,
    pub pay_reset_freq: String,
    pub rec_reset_freq: String,
    pub pay_payment_freq: String,
    pub rec_payment_freq: String,
    pub deal_status: String,
    pub flowtype: String,
    pub flowtype1: String,
    pub flowtype2: String,
    pub flowtype3: String,
    pub flowtype4: String,
    pub flowamount: f64,
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
            ed_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `ed_dt`.");
                }
            },
            ccy_pair: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ccy_pair`.");
                }
            },
            rec_leg_ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rec_leg_ccy`.");
                }
            },
            org_notional_rec_leg: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `org_notional_rec_leg`.");
                }
            },
            org_notional_rec_leg_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `org_notional_rec_leg_inr`.");
                }
            },
            ost_notional_rec_leg: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `ost_notional_rec_leg`.");
                }
            },
            ost_notional_rec_leg_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `ost_notional_rec_leg_inr`.");
                }
            },
            pay_leg_ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pay_leg_ccy`.");
                }
            },
            org_notional_pay_leg: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `org_notional_pay_leg`.");
                }
            },
            org_notional_pay_leg_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `org_notional_pay_leg_inr`.");
                }
            },
            ost_notional_pay_leg: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `ost_notional_pay_leg`.");
                }
            },
            ost_notional_pay_leg_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `ost_notional_pay_leg_inr`.");
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
            rec_side_acrl_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `rec_side_acrl_inr`.");
                }
            },
            rec_side_mtm_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `rec_side_mtm_inr`.");
                }
            },
            future_cash_proceeds_ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `future_cash_proceeds_ccy`.");
                }
            },
            future_cash_proceeds_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `future_cash_proceeds_inr`.");
                }
            },
            mrkt_val_financed: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `mrkt_val_financed`.");
                }
            },
            net_mtm_usd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `net_mtm_usd`.");
                }
            },
            net_mtm_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `net_mtm_inr`.");
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
            receive_side_modified_duration: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `receive_side_modified_duration`.");
                }
            },
            modified_duration_deal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `modified_duration_deal`.");
                }
            },
            pay_leg_exchange_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `pay_leg_exchange_rt`.");
                }
            },
            rec_leg_exchange_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `rec_leg_exchange_rt`.");
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
            index_rec_leg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `index_rec_leg`.");
                }
            },
            index_pay_leg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `index_pay_leg`.");
                }
            },
            day_count_cnvntn_rec_leg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `day_count_cnvntn_rec_leg`.");
                }
            },
            day_count_cnvntn_pay_leg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `day_count_cnvntn_pay_leg`.");
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
