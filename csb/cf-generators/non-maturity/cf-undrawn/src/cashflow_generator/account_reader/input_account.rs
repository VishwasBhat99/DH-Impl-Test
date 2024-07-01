use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub as_on: Option<NaiveDate>,
    pub acc_no: String,
    pub cust_name: String,
    pub client_id: String,
    pub tl_limit: f64,
    pub ccod_limit: f64,
    pub pbg_limit: f64,
    pub fbg_limit: f64,
    pub loc_limit: f64,
    pub bliab_bill_limit: f64,
    pub tl_blnc: f64,
    pub ccod_blnc: f64,
    pub pbg_blnc: f64,
    pub fbg_blnc: f64,
    pub loc_blnc: f64,
    pub bliab_bill_blnc: f64,
    pub tl_ualimit: f64,
    pub ccod_ualimit: f64,
    pub pbg_ualimit: f64,
    pub fbg_ualimit: f64,
    pub loc_ualimit: f64,
    pub bliab_bill_ualimit: f64,
    pub tl_dep: f64,
    pub ccod_dep: f64,
    pub pbg_dep: f64,
    pub fbg_dep: f64,
    pub loc_dep: f64,
    pub bliab_bill_dep: f64,
    pub tl_cr_eq: f64,
    pub ccod_cr_eq: f64,
    pub pbg_cr_eq: f64,
    pub fbg_cr_eq: f64,
    pub loc_cr_eq: f64,
    pub bliab_bill_cr_eq: f64,
    pub client_type_ip: String,
    pub ext_rating: String,
    pub asset_code: String,
    pub bsr: i64,
    pub client_type: String,
    pub clients_name: String,
    pub clients_bsr_type_flg: String,
    pub clients_busdivn_code: String,
    pub clients_const_code: String,
    pub clients_pan_gir_num: String,
    pub clients_risk_categorization: String,
    pub clients_risk_cntry: String,
    pub clients_segment_code: String,
    pub corpcl_orgn_qualifier: String,
    pub corpcl_indus_code: String,
    pub corpcl_nature_of_bus1: String,
    pub corpcl_nature_of_bus2: String,
    pub corpcl_nature_of_bus3: String,
    pub corpcl_central_state_flg: String,
    pub corpcl_public_sector_flg: String,
    pub corpcl_primary_dlr_flg: String,
    pub corpcl_multilateral_bank: String,
    pub corpcl_connp_inv_num: String,
    pub corpcl_bc_gross_turnover: f64,
    pub ccod_undrawn_lcr: f64,
    pub ccod_und_nsfr: f64,
    pub care_funded: f64,
    pub care_lcbg: f64,
    pub sanc_dt: Option<NaiveDate>,
    pub occp_cd: String,
    pub sens_sec: String,
    pub prior_subtype: String,
    pub restruct_flag: String,
    pub restruct_dt: Option<NaiveDate>,
    pub mor_prd: String,
    pub rating: String,
    pub consitin: String,
    pub pan: String,
    pub limit_amt: f64,
    pub gross_adv: f64,
    pub exp_amt: f64,
    pub unvail_amt: f64,
    pub gold_gram: f64,
    pub fund_flag: String,
    pub ltv_value: f64,
    pub pt_i64_1: i64,
    pub pt_i64_2: i64,
    pub pt_i64_3: i64,
    pub pt_i64_4: i64,
    pub pt_i64_5: i64,
    pub pt_f64_1: f64,
    pub pt_f64_2: f64,
    pub pt_f64_3: f64,
    pub pt_f64_4: f64,
    pub pt_f64_5: f64,
    pub pt_str_1: String,
    pub pt_str_2: String,
    pub pt_str_3: String,
    pub pt_str_4: String,
    pub pt_str_5: String,
    pub maturity_dt: Option<NaiveDate>,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            as_on: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `as_on`.");
                }
            },
            acc_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acc_no`.");
                }
            },
            cust_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_name`.");
                }
            },
            client_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `client_id`.");
                }
            },
            tl_limit: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `tl_limit`.");
                }
            },
            ccod_limit: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `ccod_limit`.");
                }
            },
            pbg_limit: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `pbg_limit`.");
                }
            },
            fbg_limit: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `fbg_limit`.");
                }
            },
            loc_limit: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `loc_limit`.");
                }
            },
            bliab_bill_limit: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `bliab_bill_limit`.");
                }
            },
            tl_blnc: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `tl_blnc`.");
                }
            },
            ccod_blnc: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `ccod_blnc`.");
                }
            },
            pbg_blnc: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `pbg_blnc`.");
                }
            },
            fbg_blnc: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `fbg_blnc`.");
                }
            },
            loc_blnc: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `loc_blnc`.");
                }
            },
            bliab_bill_blnc: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `bliab_bill_blnc`.");
                }
            },
            tl_ualimit: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `tl_ualimit`.");
                }
            },
            ccod_ualimit: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `ccod_ualimit`.");
                }
            },
            pbg_ualimit: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `pbg_ualimit`.");
                }
            },
            fbg_ualimit: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `fbg_ualimit`.");
                }
            },
            loc_ualimit: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `loc_ualimit`.");
                }
            },
            bliab_bill_ualimit: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `bliab_bill_ualimit`.");
                }
            },
            tl_dep: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `tl_dep`.");
                }
            },
            ccod_dep: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `ccod_dep`.");
                }
            },
            pbg_dep: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `pbg_dep`.");
                }
            },
            fbg_dep: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `fbg_dep`.");
                }
            },
            loc_dep: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `loc_dep`.");
                }
            },
            bliab_bill_dep: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `bliab_bill_dep`.");
                }
            },
            tl_cr_eq: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `tl_cr_eq`.");
                }
            },
            ccod_cr_eq: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `ccod_cr_eq`.");
                }
            },
            pbg_cr_eq: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `pbg_cr_eq`.");
                }
            },
            fbg_cr_eq: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `fbg_cr_eq`.");
                }
            },
            loc_cr_eq: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `loc_cr_eq`.");
                }
            },
            bliab_bill_cr_eq: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `bliab_bill_cr_eq`.");
                }
            },
            client_type_ip: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `client_type_ip`.");
                }
            },
            ext_rating: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ext_rating`.");
                }
            },
            asset_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `asset_code`.");
                }
            },
            bsr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `bsr`.");
                }
            },
            client_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `client_type`.");
                }
            },
            clients_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `clients_name`.");
                }
            },
            clients_bsr_type_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `clients_bsr_type_flg`.");
                }
            },
            clients_busdivn_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `clients_busdivn_code`.");
                }
            },
            clients_const_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `clients_const_code`.");
                }
            },
            clients_pan_gir_num: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `clients_pan_gir_num`.");
                }
            },
            clients_risk_categorization: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `clients_risk_categorization`.");
                }
            },
            clients_risk_cntry: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `clients_risk_cntry`.");
                }
            },
            clients_segment_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `clients_segment_code`.");
                }
            },
            corpcl_orgn_qualifier: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `corpcl_orgn_qualifier`.");
                }
            },
            corpcl_indus_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `corpcl_indus_code`.");
                }
            },
            corpcl_nature_of_bus1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `corpcl_nature_of_bus1`.");
                }
            },
            corpcl_nature_of_bus2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `corpcl_nature_of_bus2`.");
                }
            },
            corpcl_nature_of_bus3: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `corpcl_nature_of_bus3`.");
                }
            },
            corpcl_central_state_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `corpcl_central_state_flg`.");
                }
            },
            corpcl_public_sector_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `corpcl_public_sector_flg`.");
                }
            },
            corpcl_primary_dlr_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `corpcl_primary_dlr_flg`.");
                }
            },
            corpcl_multilateral_bank: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `corpcl_multilateral_bank`.");
                }
            },
            corpcl_connp_inv_num: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `corpcl_connp_inv_num`.");
                }
            },
            corpcl_bc_gross_turnover: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `corpcl_bc_gross_turnover`.");
                }
            },
            ccod_undrawn_lcr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `ccod_undrawn_lcr`.");
                }
            },
            ccod_und_nsfr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `ccod_und_nsfr`.");
                }
            },
            care_funded: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `care_funded`.");
                }
            },
            care_lcbg: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `care_lcbg`.");
                }
            },
            sanc_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `sanc_dt`.");
                }
            },
            occp_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `occp_cd`.");
                }
            },
            sens_sec: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sens_sec`.");
                }
            },
            prior_subtype: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `prior_subtype`.");
                }
            },
            restruct_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `restruct_flag`.");
                }
            },
            restruct_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `restruct_dt`.");
                }
            },
            mor_prd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `mor_prd`.");
                }
            },
            rating: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rating`.");
                }
            },
            consitin: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `consitin`.");
                }
            },
            pan: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pan`.");
                }
            },
            limit_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `limit_amt`.");
                }
            },
            gross_adv: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `gross_adv`.");
                }
            },
            exp_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `exp_amt`.");
                }
            },
            unvail_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `unvail_amt`.");
                }
            },
            gold_gram: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `gold_gram`.");
                }
            },
            fund_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `fund_flag`.");
                }
            },
            ltv_value: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `ltv_value`.");
                }
            },
            pt_i64_1: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `pt_i64_1`.");
                }
            },
            pt_i64_2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `pt_i64_2`.");
                }
            },
            pt_i64_3: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `pt_i64_3`.");
                }
            },
            pt_i64_4: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `pt_i64_4`.");
                }
            },
            pt_i64_5: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `pt_i64_5`.");
                }
            },
            pt_f64_1: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `pt_f64_1`.");
                }
            },
            pt_f64_2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `pt_f64_2`.");
                }
            },
            pt_f64_3: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `pt_f64_3`.");
                }
            },
            pt_f64_4: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `pt_f64_4`.");
                }
            },
            pt_f64_5: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `pt_f64_5`.");
                }
            },
            pt_str_1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pt_str_1`.");
                }
            },
            pt_str_2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pt_str_2`.");
                }
            },
            pt_str_3: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pt_str_3`.");
                }
            },
            pt_str_4: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pt_str_4`.");
                }
            },
            pt_str_5: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pt_str_5`.");
                }
            },
            maturity_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `maturity_dt`.");
                }
            },
        };
        Ok(input_account)
    }
}
