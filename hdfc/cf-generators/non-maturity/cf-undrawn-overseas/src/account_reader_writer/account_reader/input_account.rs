use rbdate::DateParser;
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub v_d_cust_ref_code: String,
    pub v_line_code: String,
    pub n_ccf_prcnt: f64,
    pub n_undrawn_amt: f64,
    pub v_basel_asset_class_desc: String,
    pub v_party_type_desc: String,
    pub gl_code: String,
    pub v_ccy_code: String,
    pub branch_code: String,
    pub country_code: String,
    pub lcr_category: String,
    pub asset_class_desc: String,
    pub final_mapping_lcr: String,
    pub f_uncond_cancelled_exp_ind:String,
    pub ccod_flag: String,
    pub fb_nfb: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
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
            n_ccf_prcnt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `n_ccf_prcnt`.");
                }
            },
            n_undrawn_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `n_undrawn_amt`.");
                }
            },
            v_basel_asset_class_desc: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `v_basel_asset_class_desc`.");
                }
            },
            v_party_type_desc: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `v_party_type_desc`.");
                }
            },
            gl_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `gl_code`.");
                }
            },
            v_ccy_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `v_ccy_code`.");
                }
            },
            branch_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `branch_code`.");
                }
            },
            country_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `country_code`.");
                }
            },
            lcr_category: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `lcr_category`.");
                }
            },
            asset_class_desc: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `asset_class_desc`.");
                }
            },
            final_mapping_lcr:match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `final_mapping_lcr`.");
                }
            },
            f_uncond_cancelled_exp_ind:match  value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `f_uncond_cancelled_exp_ind`.");
                }
            },
            ccod_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ccod_flag`.");
                }
            },
            fb_nfb: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `fb_nfb`.");
                }
            },
        };
        Ok(input_account)
    }
}
