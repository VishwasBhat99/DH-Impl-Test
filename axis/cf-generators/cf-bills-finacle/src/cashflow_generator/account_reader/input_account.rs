use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub account_number: String,
    pub acid: String,
    pub foracid: String,
    pub bacid: String,
    pub solid: String,
    pub cust_id: String,
    pub schm_code: String,
    pub schm_type: String,
    pub bill_param_type: String,
    pub bill_b2k_id: String,
    pub bill_id: String,
    pub bill_amt: f64,
    pub bill_amt_inr: f64,
    pub bill_crncy_code: String,
    pub due_date: Option<NaiveDate>,
    pub bp_acid: String,
    pub del_flg: String,
    pub cls_flg: String,
    pub reg_type: String,
    pub reg_sub_type: String,
    pub bp_liab: f64,
    pub bp_liab_crncy: String,
    pub bill_liab_inr: f64,
    pub bill_stat: String,
    pub bill_func_code: String,
    pub bill_liab: f64,
    pub bill_liab_hc_eq: f64,
    pub bill_liab_crncy: String,
    pub bill_liab_crncy_der: String,
    pub clr_bal_amt: f64,
    pub un_clr_bal_amt: f64,
    pub out_bal_amt: f64,
    pub acct_opn_date: Option<NaiveDate>,
    pub acct_crncy_code: String,
    pub cust_name: String,
    pub gl_sub_head_code: String,
    pub npa_classification: String,
    pub cust_hlth_code: String,
    pub cust_npa_class: String,
    pub final_npa_class: String,
    pub int_rate: f64,
    pub acct_exch_rt: f64,
    pub cust_grp_id: String,
    pub ucif_cust_const: String,
    pub exch_rt: String,
    pub out_bal_amt_con: String,
    pub segment_code: String,
    pub nfs: String,
    pub overdue_flg: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            account_number: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `account_number`.");
                }
            },
            acid: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acid`.");
                }
            },
            foracid: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `foracid`.");
                }
            },
            bacid: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bacid`.");
                }
            },
            solid: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `solid`.");
                }
            },
            cust_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_id`.");
                }
            },
            schm_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `schm_code`.");
                }
            },
            schm_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `schm_type`.");
                }
            },
            bill_param_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bill_param_type`.");
                }
            },
            bill_b2k_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bill_b2k_id`.");
                }
            },
            bill_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bill_id`.");
                }
            },
            bill_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `bill_amt`.");
                }
            },
            bill_amt_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `bill_amt_inr`.");
                }
            },
            bill_crncy_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bill_crncy_code`.");
                }
            },
            due_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `due_date`.");
                }
            },
            bp_acid: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bp_acid`.");
                }
            },
            del_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `del_flg`.");
                }
            },
            cls_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cls_flg`.");
                }
            },
            reg_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `reg_type`.");
                }
            },
            reg_sub_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `reg_sub_type`.");
                }
            },
            bp_liab: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `bp_liab`.");
                }
            },
            bp_liab_crncy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bp_liab_crncy`.");
                }
            },
            bill_liab_inr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `bill_liab_inr`.");
                }
            },
            bill_stat: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bill_stat`.");
                }
            },
            bill_func_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bill_func_code`.");
                }
            },
            bill_liab: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `bill_liab`.");
                }
            },
            bill_liab_hc_eq: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `bill_liab_hc_eq`.");
                }
            },
            bill_liab_crncy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bill_liab_crncy`.");
                }
            },
            bill_liab_crncy_der: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bill_liab_crncy_der`.");
                }
            },
            clr_bal_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `clr_bal_amt`.");
                }
            },
            un_clr_bal_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `un_clr_bal_amt`.");
                }
            },
            out_bal_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `out_bal_amt`.");
                }
            },
            acct_opn_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `acct_opn_date`.");
                }
            },
            acct_crncy_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acct_crncy_code`.");
                }
            },
            cust_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_name`.");
                }
            },
            gl_sub_head_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `gl_sub_head_code`.");
                }
            },
            npa_classification: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `npa_classification`.");
                }
            },
            cust_hlth_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_hlth_code`.");
                }
            },
            cust_npa_class: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_npa_class`.");
                }
            },
            final_npa_class: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `final_npa_class`.");
                }
            },
            int_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_rate`.");
                }
            },
            acct_exch_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `acct_exch_rt`.");
                }
            },
            cust_grp_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_grp_id`.");
                }
            },
            ucif_cust_const: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ucif_cust_const`.");
                }
            },
            exch_rt: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `exch_rt`.");
                }
            },
            out_bal_amt_con: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `out_bal_amt_con`.");
                }
            },
            segment_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `segment_code`.");
                }
            },
            nfs: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `nfs`.");
                }
            },
            overdue_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `overdue_flg`.");
                }
            },
        };
        Ok(input_account)
    }
}
