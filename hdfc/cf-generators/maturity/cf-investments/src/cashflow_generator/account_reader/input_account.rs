use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub deal_no: String,
    pub short_name: String,
    pub nxt_rep_dt: Option<NaiveDate>,
    pub call_dt: Option<NaiveDate>,
    pub put_dt: Option<NaiveDate>,
    pub deal_dt: Option<NaiveDate>,
    pub portfolio: String,
    pub deal_rt: f64,
    pub org_face_val: i64,
    pub os_face_val: i64,
    pub org_cst_val: f64,
    pub acrd_int: f64,
    pub book_yield: f64,
    pub int_basis: i64,
    pub avg_os_vd: f64,
    pub avg_os_dd: f64,
    pub os_cost_val: Option<f64>,
    pub org_bal: f64,
    pub coup_rt: f64,
    pub nxt_coup_dt: Option<NaiveDate>,
    pub gl: i64,
    pub mat_dt: Option<NaiveDate>,
    pub secu_desc: String,
    pub prod_desc: String,
    pub prod_cd: String,
    pub lst_coup_dt: Option<NaiveDate>,
    pub call_dt1: Option<NaiveDate>,
    pub coup_freq: String,
    pub val_dt: Option<NaiveDate>,
    pub acrl_freq: String,
    pub lst_rep_dt: Option<NaiveDate>,
    pub lst_put_dt: Option<NaiveDate>,
    pub inst: String,
    pub org_term: i64,
    pub acrl_basis: String,
    pub div: String,
    pub alm_line: String,
    pub cmpnd_freq: i64,
    pub nxt_cmpnd_dt: Option<NaiveDate>,
    pub rt_chng_freq: i64,
    pub rt_flg: String,
    pub rep_idx: String,
    pub nxt_pay_dt: Option<NaiveDate>,
    pub prev_rep_dt: Option<NaiveDate>,
    pub int_pay_freq: i64,
    pub int_rt: Option<f64>,
    pub as_on_dt: NaiveDate,
    pub port_typ: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            deal_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `deal_no`.");
                }
            },
            short_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `short_name`.");
                }
            },
            nxt_rep_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `next_repricing_date`.");
                }
            },
            call_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `call_dt`.");
                }
            },
            put_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `put_dt`.");
                }
            },
            deal_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `deal_date`.");
                }
            },
            portfolio: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `portfolio`.");
                }
            },
            deal_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `deal_rate`.");
                }
            },
            org_face_val: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `org_face_value`.");
                }
            },
            os_face_val: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `os_face_value`.");
                }
            },
            org_cst_val: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `org_cst_val`.");
                }
            },
            acrd_int: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `acrd_int`.");
                }
            },
            book_yield: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `book_yield`.");
                }
            },
            int_basis: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `int_basis`.");
                }
            },
            avg_os_vd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `avg_os_vd`.");
                }
            },
            avg_os_dd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `avg_os_dd`.");
                }
            },
            os_cost_val: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `os_cost_val`.");
                }
            },
            org_bal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `org_bal`.");
                }
            },
            coup_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `coup_rt`.");
                }
            },
            nxt_coup_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `nxt_coup_dt`.");
                }
            },
            gl: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `gl`.");
                }
            },
            mat_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `mat_dt`.");
                }
            },
            secu_desc: match value_iterator.next() {
                Some(val) => val.to_string(),

                None => {
                    return Err("Could not parse property `division`.");
                }
            },
            prod_desc: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `prod_desc`.");
                }
            },
            prod_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `prod_cd`.");
                }
            },
            lst_coup_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `lst_coup_dt`.");
                }
            },
            call_dt1: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `call_dt1`.");
                }
            },
            coup_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `coup_freq`.");
                }
            },
            val_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `val_dt`.");
                }
            },
            acrl_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `accural_frequency`.");
                }
            },
            lst_rep_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `last_repricing_date`.");
                }
            },
            lst_put_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `last_put_date`.");
                }
            },
            inst: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `institution`.");
                }
            },
            org_term: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `orginal_term`.");
                }
            },
            acrl_basis: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `accural_basis`.");
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
            cmpnd_freq: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `compound_frequency`.");
                }
            },
            nxt_cmpnd_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `next_compound_date`.");
                }
            },
            rt_chng_freq: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `rate_exchange_frequency`.");
                }
            },
            rt_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rate_flag`.");
                }
            },
            rep_idx: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `repricing_index`.");
                }
            },
            nxt_pay_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `next_payment_date`.");
                }
            },
            prev_rep_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `previous_repricing_date`.");
                }
            },
            int_pay_freq: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `interest_payment_frequency`.");
                }
            },
            int_rt: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `interest_rate`.");
                }
            },
            as_on_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse(val),
                None => {
                    return Err("Could not parse property `as_on_date.");
                }
            },
            port_typ: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `port_typ.");
                }
            },
        };
        Ok(input_account)
    }
}
