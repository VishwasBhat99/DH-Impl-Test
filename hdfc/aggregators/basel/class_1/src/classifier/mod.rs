use self::field_appender::{add_derived_fields, add_pass_through, add_std_fields};
use self::structs::CustKey;
use self::writer::{write_classified_data, write_cust_data};
use super::statics::Schema;
use classifier::account_field_names::AccFieldNames;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::{incr_dt_by_mon_presrv_eom, NaiveDate};
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use sdb_io::buf_file_wrtr;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::env;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::SystemTime;

mod account_field_names;
mod currency;
mod date_utils;
mod field_appender;
mod organize;
mod read_std_fields;
mod structs;
mod writer;

pub fn classify(config_params: ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let start_time = SystemTime::now();
    let mut acc_enc = 0;
    let mut acc_succ = 0;

    // init writers
    // init ret and non-ret writers
    let wd_ret_op = format!("{}-wd-ret.txt", config_params.output_file_path());
    let wd_non_ret_op = format!("{}-wd-non-ret.txt", config_params.output_file_path());
    let nwd_ret_op = format!("{}-nwd-ret.txt", config_params.output_file_path());
    let nwd_non_ret_op = format!("{}-nwd-non-ret.txt", config_params.output_file_path());
    let null_cust_typ_op = format!("{}-null-cust-type.txt", config_params.output_file_path());
    let unmapped_cust_id_op = format!("{}-unmapped-cust-id.txt", config_params.output_file_path());
    let as_on_date = config_params.as_on_date();
    // init writers
    let mut wd_ret_writer = match buf_file_wrtr(&wd_ret_op, None) {
        Ok(wrtr) => wrtr,
        Err(error) => {
            panic!(
                "Could not create file: `{}` on location `{}` : {:?}.",
                wd_ret_op,
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    let mut wd_non_ret_writer = match buf_file_wrtr(&wd_non_ret_op, None) {
        Ok(wrtr) => wrtr,
        Err(error) => {
            panic!(
                "Could not create file: `{}` on location `{}` : {:?}.",
                wd_non_ret_op,
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    let mut nwd_ret_writer = match buf_file_wrtr(&nwd_ret_op, None) {
        Ok(wrtr) => wrtr,
        Err(error) => {
            panic!(
                "Could not create file: `{}` on location `{}` : {:?}.",
                nwd_ret_op,
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    let mut nwd_non_ret_writer = match buf_file_wrtr(&nwd_non_ret_op, None) {
        Ok(wrtr) => wrtr,
        Err(error) => {
            panic!(
                "Could not create file: `{}` on location `{}` : {:?}.",
                nwd_non_ret_op,
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    // Writer for accounts where cust type is NULL
    let mut null_cust_typ_writer = match buf_file_wrtr(&null_cust_typ_op, None) {
        Ok(wrtr) => wrtr,
        Err(error) => {
            panic!(
                "Could not create file: `{}` on location `{}` : {:?}.",
                null_cust_typ_op,
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    // Writer for account having unmapped cust id in master file
    let mut unmapped_cust_id_writer = match buf_file_wrtr(&unmapped_cust_id_op, None) {
        Ok(wrtr) => wrtr,
        Err(error) => {
            panic!(
                "Could not create file: `{}` on location `{}` : {:?}.",
                unmapped_cust_id_op,
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    // init NWD product code file
    let nwd_file = match new_buf_rdr(config_params.nwd_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.nwd_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut nwd_prod_codes: Vec<String> = Vec::new();
    for (line_num, lines) in nwd_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.output_file_path(),
                line_num + 1,
                error
            ),
        };
        nwd_prod_codes.push(line.to_string());
    }
    // init bucket schema file
    let bkt_file = match new_buf_rdr(config_params.bkt_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.bkt_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut bkt_def: Vec<Schema> = Vec::new();
    // init exclude alm file
    let exclude_file = match new_buf_rdr(config_params.exclude_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.exclude_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut ex_alm_line: Vec<String> = Vec::new();
    for (line_num, lines) in exclude_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.output_file_path(),
                line_num + 1,
                error
            ),
        };
        ex_alm_line.push(line.to_string());
    }

    for (line_num, lines) in bkt_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.output_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        let from_days = if fields[1].contains("-") {
            -1.0 * get_days(fields[1].replace("-", "").as_str(), as_on_date)
        } else {
            get_days(fields[1], as_on_date)
        };
        let to_days = if fields[2].contains("-") {
            -1.0 * get_days(fields[2].replace("-", "").as_str(), as_on_date)
        } else {
            get_days(fields[2], as_on_date)
        };
        let new_slab = Schema {
            id: fields[0].to_string(),
            from_bkt: from_days,
            to_bkt: to_days,
        };
        bkt_def.push(new_slab);
    }
    // init currency converter
    let currency_converter = currency::create_currency_converter(
        config_params.base_currency(),
        config_params.currency_conversion_file_path(),
    );

    // read cust master file
    let cust_master_file = match new_buf_rdr(config_params.cust_master_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.cust_master_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut cust_master: HashMap<String, String> = HashMap::new();
    for (line_num, lines) in cust_master_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.output_file_path(),
                line_num + 1,
                error
            ),
        };
        if line.contains("~#~") {
            let fields: Vec<&str> = line.split("~#~").collect();
            cust_master.insert(fields[0].to_string(), fields[3].to_string());
        }
    }
    // init account reader
    let mut account_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );
    // init req fields
    let keys = AccFieldNames::new_from_path(config_params.req_fields_file_path());
    // To store encountered cust_id
    let mut cust_ids: Vec<CustKey> = Vec::new();
    let mut total_amount_processed = 0.0;
    let mut total_amount_processed_lcy = 0.0;
    for account in account_reader.iter() {
        acc_enc += 1;
        if skip_account(&account, &keys, &ex_alm_line, &logger) {
            log_warn!(logger, "Some Accounts Skipped!!!");
            continue;
        }
        let mut op = String::new();
        add_std_fields(&mut op, &account, &keys, &logger);
        let cust_id: String = read_std_fields::get_cust_id(&account, &keys, &logger);
        let currency = read_std_fields::get_currency(&account, &keys, &logger);
        let abs_flag = config_params.abs_value_flag().unwrap_or(false);
        let mut amount;
        let mut lcy_amount;
        if config_params.is_consolidated() {
            lcy_amount = read_std_fields::get_amount(&account, &keys, &logger);
            amount = currency_converter.convert_from_lcy(&currency, &lcy_amount, logger);
        } else {
            amount = read_std_fields::get_amount(&account, &keys, &logger);
            lcy_amount = currency_converter.convert_to_lcy(&currency, &amount, logger);
        }
        let cust_type: String;
        if config_params.is_acc_cust_type() != "NA" {
            cust_type =
                read_std_fields::get_cust_type(&account, config_params.is_acc_cust_type(), &logger);
        } else {
            cust_type = match cust_master.get(&cust_id) {
                Some(val) => val.trim().to_string(),
                None => "NA".to_string(),
            };
        }

        let mat_date = read_std_fields::get_mat_date(&account, &keys, &logger);
        let res_days = if config_params.as_on_date > mat_date {
            0.0
        } else {
            rbdate::num_days_start_to_end(config_params.as_on_date, mat_date) as f64
        };
        let bucket_id = organize::get_bucket_id(res_days, &bkt_def);
        let prod_code = read_std_fields::get_prod_code(&account, &keys, &logger);
        let is_nwd = if nwd_prod_codes.contains(&prod_code.to_string()) && config_params.check_wd()
        {
            "TRUE"
        } else if nwd_prod_codes.contains(&cust_id.to_string()) && config_params.check_wd() {
            "TRUE"
        } else {
            "FALSE"
        };
        let is_nwd_final = if is_nwd == "TRUE"
            && res_days > *config_params.residual_maturity_days() as f64
            && config_params.check_wd()
        {
            "TRUE"
        } else {
            "FALSE"
        };

        total_amount_processed_lcy += lcy_amount;
        total_amount_processed += amount;
        if abs_flag == true {
            if lcy_amount < 0.0 {
                lcy_amount = (-1.0) * lcy_amount;
            }
            if amount < 0.0 {
                amount = (-1.0) * amount;
            }
        }

        add_derived_fields(
            &mut op,
            &cust_type,
            amount.to_string(),
            lcy_amount.to_string(),
            res_days.to_string(),
            &bucket_id,
            is_nwd,
            is_nwd_final,
            mat_date.format("%d-%m-%Y").to_string(),
        );
        add_pass_through(&mut op, &account, &keys, &logger);
        op.push('\n');
        let ret_cust_types = config_params.ret_cust_types();

        write_classified_data(
            &config_params,
            op,
            logger,
            is_nwd_final,
            cust_type,
            ret_cust_types,
            &mut null_cust_typ_writer,
            &mut unmapped_cust_id_writer,
            &mut wd_ret_writer,
            &mut wd_non_ret_writer,
            &mut nwd_ret_writer,
            &mut nwd_non_ret_writer,
        );
        acc_succ += 1;
        let cust_key = CustKey {
            cust_id: cust_id,
            currency: currency,
        };
        cust_ids.push(cust_key);
    }
    write_cust_data(cust_ids, &config_params, logger);
    let total_duration = print_return_time_since!(start_time);
    println!("Total amount processed: {:.2}", total_amount_processed);
    println!(
        "Total amount processed (LCY): {:.2}",
        total_amount_processed_lcy
    );
    log_info!(
        logger,
        "Total time for classification-1: {:?}",
        total_duration
    );
    let health_stat = health_report::HealthReport::new(
        acc_enc,
        acc_succ,
        acc_enc - acc_succ,
        total_amount_processed,
        total_amount_processed,
        0,
    );
    health_stat.gen_health_rpt(config_params.output_file_path())
}

fn skip_account(
    account: &AccountWithCFs,
    keys: &AccFieldNames,
    ex_alm_line: &Vec<String>,
    logger: &Logger,
) -> bool {
    let alm_line = match account.get_string_for_key(&keys.skipper) {
        Ok(val) => val.to_string(),
        Err(err) => {
            log_error!(
                logger,
                "Error reading skipper or String: {:?}, Default value NONE taken.",
                err
            );
            "NONE".to_string()
        }
    };
    if ex_alm_line.contains(&alm_line) {
        true
    } else {
        false
    }
}

fn get_days(info: &str, as_on_date: &NaiveDate) -> f64 {
    let mut alpha_code: Vec<&str> = info.split(|c: char| c.is_numeric()).collect();
    alpha_code.retain(|&x| x != "");
    let mut num_code: Vec<&str> = info.split(|c: char| c.is_alphabetic()).collect();
    num_code.retain(|&x| x != "");
    let mut days = 0.0;
    for (i, num_val) in num_code.iter().enumerate() {
        let period = num_val.to_string() + alpha_code[i];
        days += num_days(&period, as_on_date);
    }
    days
}

fn num_days(info: &str, as_on_date: &NaiveDate) -> f64 {
    if info.contains("D") {
        let period: i64 = info
            .trim_matches('D')
            .parse::<i64>()
            .expect("Invalid from day format");
        return period as f64;
    } else if info.contains("M") {
        let period: usize = info
            .trim_matches('M')
            .parse::<usize>()
            .expect("Invalid from month format");
        let new_date = incr_dt_by_mon_presrv_eom(*as_on_date, period)
            .expect("Cannot add month to as on date as per prd slab config");
        return rbdate::num_days_start_to_end(*as_on_date, new_date) as f64;
    } else if info.contains("Y") {
        let period: usize = info
            .trim_matches('Y')
            .parse::<usize>()
            .expect("Invalid from year format");
        let new_date = incr_dt_by_mon_presrv_eom(*as_on_date, period * 12)
            .expect("Cannot add month to as on date as per prd slab config");
        return rbdate::num_days_start_to_end(*as_on_date, new_date) as f64;
    } else {
        panic!("Invalid period type in prd config file.");
    }
}
