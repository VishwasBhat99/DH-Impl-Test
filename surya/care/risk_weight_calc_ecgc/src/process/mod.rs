use self::account_field_names::AccFieldNames;
use self::crm_data::*;
use self::get_rw::get_rw;
use self::get_sub_id::{get_ecgc_type, get_sub_id};
use self::output_data::get_claim_info;
use self::output_data::*;
use self::writer::{write_col_data, write_data};
use config;
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

mod account_field_names;
mod crm_data;
mod get_rw;
mod get_sub_id;
mod output_data;
mod writer;

pub fn calc_rw(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    // Read Files Configuration
    let files_config = config::get_files(config_params.config_file_path());
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
    // Read currency exchange rate file
    let currency_converter = currency::create_currency_converter(
        config_params.base_currency(),
        config_params.exchange_rate_file(),
    );
    // read CRMData into a HashMap
    let mut col_data_map: HashMap<String, Vec<CRMData>> = HashMap::new();
    for file in &files_config.files {
        let claim_info = get_claim_info(&file.input_file_path);
        let claim_id = claim_info[4..8].parse().unwrap_or(0).to_string();
        let col_file_status = std::path::Path::new(config_params.col_file_path()).exists();
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
            }
        }
    }

    let mut total_amount_covered_pre = 0.0;
    let mut total_amount_covered_post = 0.0;
    let mut covered_amt_mapping: HashMap<String, f64> = HashMap::new();

    for file in &files_config.files {
        let mut file_rdr_crm: Reader =
            reader::Reader::new_at_path(&file.metadata_file_path, &file.input_file_path);
        let cover_rules_file_path = format!("{}-ecgc-cover.txt", file.rules_file_path);
        let ecgc_cover_rules = AggRules::new_from_path(&cover_rules_file_path, &file_rdr_crm);
        let ecgc_type_rules =
            AggRules::new_from_path(&files_config.ecgc_type_rules_file_path, &file_rdr_crm);
        let acc_keys = AccFieldNames::new_from_path(&file.req_fields_file_path);
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
            let currency = match account.get_string_for_key(&acc_keys.currency) {
                Ok(val) => val.to_string(),
                Err(_) => format!(""),
            };
            let (ost_bal_lcy, _) = get_acc_bal(
                &account,
                &acc_keys,
                currency.to_string(),
                &currency_converter,
                file.is_consolidated,
                file.is_negative,
                logger,
            );
            let ecgc_type = get_ecgc_type(&account, &ecgc_type_rules);
            let amount_covered = match ecgc_cover_rules.llg_for_acc(&account) {
                Some(val) => {
                    // as percentage is represented as i32, so to capture fractional percentage, rules are written as 9876 for 98.76%
                    let cover_prcnt = val.llg as f64 / 100.0;
                    if ost_bal_lcy >= *config_params.amt_set_limit() && ecgc_type == "PRE" {
                        ((*config_params.amt_set_limit() * cover_prcnt) / 100.0)
                            + (((ost_bal_lcy - *config_params.amt_set_limit())
                                * (cover_prcnt - config_params.cutoff_cover_prcnt()))
                                / 100.0)
                    } else {
                        (ost_bal_lcy * cover_prcnt) / 100.0
                    }
                }
                None => (ost_bal_lcy * config_params.default_cover_prcnt()) / 100.0,
            };

            if ecgc_type == "PRE" {
                total_amount_covered_pre += amount_covered;
            } else {
                total_amount_covered_post += amount_covered;
            };

            if covered_amt_mapping.contains_key(&acc_id) {
                log_warn!(
                    logger,
                    "Account ID {} encountered twice!! May result in covered amount mismatch!!.",
                    acc_id
                )
            } else {
                covered_amt_mapping.insert(acc_id.to_string(), amount_covered);
            }

            col_data_map.entry(acc_id).and_modify(|mut data| {
                update_data(&mut data, acc_ccy, acc_mat_date, &config_params)
            });
        }
    }
    log_info!(
        logger,
        "Number of records in collateral map: {} and in collateral input file. ",
        col_data_map.len()
    );
    log_debug!(
        diag_logger,
        "Covered Amount Mapping: {:#?}",
        covered_amt_mapping
    );
    let mut acc_enc = 0;
    let mut acc_succ = 0;
    let mut processed_acc_num: HashMap<String, i64> = HashMap::new();
    log_info!(
        logger,
        "Total covered amount pre-shipment: {}",
        total_amount_covered_pre
    );
    log_info!(
        logger,
        "Total covered amount post-shipment: {}",
        total_amount_covered_post
    );
    for file in files_config.files {
        let mut file_rdr: Reader =
            reader::Reader::new_at_path(&file.metadata_file_path, &file.input_file_path);
        let rw_rules_file_path = format!("{}-rw.txt", file.rules_file_path);
        let ecgc_sub_claim_rules_file_path = format!("{}-ecgc-sub-claim.txt", file.rules_file_path);
        let rw_rules = AggRules::new_from_path(&rw_rules_file_path, &file_rdr);
        let ecgc_sub_claim_rules =
            AggRules::new_from_path(&ecgc_sub_claim_rules_file_path, &file_rdr);
        let sub_claim_rules_file_path = format!("{}-sub-claim.txt", file.rules_file_path);
        let sub_claim_rules = AggRules::new_from_path(&sub_claim_rules_file_path, &file_rdr);
        let ecgc_type_rules =
            AggRules::new_from_path(&files_config.ecgc_type_rules_file_path, &file_rdr);
        let acc_keys = AccFieldNames::new_from_path(&file.req_fields_file_path);
        for account in file_rdr.iter() {
            acc_enc += 1;
            let claim_info = get_claim_info(&file.input_file_path);
            // get classification id
            let rw_id = get_rw(&account, &rw_rules, &file);
            // get sub classification id
            let ecgc_sub_id = get_sub_id(&account, &ecgc_sub_claim_rules, &file);
            let sub_id = get_sub_id(&account, &sub_claim_rules, &file);
            let currency = match account.get_string_for_key(&acc_keys.currency) {
                Ok(val) => val.to_string(),
                Err(_) => format!(""),
            };
            let (ost_bal_lcy, _ost_bal_ccy) = get_acc_bal(
                &account,
                &acc_keys,
                currency.to_string(),
                &currency_converter,
                file.is_consolidated,
                file.is_negative,
                logger,
            );
            let target_currency = CurrencyExchange {
                from_ccy: currency,
                to_ccy: config_params.base_currency().to_string(),
            };
            let exchange_rate = currency_converter.exchange_rate(&target_currency, logger);
            let account_number = match account.get_string_for_key(&acc_keys.account_number) {
                Ok(val) => val.to_string(),
                Err(_) => format!(""),
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
            let final_rw: f64 = rw_id[4..8].to_string().parse().unwrap_or(0.0);
            // Since we capture all account data in main mod.rs at lines 157-165, we expect a value for each account number
            let amt_covered = covered_amt_mapping
                .get(&account_number)
                .expect("Unxpected loss of account information!!");
            let ecgc_type = get_ecgc_type(&account, &ecgc_type_rules);
            let total_amount_covered;
            let max_covered = if ecgc_type == "PRE" {
                total_amount_covered = total_amount_covered_pre;
                if total_amount_covered < *config_params.cover_thrs_bal_pre_shipment() {
                    total_amount_covered
                } else {
                    *config_params.cover_thrs_bal_pre_shipment()
                }
            } else {
                total_amount_covered = total_amount_covered_post;
                if total_amount_covered < *config_params.cover_thrs_bal_post_shipment() {
                    total_amount_covered
                } else {
                    *config_params.cover_thrs_bal_post_shipment()
                }
            };
            let final_amt_covered = (amt_covered / total_amount_covered) * max_covered;
            let final_amt_uncovered = ost_bal_lcy - final_amt_covered;
            log_debug!(
                diag_logger,
                "{}|{}|{}|{}|{}|{}",
                account_number,
                ost_bal_lcy,
                amt_covered,
                final_amt_covered,
                final_amt_uncovered,
                ecgc_type
            );
            let rw_amt_covered =
                (final_amt_covered * config_params.default_risk_weight_covered()) / 100.0;
            let rw_amt_uncovered = (final_amt_uncovered * final_rw) / 100.0;
            processed_acc_num
                .entry(account_number.to_string())
                .and_modify(|count| *count += 1)
                .or_insert(1);
            let new_account_number = match processed_acc_num.get(&account_number) {
                Some(count) => format!("{}-{}", account_number, count),
                None => format!("{}", account_number),
            };
            let op_data_covered = get_op_data(
                &account,
                &acc_keys,
                &claim_info,
                &file.src_file_name,
                final_amt_covered,
                final_amt_covered,
                &new_account_number,
                tot_col_amt_lcy,
                crm_amt_lcy,
                exchange_rate,
                rw_id.to_string(),
                ecgc_sub_id.to_string(),
                *config_params.default_risk_weight_covered(),
                final_amt_covered,
                rw_amt_covered,
                &config_params,
            );
            processed_acc_num
                .entry(account_number.to_string())
                .and_modify(|count| *count += 1)
                .or_insert(1);
            let new_account_number = match processed_acc_num.get(&account_number) {
                Some(count) => format!("{}-{}", account_number, count),
                None => format!("{}", account_number),
            };
            let op_data_uncovered = get_op_data(
                &account,
                &acc_keys,
                &claim_info,
                &file.src_file_name,
                final_amt_uncovered,
                final_amt_uncovered,
                &new_account_number,
                tot_col_amt_lcy,
                crm_amt_lcy,
                exchange_rate,
                rw_id.to_string(),
                sub_id.to_string(),
                final_rw,
                final_amt_uncovered,
                rw_amt_uncovered,
                &config_params,
            );
            write_data(op_data_covered, &mut output_file);
            write_data(op_data_uncovered, &mut output_file);
            acc_succ += 1;
        }
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
    is_consolidated: bool,
    is_negative: bool,
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
    let mut bal_2 = currency_converter.convert(&account, &currency, bal_1, is_consolidated, logger);
    if is_negative {
        bal_1 *= -1.0;
        bal_2 *= -1.0;
    }
    let bal_ccy;
    let bal_lcy;
    if is_consolidated {
        bal_ccy = bal_2;
        bal_lcy = bal_1;
    } else {
        bal_ccy = bal_1;
        bal_lcy = bal_2;
    }
    (bal_lcy, bal_ccy)
}
