use rbdate::*;
use statics::*;

#[derive(Debug)]
pub struct Input {
    pub acc_no: String,
    pub acnts_internal_acnum: String,
    pub acnts_brn_cd: i64,
    pub acnts_client_num: i64,
    pub ucic: i64,
    pub acnts_curr_cd: String,
    pub acnts_prod_cd: i64,
    pub gl_cd: i64,
    pub acnts_opening_dt: Option<NaiveDate>,
    pub bal: f64,
    pub bal_ccy: String,
    pub int_rt: f64,
    pub int_type: String,
    pub int_bench: String,
    pub int_spread: String,
    pub last_reset_dt: Option<NaiveDate>,
    pub next_reset_dt: Option<NaiveDate>,
    pub reset_no_of_months: i64,
    pub disbursal_amount: f64,
    pub last_emi_dt: Option<NaiveDate>,
    pub lm_exp: Option<NaiveDate>,
    pub lim: f64,
    pub lm_ccy: String,
    pub ext_rating_agency: String,
    pub ext_rating: String,
    pub int_rating: String,
    pub asset_cd: String,
    pub prov_amt: f64,
    pub prov_dt: Option<NaiveDate>,
    pub constitn: String,
    pub loan_type: String,
    pub def_amt: f64,
    pub def_dt: Option<NaiveDate>,
    pub last_paid_emi_dt: Option<NaiveDate>,
    pub w4b_cd: i64,
    pub balm_llg: String,
    pub care_llg: String,
    pub ba_llg: String,
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
    pub npa_asset_cd: String,
    pub npa_dt: Option<NaiveDate>,
    pub acc_bal: f64,
    pub pwo: f64,
    pub ho_bal: f64,
    pub npa_prov: f64,
    pub ho_prov: f64,
    pub suspence_bal: f64,
    pub suspence_writeoff: f64,
    pub ho_suspence: f64,
    pub claim: f64,
    pub primary: f64,
    pub col: f64,
    pub priority: String,
    pub main_sector: String,
    pub industry: String,
    pub npa_amt: f64,
    pub schedules: String,
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
    pub org_code: String,
}

impl Input {
    pub fn identifier(&self) -> String {
        format!("{}", self.acc_no)
    }
}

impl Input {
    pub fn new_from_line(line: &str, dmy: &DateParser) -> InputParseResult {
        let mut value_iterator = line.split('|');

        let input = Input {
            acc_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `acc_no`."));
                }
            },
            acnts_internal_acnum: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `acnts_internal_acnum`."
                    ));
                }
            },
            acnts_brn_cd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `acnts_brn_cd`."
                    ));
                }
            },
            acnts_client_num: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `acnts_client_num`."
                    ));
                }
            },
            ucic: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `ucic`."));
                }
            },
            acnts_curr_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `acnts_curr_cd`."
                    ));
                }
            },
            acnts_prod_cd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `acnts_prod_cd`."
                    ));
                }
            },
            gl_cd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `gl_cd`."));
                }
            },
            acnts_opening_dt: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `acnts_opening_dt`."
                    ));
                }
            },
            bal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `bal`."));
                }
            },
            bal_ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `bal_ccy`."));
                }
            },
            int_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `int_rt`."));
                }
            },
            int_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `int_type`."
                    ));
                }
            },
            int_bench: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `int_bench`."
                    ));
                }
            },
            int_spread: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `int_spread`."
                    ));
                }
            },
            last_reset_dt: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `last_reset_dt`."
                    ));
                }
            },
            next_reset_dt: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `next_reset_dt`."
                    ));
                }
            },
            reset_no_of_months: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `reset_no_of_months`."
                    ));
                }
            },
            disbursal_amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `disbursal_amount`."
                    ));
                }
            },
            last_emi_dt: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `last_emi_dt`."
                    ));
                }
            },
            lm_exp: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `lm_exp`."));
                }
            },
            lim: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `lim`."));
                }
            },
            lm_ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `lm_ccy`."));
                }
            },
            ext_rating_agency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `ext_rating_agency`."
                    ));
                }
            },
            ext_rating: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `ext_rating`."
                    ));
                }
            },
            int_rating: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `int_rating`."
                    ));
                }
            },
            asset_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `asset_cd`."
                    ));
                }
            },
            prov_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `prov_amt`."
                    ));
                }
            },
            prov_dt: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `prov_dt`."));
                }
            },
            constitn: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `constitn`."
                    ));
                }
            },
            loan_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `loan_type`."
                    ));
                }
            },
            def_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `def_amt`."));
                }
            },
            def_dt: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `def_dt`."));
                }
            },
            last_paid_emi_dt: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `last_paid_emi_dt`."
                    ));
                }
            },
            w4b_cd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `w4b_cd`."));
                }
            },
            balm_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `balm_llg`."
                    ));
                }
            },
            care_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `care_llg`."
                    ));
                }
            },
            ba_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `ba_llg`."));
                }
            },
            client_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `client_type`."
                    ));
                }
            },
            clients_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `clients_name`."
                    ));
                }
            },
            clients_bsr_type_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `clients_bsr_type_flg`."
                    ));
                }
            },
            clients_busdivn_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `clients_busdivn_code`."
                    ));
                }
            },
            clients_const_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `clients_const_code`."
                    ));
                }
            },
            clients_pan_gir_num: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `clients_pan_gir_num`."
                    ));
                }
            },
            clients_risk_categorization: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `clients_risk_categorization`."
                    ));
                }
            },
            clients_risk_cntry: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `clients_risk_cntry`."
                    ));
                }
            },
            clients_segment_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `clients_segment_code`."
                    ));
                }
            },
            corpcl_orgn_qualifier: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `corpcl_orgn_qualifier`."
                    ));
                }
            },
            corpcl_indus_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `corpcl_indus_code`."
                    ));
                }
            },
            corpcl_nature_of_bus1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `corpcl_nature_of_bus1`."
                    ));
                }
            },
            corpcl_nature_of_bus2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `corpcl_nature_of_bus2`."
                    ));
                }
            },
            corpcl_nature_of_bus3: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `corpcl_nature_of_bus3`."
                    ));
                }
            },
            corpcl_central_state_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `corpcl_central_state_flg`."
                    ));
                }
            },
            corpcl_public_sector_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `corpcl_public_sector_flg`."
                    ));
                }
            },
            corpcl_primary_dlr_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `corpcl_primary_dlr_flg`."
                    ));
                }
            },
            corpcl_multilateral_bank: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `corpcl_multilateral_bank`."
                    ));
                }
            },
            corpcl_connp_inv_num: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `corpcl_connp_inv_num`."
                    ));
                }
            },
            corpcl_bc_gross_turnover: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `corpcl_bc_gross_turnover`."
                    ));
                }
            },
            npa_asset_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `npa_asset_cd`."
                    ));
                }
            },
            npa_dt: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `npa_dt`."));
                }
            },
            acc_bal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `acc_bal`."));
                }
            },
            pwo: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `pwo`."));
                }
            },
            ho_bal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `ho_bal`."));
                }
            },
            npa_prov: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `npa_prov`."
                    ));
                }
            },
            ho_prov: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `ho_prov`."));
                }
            },
            suspence_bal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `suspence_bal`."
                    ));
                }
            },
            suspence_writeoff: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `suspence_writeoff`."
                    ));
                }
            },
            ho_suspence: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `ho_suspence`."
                    ));
                }
            },
            claim: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `claim`."));
                }
            },
            primary: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `primary`."));
                }
            },
            col: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `col`."));
                }
            },
            priority: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `priority`."
                    ));
                }
            },
            main_sector: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `main_sector`."
                    ));
                }
            },
            industry: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `industry`."
                    ));
                }
            },
            npa_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `npa_amt`."));
                }
            },
            schedules: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `schedules`."
                    ));
                }
            },
            sanc_dt: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `sanc_dt`."));
                }
            },
            occp_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `occp_cd`."));
                }
            },
            sens_sec: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `sens_sec`."
                    ));
                }
            },
            prior_subtype: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `prior_subtype`."
                    ));
                }
            },
            restruct_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `restruct_flag`."
                    ));
                }
            },
            restruct_dt: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `restruct_dt`."
                    ));
                }
            },
            mor_prd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `mor_prd`."));
                }
            },
            rating: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `rating`."));
                }
            },
            consitin: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `consitin`."
                    ));
                }
            },
            pan: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `pan`."));
                }
            },
            limit_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `limit_amt`."
                    ));
                }
            },
            gross_adv: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `gross_adv`."
                    ));
                }
            },
            exp_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `exp_amt`."));
                }
            },
            unvail_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `unvail_amt`."
                    ));
                }
            },
            gold_gram: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `gold_gram`."
                    ));
                }
            },
            fund_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `fund_flag`."
                    ));
                }
            },
            ltv_value: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `ltv_value`."
                    ));
                }
            },
            pt_i64_1: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `pt_i64_1`."
                    ));
                }
            },
            pt_i64_2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `pt_i64_2`."
                    ));
                }
            },
            pt_i64_3: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `pt_i64_3`."
                    ));
                }
            },
            pt_i64_4: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `pt_i64_4`."
                    ));
                }
            },
            pt_i64_5: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `pt_i64_5`."
                    ));
                }
            },
            pt_f64_1: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `pt_f64_1`."
                    ));
                }
            },
            pt_f64_2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `pt_f64_2`."
                    ));
                }
            },
            pt_f64_3: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `pt_f64_3`."
                    ));
                }
            },
            pt_f64_4: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `pt_f64_4`."
                    ));
                }
            },
            pt_f64_5: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `pt_f64_5`."
                    ));
                }
            },
            pt_str_1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `pt_str_1`."
                    ));
                }
            },
            pt_str_2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `pt_str_2`."
                    ));
                }
            },
            pt_str_3: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `pt_str_3`."
                    ));
                }
            },
            pt_str_4: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `pt_str_4`."
                    ));
                }
            },
            pt_str_5: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `pt_str_5`."
                    ));
                }
            },
            org_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `org_code`."));
                }
            },
        };
        return InputParseResult::Some(input);
    }
}

pub enum InputParseResult {
    Error(String),
    Some(Input),
}
