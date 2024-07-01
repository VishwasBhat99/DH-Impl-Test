use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub acc_no: String,
    pub branch_cd: String,
    pub cust_no: i64,
    pub ucc_id: i64,
    pub ccy: String,
    pub produ: String,
    pub gl: i64,
    pub open_dt: Option<NaiveDate>,
    pub os_bal: f64,
    pub os_bal_cry: f64,
    pub int_rt: f64,
    pub int_type: String,
    pub int_bm: f64,
    pub spread: f64,
    pub inoperative: String,
    pub int_accrd: f64,
    pub const_cd: String,
    pub dumy: String,
    pub clients_code: String,
    pub client_type: String,
    pub clients_name: String,
    pub clients_bsr_type_flg: String,
    pub clients_busdivn_code: String,
    pub clients_const_code: String,
    pub clients_cust_sub_catg: String,
    pub clients_group_code: String,
    pub clients_pan_gir_num: String,
    pub clients_risk_categorization: String,
    pub clients_risk_cntry: String,
    pub clients_segment_code: String,
    pub corpcl_client_name: String,
    pub corpcl_orgn_qualifier: String,
    pub corpcl_indus_code: String,
    pub corpcl_sub_indus_code: String,
    pub corpcl_nature_of_bus1: String,
    pub corpcl_nature_of_bus2: String,
    pub corpcl_nature_of_bus3: String,
    pub corpcl_scheduled_bank: String,
    pub corpcl_sovereign_flg: String,
    pub corpcl_type_of_sovereign: String,
    pub corpcl_cntry_code: String,
    pub corpcl_central_state_flg: String,
    pub corpcl_public_sector_flg: String,
    pub corpcl_primary_dlr_flg: String,
    pub corpcl_multilateral_bank: String,
    pub corpcl_connp_inv_num: String,
    pub corpcl_bc_gross_turnover: f64,
    pub t1: String,
    pub t2: String,
    pub t3: String,
    pub t4: String,
    pub w4b_cd: String,
    pub balm_llg: String,
    pub care_llg: String,
    pub ba_llg: String,
    pub acc_mt_dt: String,
    pub orginal_dep_amt_lcy: String,
    pub current_outstanding_amt: String,
    pub current_outstanding_amt_lcy: String,
    pub res_tenor: String,
    pub cont_tenor: String,
    pub rep_tenor: String,
    pub comp_freq: String,
    pub cust_cons_code: String,
    pub industry: String,
    pub division: String,
    pub cust_initial_dep_total_amount: String,
    pub cust_total_deposit_amount: String,
    pub is_withdrawable: String,
    pub is_custody_ac: String,
    pub is_clearing_ac: String,
    pub is_cash_managment: String,
    pub is_tax_saving: String,
    pub is_under_lien: String,
    pub is_wealth_mang: String,
    pub pta1: String,
    pub pta2: String,
    pub pta3: String,
    pub pta4: String,
    pub pta5: String,
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
            branch_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `branch_cd`.");
                }
            },
            cust_no: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `cust_no`.");
                }
            },
            ucc_id: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `ucc_id`.");
                }
            },
            ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ccy`.");
                }
            },
            produ: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `produ`.");
                }
            },
            gl: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `gl`.");
                }
            },
            open_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `open_dt`.");
                }
            },
            os_bal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `os_bal`.");
                }
            },
            os_bal_cry: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `os_bal_cry`.");
                }
            },
            int_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_rt`.");
                }
            },
            int_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `int_type`.");
                }
            },
            int_bm: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_bm`.");
                }
            },
            spread: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `spread`.");
                }
            },
            inoperative: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `inoperative`.");
                }
            },
            int_accrd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_accrd`.");
                }
            },
            const_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `const_cd`.");
                }
            },
            dumy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `dumy`.");
                }
            },
            clients_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `clients_code`.");
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
            clients_cust_sub_catg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `clients_cust_sub_catg`.");
                }
            },
            clients_group_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `clients_group_code`.");
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
            corpcl_client_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `corpcl_client_name`.");
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
            corpcl_sub_indus_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `corpcl_sub_indus_code`.");
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
            corpcl_scheduled_bank: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `corpcl_scheduled_bank`.");
                }
            },
            corpcl_sovereign_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `corpcl_sovereign_flg`.");
                }
            },
            corpcl_type_of_sovereign: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `corpcl_type_of_sovereign`.");
                }
            },
            corpcl_cntry_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `corpcl_cntry_code`.");
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
                    return Err("Could not parse property `accorpcl_connp_inv_numc_no`.");
                }
            },
            corpcl_bc_gross_turnover: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `corpcl_bc_gross_turnover`.");
                }
            },
            t1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `t1`.");
                }
            },
            t2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `t2`.");
                }
            },
            t3: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `t3`.");
                }
            },
            t4: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `t4`.");
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
            acc_mt_dt: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acc_mt_dt`.");
                }
            },
            orginal_dep_amt_lcy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `orginal_dep_amt_lcy`.");
                }
            },
            current_outstanding_amt: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `current_outstanding_amt`.");
                }
            },
            current_outstanding_amt_lcy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `current_outstanding_amt_lcy`.");
                }
            },
            res_tenor: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `res_tenor`.");
                }
            },
            cont_tenor: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cont_tenor`.");
                }
            },
            rep_tenor: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rep_tenor`.");
                }
            },
            comp_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `comp_freq`.");
                }
            },
            cust_cons_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acccust_cons_code_no`.");
                }
            },
            industry: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `industry`.");
                }
            },
            division: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `division`.");
                }
            },
            cust_initial_dep_total_amount: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_initial_dep_total_amount`.");
                }
            },
            cust_total_deposit_amount: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_total_deposit_amount`.");
                }
            },
            is_withdrawable: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `is_withdrawable`.");
                }
            },
            is_custody_ac: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `is_custody_ac`.");
                }
            },
            is_clearing_ac: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `is_clearing_ac`.");
                }
            },
            is_cash_managment: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `is_cash_managment`.");
                }
            },
            is_tax_saving: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `is_tax_saving`.");
                }
            },
            is_under_lien: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `is_under_lien`.");
                }
            },
            is_wealth_mang: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `is_wealth_mang`.");
                }
            },
            pta1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pta1`.");
                }
            },
            pta2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pta2`.");
                }
            },
            pta3: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pta3`.");
                }
            },
            pta4: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pta4`.");
                }
            },
            pta5: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pta5`.");
                }
            },
        };
        Ok(input_account)
    }
}
