use rbdate::{DateParser, NaiveDate};
use statics::DEFAULT_FLOAT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub deal_no: String,
    pub bond_issuance: String,
    pub isin: String,
    pub issuance_dt: Option<NaiveDate>,
    pub branch_entity: String,
    pub desk: String,
    pub portfolio_type: String,
    pub category: String,
    pub security_type: String,
    pub slrnon_slr: String,
    pub short_name: String,
    pub secured_unsecured: String,
    pub rt: f64,
    pub nxt_call_dt: Option<NaiveDate>,
    pub nxt_put_dt: Option<NaiveDate>,
    pub agency: String,
    pub rating: String,
    pub agency_of_current_rating: String,
    pub listed_unlisted: String,
    pub mat_dt: Option<NaiveDate>,
    pub conversion_rt_lcy: f64,
    pub ccy: String,
    pub bv_after_amortisation: f64,
    pub wap: f64,
    pub laf_and_msf_ost_fv: f64,
    pub laf_and_msf_ost_bv: f64,
    pub reverse_laf_ost_fv: f64,
    pub reverse_repo_ost_fv: f64,
    pub collateral_placed_fv: f64,
    pub encumbered_fv: f64,
    pub encumbered_bv: f64,
    pub ytm: f64,
    pub basis: String,
    pub issue_country: String,
    pub domicile_country: String,
    pub category1: String,
    pub category2: String,
    pub category3: String,
    pub category4: String,
    pub industry_code: String,
    pub taxability: String,
    pub air_till_dt: f64,
    pub modified_duration: f64,
    pub int_coupontype: String,
    pub nxt_rep_dt: Option<NaiveDate>,
    pub sec_grp: String,
    pub sec_typ: String,
    pub sec_issuer: String,
    pub sec_guaranteed: String,
    pub mrkt: String,
    pub idx_label: String,
    pub bd_cat: String,
    pub bd_typ: String,
    pub lstd: String,
    pub npa: String,
    pub cf_dt: Option<NaiveDate>,
    pub cf_int_amt: f64,
    pub cf_prin_amt: f64,
    pub fwd_outright_buy_fv: f64,
    pub fwd_outright_buy_bv: f64,
    pub fwd_outright_sale_fv: f64,
    pub fwd_outright_sale_bv: f64,
    pub as_on_dt: Option<NaiveDate>,
    pub alm_concat: String,
    pub alm_line: String,
    pub ia_line: String,
    pub face_val: f64,
    pub book_val: f64,
    pub market_val: f64,
    pub mtm_amt: f64,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_dt_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            deal_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `deal_no`.");
                }
            },
            bond_issuance: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bond_issuance`.");
                }
            },
            isin: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `isin`.");
                }
            },
            issuance_dt: match value_iterator.next() {
                Some(val) => dmy_dt_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `issuance_dt`.");
                }
            },
            branch_entity: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `branch_entity`.");
                }
            },
            desk: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `desk`.");
                }
            },
            portfolio_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `portfolio_type`.");
                }
            },
            category: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `last_cpn_dt`.");
                }
            },
            security_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `security_type`.");
                }
            },
            slrnon_slr: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `slrnon_slr`.");
                }
            },
            short_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `short_name`.");
                }
            },
            secured_unsecured: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `secured_unsecured`.");
                }
            },
            rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `rt`.");
                }
            },
            nxt_call_dt: match value_iterator.next() {
                Some(val) => dmy_dt_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `nxt_call_dt`.");
                }
            },
            nxt_put_dt: match value_iterator.next() {
                Some(val) => dmy_dt_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `nxt_put_dt`.");
                }
            },
            agency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `agency`.");
                }
            },
            rating: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rating`.");
                }
            },
            agency_of_current_rating: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `agency_of_current_rating`.");
                }
            },
            listed_unlisted: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `listed_unlisted`.");
                }
            },
            mat_dt: match value_iterator.next() {
                Some(val) => dmy_dt_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `settle_amt_1st_leg`.");
                }
            },
            conversion_rt_lcy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `conversion_rt_lcy`.");
                }
            },
            ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ccy`.");
                }
            },
            bv_after_amortisation: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `bv_after_amortisation`.");
                }
            },
            wap: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `wap`.");
                }
            },
            laf_and_msf_ost_fv: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `laf_and_msf_ost_fv`.");
                }
            },
            laf_and_msf_ost_bv: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `laf_and_msf_ost_bv`.");
                }
            },
            reverse_laf_ost_fv: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `reverse_laf_ost_fv`.");
                }
            },
            reverse_repo_ost_fv: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `reverse_repo_ost_fv`.");
                }
            },
            collateral_placed_fv: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `collateral_placed_fv`.");
                }
            },
            encumbered_fv: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `encumbered_fv`.");
                }
            },
            encumbered_bv: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `encumbered_bv`.");
                }
            },
            ytm: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `ytm`.");
                }
            },
            basis: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `basis`.");
                }
            },
            issue_country: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `issue_country`.");
                }
            },
            domicile_country: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `domicile_country`.");
                }
            },
            category1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `category1`.");
                }
            },
            category2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `category2`.");
                }
            },
            category3: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `category3`.");
                }
            },
            category4: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `category4`.");
                }
            },
            industry_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `industry_code`.");
                }
            },
            taxability: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `taxability`.");
                }
            },
            air_till_dt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `air_till_dt`.");
                }
            },
            modified_duration: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `modified_duration`.");
                }
            },
            int_coupontype: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `int_coupontype`.");
                }
            },
            nxt_rep_dt: match value_iterator.next() {
                Some(val) => dmy_dt_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `nxt_rep_dt`.");
                }
            },
            sec_grp: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sec_grp`.");
                }
            },
            sec_typ: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sec_typ`.");
                }
            },
            sec_issuer: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sec_issuer`.");
                }
            },
            sec_guaranteed: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sec_guaranteed`.");
                }
            },
            mrkt: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `mrkt`.");
                }
            },
            idx_label: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `idx_label`.");
                }
            },
            bd_cat: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bd_cat`.");
                }
            },
            bd_typ: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bd_typ`.");
                }
            },
            lstd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `lstd`.");
                }
            },
            npa: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `npa`.");
                }
            },
            cf_dt: match value_iterator.next() {
                Some(val) => dmy_dt_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `cf_dt`.");
                }
            },
            cf_int_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `cf_int_amt`.");
                }
            },
            cf_prin_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `cf_prin_amt`.");
                }
            },
            fwd_outright_buy_fv: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `fwd_outright_buy_fv`.");
                }
            },
            fwd_outright_buy_bv: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `fwd_outright_buy_bv`.");
                }
            },
            fwd_outright_sale_fv: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `fwd_outright_sale_fv`.");
                }
            },
            fwd_outright_sale_bv: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `fwd_outright_sale_bv`.");
                }
            },
            as_on_dt: match value_iterator.next() {
                Some(val) => dmy_dt_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `as_on_dt`.");
                }
            },
            alm_concat: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `alm_concat`.");
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
            face_val: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `face_val`.");
                }
            },
            book_val: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `book_val`.");
                }
            },
            market_val: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `market_val`.");
                }
            },
            mtm_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `mtm_amt`.");
                }
            },
        };
        Ok(input_account)
    }
}
