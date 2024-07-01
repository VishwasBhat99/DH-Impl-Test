use crate::configuration_parameters::ConfigurationParameters;
use rbdate::NaiveDate;

#[derive(Debug, Clone, Default)]
pub struct GAMData {
    pub acid: String,
    pub foracid: String,
    pub bacid: String,
    pub clr_bal_amt: f64,
    pub un_clr_bal_amt: f64,
    pub sol_id: String,
    pub cust_id: String,
    pub acct_ownership: String,
    pub ledg_num: String,
    pub drwng_power: f64,
    pub mode_of_oper_code: String,
    pub lien_amt: f64,
    pub sanct_lim: f64,
    pub gl_sub_head_code: String,
    pub schm_code: String,
    pub schm_type: String,
    pub crncy_code: String,
    pub acct_crncy_code: String,
    pub acct_cls_flg: String,
    pub del_flg: String,
    pub acct_open_date: NaiveDate,
    pub entity_cre_flag: String,
    pub acct_cls_date: NaiveDate,
    pub last_tran_date: NaiveDate,
    pub notional_rate_code: String,
    pub emp_id: String,
    pub notional_rate: f64,
    pub limit_b2kid: String,
    pub adim1_gam: String,
    pub adim2_gam: String,
    pub adim3_gam: String,
    pub int_rate: f64,
    pub bm_id: String,
    pub spread: String,
    pub reprice_freq: String,
    pub last_reprice_date: NaiveDate,
    pub next_reprice_date: NaiveDate,
    pub code1: String,
    pub code2: String,
    pub code3: String,
    pub code4: String,
    pub adim1_gac: String,
    pub adim2_gac: String,
    pub adim3_gac: String,
    pub cust_name: String,
    pub pan_gir_num: String,
    pub cust_const: String,
    pub adim1_cmg: String,
    pub llg: String,
    pub adim3_cmg: String,
    pub staff_schm_flg: String,
    pub cust_grp_id: String,
    pub ucif_cust_const: String,
    pub exchg_rate: f64,
    pub out_bal_amt_con: f64,
    pub segment_code: String,
    pub nfs: String,
}

impl GAMData {
    pub fn new(
        config_params: &ConfigurationParameters,
        input_file: &str,
        input_acc: &[&str],
        row: usize,
    ) -> GAMData {
        GAMData {
            acid: get_str(input_file, input_acc, 0, row),
            foracid: get_str(input_file, input_acc, 1, row),
            bacid: get_str(input_file, input_acc, 2, row),
            clr_bal_amt: get_str(input_file, input_acc, 3, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            un_clr_bal_amt: get_str(input_file, input_acc, 4, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            sol_id: get_str(input_file, input_acc, 5, row),
            cust_id: get_str(input_file, input_acc, 6, row),
            acct_ownership: get_str(input_file, input_acc, 7, row),
            ledg_num: get_str(input_file, input_acc, 8, row),
            drwng_power: get_str(input_file, input_acc, 9, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            mode_of_oper_code: get_str(input_file, input_acc, 10, row),
            lien_amt: get_str(input_file, input_acc, 11, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            sanct_lim: get_str(input_file, input_acc, 12, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            gl_sub_head_code: get_str(input_file, input_acc, 13, row),
            schm_code: get_str(input_file, input_acc, 14, row),
            schm_type: get_str(input_file, input_acc, 15, row),
            crncy_code: get_str(input_file, input_acc, 16, row),
            acct_crncy_code: get_str(input_file, input_acc, 17, row),
            acct_cls_flg: get_str(input_file, input_acc, 18, row),
            del_flg: get_str(input_file, input_acc, 19, row),
            acct_open_date: get_date(config_params, input_file, input_acc, 20, row),
            entity_cre_flag: get_str(input_file, input_acc, 21, row),
            acct_cls_date: get_date(config_params, input_file, input_acc, 22, row),
            last_tran_date: get_date(config_params, input_file, input_acc, 23, row),
            notional_rate_code: get_str(input_file, input_acc, 24, row),
            emp_id: get_str(input_file, input_acc, 25, row),
            notional_rate: get_str(input_file, input_acc, 26, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            limit_b2kid: get_str(input_file, input_acc, 27, row),
            adim1_gam: get_str(input_file, input_acc, 28, row),
            adim2_gam: get_str(input_file, input_acc, 29, row),
            adim3_gam: get_str(input_file, input_acc, 30, row),
            int_rate: get_str(input_file, input_acc, 31, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            bm_id: get_str(input_file, input_acc, 32, row),
            spread: get_str(input_file, input_acc, 33, row),
            reprice_freq: get_str(input_file, input_acc, 34, row),
            last_reprice_date: get_date(config_params, input_file, input_acc, 35, row),
            next_reprice_date: get_date(config_params, input_file, input_acc, 36, row),
            code1: get_str(input_file, input_acc, 37, row),
            code2: get_str(input_file, input_acc, 38, row),
            code3: get_str(input_file, input_acc, 39, row),
            code4: get_str(input_file, input_acc, 40, row),
            adim1_gac: get_str(input_file, input_acc, 41, row),
            adim2_gac: get_str(input_file, input_acc, 42, row),
            adim3_gac: get_str(input_file, input_acc, 43, row),
            cust_name: get_str(input_file, input_acc, 44, row),
            pan_gir_num: get_str(input_file, input_acc, 45, row),
            cust_const: get_str(input_file, input_acc, 46, row),
            adim1_cmg: get_str(input_file, input_acc, 47, row),
            llg: get_str(input_file, input_acc, 48, row),
            adim3_cmg: get_str(input_file, input_acc, 49, row),
            staff_schm_flg: get_str(input_file, input_acc, 50, row),
            cust_grp_id: get_str(input_file, input_acc, 51, row),
            ucif_cust_const: get_str(input_file, input_acc, 52, row),
            exchg_rate: get_str(input_file, input_acc, 53, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            out_bal_amt_con: get_str(input_file, input_acc, 54, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            segment_code: get_str(input_file, input_acc, 55, row),
            nfs: get_str(input_file, input_acc, 56, row),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct LAMData {
    pub acid: String,
    pub entity_cre_flg: String,
    pub del_flg: String,
    pub rep_schdl_num: i64,
    pub dis_schdl_num: i64,
    pub dis_schdl_date: NaiveDate,
    pub dis_amt: f64,
    pub pdmd_ovdu_perd_mths: i64,
    pub pdmd_ovdu_perd_days: i64,
    pub pdmd_ovdu_eom_flg: String,
    pub idmd_ovdu_perd_mths: i64,
    pub idmd_ovdu_perd_days: i64,
    pub idmd_ovdu_eom_flg: String,
    pub rep_schdl_date: NaiveDate,
    pub rephasement_principal: f64,
    pub oflow_amt: f64,
    pub ei_perd_end_date: NaiveDate,
    pub loan_type: String,
    pub ei_schm_flg: String,
}

impl LAMData {
    pub fn new(
        config_params: &ConfigurationParameters,
        input_file: &str,
        input_acc: &[&str],
        row: usize,
    ) -> LAMData {
        LAMData {
            acid: get_str(input_file, input_acc, 0, row),
            entity_cre_flg: get_str(input_file, input_acc, 1, row),
            del_flg: get_str(input_file, input_acc, 2, row),
            rep_schdl_num: get_str(input_file, input_acc, 3, row)
                .parse::<i64>()
                .unwrap_or(0),
            dis_schdl_num: get_str(input_file, input_acc, 4, row)
                .parse::<i64>()
                .unwrap_or(0),
            dis_schdl_date: get_date(config_params, input_file, input_acc, 5, row),
            dis_amt: get_str(input_file, input_acc, 6, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            pdmd_ovdu_perd_mths: get_str(input_file, input_acc, 7, row)
                .parse::<i64>()
                .unwrap_or(0),
            pdmd_ovdu_perd_days: get_str(input_file, input_acc, 8, row)
                .parse::<i64>()
                .unwrap_or(0),
            pdmd_ovdu_eom_flg: get_str(input_file, input_acc, 9, row),
            idmd_ovdu_perd_mths: get_str(input_file, input_acc, 10, row)
                .parse::<i64>()
                .unwrap_or(0),
            idmd_ovdu_perd_days: get_str(input_file, input_acc, 11, row)
                .parse::<i64>()
                .unwrap_or(0),
            idmd_ovdu_eom_flg: get_str(input_file, input_acc, 12, row),
            rep_schdl_date: get_date(config_params, input_file, input_acc, 13, row),
            rephasement_principal: get_str(input_file, input_acc, 14, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            oflow_amt: get_str(input_file, input_acc, 15, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            ei_perd_end_date: get_date(config_params, input_file, input_acc, 16, row),
            loan_type: get_str(input_file, input_acc, 17, row),
            ei_schm_flg: get_str(input_file, input_acc, 18, row),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct EITData {
    pub entity_id: String,
    pub account_pegged_flag: String,
    pub interest_rate: f64,
    pub next_peg_review_date: NaiveDate,
    pub nrml_booked_amt_dr: f64,
    pub penal_booked_amt_dr: f64,
    pub nrml_interest_amt_dr: f64,
    pub penal_interest_amt_dr: f64,
    pub next_pidmd_adj_amt: f64,
}

impl EITData {
    pub fn new(
        config_params: &ConfigurationParameters,
        input_file: &str,
        input_acc: &[&str],
        row: usize,
    ) -> EITData {
        EITData {
            entity_id: get_str(input_file, input_acc, 0, row),
            account_pegged_flag: get_str(input_file, input_acc, 43, row),
            interest_rate: get_str(input_file, input_acc, 9, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            next_peg_review_date: get_date(config_params, input_file, input_acc, 49, row),
            nrml_booked_amt_dr: get_str(input_file, input_acc, 15, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            penal_booked_amt_dr: get_str(input_file, input_acc, 25, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            nrml_interest_amt_dr: get_str(input_file, input_acc, 18, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            penal_interest_amt_dr: get_str(input_file, input_acc, 26, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            next_pidmd_adj_amt: get_str(input_file, input_acc, 47, row)
                .parse::<f64>()
                .unwrap_or(0.0),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct RateCodeMaster {
    pub int_rate_code: String,
    pub fixed_floating: String,
    pub benchmark: String,
}

impl RateCodeMaster {
    pub fn new(input_file: &str, input_acc: &[&str], row: usize) -> RateCodeMaster {
        RateCodeMaster {
            int_rate_code: get_str(input_file, input_acc, 0, row),
            fixed_floating: get_str(input_file, input_acc, 2, row),
            benchmark: get_str(input_file, input_acc, 3, row),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ACHData {
    pub srl_num: String,
    pub main_classification_user: String,
    pub sub_classification_user: String,
    pub user_classification_date: NaiveDate,
    pub b2k_type: String,
    pub b2k_id: String,
}

impl ACHData {
    pub fn new(
        config_params: &ConfigurationParameters,
        input_file: &str,
        input_acc: &[&str],
        row: usize,
    ) -> ACHData {
        ACHData {
            srl_num: get_str(input_file, input_acc, 0, row),
            main_classification_user: get_str(input_file, input_acc, 1, row),
            sub_classification_user: get_str(input_file, input_acc, 2, row),
            user_classification_date: get_date(config_params, input_file, input_acc, 3, row),
            b2k_type: get_str(input_file, input_acc, 4, row),
            b2k_id: get_str(input_file, input_acc, 5, row),
        }
    }
}

pub fn get_str(input_file: &str, data: &[&str], index: usize, row: usize) -> String {
    data.get(index)
        .unwrap_or_else(|| {
            panic!(
                "Could not get data at column-no: `{}` in row-no: `{:?}` from File: {}",
                index + 1,
                row,
                input_file,
            )
        })
        .trim()
        .trim_matches(|pat| pat == ' ' || pat == '"')
        .to_string()
}

pub fn get_date(
    config_params: &ConfigurationParameters,
    input_file: &str,
    data: &[&str],
    index: usize,
    row: usize,
) -> NaiveDate {
    let date_parser = rbdate::DateParser::new("%d-%m-%Y".to_string(), false);
    date_parser
        .parse_opt(
            &data
                .get(index)
                .unwrap_or_else(|| {
                    panic!(
                        "Could not get data at column-no: `{}` in row-no: `{:?}` from File: {}",
                        index + 1,
                        row,
                        input_file,
                    )
                })
                .replace('.', ""),
        )
        .unwrap_or(*config_params.as_on_date())
}

#[derive(Debug, Clone, Default)]
pub struct NPAData {
    pub npa_classification: String,
    pub cust_hlth_code: String,
    pub cust_npa_class: String,
    pub final_npa_class: String,
    pub npa_amount: String,
}
impl NPAData {
    pub fn new() -> NPAData {
        NPAData {
            npa_classification: "".to_string(),
            cust_hlth_code: "".to_string(),
            cust_npa_class: "".to_string(),
            final_npa_class: "".to_string(),
            npa_amount: "0.0".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ITCData {
    pub entity_id: String,
    pub int_tbl_code: String,
    pub int_tbl_code_srl_num: i64,
    pub peg_review_date: NaiveDate,
}

#[derive(Debug, Clone, Default)]
pub struct LOANFILEData {
    pub foracid: String,
    pub acid: String,
    pub cust_id: String,
    pub gnt_type: String,
    pub status_code: String,
    pub occupation_code: String,
    pub sector: String,
    pub sector_code: String,
    pub subsector_code: String,
    pub staffflag: String,
    pub cre_free_text_1: String,
    pub pres_val_sec: f64,
    pub paripassu_perc: f64,
    pub prov_perc: f64,
    pub dumm1: f64,
    pub dumm2: f64,
    pub dumm3: f64,
    pub dumm4: f64,
    pub dumm5: String,
    pub dumm6: String,
    pub dumm7: String,
    pub dumm8: String,
    pub dumm9: NaiveDate,
    pub dumm10: NaiveDate,
    pub constcatgorycode: String,
    pub ratingagc: String,
    pub rating: String,
    pub supperannuation_flag: String,
    pub turn_amt1: f64,
    pub turn_amt2: f64,
    pub turn_amt3: f64,
    pub ftp_char1: String,
    pub ftp_char2: String,
    pub ftp_amt1: f64,
    pub ftp_amt2: f64,
    pub ftp_date1: NaiveDate,
    pub ftp_date2: NaiveDate,
}

impl LOANFILEData {
    pub fn new(
        config_params: &ConfigurationParameters,
        input_file: &str,
        input_acc: &[&str],
        row: usize,
    ) -> LOANFILEData {
        LOANFILEData {
            foracid: get_str(input_file, input_acc, 0, row),
            acid: get_str(input_file, input_acc, 1, row),
            cust_id: get_str(input_file, input_acc, 2, row),
            gnt_type: get_str(input_file, input_acc, 3, row),
            status_code: get_str(input_file, input_acc, 4, row),
            occupation_code: get_str(input_file, input_acc, 5, row),
            sector: get_str(input_file, input_acc, 6, row),
            sector_code: get_str(input_file, input_acc, 7, row),
            subsector_code: get_str(input_file, input_acc, 8, row),
            staffflag: get_str(input_file, input_acc, 9, row),
            cre_free_text_1: get_str(input_file, input_acc, 10, row),
            pres_val_sec: get_str(input_file, input_acc, 11, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            paripassu_perc: get_str(input_file, input_acc, 12, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            prov_perc: get_str(input_file, input_acc, 13, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            dumm1: get_str(input_file, input_acc, 14, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            dumm2: get_str(input_file, input_acc, 15, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            dumm3: get_str(input_file, input_acc, 16, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            dumm4: get_str(input_file, input_acc, 17, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            dumm5: get_str(input_file, input_acc, 18, row),
            dumm6: get_str(input_file, input_acc, 19, row),
            dumm7: get_str(input_file, input_acc, 20, row),
            dumm8: get_str(input_file, input_acc, 21, row),
            dumm9: get_date(config_params, input_file, input_acc, 22, row),
            dumm10: get_date(config_params, input_file, input_acc, 23, row),
            constcatgorycode: get_str(input_file, input_acc, 24, row),
            ratingagc: get_str(input_file, input_acc, 25, row),
            rating: get_str(input_file, input_acc, 26, row),
            supperannuation_flag: get_str(input_file, input_acc, 27, row),
            turn_amt1: get_str(input_file, input_acc, 28, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            turn_amt2: get_str(input_file, input_acc, 29, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            turn_amt3: get_str(input_file, input_acc, 30, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            ftp_char1: get_str(input_file, input_acc, 31, row),
            ftp_char2: get_str(input_file, input_acc, 32, row),
            ftp_amt1: get_str(input_file, input_acc, 33, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            ftp_amt2: get_str(input_file, input_acc, 34, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            ftp_date1: get_date(config_params, input_file, input_acc, 35, row),
            ftp_date2: get_date(config_params, input_file, input_acc, 36, row),
        }
    }
}
