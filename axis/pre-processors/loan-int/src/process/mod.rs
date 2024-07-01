use self::io::*;
use configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod io;
mod structs;
use self::structs::{BalmIcvKey, BalmIcvVal, LavsVal, LoanInt};
use health_report::HealthReport;
use macros;
use process::structs::LavsKey;
use rbdate::DateParser;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let mut op_writer = get_writer(config_params.output_file_path());
    let date_parser: DateParser = DateParser::new("%d-%m-%Y".to_string(), true);
    let mut acc_enctrd = 0;
    let mut acc_succ = 0;

    let mut final_out_acc: HashMap<String, LoanInt> = HashMap::new();

    let gam_input = File::open(&config_params.input_file_gam()).expect("Could Not Read File");
    let gam_input_reader = BufReader::new(gam_input);

    //GAM
    for (_index, line) in gam_input_reader.lines().enumerate() {
        let line = line.expect("Could Not Read Line in GAM file.").to_string();
        let input_fields: Vec<&str> = line.split('|').collect();
        let mut new_acc = LoanInt::new();
        if !input_fields[5].to_string().contains("NFS")
            && (*input_fields[15].to_string() == *"LAA"
                || *input_fields[15].to_string() == *"".to_string())
        {
            new_acc.schm_type = "LAA".to_string();
            new_acc.foracid = input_fields[1].to_string();
            new_acc.cust_id = input_fields[6].to_string();
            new_acc.acct_crncy_code = input_fields[17].to_string();
            if input_fields[3].to_string().parse::<f64>().unwrap_or(0.0) >= 0.0 {
                new_acc.int_slab_dr_cr_flg = "C".to_string();
            } else {
                new_acc.int_slab_dr_cr_flg = "D".to_string();
            }
            new_acc.datachanged = "TRUE".to_string();
            final_out_acc.insert(input_fields[0].to_string(), new_acc);
            acc_enctrd += 1;
        }
    }

    let itc_input = File::open(&config_params.input_file_itc()).expect("Could Not Read File");
    let itc_input_reader = BufReader::new(itc_input);

    //ITC
    for (_index, line) in itc_input_reader.lines().enumerate() {
        let line = line.expect("Could Not Read Line").to_string();
        let input_fields: Vec<&str> = line.split('|').collect();
        let mut loan_acc;
        if final_out_acc.contains_key(&input_fields[0].to_string()) {
            loan_acc = final_out_acc
                .get_mut(&input_fields[0].to_string())
                .expect("could not find struct for an account");
        } else {
            log_info!(
                logger,
                "Could not find struct for an account in ITC={}",
                &input_fields[0]
            );
            continue;
        }
        if date_parser.parse(&input_fields[15].to_string()) >= loan_acc.itc_lchg_time {
            loan_acc.itc_lchg_time = date_parser.parse(&input_fields[15].to_string());
            let int_tbl_code_srl_num_read = input_fields[17].parse::<i64>().unwrap_or(0);
            if int_tbl_code_srl_num_read >= loan_acc.int_tbl_code_srl_num {
                loan_acc.int_tbl_code_srl_num = int_tbl_code_srl_num_read;
                loan_acc.datachanged = "TRUE".to_string();
                loan_acc.int_tbl_code = input_fields[2].to_string();
                loan_acc.cust_pref_pcnt = input_fields[6].to_string().parse::<f64>().unwrap_or(0.0);
                loan_acc.id_pref_pcnt = input_fields[8].to_string().parse::<f64>().unwrap_or(0.0);
                loan_acc.min_int_pcnt = input_fields[10].to_string().parse::<f64>().unwrap_or(0.0);
                loan_acc.max_int_pcnt = input_fields[12].to_string().parse::<f64>().unwrap_or(0.0);
                loan_acc.end_date = date_parser.parse(&input_fields[16].to_string());

                loan_acc.pegged_flg = input_fields[4].to_string();
            }
        }
    }

    let icv_input = File::open(&config_params.input_file_icv()).expect("Could Not Read File");
    let icv_input_reader = BufReader::new(icv_input);

    let mut balm_icv_data: HashMap<BalmIcvKey, BalmIcvVal> = HashMap::new();

    //ICV
    for (_index, line) in icv_input_reader.lines().enumerate() {
        let line = line.expect("Could Not Read Line").to_string();
        let input_fields: Vec<&str> = line.split('|').collect();
        let new_balm_icv = BalmIcvKey {
            int_tbl_code: input_fields[5].trim().to_string(),
            crncy_code: input_fields[9].trim().to_string(),
        };
        let mut new_balm_icv_val = BalmIcvVal {
            icv_int_tbl_ver_num: input_fields[12].trim().to_string(),
            int_tbl_ver_num: input_fields[0]
                .trim()
                .to_string()
                .parse::<i64>()
                .unwrap_or(0),
            base_pcnt: input_fields[2]
                .trim()
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0),
        };

        balm_icv_data
            .entry(new_balm_icv)
            .and_modify(|val| {
                if val <= &mut new_balm_icv_val {
                    *val = new_balm_icv_val.to_owned()
                }
            })
            .or_insert(new_balm_icv_val);
    }

    // LAVS
    let lavs_input = File::open(&config_params.input_file_lavs()).expect("Could Not Read File");
    let lavs_input_reader = BufReader::new(lavs_input);
    let mut lavs_map: HashMap<LavsKey, Vec<LavsVal>> = HashMap::new();
    for (_index, line) in lavs_input_reader.lines().enumerate() {
        let line = line.expect("Could Not Read Line").to_string();
        let input_fields: Vec<&str> = line.split('|').collect();
        let key = LavsKey {
            int_tbl_code: input_fields[1].trim().to_string(),
            int_tbl_ver_num: input_fields[3]
                .trim()
                .to_string()
                .parse::<i64>()
                .unwrap_or(0),
            crncy_code: input_fields[2].trim().to_string(),
        };
        let value = LavsVal {
            nrml_int_pcnt: input_fields[0]
                .trim()
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0),
            int_slab_srl_no: input_fields[5]
                .trim()
                .to_string()
                .parse::<i64>()
                .unwrap_or(0),
            end_slab_amt: input_fields[6]
                .trim()
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0),
        };
        lavs_map
            .entry(key)
            .and_modify(|val| val.push(value))
            .or_insert(vec![value]);
    }

    //LAM
    let lam_input = File::open(&config_params.input_file_lam()).expect("Could Not Read File LAM.");
    let lam_input_reader = BufReader::new(lam_input);
    let mut lam_map: HashMap<String, f64> = HashMap::new();
    for (_index, line) in lam_input_reader.lines().enumerate() {
        let line = line.expect("Could Not Read Line").to_string();
        let input_fields: Vec<&str> = line.split('|').collect();
        lam_map.insert(
            input_fields[0].trim().to_string(),
            input_fields[6]
                .trim()
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0),
        );
    }

    // //LRS
    let lrs_input = File::open(&config_params.input_file_lrs()).expect("Could Not Read File LRS");
    let lrs_input_reader = BufReader::new(lrs_input);

    let mut balm_lrs_data: HashMap<String, i64> = HashMap::new();

    for (_index, line) in lrs_input_reader.lines().enumerate() {
        let line = line.expect("Could Not Read Line").to_string();
        let input_fields: Vec<&str> = line.split('|').collect();
        if input_fields[13].trim() == "PRDEM" || input_fields[13].trim() == "EIDEM" {
            let mut update_lrs_val = input_fields[15]
                .trim()
                .to_string()
                .parse::<i64>()
                .unwrap_or(0);
            balm_lrs_data
                .entry(input_fields[0].trim().to_string())
                .and_modify(|val| {
                    if &mut update_lrs_val >= val {
                        *val = update_lrs_val.to_owned()
                    }
                })
                .or_insert(
                    input_fields[15]
                        .trim()
                        .to_string()
                        .parse::<i64>()
                        .unwrap_or(0),
                );
        }
    }
    //BALM_LRP_FLT
    let lrp_input = File::open(&config_params.input_file_balm_lrp_flt())
        .expect("Could Not Read File BALM LRP FLT");
    let lrp_input_reader = BufReader::new(lrp_input);

    let mut balm_lrp_flt_data: HashMap<String, f64> = HashMap::new();

    for (_index, line) in lrp_input_reader.lines().enumerate() {
        let line = line.expect("Could Not Read LRP line").to_string();
        let input_fields: Vec<&str> = line.split('|').collect();
        balm_lrp_flt_data.insert(input_fields[0].trim().to_string(), 0.0);
    }

    //Update data in loan_int buffer.
    for (key, mut value) in final_out_acc {
        //Set ICV fields.
        let new_icv_key = BalmIcvKey {
            int_tbl_code: value.int_tbl_code.clone(),
            crncy_code: value.acct_crncy_code.clone(),
        };

        let default_icv = &BalmIcvVal {
            icv_int_tbl_ver_num: "0".to_string(),
            int_tbl_ver_num: 0,
            base_pcnt: 0.0,
        };
        let icv_value = balm_icv_data.get(&new_icv_key).unwrap_or(default_icv);
        value.icv_int_tbl_ver_num = icv_value.icv_int_tbl_ver_num.to_owned();
        value.int_tbl_ver_num = icv_value.int_tbl_ver_num;
        value.base_pcnt = icv_value.base_pcnt;

        // //Set LAVS and LAM fields.
        let default_lavs_val = LavsVal {
            nrml_int_pcnt: 0.0,
            int_slab_srl_no: 0,
            end_slab_amt: 0.0,
        };
        let input_lavs_key = LavsKey {
            int_tbl_code: value.int_tbl_code.to_owned(),
            int_tbl_ver_num: value.int_tbl_ver_num,
            crncy_code: value.acct_crncy_code.to_owned(),
        };
        let mut lam_dis_amt = 0.0;
        if lam_map.contains_key(&key) {
            lam_dis_amt = lam_map.get(&key).unwrap_or(&0.0).to_owned();
        }
        let max_int_slab_srl_no = 0;
        let lavs_val = lavs_map
            .get(&input_lavs_key)
            .unwrap_or(&vec![default_lavs_val])
            .to_owned();
        for lavs_val_field in lavs_val {
            if lavs_val_field.end_slab_amt >= lam_dis_amt
                && lavs_val_field.int_slab_srl_no >= max_int_slab_srl_no
            {
                value.nrml_int_pcnt = lavs_val_field.nrml_int_pcnt;
            }
        }

        //LRS
        let lrs_val = balm_lrs_data.get(&key).unwrap_or(&0);
        value.lrs_shdl_num = *lrs_val;

        //FINAL UPDATES
        if (value.cust_pref_pcnt + value.id_pref_pcnt + value.nrml_int_pcnt + value.base_pcnt)
            < value.min_int_pcnt
        {
            value.cust_pref_pcnt = 0.0;
            value.id_pref_pcnt = 0.0;
            value.nrml_int_pcnt = value.min_int_pcnt;
            value.base_pcnt = 0.0;
        }
        if value.max_int_pcnt > 0.0
            && value.cust_pref_pcnt + value.id_pref_pcnt + value.nrml_int_pcnt + value.base_pcnt
                > value.max_int_pcnt
        {
            value.cust_pref_pcnt = 0.0;
            value.id_pref_pcnt = 0.0;
            value.nrml_int_pcnt = value.max_int_pcnt;
            value.base_pcnt = 0.0;
        }
        if value.int_slab_dr_cr_flg == "C" {
            value.cust_pref_pcnt = 0.0;
            value.id_pref_pcnt = 0.0;
            value.nrml_int_pcnt = 0.0;
            value.base_pcnt = 0.0;
        }
        //Update pegged flag
        if balm_lrp_flt_data.contains_key(&key) {
            value.pegged_flg = "N".to_string();
        }
        let final_int_rate =
            value.cust_pref_pcnt + value.id_pref_pcnt + value.base_pcnt + value.nrml_int_pcnt;
        writeln!(
            op_writer,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            key,
            value.del_flg,
            value.int_slab_dr_cr_flg,
            value.itc_lchg_time.format("%d-%m-%Y"),
            value.schm_type,
            value.int_tbl_code,
            value.int_tbl_code_srl_num,
            value.icv_int_tbl_ver_num,
            value.int_tbl_ver_num,
            value.min_int_pcnt,
            value.max_int_pcnt,
            value.cust_pref_pcnt,
            value.id_pref_pcnt,
            value.nrml_int_pcnt,
            value.base_int_tbl_code,
            value.base_pcnt,
            value.acct_crncy_code,
            value.datachanged,
            value.end_date.format("%d-%m-%Y"),
            value.pegged_flg,
            value.lrs_shdl_num,
            value.npa_classification,
            value.npa_amount,
            value.foracid,
            value.cust_id,
            final_int_rate,
        )
        .expect("Could not write to output file.");
        acc_succ += 1;
    }

    let health_report = HealthReport::new(acc_enctrd, acc_succ, acc_enctrd - acc_succ, 0.0, 0.0, 0);
    health_report.gen_health_rpt(config_params.output_file_path());
}
