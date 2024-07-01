use rbdate::DateParser;
use rbdate::NaiveDate;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub acid: String,
    pub foracid: String,
    pub bacid: String,
    pub clr_bal_amt: f64,
    pub un_clr_bal_amt: f64,
    pub sol_id: String,
    pub cust_id: String,
    pub acct_ownership: String,
    pub ledg_num: String,
    pub drwng_power: String,
    pub mode_of_oper_code: String,
    pub lien_amt: f64,
    pub sanct_lim: String,
    pub gl_sub_head_code: String,
    pub schm_code: String,
    pub schm_type: String,
    pub crncy_code: String,
    pub acct_crncy_code: String,
    pub acct_cls_flg: String,
    pub del_flg: String,
    pub acct_opn_date: Option<NaiveDate>,
    pub entity_cre_flg: String,
    pub acct_cls_date: Option<NaiveDate>,
    pub last_tran_date: Option<NaiveDate>,
    pub notional_rate_code: String,
    pub emp_id: String,
    pub notional_rate: f64,
    pub limit_b2kid: String,
    pub adim1_gam: String,
    pub adim2_gam: i64,
    pub adim3_gam: i64,
    pub int_rate: f64,
    pub bm_id: String,
    pub spread: f64,
    pub reprice_freq: String,
    pub last_reprice_dt: Option<NaiveDate>,
    pub next_reprice_dt: Option<NaiveDate>,
    pub code1: String,
    pub code2: String,
    pub code3: String,
    pub code4: String,
    pub adim1_gac: String,
    pub adim2_gac: String,
    pub adim3_gac: String,
    pub cust_name: String,
    pub cmg_pan_gir_num: String,
    pub cmg_cust_const: String,
    pub adim1_cmg: String,
    pub adim2_cmg: String,
    pub adim3_cmg: String,
    pub out_bal_amt: f64,
    pub cust_grp_id: String,
    pub ucif_cust_const: String,
    pub exch_rt: String,
    pub out_bal_amt_con: f64,
    pub segment_code: String,
    pub nfs: String,
    pub oth_del_flg: String,
    pub open_effective_date: Option<NaiveDate>,
    pub oth_schm_type: String,
    pub int_tbl_code: String,
    pub int_version: i64,
    pub int_tbl_ver_num: i64,
    pub min_int_pcnt_cr: f64,
    pub max_int_pcnt_cr: f64,
    pub cust_cr_pref_pcnt: f64,
    pub id_cr_pref_pcnt: f64,
    pub nrml_int_pcnt: f64,
    pub id_dr_pref_pcnt: f64,
    pub base_int_tbl_code: String,
    pub base_pcnt_dr: f64,
    pub base_pcnt_cr: f64,
    pub base_pcnt: f64,
    pub deposit_period_mths: i64,
    pub deposit_period_days: i64,
    pub deposit_amount: f64,
    pub oth_acct_crncy_code: String,
    pub deposit_type: String,
    pub spl_catg_ind: String,
    pub nrml_int_pcnt_cr: f64,
    pub base_differential_exists: String,
    pub deposit_status: String,
    pub maturity_amount: f64,
    pub maturity_date: Option<NaiveDate>,
    pub rcre_time: Option<NaiveDate>,
    pub auto_renewed_counter: String,
    pub overdue_flg: String,
    pub final_int_rate: f64,
    pub waiver: String,
    pub maturity_modify: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            acid: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acid`.");
                }
            },
            foracid: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `foracid`.");
                }
            },
            bacid: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bacid`.");
                }
            },
            clr_bal_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `clr_bal_Amt`.");
                }
            },
            un_clr_bal_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `un_clr_bal_amt`.");
                }
            },
            sol_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sol_id`.");
                }
            },
            cust_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_id`.");
                }
            },
            acct_ownership: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acct_ownership`.");
                }
            },
            ledg_num: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ledg_num`.");
                }
            },
            drwng_power: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `drwng_power`.");
                }
            },
            mode_of_oper_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `mode_of_oper_code`.");
                }
            },
            lien_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `lien_amt`.");
                }
            },
            sanct_lim: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sanct_lim`.");
                }
            },
            gl_sub_head_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `gl_sub_head_code`.");
                }
            },
            schm_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `schm_code`.");
                }
            },
            schm_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `schm_type`.");
                }
            },
            crncy_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `crncy_code`.");
                }
            },
            acct_crncy_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acct_cuurency_code`.");
                }
            },
            acct_cls_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acct_cls_flg`.");
                }
            },
            del_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `del_flg`.");
                }
            },
            acct_opn_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `acc_open_dt`.");
                }
            },
            entity_cre_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `entity_cre_flg`.");
                }
            },
            acct_cls_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `acct_cls_date`.");
                }
            },
            last_tran_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `last_tran_date`.");
                }
            },
            notional_rate_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `notional_rate_code`.");
                }
            },
            emp_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `emp_id`.");
                }
            },
            notional_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `notional_rate`.");
                }
            },
            limit_b2kid: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `limit_b2kid`.");
                }
            },
            adim1_gam: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `adim1_gam`.");
                }
            },
            adim2_gam: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return Err("Could not parse property `adim2_gam`.");
                }
            },
            adim3_gam: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return Err("Could not parse property `adim3_gam`.");
                }
            },
            int_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `int_rt`.");
                }
            },
            bm_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bm_id`.");
                }
            },
            spread: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `spread`.");
                }
            },
            reprice_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `reprice_freq`.");
                }
            },
            last_reprice_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `last_reprice_dt`.");
                }
            },
            next_reprice_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `next_reprice_dt`.");
                }
            },
            code1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `code1`.");
                }
            },
            code2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `code2`.");
                }
            },
            code3: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `code3`.");
                }
            },
            code4: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `code4`.");
                }
            },
            adim1_gac: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `adim1_gac`.");
                }
            },
            adim2_gac: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `adim2_gac`.");
                }
            },
            adim3_gac: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `adim3_gac`.");
                }
            },
            cust_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_name`.");
                }
            },
            cmg_pan_gir_num: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cmg_pan_gir_num`.");
                }
            },
            cmg_cust_const: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cmg_cust_const`.");
                }
            },
            adim1_cmg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `adm1_cmg`.");
                }
            },
            adim2_cmg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `adim2_cmg`.");
                }
            },
            adim3_cmg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `adim3_cmg`.");
                }
            },
            out_bal_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `out_bal_amt`.");
                }
            },
            cust_grp_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_grp_id`.");
                }
            },
            ucif_cust_const: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ucif_cust_const`.");
                }
            },
            exch_rt: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `exch_rt`.");
                }
            },
            out_bal_amt_con: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `out_bal_amt_con`.");
                }
            },
            segment_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `segment_code`.");
                }
            },
            nfs: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `nfs`.");
                }
            },
            oth_del_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `other_del_flg`.");
                }
            },
            open_effective_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `open_effective_dt`.");
                }
            },
            oth_schm_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `oth_schm_type`.");
                }
            },
            int_tbl_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `int_tbl_code`.");
                }
            },
            int_version: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return Err("Could not parse property `lim`.");
                }
            },
            int_tbl_ver_num: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return Err("Could not parse property `int_version`.");
                }
            },
            min_int_pcnt_cr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `min_int_pcnt_cr`.");
                }
            },
            max_int_pcnt_cr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `max_int_pcnt_cr`.");
                }
            },
            cust_cr_pref_pcnt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `cust_cr_pref_pcnt`.");
                }
            },
            id_cr_pref_pcnt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `id_cr_pref_pcnt`.");
                }
            },
            nrml_int_pcnt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `nrml_int_pcnt`.");
                }
            },
            id_dr_pref_pcnt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `id_pref_pcnt`.");
                }
            },
            base_int_tbl_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `base_int_tbl_code`.");
                }
            },
            base_pcnt_dr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `base_pcnt_dr`.");
                }
            },
            base_pcnt_cr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `base_pcnt_cr`.");
                }
            },
            base_pcnt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `base_pcnt`.");
                }
            },
            deposit_period_mths: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return Err("Could not parse property `deposit_period_mths`.");
                }
            },
            deposit_period_days: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return Err("Could not parse property `deposit_period_days`.");
                }
            },
            deposit_amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `deposit_amount`.");
                }
            },
            oth_acct_crncy_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `oth_acct_crncy_code`.");
                }
            },
            deposit_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `deposit_type`.");
                }
            },
            spl_catg_ind: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `spl_catg_ind`.");
                }
            },
            nrml_int_pcnt_cr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `nrml_int_pcnt_cr`.");
                }
            },
            base_differential_exists: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `base_differential_exists`.");
                }
            },
            deposit_status: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `deposit_status`.");
                }
            },
            maturity_amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `maturity_amount`.");
                }
            },
            maturity_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `maturity_date`.");
                }
            },
            rcre_time: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `rcre_time`.");
                }
            },
            auto_renewed_counter: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `auto_renewed_counter`.");
                }
            },
            overdue_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `overdue_flg`.");
                }
            },
            final_int_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `final_int_rate`.");
                }
            },
            waiver: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `waiver`.");
                }
            },
            maturity_modify: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `maturity_modify`.");
                }
            },
        };
        Ok(input_account)
    }
}
