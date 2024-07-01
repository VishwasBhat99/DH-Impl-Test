use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::{DEFAULT_FLOAT, DEFAULT_INT};

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub acc_no: String,
    pub acnts_internal_acnum: String,
    pub acnts_brn_code: String,
    pub acnts_client_num: String,
    pub ucic: String,
    pub acnts_curr_code: String,
    pub acnts_prod_code: String,
    pub gl_cd: String,
    pub acc_open_dt: Option<NaiveDate>,
    pub bal: f64,
    pub balccy: f64,
    pub int_rate: f64,
    pub int_type: String,
    pub int_bench: String,
    pub int_spread: String,
    pub last_reset_dt: Option<NaiveDate>,
    pub next_reset_dt: Option<NaiveDate>,
    pub reset_no_of_months: String,
    pub int_accrued_amt: f64,
    pub constitn: String,
    pub lm_exp: Option<NaiveDate>,
    pub lim: f64,
    pub lm_ccy: String,
    pub ext_rating_agency: String,
    pub ext_rating: String,
    pub int_rating: String,
    pub asset_cd: String,
    pub provision_amt: f64,
    pub prov_dt: Option<NaiveDate>,
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
    pub w4b_cd: String,
    pub balm_llg: String,
    pub care_llg: String,
    pub ba_llg: String,
    pub asset_code: String,
    pub npa_dt: Option<NaiveDate>,
    pub account_balance: f64,
    pub pwo: f64,
    pub written_off_dt: Option<NaiveDate>,
    pub ho_balance: f64,
    pub npa_provision: f64,
    pub ho_provision: f64,
    pub suspencebalance: f64,
    pub suspence_writeoff: f64,
    pub ho_suspence: f64,
    pub claim: f64,
    pub primary: f64,
    pub collateral: f64,
    pub total_security: f64,
    pub primary_valuation_dt: Option<NaiveDate>,
    pub collateral_valuation_dt: Option<NaiveDate>,
    pub gold_deficit: f64,
    pub fraud: f64,
    pub wilful_default: f64,
    pub subsidy: f64,
    pub priority: String,
    pub priority_type: String,
    pub main_sector: String,
    pub sub_sector: String,
    pub activity: String,
    pub industry: String,
    pub categoryofborrower: String,
    pub org_gl_head: String,
    pub npa_amt: f64,
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
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            acc_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acc_no`.");
                }
            },
            acnts_internal_acnum: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acnts_internal_acnum`.");
                }
            },
            acnts_brn_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acnts_brn_code`.");
                }
            },
            acnts_client_num: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acnts_client_num`.");
                }
            },
            ucic: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ucic`.");
                }
            },
            acnts_curr_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acnts_curr_code`.");
                }
            },
            acnts_prod_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acnts_prod_code`.");
                }
            },
            gl_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `gl_cd`.");
                }
            },
            acc_open_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `acc_open_dt`.");
                }
            },
            bal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `bal`.");
                }
            },
            balccy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `balccy`.");
                }
            },
            int_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_rate`.");
                }
            },
            int_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `int_type`.");
                }
            },
            int_bench: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `int_bench`.");
                }
            },
            int_spread: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `int_spread`.");
                }
            },
            last_reset_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `last_reset_dt`.");
                }
            },
            next_reset_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `next_reset_dt`.");
                }
            },
            reset_no_of_months: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `reset_no_of_months`.");
                }
            },
            int_accrued_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_accrued_amt`.");
                }
            },
            constitn: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `constitn`.");
                }
            },
            lm_exp: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `lm_exp`.");
                }
            },
            lim: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `lim`.");
                }
            },
            lm_ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `lm_ccy`.");
                }
            },
            ext_rating_agency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ext_rating_agency`.");
                }
            },
            ext_rating: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ext_rating`.");
                }
            },
            int_rating: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `int_rating`.");
                }
            },
            asset_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `asset_cd`.");
                }
            },
            provision_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `provision_amt`.");
                }
            },
            prov_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `prov_dt`.");
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
            w4b_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `w4b_cd`.");
                }
            },
            balm_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `balm_llg`.");
                }
            },
            care_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `care_llg`.");
                }
            },
            ba_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ba_llg`.");
                }
            },
            asset_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `asset_code`.");
                }
            },
            npa_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `npa_dt`.");
                }
            },
            account_balance: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `account_balance`.");
                }
            },
            pwo: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `pwo`.");
                }
            },
            written_off_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `written_off_dt`.");
                }
            },
            ho_balance: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `ho_balance`.");
                }
            },
            npa_provision: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `npa_provision`.");
                }
            },
            ho_provision: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `ho_provision`.");
                }
            },
            suspencebalance: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `suspencebalance`.");
                }
            },
            suspence_writeoff: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `suspence_writeoff`.");
                }
            },
            ho_suspence: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `ho_suspence`.");
                }
            },
            claim: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `claim`.");
                }
            },
            primary: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `primary`.");
                }
            },
            collateral: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `collateral`.");
                }
            },
            total_security: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `total_security`.");
                }
            },
            primary_valuation_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `primary_valuation_dt`.");
                }
            },
            collateral_valuation_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `collateral_valuation_dt`.");
                }
            },

            gold_deficit: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `gold_deficit`.");
                }
            },
            fraud: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `fraud`.");
                }
            },
            wilful_default: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `wilful_default`.");
                }
            },
            subsidy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `subsidy`.");
                }
            },
            priority: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `priority`.");
                }
            },
            priority_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `priority_type`.");
                }
            },
            main_sector: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `main_sector`.");
                }
            },
            sub_sector: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sub_sector`.");
                }
            },
            activity: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `activity`.");
                }
            },
            industry: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `industry`.");
                }
            },
            categoryofborrower: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `categoryofborrower`.");
                }
            },
            org_gl_head: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `org_gl_head`.");
                }
            },
            npa_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `npa_amt`.");
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
        };
        Ok(input_account)
    }
}
