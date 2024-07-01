extern crate serde;
mod exrt;
mod structs;
use self::exrt::*;
use self::structs::NpaData;
use crate::pre_processor::structs::BalmGamKey;
use crate::pre_processor::structs::BalmGamValue;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::num_days_start_to_end;
use rbdate::DateParser;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::{HashMap, HashSet};
use std::env::current_dir;
use std::io::prelude::*;
use std::io::BufWriter;
use std::time::SystemTime;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let output_file = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{} Cannot read output file path", error),
    };

    let mut tot_input_acc_encntrd: i64 = 0;
    let mut tot_succ_rec: i64 = 0;
    let mut writer = BufWriter::new(output_file);
    let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
    let npa_data_file = match new_buf_rdr(config_param.npa_data_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found npa data file: `{}` on location `{}` : {}.",
            config_param.npa_data_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let start_datafile_derive_timer = SystemTime::now();
    let mut npa_data_map: HashMap<String, NpaData> = HashMap::new();
    for (line_num, lines) in npa_data_file.lines().enumerate() {
        let npa_data_line = match lines {
            Ok(npa_data_line) => npa_data_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.npa_data_file_path(),
                line_num + 1,
                error
            ),
        };

        let npa_data_fields = npa_data_line.split('|').collect::<Vec<&str>>();
        if npa_data_fields.len() >= 5 {
            //Store the LoanAccNo and NPA_Classification as key-value pairs.
            npa_data_map.insert(
                npa_data_fields[0].to_string(),
                NpaData {
                    classification: npa_data_fields[2].to_string(),
                    npa_amt: npa_data_fields[4].parse::<f64>().unwrap_or(0.0),
                },
            );
        } else {
            log_error!(
                log,
                "Found line :{} at line number {} defective in NPA data file.",
                line_num,
                npa_data_line
            );
        }
    }
    let end_datafile_derive_timer = SystemTime::now();
    let duration = end_datafile_derive_timer
        .duration_since(start_datafile_derive_timer)
        .expect("Could not calculate NPA Data File derive process duration.");
    debug!(
        diag_log,
        "NPA Data File Derive Process Total Duration: {:?}.", duration
    );

    let npa_live_file = match new_buf_rdr(config_param.npa_live_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found npa live file: `{}` on location `{}` : {}.",
            config_param.npa_live_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let start_livefile_derive_timer = SystemTime::now();
    let mut npa_live_hashmap: HashMap<String, String> = HashMap::new();

    for (line_num, lines) in npa_live_file.lines().enumerate() {
        let npa_live_line = match lines {
            Ok(npa_live_line) => npa_live_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.npa_live_file_path(),
                line_num + 1,
                error
            ),
        };

        let npa_live_fields = npa_live_line.split('|').collect::<Vec<&str>>();
        if npa_live_fields.len() >= 7 {
            //Store the Finacle_CustID and cust_hlth_code as key-value pairs.
            npa_live_hashmap.insert(
                npa_live_fields[0].to_string(),
                npa_live_fields[2].to_string(),
            );
        } else {
            log_error!(
                log,
                "Found line :{} at line number {} defective in NPA Live file.",
                line_num,
                npa_live_line
            );
        }
    }
    let end_livefile_derive_timer = SystemTime::now();
    let duration = end_livefile_derive_timer
        .duration_since(start_livefile_derive_timer)
        .expect("Could not calculate NPA Live File derive process duration.");
    debug!(
        diag_log,
        "NPA Live File Derive Process Total Duration: {:?}.", duration
    );

    let npa_config_file = match new_buf_rdr(config_param.npa_config_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found npa config file: `{}` on location `{}` : {}.",
            config_param.npa_config_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let start_configfile_derive_timer = SystemTime::now();

    let mut npa_config_hashmap: HashMap<String, String> = HashMap::new();
    for (line_num, lines) in npa_config_file.lines().enumerate() {
        let npa_config_line = match lines {
            Ok(npa_config_line) => npa_config_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.npa_config_file_path(),
                line_num + 1,
                error
            ),
        };

        let npa_config_fields = npa_config_line.split('|').collect::<Vec<&str>>();
        npa_config_hashmap.insert(
            npa_config_fields[0].to_string(),
            npa_config_fields[1].to_string(),
        );
    }

    let end_configfile_derive_timer = SystemTime::now();
    let duration = end_configfile_derive_timer
        .duration_since(start_configfile_derive_timer)
        .expect("Could not calculate NPA Config File derive process duration.");
    debug!(
        diag_log,
        "NPA Config File Derive Process Total Duration: {:?}.", duration
    );

    let mut balm_gac_map: HashMap<String, String> = HashMap::new();

    let balm_gac_file_path = match new_buf_rdr(config_param.balm_gac_file_path()){
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input data file: `{}` on location `{}` : {}.",
            config_param.balm_gac_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    for (line_num, lines) in balm_gac_file_path.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.exchange_rate_file(),
                line_num + 1,
                error
            ),
        };

        let gac_data: Vec<&str> = line.split("|").collect();
        if balm_gac_map.contains_key(gac_data[0]){
            balm_gac_map.entry(gac_data[0].to_string()).and_modify(|data| *data = gac_data[1].to_string());
        }else {
            balm_gac_map.insert(gac_data[0].to_string(),gac_data[1].to_string());
        }
    }

    let mut balm_rct_map: HashMap<String, String> = HashMap::new();

    let balm_rct_file = match new_buf_rdr(config_param.balm_rct_file_path()){
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input data file: `{}` on location `{}` : {}.",
            config_param.balm_rct_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    for (line_num, lines) in balm_rct_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.exchange_rate_file(),
                line_num + 1,
                error
            ),
        };
        // let dd = line
        let rct_data: Vec<&str> = line.split("|").collect();
        if balm_rct_map.contains_key(rct_data[0]){
            balm_rct_map.entry(rct_data[0].to_string()).and_modify(|data| *data = rct_data[1].to_string());
        }else {
            balm_rct_map.insert(rct_data[0].to_string(),rct_data[1].to_string());
        }
    }
    
    let mut balm_gam_map: HashMap<BalmGamKey, BalmGamValue> = HashMap::new();

    let balm_gam_file_path = match new_buf_rdr(config_param.balm_gam_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input data file: `{}` on location `{}` : {}.",
            config_param.balm_gam_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let start_ex_rt_read_timer = SystemTime::now();
    let mut ex_rt_map: HashMap<ExrtKey, f64> = HashMap::new();
    let ex_rt_file = match new_buf_rdr(config_param.exchange_rate_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.exchange_rate_file(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in ex_rt_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.exchange_rate_file(),
                line_num + 1,
                error
            ),
        };
        let derived_fields: Vec<&str> = line.split("|").collect();
        let ex_rt_key = ExrtKey::new(derived_fields[0].to_string(), derived_fields[1].to_string());
        ex_rt_map.insert(ex_rt_key, derived_fields[2].parse::<f64>().unwrap_or(1.0));
    }
    let end_ex_rt_read_timer = SystemTime::now();
    let duration = end_ex_rt_read_timer
        .duration_since(start_ex_rt_read_timer)
        .expect("Could not calculate total duration for read timer.");
    log_debug!(
        log,
        "Reading EXCHANGE RATE File, Total Duration: {:?}.",
        duration
    );

    for (line_no, lines) in balm_gam_file_path.lines().enumerate() {
        let fields = match lines {
            Ok(data) => data,
            Err(error) => panic!(
                "Unable to read file `{}` at line no:{}: {}",
                config_param.balm_gam_file_path(),
                line_no + 1,
                error
            ),
        };
        tot_input_acc_encntrd += 1;
        let input_account: Vec<&str> = fields.split('|').collect();

        let balm_gam_key = BalmGamKey {
            acid: input_account[0].to_string(),
            foracid: input_account[1].to_string(),
        };

        let balm_gam_value = BalmGamValue {
            acid: input_account[0].to_string(), 
            clr_bal_amt: input_account[3].parse::<f64>().unwrap_or(0.0).abs(),
            cust_id: input_account[6].to_string(),
            gl_sub_head_code: input_account[13].to_string(),
            schm_code: input_account[14].to_string(),
            acct_crncy_code: input_account[17].to_string(),
            acct_open_date: date_parser.parse(input_account[20]),
        };
        balm_gam_map.insert(balm_gam_key, balm_gam_value);
    }
    let mut balmset: HashSet<BalmGamKey> = HashSet::new();
    let wcdl_file = match new_buf_rdr(config_param.wcdl_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input data file: `{}` on location `{}` : {}.",
            config_param.wcdl_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    // Printing all acid which is present in both WCDL and BALM GAM file.
    for (line_no, lines) in wcdl_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line no:{}: {}",
                config_param.wcdl_file_path(),
                line_no + 1,
                error
            ),
        };

        let input_rec: Vec<&str> = line.split("|").collect();
        let for_acid = input_rec[5].to_string();
        let cust_id = input_rec[0].to_string();
        let mut out_amt = input_rec[10].parse::<f64>().unwrap_or(0.0);
        let due_date = rbdate::NaiveDate::parse_from_str(&input_rec[6], "%d-%m-%Y")
            .unwrap_or(config_param.as_on_date);
        let as_on =
            rbdate::NaiveDate::parse_from_str(&config_param.as_on_date().to_string(), "%d-%m-%Y")
                .unwrap_or(config_param.as_on_date);
        let mut days_diff;
        if due_date >= as_on {
            days_diff = num_days_start_to_end(*config_param.as_on_date(), due_date);
        } else {
            days_diff = num_days_start_to_end(due_date, *config_param.as_on_date());
            days_diff *= -1;
        }
        let npa_classification;
        match npa_data_map.get(&for_acid) {
            Some(typ) => {
                npa_classification = typ.classification.to_owned();
                out_amt = typ.npa_amt;
            }
            None => {
                npa_classification = "0".to_string();
            }
        };

        let cust_hlth_code = match npa_live_hashmap.get(&cust_id) {
            Some(code) => code.to_owned(),
            None => "0".to_string(),
        };

        let cust_npa_class = match npa_config_hashmap.get(&cust_hlth_code) {
            Some(class) => class.to_owned(),
            None => "0".to_string(),
        };
        let final_npa_class = if npa_classification != "0".to_string() {
            npa_classification.to_owned()
        } else if cust_npa_class != "0".to_string() {
            cust_npa_class.to_owned()
        } else {
            "0".to_string()
        };

        let wcdl_bucket_days = if days_diff <= -30 {
            "LN30"
        } else if days_diff > -30 && days_diff <= 0 {
            "UN30"
        } else {
            "G0"
        };
        let def_data = "".to_string();
        let key = BalmGamKey {
            acid: input_rec[4].to_string(),
            foracid: input_rec[5].to_string(),
        };
        let balm_value = BalmGamValue::new();
        tot_input_acc_encntrd += 1;
        if balm_gam_map.contains_key(&key) {
            balmset.insert(key.clone());
            let seg_code = if balm_gac_map.contains_key(&input_rec[4].to_string()) {
                balm_gac_map.get(&input_rec[4].to_string()).unwrap_or(&def_data)
            } else {
                &def_data  
            };

            let final_seg_code = if balm_rct_map.contains_key(seg_code){
                balm_gac_map.get(&input_rec[4].to_string()).unwrap_or(&def_data)
            }else{
                &def_data
            };
            
            let balm_gam_data = balm_gam_map.get(&key).unwrap_or(&balm_value);

            let ex_rate = ex_rt_map
                .get(&ExrtKey {
                    from_currency: config_param.currency().to_owned(),
                    to_currency: input_rec[2].to_string(),
                })
                .unwrap_or(&1.0);

            let acct_num = input_rec[5].to_string() + "/" + input_rec[7];
            let op_line = format!(
                "|{}|{}|{}||{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|||||{}|{}|{}||{}||||||{}|{}|{}|{}|{}|{}|{}\n",
                balm_gam_data.acct_crncy_code,
                ex_rate,
                acct_num,
                input_rec[0],
                input_rec[1],
                balm_gam_data.acct_open_date.format("%d-%m-%Y"),
                input_rec[10],
                input_rec[9],
                out_amt,
                input_rec[3],
                date_parser.parse(input_rec[6]).format("%d-%m-%Y"),
                input_rec[10],
                0,
                date_parser.parse(input_rec[6]).format("%d-%m-%Y"),
                0,
                99,
                date_parser.parse(input_rec[6]).format("%d-%m-%Y"),
                balm_gam_data.schm_code,
                final_npa_class,
                days_diff,
                wcdl_bucket_days,
                balm_gam_data.gl_sub_head_code,
                balm_gam_data.schm_code,
                seg_code,
                final_seg_code
            );
            tot_succ_rec += 1;
            match writer.write_all(op_line.as_bytes()) {
                Ok(val) => val,
                Err(error) => {
                    panic!("Error writing processed data: {:?}", error);
                }
            }
        }
    }

    // Printing remaining acid which is not present in WCLD Ffle from BALM GAM file.
    for (key, value) in balm_gam_map.iter() {
        if balmset.contains(&key) {
            continue;
        }
        let due_date = rbdate::NaiveDate::parse_from_str("31-12-2099", "%d-%m-%Y")
            .unwrap_or(config_param.as_on_date);
        let as_on =
            rbdate::NaiveDate::parse_from_str(&config_param.as_on_date().to_string(), "%d-%m-%Y")
                .unwrap_or(config_param.as_on_date);
        let mut days_diff;
        if due_date >= as_on {
            days_diff = num_days_start_to_end(*config_param.as_on_date(), due_date);
        } else {
            days_diff = num_days_start_to_end(due_date, *config_param.as_on_date());
            days_diff *= -1;
        }

        let for_acid = key.foracid.clone();
        let mut out_amt = value.clr_bal_amt;

        let npa_classification;
        match npa_data_map.get(&for_acid) {
            Some(typ) => {
                npa_classification = typ.classification.to_owned();
                out_amt = typ.npa_amt;
            }
            None => {
                npa_classification = "0".to_string();
            }
        };
        let cust_id = value.cust_id.clone();
        let cust_hlth_code = match npa_live_hashmap.get(&cust_id) {
            Some(code) => code.to_owned(),
            None => "0".to_string(),
        };

        let cust_npa_class = match npa_config_hashmap.get(&cust_hlth_code) {
            Some(class) => class.to_owned(),
            None => "0".to_string(),
        };

        let final_npa_class = if npa_classification != "0".to_string() {
            npa_classification.to_owned()
        } else if cust_npa_class != "0".to_string() {
            cust_npa_class.to_owned()
        } else {
            "0".to_string()
        };

        let wcdl_bucket_days = if days_diff <= -30 {
            "LN30"
        } else if days_diff > -30 && days_diff <= 0 {
            "UN30"
        } else {
            "G0"
        };
        let def_data = "".to_string();
        let acid = &value.acid;
        let seg_code = if balm_gac_map.contains_key(acid) {
            balm_gac_map.get(acid).unwrap_or(&def_data)
        } else {
            &def_data  
        };

        let final_seg_code = if balm_rct_map.contains_key(seg_code){
            balm_gac_map.get(acid).unwrap_or(&def_data)
        }else{
            &def_data
        };

        let ex_rate = ex_rt_map
            .get(&ExrtKey {
                from_currency: config_param.currency().to_owned(),
                to_currency: value.acct_crncy_code.clone(),
            })
            .unwrap_or(&1.0);

        let op_line = format!(
            "|{}|{}|{}||{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|||||{}|{}|{}||{}||||||{}|{}|{}|{}|{}|{}|{}\n",
            value.acct_crncy_code,
            ex_rate,
            key.foracid,
            value.cust_id,
            "",
            value.acct_open_date.format("%d-%m-%Y"),
            value.clr_bal_amt,
            value.clr_bal_amt,
            out_amt,
            0,
            date_parser.parse("31-12-2099").format("%d-%m-%Y"),
            value.clr_bal_amt,
            0,
            date_parser.parse("31-12-2099").format("%d-%m-%Y"),
            0,
            99,
            date_parser.parse("31-12-2099").format("%d-%m-%Y"),
            value.schm_code,
            final_npa_class,
            days_diff,
            wcdl_bucket_days,
            value.gl_sub_head_code,
            value.schm_code,
            seg_code,
            final_seg_code
        );
        tot_succ_rec += 1;
        match writer.write_all(op_line.as_bytes()) {
            Ok(val) => val,
            Err(error) => {
                panic!("Error writing processed data: {:?}", error);
            }
        }
    }

    let health_report = HealthReport::new(
        tot_input_acc_encntrd,
        tot_succ_rec,
        tot_input_acc_encntrd - tot_succ_rec,
        0.0,
        0.0,
        0,
    );
    health_report.gen_health_rpt(config_param.output_file_path());
}
