use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

#[derive(Debug)]
pub struct InputAccount {
    pub acc_no: String,
    pub br_cd: i64,
    pub cust_id: i64,
    pub ucic_id: i64,
    pub ccy: String,
    pub prod_cd: i64,
    pub gl_cd: i64,
    pub gl_comp_portion: String,
    pub acc_open_dt: NaiveDate,
    pub effc_dt: Option<NaiveDate>,
    pub bal_os: f64,
    pub bal_os_cly: f64,
    pub int_comp_type: String,
    pub compo_int_amt: f64,
    pub int_rt: f64,
    pub mat_dt: NaiveDate,
    pub dep_amt: f64,
    pub dep_amt_lcy: f64,
    pub int_amt: f64,
    pub int_acc_amt: f64,
    pub non_with_flag: String,
    pub notice_day: String,
    pub cust_const_code: i64,
    pub cntrct_num: i64,
    pub as_on: Option<NaiveDate>,
    pub comp_freq: i64,
    pub pay_freq: i64,
    pub over_dt: Option<NaiveDate>,
    pub resid_days: i64,
    pub cntrct_days: i64,
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
    pub w4b_cd: i64,
    pub balm_llg: String,
    pub care_llg: String,
    pub ba_llg: String,
    pub res_tenor: String,
    pub cont_tenor: String,
    pub rep_tenor: String,
    pub cust_cons_code: String,
    pub industry: String,
    pub division: String,
    pub cust_initial_dep_total_amount: String,
    pub cust_total_deposit_amount: String,
    pub is_with_drawable: String,
    pub is_custody_ac: String,
    pub is_clearing_ac: String,
    pub is_cash_managment: String,
    pub is_tax_saving: String,
    pub is_under_lien: String,
    pub is_wealth_mang: String,
    pub pta_1: String,
    pub pta_2: String,
    pub pta_3: String,
    pub pta_4: String,
    pub pta_5: String,
}

impl InputAccount {
    pub fn new_from_line<'a>(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &str> {
        let mut value_iterator = line.split('|');

        let input_account = InputAccount {
            acc_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `acc_no`.");
                }
            },
            br_cd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `br_cd`.");
                }
            },
            cust_id: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `cust_id`.");
                }
            },
            ucic_id: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `ucic_id`.");
                }
            },
            ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `ccy`.");
                }
            },
            prod_cd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `prod_cd`.");
                }
            },
            gl_cd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `gl_cd`.");
                }
            },
            gl_comp_portion: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `gl_comp_portion`.");
                }
            },
            acc_open_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse(val),
                None => {
                    return Err("Could not read property `acc_open_dt`.");
                }
            },
            effc_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not read property `effc_dt`.");
                }
            },
            bal_os: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `bal_os`.");
                }
            },
            bal_os_cly: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `bal_os_cly`.");
                }
            },
            int_comp_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `int_comp_type`.");
                }
            },
            compo_int_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `compo_int_amt`.");
                }
            },
            int_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `int_rt`.");
                }
            },
            mat_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse(val),
                None => {
                    return Err("Could not read property `mat_dt`.");
                }
            },
            dep_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `dep_amt`.");
                }
            },
            dep_amt_lcy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `dep_amt_lcy`.");
                }
            },
            int_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `int_amt`.");
                }
            },
            int_acc_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `int_acc_amt`.");
                }
            },
            non_with_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `non_with_flag`.");
                }
            },
            notice_day: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `notice_day`.");
                }
            },
            cust_const_code: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `cust_const_code`.");
                }
            },
            cntrct_num: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `cntrct_num`.");
                }
            },
            as_on: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not read property `as_on`.");
                }
            },
            comp_freq: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `comp_freq`.");
                }
            },
            pay_freq: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `pay_freq`.");
                }
            },
            over_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not read property `as_on`.");
                }
            },
            resid_days: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `resid_days`.");
                }
            },
            cntrct_days: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `cntrct_days`.");
                }
            },
            dumy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `dumy`.");
                }
            },
            clients_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `clients_code`.");
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
                    return Err("Could not parse property `corpcl_connp_inv_num`.");
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
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `w4b_cd`.");
                }
            },
            balm_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `balm_llg`.");
                }
            },
            care_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `care_llg`.");
                }
            },
            ba_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `ba_llg`.");
                }
            },
            res_tenor: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `res_tenor`.");
                }
            },
            cont_tenor: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `cont_tenor`.");
                }
            },
            rep_tenor: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `rep_tenor`.");
                }
            },
            cust_cons_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `cust_cons_code`.");
                }
            },
            industry: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `industry`.");
                }
            },
            division: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `division`.");
                }
            },
            cust_initial_dep_total_amount: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `cust_initial_dep_total_amount`.");
                }
            },
            cust_total_deposit_amount: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `cust_total_deposit_amount`.");
                }
            },
            is_with_drawable: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `is_with_drawable`.");
                }
            },
            is_custody_ac: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `is_custody_ac`.");
                }
            },
            is_clearing_ac: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `is_clearing_ac`.");
                }
            },
            is_cash_managment: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `is_cash_managment`.");
                }
            },
            is_tax_saving: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `is_tax_saving`.");
                }
            },
            is_under_lien: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `is_under_lien`.");
                }
            },
            is_wealth_mang: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `is_wealth_mang`.");
                }
            },
            pta_1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `pta_1`.");
                }
            },
            pta_2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `pta_2`.");
                }
            },
            pta_3: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `pta_3`.");
                }
            },
            pta_4: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `pta_4`.");
                }
            },
            pta_5: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `pta_5`.");
                }
            },
        };

        Ok(input_account)
    }
}
