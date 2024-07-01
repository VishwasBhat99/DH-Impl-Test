use super::{
    input_account::{ACHData, EITData, GAMData, LAMData, LOANFILEData, NPAData, RateCodeMaster},
    ITCData,
};
use crate::{configuration_parameters::ConfigurationParameters, macros};
use chrono::Duration;
use rbdate::{incr_dt_by_mon_presrv_eom_checked, num_days_start_to_end, NaiveDate};
use slog::Logger;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct OutputAccount {
    pub acid: String,
    pub foracid: String,
    pub sol_id: String,
    pub acct_opn_date: NaiveDate,
    pub gl_sub_head_code: String,
    pub schm_code: String,
    pub schm_type: String,
    pub acct_crncy_code: String,
    pub rep_shdl_num: i64,
    pub rep_shdl_date: NaiveDate,
    pub dis_shdl_num: i64,
    pub next_repricing_date: NaiveDate,
    pub dis_amt: f64,
    pub clr_bal_amt: f64,
    pub sanct_lim: f64,
    pub rephasement_principal: f64,
    pub ei_perd_end_date: NaiveDate,
    pub cust_id: String,
    pub cust_name: String,
    pub ei_schm_flg: String,
    pub int_basis: String,
    pub ei_formula_flg: String,
    pub ei_intcalc_freq: String,
    pub ei_method: String,
    pub int_rate: f64,
    pub int_type: String,
    pub peg_review_date: NaiveDate,
    pub last_repricing_date: NaiveDate,
    pub repricing_freq: String,
    pub float_rate_benchmark: String,
    pub spread: String,
    pub npa_flg: String,
    pub npa_classification: String,
    pub npa_amt: f64,
    pub cust_country_cd: String,
    pub cust_credit_rating: String,
    pub cust_sector_cd: String,
    pub cust_industry_cd: String,
    pub exchangert: f64,
    pub custom1: String,
    pub custom2: String,
    pub custom3: String,
    pub gnt_type: String,
    pub status_code: String,
    pub occupation_code: String,
    pub sector: String,
    pub sector_code: String,
    pub subsector_code: String,
    pub staffflag: String,
    pub cre_free_text_1: String,
    pub prov_perc: f64,
    pub ltv: f64,
    pub npa_prov: f64,
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

impl OutputAccount {
    pub fn new(
        inp_acc: GAMData,
        config_params: &ConfigurationParameters,
        ach_map: &mut HashMap<String, ACHData>,
        eit_map: &mut HashMap<String, EITData>,
        overdue_map: &mut HashMap<String, f64>,
        intrate_map: &mut HashMap<String, f64>,
        lam_map: &mut HashMap<String, LAMData>,
        itc_map: &mut HashMap<String, ITCData>,
        rate_code_master: &mut HashMap<String, RateCodeMaster>,
        loan_file_add_map: &mut HashMap<String, LOANFILEData>,
        npa_map: &HashMap<String, NPAData>,
        _logger: &Logger,
    ) -> OutputAccount {
        let (main_classification_user, sub_classification_user) =
            if ach_map.contains_key(&inp_acc.acid) {
                let asset_data = ach_map.get(&inp_acc.acid).expect("Could not get ACH Data");
                (
                    asset_data.main_classification_user.to_string(),
                    asset_data.sub_classification_user.to_string(),
                )
            } else {
                log_warn!(_logger, "Acid: {} not found in ACH-Data", inp_acc.acid);
                ("NA".to_string(), "NA".to_string())
            };
        let (pegged_flag, int_rate) = if eit_map.contains_key(&inp_acc.acid) {
            let eit_data = eit_map.get(&inp_acc.acid).expect("Could not get EIT Data");
            (
                eit_data.account_pegged_flag.to_string(),
                eit_data.interest_rate * 1.0,
            )
        } else {
            log_warn!(_logger, "Acid: {} not found in EIT-Data", inp_acc.acid);
            ("N".to_string(), 0.0)
        };
        let default_lam_data = LAMData {
            acid: "NA".to_string(),
            entity_cre_flg: "NA".to_string(),
            del_flg: "NA".to_string(),
            rep_schdl_num: 0,
            dis_schdl_num: 0,
            dis_schdl_date: NaiveDate::from_ymd_opt(2099, 1, 1)
                .unwrap_or(*config_params.as_on_date()),
            dis_amt: 0.0,
            pdmd_ovdu_perd_mths: 0,
            pdmd_ovdu_perd_days: 0,
            pdmd_ovdu_eom_flg: "NA".to_string(),
            idmd_ovdu_perd_mths: 0,
            idmd_ovdu_perd_days: 0,
            idmd_ovdu_eom_flg: "NA".to_string(),
            rep_schdl_date: NaiveDate::from_ymd_opt(2099, 1, 1)
                .unwrap_or(*config_params.as_on_date()),
            rephasement_principal: 0.0,
            oflow_amt: 0.0,
            ei_perd_end_date: NaiveDate::from_ymd_opt(2099, 1, 1)
                .unwrap_or(*config_params.as_on_date()),
            loan_type: "NA".to_string(),
            ei_schm_flg: "NA".to_string(),
        };
        let lam_data = match lam_map.get(&inp_acc.acid) {
            Some(lam_val) => lam_val,
            None => {
                log_warn!(_logger, "Acid: {} not found in LAM-Data", inp_acc.acid);
                &default_lam_data
            }
        };
        let int_tbl_code = match itc_map.get(&inp_acc.acid) {
            Some(itc_data) => itc_data.int_tbl_code.to_owned(),
            None => "".to_string(),
        };
        let peg_review_date = match itc_map.get(&inp_acc.acid) {
            Some(itc_data) => itc_data.peg_review_date.to_owned(),
            None => NaiveDate::from_ymd_opt(2099, 1, 1).unwrap_or(*config_params.as_on_date()),
        };
        let fixed_floating = match rate_code_master.get(&int_tbl_code) {
            Some(val) => val.fixed_floating.to_owned(),
            None => "NA".to_string(),
        };
        let benchmark = match rate_code_master.get(&int_tbl_code) {
            Some(val) => val.benchmark.to_owned(),
            None => "NA".to_string(),
        };

        let default_npa_data = &NPAData::new();
        let npa_data = match npa_map.get(&inp_acc.foracid) {
            Some(data) => data,
            None => default_npa_data,
        };

        let default_loan_additional_flag_data = LOANFILEData {
            foracid: "".to_string(),
            acid: "".to_string(),
            cust_id: "".to_string(),
            gnt_type: "".to_string(),
            status_code: "".to_string(),
            occupation_code: "".to_string(),
            sector: "".to_string(),
            sector_code: "".to_string(),
            subsector_code: "".to_string(),
            staffflag: "".to_string(),
            cre_free_text_1: "".to_string(),
            pres_val_sec: 0.0,
            paripassu_perc: 0.0,
            prov_perc: 0.0,
            dumm1: 0.0,
            dumm2: 0.0,
            dumm3: 0.0,
            dumm4: 0.0,
            dumm5: "".to_string(),
            dumm6: "".to_string(),
            dumm7: "".to_string(),
            dumm8: "".to_string(),
            dumm9: NaiveDate::from_ymd_opt(2099, 1, 1).unwrap_or(*config_params.as_on_date()),
            dumm10: NaiveDate::from_ymd_opt(2099, 1, 1).unwrap_or(*config_params.as_on_date()),
            constcatgorycode: "".to_string(),
            ratingagc: "".to_string(),
            rating: inp_acc.adim3_gam.to_string(),
            supperannuation_flag: "".to_string(),
            turn_amt1: 0.0,
            turn_amt2: 0.0,
            turn_amt3: 0.0,
            ftp_char1: "".to_string(),
            ftp_char2: "".to_string(),
            ftp_amt1: 0.0,
            ftp_amt2: 0.0,
            ftp_date1: NaiveDate::from_ymd_opt(2099, 1, 1).unwrap_or(*config_params.as_on_date()),
            ftp_date2: NaiveDate::from_ymd_opt(2099, 1, 1).unwrap_or(*config_params.as_on_date()),
        };
        let loan_additional_flag_data = match loan_file_add_map.get(&inp_acc.foracid) {
            Some(data) => data,
            None => {
                log_warn!(
                    _logger,
                    "Foracid: {} not found in LOAN-ADDITIONAL-FILE",
                    inp_acc.foracid
                );
                &default_loan_additional_flag_data
            }
        };

        let next_rep_date = match fixed_floating.to_uppercase().as_str() {
            "FIXED" => lam_data.ei_perd_end_date,
            "FLOATING" => match benchmark.to_uppercase().as_str() {
                "BPLR/PLR" | "BASE RATE" | "EBLR" => {
                    if num_days_start_to_end(*config_params.as_on_date(), lam_data.ei_perd_end_date)
                        > 365
                    {
                        *config_params.as_on_date() + Duration::days(365)
                    } else {
                        lam_data.ei_perd_end_date
                    }
                }
                "MCLR" => {
                    if peg_review_date < lam_data.ei_perd_end_date
                        && peg_review_date
                            < incr_dt_by_mon_presrv_eom_checked(*config_params.as_on_date(), 12)
                                .unwrap_or(peg_review_date)
                    {
                        peg_review_date
                    } else if lam_data.ei_perd_end_date < peg_review_date
                        && lam_data.ei_perd_end_date
                            < incr_dt_by_mon_presrv_eom_checked(*config_params.as_on_date(), 12)
                                .unwrap_or(lam_data.ei_perd_end_date)
                    {
                        lam_data.ei_perd_end_date
                    } else {
                        incr_dt_by_mon_presrv_eom_checked(*config_params.as_on_date(), 12)
                            .unwrap_or(lam_data.ei_perd_end_date)
                    }
                }
                _ => lam_data.ei_perd_end_date,
            },
            _ => lam_data.ei_perd_end_date,
        };
        let ltv_value: f64;
        if loan_additional_flag_data.pres_val_sec == 0.0
            || loan_additional_flag_data.paripassu_perc == 0.0
        {
            ltv_value = 0.0;
        } else {
            ltv_value = inp_acc.clr_bal_amt.abs().to_owned()
                / (loan_additional_flag_data.pres_val_sec
                    * loan_additional_flag_data.paripassu_perc)
                * 100.0
        }
        OutputAccount {
            acid: inp_acc.acid.to_string(),
            foracid: inp_acc.foracid,
            sol_id: inp_acc.sol_id,
            acct_opn_date: inp_acc.acct_open_date,
            gl_sub_head_code: inp_acc.gl_sub_head_code,
            schm_code: inp_acc.schm_code,
            schm_type: inp_acc.schm_type,
            acct_crncy_code: inp_acc.acct_crncy_code,
            rep_shdl_num: lam_data.rep_schdl_num,
            rep_shdl_date: lam_data.rep_schdl_date,
            dis_shdl_num: lam_data.dis_schdl_num,
            next_repricing_date: next_rep_date,
            dis_amt: lam_data.dis_amt,
            clr_bal_amt: inp_acc.clr_bal_amt.abs(),
            sanct_lim: inp_acc.sanct_lim,
            rephasement_principal: lam_data.rephasement_principal,
            ei_perd_end_date: lam_data.ei_perd_end_date,
            cust_id: inp_acc.cust_id,
            cust_name: npa_data.npa_classification.to_owned(),
            ei_schm_flg: lam_data.ei_schm_flg.to_owned(),
            int_basis: npa_data.npa_amount.to_owned(),
            ei_formula_flg: npa_data.cust_hlth_code.to_owned(),
            ei_intcalc_freq: npa_data.cust_npa_class.to_owned(),
            ei_method: npa_data.final_npa_class.to_owned(),
            int_rate: *intrate_map
                .get(&inp_acc.acid.to_string())
                .unwrap_or(&int_rate),
            int_type: pegged_flag,
            peg_review_date,
            last_repricing_date: lam_data.ei_perd_end_date,
            repricing_freq: "NA".to_string(),
            float_rate_benchmark: "NA".to_string(),
            spread: loan_additional_flag_data.rating.to_string(),
            npa_flg: main_classification_user,
            npa_classification: sub_classification_user,
            npa_amt: 0.0,
            cust_country_cd: inp_acc.code1,
            cust_credit_rating: inp_acc.staff_schm_flg,
            cust_sector_cd: inp_acc.llg,
            cust_industry_cd: if *overdue_map.get(&inp_acc.acid.to_string()).unwrap_or(&0.0) <= 0.0
            {
                "N".to_string()
            } else {
                "Y".to_string()
            },
            exchangert: 1.0,
            custom1: int_tbl_code,
            custom2: fixed_floating,
            custom3: benchmark,
            gnt_type: loan_additional_flag_data.gnt_type.to_owned(),
            status_code: loan_additional_flag_data.status_code.to_owned(),
            occupation_code: loan_additional_flag_data.occupation_code.to_owned(),
            sector: loan_additional_flag_data.sector.to_owned(),
            sector_code: loan_additional_flag_data.sector_code.to_owned(),
            subsector_code: loan_additional_flag_data.subsector_code.to_owned(),
            staffflag: loan_additional_flag_data.staffflag.to_owned(),
            cre_free_text_1: loan_additional_flag_data.cre_free_text_1.to_owned(),
            prov_perc: loan_additional_flag_data.prov_perc,
            ltv: ltv_value,
            npa_prov: loan_additional_flag_data.dumm2,
            dumm3: loan_additional_flag_data.dumm3,
            dumm4: loan_additional_flag_data.dumm4,
            dumm5: loan_additional_flag_data.dumm5.to_owned(),
            dumm6: loan_additional_flag_data.dumm6.to_owned(),
            dumm7: loan_additional_flag_data.dumm7.to_owned(),
            dumm8: loan_additional_flag_data.dumm8.to_owned(),
            dumm9: loan_additional_flag_data.dumm9,
            dumm10: loan_additional_flag_data.dumm10,
            constcatgorycode: loan_additional_flag_data.constcatgorycode.to_owned(),
            ratingagc: loan_additional_flag_data.ratingagc.to_owned(),
            rating: loan_additional_flag_data.rating.to_owned(),
            supperannuation_flag: loan_additional_flag_data.supperannuation_flag.to_owned(),
            turn_amt1: loan_additional_flag_data.turn_amt1,
            turn_amt2: loan_additional_flag_data.turn_amt2,
            turn_amt3: loan_additional_flag_data.turn_amt3,
            ftp_char1: loan_additional_flag_data.ftp_char1.to_owned(),
            ftp_char2: loan_additional_flag_data.ftp_char2.to_owned(),
            ftp_amt1: loan_additional_flag_data.ftp_amt1,
            ftp_amt2: loan_additional_flag_data.ftp_amt2,
            ftp_date1: loan_additional_flag_data.ftp_date1,
            ftp_date2: loan_additional_flag_data.ftp_date2,
        }
    }
}

pub fn format_output(output_rec: OutputAccount) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}", 
        output_rec.acid,
        output_rec.foracid,
        output_rec.sol_id,
        check_valid_date(output_rec.acct_opn_date),
        output_rec.gl_sub_head_code,
        output_rec.schm_code,
        output_rec.schm_type,
        output_rec.acct_crncy_code,
        output_rec.rep_shdl_num,
        check_valid_date(output_rec.rep_shdl_date),
        output_rec.dis_shdl_num,
        check_valid_date(output_rec.next_repricing_date),
        output_rec.dis_amt,
        output_rec.clr_bal_amt,
        output_rec.sanct_lim,
        output_rec.rephasement_principal,
        check_valid_date(output_rec.ei_perd_end_date),
        output_rec.cust_id,
        output_rec.cust_name,
        output_rec.ei_schm_flg,
        output_rec.int_basis,
        output_rec.ei_formula_flg,
        output_rec.ei_intcalc_freq,
        output_rec.ei_method,
        output_rec.int_rate,
        output_rec.int_type,
        check_valid_date(output_rec.peg_review_date),
        check_valid_date(output_rec.last_repricing_date),
        output_rec.repricing_freq,
        output_rec.float_rate_benchmark,
        output_rec.spread,
        output_rec.npa_flg,
        output_rec.npa_classification,
        output_rec.npa_amt,
        output_rec.cust_country_cd,
        output_rec.cust_credit_rating,
        output_rec.cust_sector_cd,
        output_rec.cust_industry_cd,
        output_rec.exchangert,
        output_rec.custom1,
        output_rec.custom2,
        output_rec.custom3,
        output_rec.gnt_type,
        output_rec.status_code,
        output_rec.occupation_code,
        output_rec.sector,
        output_rec.sector_code,
        output_rec.subsector_code,
        output_rec.staffflag,
        output_rec.cre_free_text_1,
        output_rec.prov_perc,
        output_rec.ltv,
        output_rec.npa_prov,
        output_rec.dumm3,
        output_rec.dumm4,
        output_rec.dumm5,
        output_rec.dumm6,
        output_rec.dumm7,
        output_rec.dumm8,
        check_valid_date(output_rec.dumm9),
        check_valid_date(output_rec.dumm10),
        output_rec.constcatgorycode,
        output_rec.ratingagc,
        output_rec.rating,
        output_rec.supperannuation_flag,
        output_rec.turn_amt1,
        output_rec.turn_amt2,
        output_rec.turn_amt3,
        output_rec.ftp_char1,
        output_rec.ftp_char2,
        output_rec.ftp_amt1,
        output_rec.ftp_amt2,
        check_valid_date(output_rec.ftp_date1),
        check_valid_date(output_rec.dumm10),
    )
}

pub fn get_writer(file_path: &str) -> std::io::BufWriter<std::fs::File> {
    match sdb_io::buf_file_wrtr(file_path, None) {
        Ok(file) => file,
        Err(error) => panic!("Unable to create file `{}` due to: {}", file_path, error),
    }
}

pub fn check_valid_date(date: NaiveDate) -> String {
    let min_date = NaiveDate::from_ymd_opt(1970, 1, 1).expect("Error readung Minimun Date");
    if date < min_date {
        min_date.format("%d-%m-%Y").to_string()
    } else {
        date.format("%d-%m-%Y").to_string()
    }
}
