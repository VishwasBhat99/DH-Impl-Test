use self::io::*;
use configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod io;
mod structs;
use self::structs::{BalmIcvKey, BalmIcvVal, IvsKey, IvsVal, OdInt};
use health_report::HealthReport;
use macros;
use rbdate::DateParser;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let mut op_writer = get_writer(config_params.output_file_path());
    let date_parser: DateParser = DateParser::new("%d-%m-%Y".to_string(), true);
    let mut tot_amt = 0.0;

    let mut final_out_acc: HashMap<String, OdInt> = HashMap::new();

    let gam_input = File::open(&config_params.input_file_gam()).expect("Could Not Read File");
    let gam_input_reader = BufReader::new(gam_input);
    let schm_types = config_params.schm_types();
    let gam_schm_type: Vec<&str> = schm_types.split('|').collect();
    //GAM
    for (_index, line) in gam_input_reader.lines().enumerate() {
        let line = line.expect("Could Not Read Line in GAM file.").to_string();
        let input_fields: Vec<&str> = line.split('|').collect();
        let mut new_acc = OdInt::new();
        let schm_type = input_fields[15];
        let gam_clr_bal_amt = input_fields[3].to_string().parse::<f64>().unwrap_or(0.0);
        if !input_fields[5].to_string().contains("NFS") {
            new_acc.foracid = input_fields[1].to_string();
            new_acc.cust_id = input_fields[6].to_string();
            if gam_schm_type.contains(&schm_type) && gam_clr_bal_amt < 0.0 {
                new_acc.schm_type = schm_type.to_string();
                new_acc.acct_crncy_code = input_fields[17].to_string();
            } else if schm_type.to_uppercase() == "LAA" {
                if gam_clr_bal_amt >= 0.0 {
                    new_acc.int_slab_dr_cr_flg = "C".to_string();
                } else {
                    new_acc.int_slab_dr_cr_flg = "D".to_string();
                }
            }
        }
        new_acc.datachanged = "TRUE".to_string();
        new_acc.gam_last_tran_date = *config_params.as_on_date();

        if &input_fields[23].to_string() != ""
            && config_params.as_on_date() < &date_parser.parse(&input_fields[23].to_string())
        {
            new_acc.gam_last_tran_date = date_parser.parse(&input_fields[23].to_string());
        }
        new_acc.gam_clr_bal_amt = input_fields[3].parse::<f64>().unwrap_or(0.0);
        final_out_acc.insert(input_fields[0].to_string(), new_acc);
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
            log_debug!(
                logger,
                "Could not find struct for an account in ITC={}",
                &input_fields[0]
            );
            continue;
        }
        let itc_lchg_time = date_parser.parse(&input_fields[15].to_string());
        if itc_lchg_time >= loan_acc.itc_lchg_time {
            loan_acc.itc_lchg_time = itc_lchg_time;
            let int_tbl_code_srl_num_read = input_fields[17].parse::<i64>().unwrap_or(0);
            if int_tbl_code_srl_num_read >= loan_acc.int_tbl_code_srl_num {
                loan_acc.int_tbl_code_srl_num = int_tbl_code_srl_num_read;
                if gam_schm_type.contains(&loan_acc.schm_type.as_str()) {
                    loan_acc.datachanged = "TRUE".to_string();
                    loan_acc.int_tbl_code = input_fields[2].to_string();
                    loan_acc.cust_pref_pcnt =
                        input_fields[6].to_string().parse::<f64>().unwrap_or(0.0);
                    loan_acc.id_pref_pcnt =
                        input_fields[8].to_string().parse::<f64>().unwrap_or(0.0);
                    loan_acc.min_int_pcnt =
                        input_fields[10].to_string().parse::<f64>().unwrap_or(0.0);
                    loan_acc.max_int_pcnt =
                        input_fields[12].to_string().parse::<f64>().unwrap_or(0.0);
                    loan_acc.end_date = date_parser
                        .parse(&input_fields[16].to_string())
                        .format("%Y-%m-%d")
                        .to_string();
                    loan_acc.pegged_flg = input_fields[4].trim().to_string();
                }
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
            int_tbl_code: input_fields[5].to_string(),
            crncy_code: input_fields[9].to_string(),
        };
        let mut new_balm_icv_val = BalmIcvVal {
            icv_int_tbl_ver_num: input_fields[12].to_string(),
            int_tbl_ver_num: input_fields[0].to_string().parse::<i64>().unwrap_or(0),
            base_pcnt: input_fields[2].to_string().parse::<f64>().unwrap_or(0.0),
        };
        let mut update_balm_icv_val;
        if balm_icv_data.contains_key(&new_balm_icv) {
            update_balm_icv_val = balm_icv_data
                .get_mut(&new_balm_icv)
                .expect("Cannot fetch value from ICV file.");
            if update_balm_icv_val.icv_int_tbl_ver_num <= new_balm_icv_val.icv_int_tbl_ver_num
                && update_balm_icv_val.int_tbl_ver_num <= new_balm_icv_val.int_tbl_ver_num
            {
                balm_icv_data.remove(&new_balm_icv);
                balm_icv_data.insert(new_balm_icv, new_balm_icv_val);
            }
        } else {
            balm_icv_data.insert(new_balm_icv, new_balm_icv_val);
        }
    }

    //IVS

    let ivs_input = File::open(&config_params.input_file_ivs()).expect("Could Not Read File");
    let ivs_input_reader = BufReader::new(ivs_input);
    let mut ivs_map: HashMap<IvsKey, Vec<IvsVal>> = HashMap::new();
    for (_index, line) in ivs_input_reader.lines().enumerate() {
        let line = line.expect("Could Not Read Line").to_string();
        let input_fields: Vec<&str> = line.split('|').collect();
        let key = IvsKey {
            int_tbl_code: input_fields[12].to_string(),
            int_tbl_ver_num: input_fields[13].to_string().parse::<i64>().unwrap_or(0),
            crncy_code: input_fields[11].to_string(),
        };
        let value = IvsVal {
            nrml_int_pcnt: input_fields[1].to_string().parse::<f64>().unwrap_or(0.0),
            int_slab_srl_no: input_fields[18].to_string().parse::<i64>().unwrap_or(0),
            end_slab_amt: input_fields[17].to_string().parse::<f64>().unwrap_or(0.0),
        };

        if ivs_map.contains_key(&key) {
            let update_ivs_val = ivs_map.get_mut(&key).expect("Cannot fetch value.");
            update_ivs_val.push(value);
        } else {
            ivs_map.insert(key, vec![value]);
        }
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
        let itc_value = balm_icv_data.get(&new_icv_key).unwrap_or(default_icv);
        value.icv_int_tbl_ver_num = itc_value.icv_int_tbl_ver_num.to_owned();
        value.int_tbl_ver_num = itc_value.int_tbl_ver_num;
        value.base_pcnt = itc_value.base_pcnt;

        //Set IVS values.
        let default_ivs_val = IvsVal {
            nrml_int_pcnt: 0.0,
            int_slab_srl_no: 0,
            end_slab_amt: 0.0,
        };

        let ivs_key = IvsKey {
            int_tbl_code: value.int_tbl_code.to_owned(),
            int_tbl_ver_num: value.int_tbl_ver_num,
            crncy_code: value.acct_crncy_code.to_owned(),
        };
        let ivs_value_vec = ivs_map
            .get(&ivs_key)
            .unwrap_or(&vec![default_ivs_val])
            .to_owned();
        let slab_srl_no = 0;
        for ivs_value in ivs_value_vec {
            if ivs_value.end_slab_amt >= value.gam_clr_bal_amt
                && ivs_value.int_slab_srl_no >= slab_srl_no
            {
                value.nrml_int_pcnt = ivs_value.nrml_int_pcnt;
            }
        }

        //Update values percentage.
        if value.cust_pref_pcnt + value.id_pref_pcnt + value.nrml_int_pcnt + value.base_pcnt
            < value.min_int_pcnt
        {
            value.cust_pref_pcnt = 0.0;
            value.id_pref_pcnt = 0.0;
            value.base_pcnt = 0.0;
            value.nrml_int_pcnt = value.min_int_pcnt;
        }

        if value.cust_pref_pcnt + value.id_pref_pcnt + value.nrml_int_pcnt + value.base_pcnt
            > value.max_int_pcnt
            && value.max_int_pcnt > 0.0
        {
            value.cust_pref_pcnt = 0.0;
            value.id_pref_pcnt = 0.0;
            value.base_pcnt = 0.0;
            value.nrml_int_pcnt = value.max_int_pcnt;
        }

        value.datachanged = "FALSE".to_string();
        tot_amt += value.base_pcnt;
        writeln!(
            op_writer,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            key,
            value.del_flg,
            value.int_slab_dr_cr_flg,
            value.itc_lchg_time,
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
            value.end_date,
            value.pegged_flg,
            value.npa_classification,
            value.npa_amount,
            value.foracid,
            value.cust_id,
            value.gam_last_tran_date,
            value.gam_clr_bal_amt,
        )
        .expect("Could not write to output file.");
    }

    let health_report = HealthReport::new(0, 0, 0, tot_amt, tot_amt, 0);
    health_report.gen_health_rpt(config_params.output_file_path());
}
