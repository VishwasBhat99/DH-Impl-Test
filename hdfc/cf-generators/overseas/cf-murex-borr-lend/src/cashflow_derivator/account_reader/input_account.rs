use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub deal_id: String,
    pub branch: String,
    pub inst_name: String,
    pub lend_borr_typ: String,
    pub typology: String,
    pub usage: String,
    pub sub_typ_borr_lend: String,
    pub cntrprty: String,
    pub crtn_dt: Option<NaiveDate>,
    pub val_date: Option<NaiveDate>,
    pub deal_date: Option<NaiveDate>,
    pub ccy: String,
    pub crnt_deal_amt: f64,
    pub crnt_conv_rt_lcy: f64,
    pub crnt_deal_amt_lcy: f64,
    pub roi: f64,
    pub tenor_days: i64,
    pub mat_dt: Option<NaiveDate>,
    pub prin_amt: f64,
    pub int_amt: f64,
    pub cf_typ: String,
    pub flow_typ: String,
    pub mat_amt: f64,
    pub dealer_name: String,
    pub nds_ref_no: String,
    pub nxt_fix_dt: Option<NaiveDate>,
    pub residual_tenor: i64,
    pub nxt_put_dt: Option<NaiveDate>,
    pub nxt_call_dt: Option<NaiveDate>,
    pub nxt_int_pay_dt: Option<NaiveDate>,
    pub int_pay_tenor: i64,
    pub aip_air: f64,
    pub downgrade_clause: String,
    pub avg_monthly_bal: String,
    pub glcode: String,
    pub cntrprty_ctgry_1: String,
    pub cntrprty_ctgry_2: String,
    pub cntrprty_ctgry_3: String,
    pub cntrprty_ctgry_4: String,
    pub int_pay_rec: String,
    pub bckt_days: i64,
    pub country: String,
    pub system_gl: String,
    pub prod_concat: String,
    pub alm_concat: String,
    pub div: String,
    pub alm_line: String,
    pub ia_line: String,
    pub balm_l2: String,
    pub funding_source: String,
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
            branch: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `branch`.");
                }
            },
            inst_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `instrument_name`.");
                }
            },
            lend_borr_typ: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `lending_borrowing_type`.");
                }
            },
            typology: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `typology`.");
                }
            },
            usage: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `usage`.");
                }
            },
            sub_typ_borr_lend: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sub_type_borrowing_lending`.");
                }
            },
            cntrprty: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `counterparty`.");
                }
            },
            crtn_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `creation_date`.");
                }
            },
            val_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `val_date`.");
                }
            },
            deal_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `deal_date`.");
                }
            },
            ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `currency`.");
                }
            },
            crnt_deal_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `current_deal_amount`.");
                }
            },
            crnt_conv_rt_lcy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `current_conversion_rate_lcy`.");
                }
            },
            crnt_deal_amt_lcy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `current_deal_amount_lcy`.");
                }
            },
            roi: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `roi`.");
                }
            },
            tenor_days: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `tenor_days`.");
                }
            },
            mat_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `maturity_dt`.");
                }
            },
            prin_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `principal_amount`.");
                }
            },
            int_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `interest_amount`.");
                }
            },
            cf_typ: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cf_type`.");
                }
            },
            flow_typ: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `flow_type`.");
                }
            },
            mat_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `maturity_amount`.");
                }
            },
            dealer_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `dealer_name`.");
                }
            },
            nds_ref_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ndsreferenceno`.");
                }
            },
            nxt_fix_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `next_fixing_date`.");
                }
            },
            residual_tenor: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `residual_tenor`.");
                }
            },
            nxt_put_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `next_put_date`.");
                }
            },
            nxt_call_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `asofdate`.");
                }
            },
            nxt_int_pay_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `asofdate`.");
                }
            },
            int_pay_tenor: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `interest_payout_tenor`.");
                }
            },
            aip_air: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `aip_air`.");
                }
            },
            downgrade_clause: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `downgrade_clause`.");
                }
            },
            avg_monthly_bal: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `average_monthly_balance`.");
                }
            },
            glcode: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `glcode`.");
                }
            },
            cntrprty_ctgry_1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `counterparty_category_1`.");
                }
            },
            cntrprty_ctgry_2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `counterparty_category_2`.");
                }
            },
            cntrprty_ctgry_3: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `counterparty_category_3`.");
                }
            },
            cntrprty_ctgry_4: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `counterparty_category_4`.");
                }
            },
            int_pay_rec: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `interest_payable_receivable_till_the_next_payout_date`.");
                }
            },
            bckt_days: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `bucket_days`.");
                }
            },
            country: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `country`.");
                }
            },
            system_gl: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `system_gl`.");
                }
            },
            prod_concat: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `product_concat`.");
                }
            },
            alm_concat: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `alm_concat`.");
                }
            },
            div: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `division`.");
                }
            },
            alm_line: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `alm_line`.");
                }
            },
            ia_line: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ia_line`.");
                }
            },
            balm_l2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `balm_l2`.");
                }
            },
            funding_source: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `funding_source`.");
                }
            },
        };
        Ok(input_account)
    }
}
