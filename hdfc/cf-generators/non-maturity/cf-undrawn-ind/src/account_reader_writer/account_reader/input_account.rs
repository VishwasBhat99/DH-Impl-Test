use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub v_src_sys_id: String,
    pub v_exp_id: String,
    pub v_d_cust_ref_code: String,
    pub v_line_code: String, 
    pub v_prod_code: String,
    pub v_pp_table: String,
    pub n_ccf_prcnt: f64,
    pub d_exp_strt_dt: Option<NaiveDate>,
    pub d_exp_end_dt: Option<NaiveDate>,
    pub n_exp_amt: f64,
    pub n_undrawn_amt: f64,
    pub v_basel_prod_typ_desc_lv1: String,
    pub v_basel_prod_typ_desc: String,
    pub v_basel_asst_class_desc: String,
    pub v_party_typ_desc: String,
    pub gl_code: String,
    pub v_party_name: String,
    pub v_ram_id: String,
    pub v_ccy_code: String,
    pub v_fclty_desc: String,
    pub v_ret_corp_ind: String,  
    pub fb_nfb: String,
    pub ccod_flag: String,
    pub lcr_cat: String,
    pub asst_class_desc: String,
    pub final_map_lcr: String,
    pub flag_uncond_cancelled_exp_ind: String,
    pub slr_amt: f64,
    pub lcr_amt: f64,

}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            v_src_sys_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `v_src_sys_id`.");
                }
            },
            v_exp_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `v_exp_id`.");
                }
            },
            v_d_cust_ref_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `v_d_cust_ref_code`.");
                }
            },
            v_line_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `v_line_code`.");
                }
            },
            v_prod_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `v_prod_code`.");
                }
            },
            v_pp_table: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `v_prod_code`.");
                }
            },
            n_ccf_prcnt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `n_ccf_prcnt`.");
                }
            },
            d_exp_strt_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `d_exp_strt_dt`.");
                }
            },
            d_exp_end_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `d_exp_end_dt`.");
                }
            },
            n_exp_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `n_exp_amt`.");
                }
            },
            n_undrawn_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `n_undrawn_amt`.");
                }
            },
            v_basel_prod_typ_desc_lv1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `v_basel_prod_typ_desc_lv1`.");
                }
            },
            v_basel_prod_typ_desc: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `v_basel_prod_typ_desc`.");
                }
            },
            v_basel_asst_class_desc: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `v_basel_asst_class_desc`.");
                }
            },
            v_party_typ_desc: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `v_party_typ_desc`.");
                }
            },
            gl_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `gl_code`.");
                }
            },
            v_party_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `v_party_name`.");
                }
            },
            v_ram_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `v_ram_id`.");
                }
            },
            v_ccy_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `v_ccy_code`.");
                }
            },
            v_fclty_desc: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `v_fclty_desc`.");
                }
            },
            v_ret_corp_ind: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `v_ret_corp_ind`.");
                }
            },
            fb_nfb: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `fb_nfb`.");
                }
            },
            ccod_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ccod_flag`.");
                }
            },
            lcr_cat: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `lcr_cat`.");
                }
            },
            asst_class_desc: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `asst_class_desc`.");
                }
            },
            final_map_lcr: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `final_map_lcr`.");
                }
            },
            flag_uncond_cancelled_exp_ind:match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `flag_uncond_cancelled_exp_ind`.");
                }
            },
            slr_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `slr_amt`.");
                }
            },
            lcr_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `lcr_amt`.");
                }
            },
        };
        Ok(input_account)
    }
}
