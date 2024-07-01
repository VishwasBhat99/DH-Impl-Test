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
    pub prin_amt: f64,
    pub org_bal: f64,
    pub coup_rt: f64,
    pub nxt_coup_dt: Option<NaiveDate>,
    pub gl: i64,
    pub cf_dt: Option<NaiveDate>,
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
    pub prod_concat: String,
    pub concat: String,
    pub div: String,
    pub alm_line: String,
    pub ia_line: String,
    pub cmpnd_freq: i64,
    pub nxt_cmpnd_dt: Option<NaiveDate>,
    pub rt_chng_freq: i64,
    pub rt_flg: String,
    pub rep_idx: String,
    pub nxt_pay_dt: Option<NaiveDate>,
    pub prev_rep_dt: Option<NaiveDate>,
    pub int_pay_freq: i64,
    pub int_rt: f64,
    pub as_on_dt: Option<NaiveDate>,
    pub port_typ: String,
    pub sec_grp: String,
    pub sec_type: String,
    pub sec_issuer: String,
    pub sec_guaranteed: String,
    pub mrkt: String,
    pub idx_label: String,
    pub bd_categ: String,
    pub bd_type: String,
    pub listed: String,
    pub npa_class: String,
    pub entity: String,
    pub desk: String,
    pub acc_sec_igaap: String,
    pub os_cv_before_amort: f64,
    pub os_cv_after_amort: f64,
    pub mat_dt: Option<NaiveDate>,
    pub int_amt: f64,
    pub flow_type: String,
    pub isin: String,
    pub wap_igaap: f64,
    pub ost_bal: f64,
    pub contract_no: String,
    pub instr_id: String,
    pub parent_code: String,
    pub issuer_name: String,
    pub rating: String,
    pub tax_status: String,
    pub slr_nslr: String,
    pub deal_ytm: f64,
    pub intr_app_freq: String,
    pub comp_freq: String,
    pub intr_prac: String,
    pub rt_spread: String,
    pub asset_class: String,
    pub intr_typ: String,
    pub sec_issuance_date: Option<NaiveDate>,
    pub coupon: String,
    pub last_intr_dt: Option<NaiveDate>,
    pub next_intr_dt: Option<NaiveDate>,
    pub amort_till_date: f64,
    pub ftp_lst_repr_dt: Option<NaiveDate>,
    pub ftp_nxt_repr_dt: Option<NaiveDate>,
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
            prin_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt`.");
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
            cf_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `cf_dt`.");
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
            prod_concat: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `prod_concat`.");
                }
            },
            concat: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `concat`.");
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
                    return Err("Could not parse property `repricing_idx`.");
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
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `interest_rate`.");
                }
            },
            as_on_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
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
            sec_grp: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sec_grp.");
                }
            },
            sec_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sec_type.");
                }
            },
            sec_issuer: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sec_issuer.");
                }
            },
            sec_guaranteed: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sec_guaranteed.");
                }
            },
            mrkt: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `mrkt.");
                }
            },
            idx_label: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `idx_label.");
                }
            },
            bd_categ: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bd_categ.");
                }
            },
            bd_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bd_type.");
                }
            },
            listed: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `listed.");
                }
            },
            npa_class: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `npa_class.");
                }
            },
            entity: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `entity.");
                }
            },
            desk: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `desk.");
                }
            },
            acc_sec_igaap: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acc_sec_igaap.");
                }
            },
            os_cv_before_amort: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `os_cv_before_amort`.");
                }
            },
            os_cv_after_amort: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `os_cv_after_amort`.");
                }
            },
            mat_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `cf_dt`.");
                }
            },
            int_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt`.");
                }
            },
            flow_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `flow_type.");
                }
            },
            isin: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `isin.");
                }
            },
            wap_igaap: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `wap_igaap`.");
                }
            },
            ost_bal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `ost_bal`.");
                }
            },
            contract_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `contract_no`.");
                }
            },
            instr_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `instr_id`.");
                }
            },
            parent_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `parent_code`.");
                }
            },
            issuer_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `issuer_name`.");
                }
            },
            rating: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rating`.");
                }
            },
            tax_status: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `tax_status`.");
                }
            },
            slr_nslr: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `slr_nslr`.");
                }
            },
            deal_ytm: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `deal_ytm`.");
                }
            },
            intr_app_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `intr_app_freq`.");
                }
            },
            comp_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `comp_freq`.");
                }
            },
            intr_prac: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `intr_prac`.");
                }
            },
            rt_spread: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rt_spread`.");
                }
            },
            asset_class: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `asset_class`.");
                }
            },
            intr_typ: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `intr_typ`.");
                }
            },
            sec_issuance_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `sec_issuance_date`.");
                }
            },
            coupon: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `coupon`.");
                }
            },
            last_intr_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `last_intr_dt`.");
                }
            },
            next_intr_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `next_intr_dt`.");
                }
            },
            amort_till_date: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `amort_till_date`.");
                }
            },
            ftp_lst_repr_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `ftp_lst_repr_dt`.");
                }
            },
            ftp_nxt_repr_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `ftp_nxt_repr_dt`.");
                }
            },
        };
        Ok(input_account)
    }
}
