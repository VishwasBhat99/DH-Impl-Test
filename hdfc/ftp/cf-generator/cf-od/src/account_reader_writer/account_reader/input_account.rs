use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub cod_acc_no: String,
    pub cod_cc_brn: String,
    pub cod_prod: String,
    pub bal_book: f64,
    pub bal_book_lcy: f64,
    pub amt_od_lmt: f64,
    pub amt_od_lmt_lcy: f64,
    pub cod_cust: i64,
    pub cod_acc_title: String,
    pub dt_open_acc: Option<NaiveDate>,
    pub cod_int_accr_bas: String,
    pub freq_int_accr: String,
    pub dt_acc_close: Option<NaiveDate>,
    pub cod_collat_id: String,
    pub collat_desc: String,
    pub as_of_dt: NaiveDate,
    pub cost_cntr: String,
    pub gl_acc_no: String,
    pub rt_flg: String,
    pub inst: String,
    pub crnt_book_bal: Option<f64>,
    pub acrl_basis: String,
    pub int_rt: Option<f64>,
    pub div: String,
    pub alm_line: String,
    pub ia_llg: String,
    pub balm_llg: String,
    pub mis1: i64,
    pub npa_flg: String,
    pub benchmark: String,
    pub rep_freq: String,
    pub nxt_rep_dt: Option<NaiveDate>,
    pub lst_rep_dt: Option<NaiveDate>,
    pub cust_typ: String,
    pub country: String,
    pub bm_id_lookup: String,
    pub mis2: i64,
    pub avg_bal: f64,
    pub is_acc_weaker: String,
    pub ews_weaker_value: String,
    pub alm_concat: String,
    pub two_point_concat: String,
    pub weaker_desc: String,
    pub bdp_division: String,
    pub bdp_coa: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            cod_acc_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cod_acc_no`.");
                }
            },
            cod_cc_brn: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cod_cc_brn`.");
                }
            },
            cod_prod: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cod_prod`.");
                }
            },
            bal_book: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `bal_book`.");
                }
            },
            bal_book_lcy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `bal_book_lcy`.");
                }
            },
            amt_od_lmt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `amt_od_lmt`.");
                }
            },
            amt_od_lmt_lcy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `flex_cube_cust_id`.");
                }
            },
            cod_cust: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `cod_cust`.");
                }
            },
            cod_acc_title: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cod_acc_title`.");
                }
            },
            dt_open_acc: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `dt_open_acc`.");
                }
            },
            cod_int_accr_bas: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cod_int_accr_bas`.");
                }
            },
            freq_int_accr: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `freq_int_accr`.");
                }
            },
            dt_acc_close: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `dt_acc_close`.");
                }
            },
            cod_collat_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cod_collat_id`.");
                }
            },
            collat_desc: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `collat_desc`.");
                }
            },
            as_of_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse(val),
                None => {
                    return Err("Could not parse property `as_of_dt`.");
                }
            },
            cost_cntr: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cost_cntr`.");
                }
            },
            gl_acc_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `gl_acc_no`.");
                }
            },
            rt_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rt_flg`.");
                }
            },
            inst: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `institution`.");
                }
            },
            crnt_book_bal: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `current_book_balance`.");
                }
            },
            acrl_basis: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acrl_basis`.");
                }
            },
            int_rt: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `int_rt`.");
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
            ia_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ia_llg`.");
                }
            },
            balm_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `balm_llg`.");
                }
            },
            mis1: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `mis1`.");
                }
            },
            npa_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `npa_flg`.");
                }
            },
            benchmark: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `benchmark`.");
                }
            },
            rep_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `interest_index_code`.");
                }
            },
            nxt_rep_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `next_repricing_date`.");
                }
            },
            lst_rep_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `last_repricing_date`.");
                }
            },
            cust_typ: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_typ`.");
                }
            },
            country: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `country`.");
                }
            },
            bm_id_lookup: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bm_id_lookup`.");
                }
            },
            mis2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `mis2`.");
                }
            },
            avg_bal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `avg_bal`.");
                }
            },
            is_acc_weaker: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `is_acc_weaker`.");
                }
            },
            ews_weaker_value: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ews_weaker_value`.");
                }
            },
            alm_concat: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `alm_concat`.");
                }
            },
            two_point_concat: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `two_point_concat`.");
                }
            },
            weaker_desc: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `weaker_description`.");
                }
            },
            bdp_division: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bdp_division`.");
                }
            },
            bdp_coa: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bdp coa`.");
                }
            },
        };
        Ok(input_account)
    }
}
