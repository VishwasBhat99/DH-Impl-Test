use rbdate::DateParser;
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub v_src_sys_id: String,
    pub n_ccf_prcnt: f64,
    pub v_basel_asset_class_desc: String,
    pub v_party_type_desc: String,
    pub v_ccy_code: String,
    pub v_exp_amt: f64,
    pub n_undrawn_amt: f64,
    pub lcr_category: String,
    pub asset_class_desc: String,
    pub final_mapping: String,
    pub f_uncond_cancelled_exp_ind: String,
    pub slr: f64,
    pub lcr: f64,
    pub v_prod_code:String
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
            n_ccf_prcnt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `n_ccf_prcnt`.");
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
            v_ccy_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `v_ccy_code`.");
                }
            },
            v_exp_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `v_exp_amt`.");
                }
            },
            n_undrawn_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `n_undrawn_amt`.");
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
            final_mapping: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `final_mapping`.");
                }
            },
            f_uncond_cancelled_exp_ind: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `f_uncond_cancelled_exp_ind`.");
                }
            },

            slr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `slr`.");
                }
            },
            lcr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `lcr`.");
                }
            },
            v_prod_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `v_prod_code`.");
                }
            },

        };
        Ok(input_account)
    }
}
