use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub deal_id: String,
    pub instrument_type: String,
    pub trading_banking: String,
    pub counter_party_id: String,
    pub counterparty_name: String,
    pub internal_external: String,
    pub trade_date: Option<NaiveDate>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub currency: String,
    pub original_notional_rec_leg: String,
    pub original_notional_rec_leg_lcy: String,
    pub outstanding_notional_rec_leg: String,
    pub outstanding_notional_rec_leg_lcy: Option<f64>,
    pub app1: String,
    pub original_notional_pay_leg: String,
    pub original_notional_pay_leg_lcy: String,
    pub outstanding_notional_pay_leg: String,
    pub outstanding_notional_pay_leg_lcy: Option<f64>,
    pub contingent_notional: String,
    pub pay_leg_index: String,
    pub pay_int_rate: String,
    pub spread_pay_leg: String,
    pub rec_leg_index: String,
    pub rec_int_rate: String,
    pub spread_rec_leg: String,
    pub modified_duration_deal: String,
    pub exchange_rate: String,
    pub app5: String,
    pub pay_reset_date: Option<NaiveDate>,
    pub rec_reset_date: Option<NaiveDate>,
    pub pay_payment_date: Option<NaiveDate>,
    pub rec_payment_date: Option<NaiveDate>,
    pub day_count_convention_rec: String,
    pub day_count_convention_pay: String,
    pub pay_reset_frequency: String,
    pub rec_reset_frequency: String,
    pub pay_payment_frequency: String,
    pub rec_payment_frequency: String,
    pub leg_type: String,
    pub underlying_pp: String,
    pub net_pl_amount: String,
    pub counterpartycategory1: String,
    pub counterpartycategory2: String,
    pub counterpartycategory3: String,
    pub cashflow_type: String,
    pub treasury_gL_code: String,
    pub app2: String,
    pub app3: String,
    pub app4: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            deal_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `deal_id`.");
                }
            },
            instrument_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `instrument_type`.");
                }
            },
            trading_banking: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `trading_banking`.");
                }
            },
            counter_party_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `counter_party_id`.");
                }
            },
            counterparty_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `counterparty_name`.");
                }
            },
            internal_external: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `internal_external`.");
                }
            },
            trade_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `trade_date`.");
                }
            },
            start_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `start_date`.");
                }
            },
            end_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `end_date`.");
                }
            },
            currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `currency`.");
                }
            },
            original_notional_rec_leg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `original_notional_rec_leg`.");
                }
            },
            original_notional_rec_leg_lcy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `original_notional_rec_leg_lcy`.");
                }
            },
            outstanding_notional_rec_leg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `outstanding_notional_rec_leg`.");
                }
            },
            outstanding_notional_rec_leg_lcy: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `outstanding_notional_rec_leg_lcy`.");
                }
            },
            app1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `app1`.");
                }
            },
            original_notional_pay_leg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `original_notional_pay_leg`.");
                }
            },
            original_notional_pay_leg_lcy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `original_notional_pay_leg_lcy`.");
                }
            },
            outstanding_notional_pay_leg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `outstanding_notional_pay_leg`.");
                }
            },
            outstanding_notional_pay_leg_lcy: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `outstanding_notional_pay_leg_lcy`.");
                }
            },
            contingent_notional: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `contingent_notional`.");
                }
            },
            pay_leg_index: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pay_leg_index`.");
                }
            },
            pay_int_rate: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pay_int_rate`.");
                }
            },
            spread_pay_leg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `spread_pay_leg`.");
                }
            },
            rec_leg_index: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rec_leg_index`.");
                }
            },
            rec_int_rate: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rec_int_rate`.");
                }
            },
            spread_rec_leg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `spread_rec_leg`.");
                }
            },
            modified_duration_deal: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `modified_duration_deal`.");
                }
            },
            exchange_rate: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `exchange_rate`.");
                }
            },
            app5: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `app5`.");
                }
            },
            pay_reset_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `pay_reset_date`.");
                }
            },
            rec_reset_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `rec_reset_date`.");
                }
            },
            pay_payment_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `pay_payment_date`.");
                }
            },
            rec_payment_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `rec_payment_date`.");
                }
            },
            day_count_convention_rec: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `day_count_convention_rec`.");
                }
            },
            day_count_convention_pay: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `day_count_convention_pay`.");
                }
            },
            pay_reset_frequency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pay_reset_frequency`.");
                }
            },
            rec_reset_frequency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rec_reset_frequency`.");
                }
            },
            pay_payment_frequency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pay_payment_frequency`.");
                }
            },
            rec_payment_frequency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rec_payment_frequency`.");
                }
            },
            leg_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `leg_type`.");
                }
            },
            underlying_pp: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `underlying_pp`.");
                }
            },
            net_pl_amount: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `net_pl_amount`.");
                }
            },
            counterpartycategory1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `counterpartycategory1`.");
                }
            },
            counterpartycategory2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `counterpartycategory2`.");
                }
            },
            counterpartycategory3: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `counterpartycategory3`.");
                }
            },
            cashflow_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cashflow_type`.");
                }
            },
            treasury_gL_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `treasury_gL_code`.");
                }
            },
            app2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `app2`.");
                }
            },
            app3: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `app3`.");
                }
            },
            app4: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `app4`.");
                }
            },
        };
        Ok(input_account)
    }
}
