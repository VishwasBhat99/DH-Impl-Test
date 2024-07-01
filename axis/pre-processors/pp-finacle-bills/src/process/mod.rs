use self::io::*;
use crate::configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod io;
use crate::macros;
use input::*;
use rbdate::{num_days_start_to_end, DateParser, NaiveDate};
use std::collections::HashMap;
use std::fs;
use std::io::Write;

mod input;
pub fn process(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let fbm_reader =
        fs::read_to_string(&config_params.fbm_input_file()).expect("Could Not Read FBM File");
    let mut fbm_hashmap: HashMap<String, Vec<FBMData>> = HashMap::new();
    let mut acc_enc = 0;
    let mut acc_succ = 0;
    let mut tot_amt = 0.0;
    for line in fbm_reader.lines() {
        let vecfbm: Vec<&str> = line.split(config_params.delimeter()).collect();

        let key = vecfbm[7].trim().to_string();
        let fbm_acc = FBMData::new(vecfbm);
        if fbm_hashmap.contains_key(&key) {
            fbm_hashmap
                .get_mut(&key)
                .as_mut()
                .expect("Could not get value from fbm file for a key.")
                .push(fbm_acc);
        } else {
            fbm_hashmap.insert(key.to_string(), vec![fbm_acc]);
        }
    }

    let npa_reader =
        fs::read_to_string(&config_params.npa_input_file()).expect("Could Not Read NPA File");
    let mut npa_hashmap: HashMap<String, NPAData> = HashMap::new();
    for line in npa_reader.lines() {
        let vecnpa = line.split(config_params.delimeter()).collect::<Vec<&str>>();
        npa_hashmap.insert(vecnpa[1].trim().to_string(), NPAData::new(vecnpa));
    }
    let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
    let mut op_writer = get_writer(config_params.finacle_bills_output_file_path());

    let gam_reader =
        fs::read_to_string(&config_params.gam_input_file()).expect("Could Not Read GAM File");
    for line in gam_reader.lines() {
        acc_enc += 1;
        let vecgam = line.split(config_params.delimeter()).collect::<Vec<&str>>();
        let acid = vecgam[0].trim();
        let foracid = vecgam[1].trim().to_string();
        let npa_data: NPAData = if npa_hashmap.contains_key(&foracid as &str) {
            npa_hashmap
                .get(&foracid as &str)
                .expect("Cannot find foracid ID in NPA.")
                .to_owned()
        } else {
            log_debug!(logger, "NPA does not contain lookup key:{}", foracid);
            NPAData {
                npa_classification: "0".to_string(),
                cust_hlth_code: "0".to_string(),
                cust_npa_class: "0".to_string(),
                final_npa_class: "0".to_string(),
                npa_amount: "0.0".to_string(),
            }
        };
        if fbm_hashmap.contains_key(acid as &str) {
            let fbm_data_vec = fbm_hashmap
                .get(acid as &str)
                .expect("Cannot find ACID ID in FBM.")
                .to_vec();
            for fbm_data in fbm_data_vec.iter() {
                let acc_num = if fbm_data.bill_b2k_id.is_empty() {
                    acid.to_string()
                } else {
                    acid.to_owned() + "/" + &fbm_data.bill_b2k_id
                };

                let bp_liab_crncy_der = if fbm_data.bp_liab_crncy.is_empty() {
                    "INR".to_string()
                } else {
                    fbm_data.bp_liab_crncy.to_owned()
                };
                let start_dt = date_parser.parse(fbm_data.due_date.trim());
                let end_dt = date_parser.parse(vecgam[20]);
                let int_rt = if (fbm_data.bill_amt.trim().parse::<f64>().unwrap_or(0.0)
                    * (num_days_start_to_end(start_dt, end_dt)) as f64)
                    != 0.0
                {
                    ((fbm_data.bill_liab.parse::<f64>().unwrap_or(0.0)
                        - fbm_data.bill_amt.parse::<f64>().unwrap_or(0.0))
                        * 36500.0)
                        / (fbm_data.bill_amt.parse::<f64>().unwrap_or(0.0)
                            * (num_days_start_to_end(start_dt, end_dt)) as f64)
                } else {
                    0.0
                };
                let acc_exch_rt_1 = if fbm_data.bill_liab_hc_eq == "0" {
                    "1".to_string()
                } else {
                    fbm_data.bill_liab_hc_eq.to_owned()
                };
                let acc_exch_rt_2 = if fbm_data.bill_liab == "0" {
                    "1".to_string()
                } else {
                    fbm_data.bill_liab.to_owned()
                };
                let overdue_flg = if NaiveDate::parse_from_str(&fbm_data.due_date, "%d-%m-%Y")
                    .unwrap_or(*config_params.as_on_date())
                    <= *config_params.as_on_date()
                {
                    "Y"
                } else {
                    "N"
                };
                let acc_exch_rt = (acc_exch_rt_1.parse::<f64>().unwrap_or(0.0))
                    / (acc_exch_rt_2.parse::<f64>().unwrap_or(0.0));
                writeln!(
                    op_writer,
                    "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
                    acc_num,
                    acid,
                    foracid,
                    vecgam[2],
                    vecgam[5],
                    vecgam[6],
                    vecgam[14],
                    vecgam[15],
                    fbm_data.bill_param_type,
                    fbm_data.bill_b2k_id,
                    fbm_data.bill_id,
                    fbm_data.bill_amt,
                    fbm_data.bill_amt_inr,
                    fbm_data.bill_crncy_code,
                    fbm_data.due_date,
                    fbm_data.bp_acid,
                    fbm_data.del_flg,
                    fbm_data.cls_flg,
                    fbm_data.reg_type,
                    fbm_data.reg_sub_type,
                    fbm_data.bp_liab,
                    fbm_data.bp_liab_crncy,
                    fbm_data.bill_liab_inr,
                    fbm_data.bill_stat,
                    fbm_data.bill_func_code,
                    if npa_data.npa_classification == "0".to_string() {fbm_data.bill_liab.clone()} else {npa_data.npa_amount.parse().unwrap_or(0.0).to_string()},
                    fbm_data.bill_liab_hc_eq,
                    fbm_data.bill_liab_crncy,
                    bp_liab_crncy_der,
                    vecgam[3],
                    vecgam[4],
                    vecgam[50],
                    vecgam[20],
                    vecgam[17],
                    vecgam[44],
                    vecgam[13],
                    npa_data.npa_classification.to_owned(),
                    npa_data.cust_hlth_code.to_owned(),
                    npa_data.cust_npa_class.to_owned(),
                    npa_data.final_npa_class.to_owned(),
                    int_rt,
                    acc_exch_rt,
                    vecgam[51],
                    vecgam[52],
                    vecgam[53],
                    vecgam[54],
                    vecgam[55],
                    vecgam[56],
                    overdue_flg,
                ).ok();
                acc_succ += 1;
                tot_amt += if npa_data.npa_classification == "0".to_string() {
                    fbm_data.bill_liab.parse::<f64>().unwrap_or(0.0)
                } else {
                    npa_data.npa_amount.parse().unwrap_or(0.0)
                };
            }
        } else {
            log_debug!(logger, "FBM does not contain lookup ID:{}", acid);
        }
    }
    let health_stat = health_report::HealthReport::new(
        acc_enc,
        acc_succ,
        acc_enc - acc_succ,
        tot_amt,
        tot_amt,
        0,
    );
    health_stat.gen_health_rpt(config_params.finacle_bills_output_file_path())
}
