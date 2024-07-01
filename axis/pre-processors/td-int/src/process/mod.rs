use self::io::*;
use configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod io;
mod structs;
use self::structs::{BalmIcv, BalmIcvVal, TdInt};
use health_report::HealthReport;
use macros;
use rbdate::DateParser;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;
use std::time::SystemTime;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let mut op_writer = get_writer(config_params.output_file_path());
    let date_parser: DateParser = DateParser::new("%d-%m-%Y".to_string(), true);
    let tot_amt = 0.0;

    let mut final_out_td: HashMap<String, TdInt> = HashMap::new();

    let gam_input = File::open(&config_params.input_file_gam()).expect("Could not open GAM File.");
    let gam_input_reader = BufReader::new(gam_input);

    let tam_input = File::open(&config_params.input_file_tam()).expect("Could Not open TAM file");
    let itc_input = File::open(&config_params.input_file_itc()).expect("Could Not open ITC file");
    let icv_input = File::open(&config_params.input_file_icv()).expect("Could Not open ICV file");

    //GAM
    let start_gam_reader = SystemTime::now();
    for (_index, line) in gam_input_reader.lines().enumerate() {
        let line = line.expect("Could not read line in GAM file.").to_string();
        let input_fields: Vec<&str> = line.split('|').collect();
        let mut new_td = TdInt::new();
        if input_fields[17].trim() != "TDA" {
            new_td.schm_type = "TDA".to_string();
            new_td.acct_crncy_code = input_fields[17].trim().to_string();
            final_out_td.insert(input_fields[0].trim().to_string(), new_td);
        }
    }
    let end_gam_reader = SystemTime::now();
    let total_gam_duration = end_gam_reader
        .duration_since(start_gam_reader)
        .expect("Could not calculate total duration for processing GAM.");
    log_debug!(
        logger,
        "Reading and processing GAM File, Total Duration: {:?}.",
        total_gam_duration
    );

    let tam_input_reader = BufReader::new(tam_input);
    let start_tam_reader = SystemTime::now();
    //TAM
    for (_index, line) in tam_input_reader.lines().enumerate() {
        let line = line.expect("Could not read line in TAM file.").to_string();
        let input_fields: Vec<&str> = line.split('|').collect();
        let mut td_acc;
        if final_out_td.contains_key(&input_fields[0].trim().to_string()) {
            td_acc = final_out_td
                .get_mut(&input_fields[0].trim().to_string())
                .expect("Could not fetch struct for an account from TAM file.");
        } else {
            log_debug!(
                logger,
                "Could not find struct for an account in TAM for ID: {}",
                &input_fields[0]
            );
            continue;
        }
        td_acc.deposit_period_mths = input_fields[3]
            .trim()
            .to_string()
            .parse::<f64>()
            .unwrap_or(0.0);
        td_acc.deposit_period_days = input_fields[4]
            .trim()
            .to_string()
            .parse::<f64>()
            .unwrap_or(0.0);
        td_acc.deposit_amount = input_fields[7]
            .trim()
            .to_string()
            .parse::<f64>()
            .unwrap_or(0.0);
        td_acc.deposit_type = input_fields[10].trim().to_string();
        td_acc.spl_catg_ind = input_fields[9].trim().to_string();
        td_acc.open_effective_date = date_parser.parse(input_fields[5].trim());
        td_acc.deposit_status = input_fields[1].trim().to_string();
        if !input_fields[6].is_empty() {
            td_acc.maturity_date = date_parser.parse(input_fields[6].trim());
        } else {
            td_acc.maturity_date =
                date_parser.parse(&config_params.as_on_date().format("%d-%m-%Y").to_string());
        }
        td_acc.rcre_time = date_parser.parse(input_fields[8].trim());
        td_acc.auto_renewed_counter = input_fields[11].trim().parse::<i64>().unwrap_or(0);
    }

    let end_tam_reader = SystemTime::now();
    let total_tam_duration = end_tam_reader
        .duration_since(start_tam_reader)
        .expect("Could not calculate total duration for TAM.");
    log_debug!(
        logger,
        "Reading and processing tam File, Total Duration: {:?}.",
        total_tam_duration
    );
    let itc_input_reader = BufReader::new(itc_input);
    let start_itc_reader = SystemTime::now();
    //ITC
    for (_index, line) in itc_input_reader.lines().enumerate() {
        let line = line.expect("Could Not Read Line").to_string();
        let input_fields: Vec<&str> = line.split('|').collect();
        let mut td_acc;
        if final_out_td.contains_key(&input_fields[0].trim().to_string()) {
            td_acc = final_out_td
                .get_mut(&input_fields[0].trim().to_string())
                .expect("Could not find struct for an account in ITC ");
        } else {
            log_debug!(
                logger,
                "Could not find struct for an account in ITC for acid:{}",
                &input_fields[0]
            );
            continue;
        }
        if date_parser.parse(input_fields[3].trim()) <= td_acc.open_effective_date
            && date_parser.parse(input_fields[16].trim()) >= td_acc.open_effective_date
            && input_fields[17].to_string().parse::<i64>().unwrap_or(0) >= td_acc.int_tbl_ver_num
            && date_parser.parse(input_fields[15].trim()) >= td_acc.lchg_time
        {
            td_acc.lchg_time = date_parser.parse(input_fields[15].trim());
            td_acc.int_tbl_code = input_fields[2].trim().to_string();
            td_acc.cust_cr_pref_pcnt = input_fields[5]
                .trim()
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0);
            td_acc.id_cr_pref_pcnt = input_fields[7]
                .trim()
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0);
            td_acc.min_int_pcnt_cr = input_fields[9]
                .trim()
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0);
            td_acc.max_int_pcnt_cr = input_fields[11]
                .trim()
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0);
            td_acc.int_tbl_ver_num = input_fields[17]
                .trim()
                .to_string()
                .parse::<i64>()
                .unwrap_or(0);
            td_acc.nrml_int_pcnt = input_fields[22]
                .trim()
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0);
            td_acc.base_differential_exists = if input_fields[20].trim() == "Null" {
                "N".to_string()
            } else {
                input_fields[20].trim().to_string()
            };
            td_acc.base_pcnt = 0.0;
        }
    }
    let end_itc_reader = SystemTime::now();
    let total_itc_duration = end_itc_reader
        .duration_since(start_itc_reader)
        .expect("Could not calculate total duration for ITC.");
    log_debug!(
        logger,
        "Reading and processing ITC File, Total Duration: {:?}.",
        total_itc_duration
    );

    let icv_input_reader = BufReader::new(icv_input);
    let mut balm_icv_data: HashMap<BalmIcv, Vec<BalmIcvVal>> = HashMap::new();
    let start_icv_reader = SystemTime::now();
    //ICV
    for (_index, line) in icv_input_reader.lines().enumerate() {
        let line = line.expect("Could Not Read Line in ICV file.").to_string();
        let input_fields: Vec<&str> = line.split('|').collect();
        let new_balm_icv = BalmIcv {
            crncy_code: input_fields[9].trim().to_string(),
            int_tbl_code: input_fields[5].trim().to_string(),
        };

        let new_balm_icv_val = BalmIcvVal {
            start_date: date_parser.parse(input_fields[6].trim()),
            end_date: date_parser.parse(input_fields[7].trim()),
            int_version: input_fields[0]
                .trim()
                .to_string()
                .parse::<i64>()
                .unwrap_or(0),
            int_tbl_ver_num: input_fields[12]
                .trim()
                .to_string()
                .parse::<i64>()
                .unwrap_or(0),
            base_pcnt_cr: input_fields[1]
                .trim()
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0),
        };

        if let std::collections::hash_map::Entry::Vacant(e) =
            balm_icv_data.entry(new_balm_icv.to_owned())
        {
            e.insert(vec![new_balm_icv_val.to_owned()]);
        } else {
            let update_balm_icv_val = balm_icv_data
                .get_mut(&new_balm_icv)
                .expect("Could not get field from balm ICV");
            update_balm_icv_val.push(new_balm_icv_val);
        }
    }

    let end_icv_reader = SystemTime::now();
    let total_icv_duration = end_icv_reader
        .duration_since(start_icv_reader)
        .expect("Could not calculate total duration for ICV.");
    log_debug!(
        logger,
        "Reading and processing icv File, Total Duration: {:?}.",
        total_icv_duration
    );

    let start_update_write_reader = SystemTime::now();
    //update icv data in intrate buffer
    for (key, mut value) in final_out_td {
        if value.schm_type == "TDA" && value.base_differential_exists == "Y" {
            let mut updated_balm_icv_val: Vec<BalmIcvVal> = Vec::new();
            let new_balm_icv = BalmIcv {
                crncy_code: value.acct_crncy_code.to_owned(),
                int_tbl_code: value.int_tbl_code.to_owned(),
            };
            let new_balm_icv_val_int = BalmIcvVal {
                start_date: date_parser.parse("31-01-2199"),
                end_date: date_parser.parse("31-01-2199"),
                int_version: 0,
                int_tbl_ver_num: 0,
                base_pcnt_cr: 0.,
            };
            let mut new_balm_icv_val: Vec<BalmIcvVal> = vec![new_balm_icv_val_int];
            if balm_icv_data.contains_key(&new_balm_icv) {
                new_balm_icv_val = balm_icv_data
                    .get(&new_balm_icv)
                    .expect("Could not fetch ICV data.")
                    .to_vec();
            } else {
                log_debug!(
                    logger,
                    "Could not find struct for an ICV in ICV={:?}",
                    new_balm_icv
                );
            }

            let mut max_int_ver = 0;
            let mut max_int_tbl_ver = 0;
            let mut base_per_cr = 0.0;
            for val in new_balm_icv_val {
                if val.start_date <= value.open_effective_date
                    && val.end_date >= value.open_effective_date
                    && val.int_tbl_ver_num >= max_int_tbl_ver
                {
                    max_int_tbl_ver = val.int_tbl_ver_num;
                    if val.int_version >= max_int_tbl_ver {
                        max_int_ver = val.int_version;
                    }
                    base_per_cr = val.base_pcnt_cr;
                    updated_balm_icv_val.push(val);
                }
            }
            value.base_pcnt = base_per_cr;
            value.int_tbl_ver_num = max_int_ver;
        }
        //write to OP
        writeln!(
            op_writer,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            key,
            value.del_flg,
            value.open_effective_date.format("%d-%m-%Y"),
            value.schm_type,
            value.int_tbl_code,
            value.int_version,
            value.int_tbl_ver_num,
            value.min_int_pcnt_cr,
            value.max_int_pcnt_cr,
            value.cust_cr_pref_pcnt,
            value.id_cr_pref_pcnt,
            value.nrml_int_pcnt,
            value.id_dr_pref_pcnt,
            value.base_int_tbl_code,
            value.base_pcnt_dr,
            value.base_pcnt_cr,
            value.base_pcnt,
            value.deposit_period_mths,
            value.deposit_period_days,
            value.deposit_amount,
            value.acct_crncy_code,
            value.deposit_type,
            value.spl_catg_ind,
            value.nrml_int_pcnt_cr,
            value.base_differential_exists,
            value.deposit_status,
            value.maturity_amount,
            value.maturity_date.format("%d-%m-%Y"),
            value.rcre_time.format("%d-%m-%Y"),
            value.auto_renewed_counter
        )
        .expect("Unable to write account to output file.");
    }

    let end_update_write_reader = SystemTime::now();
    let total_update_write_duration = end_update_write_reader
        .duration_since(start_update_write_reader)
        .expect("Could not calculate total duration for update_write.");
    log_debug!(
        logger,
        "Reading and processing update_write File, Total Duration: {:?}.",
        total_update_write_duration
    );

    let health_report = HealthReport::new(0, 0, 0, tot_amt, tot_amt, 0);
    health_report.gen_health_rpt(config_params.output_file_path());
}
