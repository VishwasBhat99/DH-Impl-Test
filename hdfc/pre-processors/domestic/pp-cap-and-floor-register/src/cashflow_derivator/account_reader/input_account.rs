use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub entity: String,
    pub trade_id: String,
    pub structureid_link: f64,
    pub component_typo: String,
    pub contract_type: String,
    pub package_typo: String,
    pub desk: String,
    pub book: String,
    pub folder: String,
    pub trading_banking: String,
    pub internal_external: String,
    pub counterparty_group_code: String,
    pub counterparty_parent_code: String,
    pub counterparty_child_code: String,
    pub bank_non_bank: String,
    pub trade_date: Option<NaiveDate>,
    pub maturity_date: Option<NaiveDate>,
    pub buy_sale: String,
    pub underlying_index: String,
    pub notional_currency: String,
    pub original_notional_amount: f64,
    pub mtm_in_inr: f64,
    pub net_pv01_in_inr: f64,
    pub modified_duration_of_the_deal: f64,
    pub reset_frequency: String,
    pub next_reset_date: Option<NaiveDate>,
    pub underlying_pp: String,
    pub deal_status: String,
    pub counterparty_category1: String,
    pub counterparty_category2: String,
    pub counterparty_category3: String,
    pub accounting_section: String,
    pub flowtype4: String,
    pub flow_amount: f64,
    pub cashflow_date: Option<NaiveDate>,
    pub flow_currency: String,
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
            structureid_link: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `structureid_link`.");
                }
            },
            component_typo: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `component_typo`.");
                }
            },
            contract_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `contract_type`.");
                }
            },
            package_typo: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `package_typo`.");
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
            internal_external: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `internal_external`.");
                }
            },
            counterparty_group_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `counterparty_group_code`.");
                }
            },
            counterparty_parent_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `counterparty_parent_code`.");
                }
            },
            counterparty_child_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `counterparty_child_code`.");
                }
            },
            bank_non_bank: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bank_non_bank`.");
                }
            },
            trade_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `trade_dt`.");
                }
            },
            maturity_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `maturity_date`.");
                }
            },
            buy_sale: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `buy_sale`.");
                }
            },
            underlying_index: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `underlying_index`.");
                }
            },
            notional_currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `notional_currency`.");
                }
            },
            original_notional_amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `original_notional_amount`.");
                }
            },
            mtm_in_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `mtm_in_inr`.");
                }
            },
            net_pv01_in_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `net_pv01_in_inr`.");
                }
            },
            modified_duration_of_the_deal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `modified_duration_of_the_deal`.");
                }
            },
            reset_frequency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `reset_frequency`.");
                }
            },
            next_reset_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `next_reset_date`.");
                }
            },
            underlying_pp: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `underlying_pp`.");
                }
            },
            deal_status: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `deal_status`.");
                }
            },
            counterparty_category1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `counterparty_category1`.");
                }
            },
            counterparty_category2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `counterparty_category2`.");
                }
            },
            counterparty_category3: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `counterparty_category3`.");
                }
            },
            accounting_section: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `accounting_section`.");
                }
            },
            flowtype4: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `flowtype4`.");
                }
            },
            flow_amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `flow_amount`.");
                }
            },
            cashflow_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `cashflow_date`.");
                }
            },
            flow_currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `flow_currency`.");
                }
            },
        };
        Ok(input_account)
    }
}
