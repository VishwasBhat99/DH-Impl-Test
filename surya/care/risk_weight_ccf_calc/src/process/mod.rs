use self::account_field_names::AccFieldNames;
use self::crm_data::*;
use self::get_ccf_prcnt::get_ccf_prcnt;
use self::get_rw::get_rw;
use self::get_sub_id::get_sub_id;
use self::output_data::get_claim_info;
use self::output_data::*;
use self::writer::{write_col_data, write_data};
use configuration_parameters::ConfigurationParameters;
use currency;
use currency::currency_converter::CurrencyConverter;
use currency::CurrencyExchange;
use macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use sdb_dyn_proto_rdr::reader::Reader;
use slog::Logger;
use std::collections::HashMap;
use std::io::BufRead;
use std::path::Path;

mod account_field_names;
mod crm_data;
mod get_ccf_prcnt;
mod get_rw;
mod get_sub_id;
mod output_data;
mod writer;

pub fn calc_rw(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    // init output writer
    let op_file_path = format!("{}-rw.txt", config_params.output_file_path());
    let mut output_file = match sdb_io::buf_file_wrtr(&op_file_path, None) {
        Ok(create) => create,
        Err(_) => {
            panic!(
                "Could not create output file: `{}`.",
                config_params.output_file_path(),
            );
        }
    };
    // init output crm writer
    let op_crm_file_path = format!("{}-crm.txt", config_params.output_file_path());
    let mut output_crm_file = match sdb_io::buf_file_wrtr(&op_crm_file_path, None) {
        Ok(create) => create,
        Err(_) => {
            panic!(
                "Could not create output file: `{}`.",
                config_params.output_file_path(),
            );
        }
    };
    let acc_keys = AccFieldNames::new_from_path(config_params.req_fields_file_path());
    let mut file_rdr_crm: Reader = reader::Reader::new_at_path(
        config_params.metadata_file_path(),
        config_params.input_file_path(),
    );
    let rw_rules_file_path = format!("{}-rw.txt", config_params.rules_file_path());
    let sub_claim_rules_file_path = format!("{}-sub-claim.txt", config_params.rules_file_path());
    let ccf_rules_file_path = format!("{}-ccf.txt", config_params.rules_file_path());

    if !Path::new(ccf_rules_file_path.as_str()).exists() {
        match std::fs::File::create(ccf_rules_file_path.as_str()) {
            Ok(_dt) => {}
            Err(err) => {
                log_info!(logger, "{}", err)
            }
        }
    }
    let ccf_rules = AggRules::new_from_path(&ccf_rules_file_path, &file_rdr_crm);
    let rw_rules = AggRules::new_from_path(&rw_rules_file_path, &file_rdr_crm);
    let sub_claim_rules = AggRules::new_from_path(&sub_claim_rules_file_path, &file_rdr_crm);
    // Read currency exchange rate file
    let currency_converter = currency::create_currency_converter(
        config_params.base_currency(),
        config_params.exchange_rate_file(),
    );
    // read CRMData into a HashMap
    let mut col_data_map: HashMap<String, Vec<CRMData>> = HashMap::new();
    let claim_info = get_claim_info(config_params);
    let claim_id = claim_info[4..8].parse().unwrap_or(0).to_string();
    let col_file_status = std::path::Path::new(config_params.col_file_path()).exists();
    let mut line_count = 0;
    if col_file_status {
        let col_rdr = match sdb_io::new_buf_rdr(config_params.col_file_path()) {
            Ok(r) => r,
            Err(e) => panic!(format!(
                "Cannot read file at path: '{}', Error: '{}'",
                config_params.col_file_path(),
                e
            )),
        };
        for (line_num, lines) in col_rdr.lines().enumerate() {
            let line = match lines {
                Ok(line) => line,
                Err(error) => panic!(
                    "Unable to read rules file at line number: `{}` : {}",
                    line_num + 1,
                    error
                ),
            };
            let col_data: CRMData = get_crm_data(config_params.as_on_date(), line, &claim_id);
            if col_data_map.contains_key(&col_data.acc_id) {
                col_data_map
                    .entry(col_data.acc_id.to_string())
                    .and_modify(|data| data.push(col_data));
            } else {
                let mut data = Vec::new();
                let acc_key = col_data.acc_id.to_string();
                data.push(col_data);
                col_data_map.insert(acc_key, data);
            }
            line_count += 1;
        }
    }
    log_info!(
        logger,
        "Number of records in collateral map: {} and in collateral input file: {} ",
        col_data_map.len(),
        line_count
    );
    // read ProvData into a HashMap
    let mut prov_data_map: HashMap<String, f64> = HashMap::new();
    let prov_file_status = std::path::Path::new(config_params.prov_file_path()).exists();
    if prov_file_status {
        let prov_rdr = match sdb_io::new_buf_rdr(config_params.prov_file_path()) {
            Ok(r) => r,
            Err(e) => panic!(format!(
                "Cannot read file at path: '{}', Error: '{}'",
                config_params.prov_file_path(),
                e
            )),
        };
        for (line_num, lines) in prov_rdr.lines().enumerate() {
            let line = match lines {
                Ok(line) => line,
                Err(error) => panic!(
                    "Unable to read rules file at line number: `{}` : {}",
                    line_num + 1,
                    error
                ),
            };
            let line_info: Vec<&str> = line.split('|').collect();
            prov_data_map.insert(
                line_info[0].to_string(),
                line_info[1].parse().unwrap_or(0.0),
            );
        }
    }
    log_info!(
        logger,
        "Number of records in provisional data map: {}.",
        prov_data_map.len(),
    );
    for account in file_rdr_crm.iter() {
        let acc_id = match account.get_string_for_key(&acc_keys.account_number) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        };
        let acc_ccy = match account.get_string_for_key(&acc_keys.currency) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        };
        let acc_mat_date = match account.get_i64_for_key(&acc_keys.account_maturity_date) {
            Ok(val) => naivedate_from_timestamp(val),
            Err(_) => naivedate_from_timestamp(0),
        };

        col_data_map
            .entry(acc_id)
            .and_modify(|mut data| update_data(&mut data, acc_ccy, acc_mat_date, &config_params));
    }
    let mut file_rdr: Reader = reader::Reader::new_at_path(
        config_params.metadata_file_path(),
        config_params.input_file_path(),
    );
    let mut acc_enc = 0;
    let mut acc_succ = 0;
    let mut processed_acc_num: HashMap<String, i64> = HashMap::new();
    for account in file_rdr.iter() {
        acc_enc += 1;
        // get CCF Percentage
        let (ccf_rule_id, ccf_prcnt) = get_ccf_prcnt(&account, &ccf_rules, &config_params);
        // get classification id
        let rw_id = get_rw(&account, &rw_rules, &config_params);
        // get sub classification id
        let sub_id = get_sub_id(&account, &sub_claim_rules, &config_params);
        let currency = match account.get_string_for_key(&acc_keys.currency) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        };
        let (ost_bal_lcy, ost_bal_ccy) = get_acc_bal(
            &account,
            &acc_keys,
            currency.to_string(),
            &currency_converter,
            config_params,
            logger,
        );
        let target_currency = CurrencyExchange {
            from_ccy: currency,
            to_ccy: config_params.base_currency().to_string(),
        };
        let exchange_rate = currency_converter.exchange_rate(&target_currency, logger);
        let mut account_number = match account.get_string_for_key(&acc_keys.account_number) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        };
        processed_acc_num
            .entry(account_number.to_string())
            .and_modify(|count| *count += 1)
            .or_insert(1);
        account_number = match processed_acc_num.get(&account_number) {
            Some(count) => format!("{}-{}", account_number, count),
            None => format!("{}", account_number),
        };
        let mut tot_col_amt_lcy = 0.0;
        let mut crm_amt_lcy = 0.0;
        if let Some(col_data) = col_data_map.get(&account_number) {
            for data in col_data {
                tot_col_amt_lcy += data.tot_col_amt_lcy;
                crm_amt_lcy += data.crm_amt_lcy;
                write_col_data(data, &mut output_crm_file);
            }
        }
        let op_data = get_op_data(
            &account,
            &acc_keys,
            ost_bal_lcy,
            ost_bal_ccy,
            account_number,
            tot_col_amt_lcy,
            crm_amt_lcy,
            exchange_rate,
            rw_id.to_string(),
            sub_id.to_string(),
            ccf_rule_id,
            ccf_prcnt,
            &mut prov_data_map,
            &config_params,
        );
        write_data(op_data, &mut output_file);
        acc_succ += 1;
    }

    // TODO: use health check lib
    println!("Total account encountered: {}", acc_enc);
    println!("Total account processed: {}", acc_succ);
}

fn get_acc_bal(
    account: &AccountWithCFs,
    acc_keys: &AccFieldNames,
    currency: String,
    currency_converter: &CurrencyConverter,
    config_params: &ConfigurationParameters,
    logger: &Logger,
) -> (f64, f64) {
    let mut bal_1 = match account.get_f64_for_key(&acc_keys.ost_bal_lcy) {
        Ok(val) => val,
        Err(_) => {
            let def = "0.0".to_string();
            let amt = account
                .get_string_for_key(&acc_keys.ost_bal_lcy)
                .unwrap_or(&def);
            amt.parse().unwrap_or(0.0)
        }
    };
    let mut bal_2 = currency_converter.convert(
        &account,
        &currency,
        bal_1,
        config_params.is_consolidated(),
        logger,
    );
    if config_params.is_negative() {
        bal_1 *= -1.0;
        bal_2 *= -1.0;
    }
    let bal_ccy;
    let bal_lcy;
    if config_params.is_consolidated() {
        bal_ccy = bal_2;
        bal_lcy = bal_1;
    } else {
        bal_ccy = bal_1;
        bal_lcy = bal_2;
    }
    (bal_lcy, bal_ccy)
}
