use self::account_field_names::metadata_reader;
use self::buff_reader_writer::{buff_reader, buff_writer};
use aggregator::account_field_names::AccFieldNames;
use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use macros;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::account_with_cfs::get_field_value;

use self::llg_implmentation::{llg_for_cf_account, llg_for_txt_account};
use self::output::{AmountStore, OutputData};
use sdb_agg_rules::agg_rules::AggRules;
use sdb_agg_rules_txt::agg_rules::AggRules as AggRules_txt;
use slog::Logger;
use std::collections::HashMap;
use std::fs::{self, metadata};
use std::io::{BufRead, Write};
use std::time::SystemTime;

mod account_field_names;
mod buff_reader_writer;
mod llg_implmentation;
mod llg_keys;
mod output;

pub fn aggregate(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let mut tot_rec = 0;
    let mut succ_rec = 0;

    let keys = AccFieldNames::new_from_path(config_params.req_field_path());

    let conv_map = get_exchange_rate_map(
        config_params.base_currency().to_string(),
        config_params.exchange_rate_file_path().to_string(),
    );
    let mut bkt_writer = buff_writer(config_params.output_file_path());
    let llg_id_list: Vec<&str> = config_params.llg_id_list().split(",").collect();
    let mut op_str = String::new();

    if config_params.by_bucket_struct() == "Y" {
        let bkt_map = get_maturity_bucket_scheme_map(
            &config_params.maturity_bkt_def_file_path().to_string(),
            config_params.bucket_scheme_id(),
            config_params.as_on_date(),
        );
        let input_reader = reader::Reader::new_at_path(
            config_params.metadata_file_path(),
            config_params.input_file_path(),
        );
        let rules = AggRules_txt::new_from_path(config_params.rules_file_path(), &input_reader);

        let input_record = buff_reader(config_params.input_file_path());
        let mut output_map: HashMap<String, OutputData> = HashMap::new();
        for (line_num, lines) in input_record.lines().enumerate() {
            tot_rec += 1;
            let line = match lines {
                Ok(line) => line,
                Err(error) => panic!(
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_params.input_file_path(),
                    line_num + 1,
                    error
                ),
            };
            let llg = llg_for_txt_account(&line.to_string(), &rules, &input_reader, config_params)
                .to_string();
            let split_record: Vec<&str> = line.split('|').collect();
            if split_record.len() != 207 {
                continue;
            }
            let metadata_map = metadata_reader(config_params.metadata_file_path());
            let amt_col_num = metadata_map
                .get(&keys.amt_column_no_start_203)
                .unwrap_or(&5)
                - 1;
            if !(0..=203).contains(&amt_col_num) {
                panic!("Amount column is not passed in required fields correctly. Account passed: `{}`",amt_col_num);
            }

            let neg_abs = if llg_id_list.contains(&llg.as_str()) {
                -1.0
            } else {
                1.0
            };

            let amt_data = split_record.split_at(amt_col_num as usize);
            let mut bkt1_value: f64 = 0.0;
            let mut bkt2_value: f64 = 0.0;
            let mut bkt3_value: f64 = 0.0;
            let mut bkt4_value: f64 = 0.0;
            let mut op_data = OutputData::new();

            let dim1_col = (metadata_map.get(&keys.dimid1).unwrap_or(&0) - 1) as usize;

            let dim1 = match split_record.get(dim1_col) {
                Some(value) => {
                    if value == &"0" {
                        0.to_string()
                    } else {
                        value.to_string()
                    }
                }
                None => 0.to_string(),
                // }
            };

            let dim2_col = (metadata_map.get(&keys.dimid2).unwrap_or(&0) - 1) as usize;

            let dim2 = match split_record.get(dim2_col) {
                Some(value) => {
                    if value == &"0" {
                        0.to_string()
                    } else {
                        value.to_string()
                    }
                }
                None => 0.to_string(),
                // }
            };

            let dim3_col = (metadata_map.get(&keys.dimid3).unwrap_or(&0) - 1) as usize;

            let dim3 = match split_record.get(dim3_col) {
                Some(value) => {
                    if value == &"0" {
                        0.to_string()
                    } else {
                        value.to_string()
                    }
                }
                None => 0.to_string(),
                // }
            };

            let dim4_col = (metadata_map.get(&keys.dimid4).unwrap_or(&0) - 1) as usize;

            let dim4 = match split_record.get(dim4_col) {
                Some(value) => {
                    if value == &"0" {
                        0.to_string()
                    } else {
                        value.to_string()
                    }
                }
                None => 0.to_string(),
                // }
            };

            let dim5_col = (metadata_map.get(&keys.dimid5).unwrap_or(&0) - 1) as usize;

            let dim5 = match split_record.get(dim5_col) {
                Some(value) => {
                    if value == &"0" {
                        0.to_string()
                    } else {
                        value.to_string()
                    }
                }
                None => 0.to_string(),
                // }
            };

            op_data.asondt = NaiveDate::parse_from_str(split_record[0], "%d-%m-%Y")
                .unwrap_or(*config_params.as_on_date());
            op_data.llgid = llg;
            op_data.country = split_record[1].to_string();
            op_data.ccy = split_record[2].to_string();
            op_data.bktschemeid = config_params.bucket_scheme_id().to_string();
            op_data.dim1itemid = dim1;
            op_data.dim2itemid = dim2;
            op_data.dim3itemid = dim3;
            op_data.dim4itemid = dim4;
            op_data.dim5itemid = dim5;

            bkt1_value = amt_data
                .1
                .iter()
                .clone()
                .take(30)
                .filter_map(|amt| amt.parse::<f64>().ok())
                .sum();
            bkt2_value = amt_data
                .1
                .iter()
                .clone()
                .skip(30)
                .take(184 - 30)
                .filter_map(|amt| amt.parse::<f64>().ok())
                .sum();
            bkt3_value = amt_data
                .1
                .iter()
                .clone()
                .skip(184)
                .take(190 - 184)
                .filter_map(|amt| amt.parse::<f64>().ok())
                .sum();
            bkt4_value = amt_data
                .1
                .iter()
                .clone()
                .skip(190)
                .take(203 - 190)
                .filter_map(|amt| amt.parse::<f64>().ok())
                .sum();

            let op_key = op_data.asondt.to_string()
                + &op_data.llgid
                + &op_data.ccy
                + &op_data.bktschemeid
                + &op_data.dim1itemid
                + &op_data.dim2itemid
                + &op_data.dim3itemid
                + &op_data.dim4itemid
                + &op_data.dim5itemid;

            let ccy = split_record[2].to_string();
            let exrt = conv_map.get(&ccy).unwrap_or(&1.0);

            if config_params.is_consolidated() {
                op_data.b1amthcy = bkt1_value * neg_abs;
                op_data.b2amthcy = bkt2_value * neg_abs;
                op_data.b3amthcy = bkt3_value * neg_abs;
                op_data.b4amthcy = bkt4_value * neg_abs;

                op_data.b1amtccy = (bkt1_value * neg_abs) / exrt;
                op_data.b2amtccy = (bkt2_value * neg_abs) / exrt;
                op_data.b3amtccy = (bkt3_value * neg_abs) / exrt;
                op_data.b4amtccy = (bkt4_value * neg_abs) / exrt;
            } else {
                op_data.b1amthcy = bkt1_value * neg_abs * exrt;
                op_data.b2amthcy = bkt2_value * neg_abs * exrt;
                op_data.b3amthcy = bkt3_value * neg_abs * exrt;
                op_data.b4amthcy = bkt4_value * neg_abs * exrt;

                op_data.b1amtccy = bkt1_value * neg_abs;
                op_data.b2amtccy = bkt2_value * neg_abs;
                op_data.b3amtccy = bkt3_value * neg_abs;
                op_data.b4amtccy = bkt4_value * neg_abs;
            }
            op_data.totalamtccy =
                op_data.b1amtccy + op_data.b2amtccy + op_data.b3amtccy + op_data.b4amtccy;
            op_data.totalamthcy =
                op_data.b1amthcy + op_data.b2amthcy + op_data.b3amthcy + op_data.b4amthcy;

            output_map
                .entry(op_key.clone())
                .and_modify(|data| {
                    data.b1amtccy += op_data.b1amtccy;
                    data.b1amthcy += op_data.b1amthcy;
                    data.b2amtccy += op_data.b2amtccy;
                    data.b2amthcy += op_data.b2amthcy;
                    data.b3amtccy += op_data.b3amtccy;
                    data.b3amthcy += op_data.b3amthcy;
                    data.b4amtccy += op_data.b4amtccy;
                    data.b4amthcy += op_data.b4amthcy;
                    data.totalamtccy += op_data.totalamtccy;
                    data.totalamthcy += op_data.totalamthcy;
                })
                .or_insert(op_data.clone());
        }

        for (_record, value) in output_map.iter() {
            op_str = value.print();
            writeln!(bkt_writer, "{}", op_str).unwrap_or_else(|error| {
                panic!("Unable to write to the bucketed output file: {}", error);
            });
        }
    } else {
        let mut output_map: HashMap<String, OutputData> = HashMap::new();
        let mut account_reader = reader::Reader::new_at_path(
            config_params.metadata_file_path(),
            config_params.input_file_path(),
        );
        let input_reader = reader::Reader::new_at_path(
            config_params.metadata_file_path(),
            config_params.input_file_path(),
        );
        if config_params.input_file_path().contains(".txt") {
            let rules = AggRules_txt::new_from_path(config_params.rules_file_path(), &input_reader);

            let input_record = buff_reader(config_params.input_file_path());
            for (line_num, lines) in input_record.lines().enumerate() {
                let line = match lines {
                    Ok(line) => line,
                    Err(error) => panic!(
                        "Unable to read file `{}` at line number: `{}` : {}",
                        config_params.input_file_path(),
                        line_num + 1,
                        error
                    ),
                };
                let metadata_map = metadata_reader(config_params.metadata_file_path());

                let split_record: Vec<&str> = line.split('|').collect();
                let mut op_data = OutputData::new();
                let llg =
                    llg_for_txt_account(&line.to_string(), &rules, &input_reader, config_params)
                        .to_string();

                let neg_abs = if llg_id_list.contains(&llg.as_str()) {
                    -1.0
                } else {
                    1.0
                };
                let currency_col = (*metadata_map.get(&keys.ccy_column).unwrap_or(&0) - 1) as usize;
                let currency = match split_record.get(currency_col) {
                    Some(value) => {
                        if value == &"0" {
                            0.to_string()
                        } else {
                            value.to_string()
                        }
                    }
                    None => 0.to_string(),
                };

                let dim1 = (*metadata_map.get(&keys.dimid1).unwrap_or(&0) - 1) as usize;

                let dimid1 = match split_record.get(dim1) {
                    Some(value) => {
                        if value == &"0" {
                            0.to_string()
                        } else {
                            value.to_string()
                        }
                    }
                    None => 0.to_string(),
                };

                let dim2 = (*metadata_map.get(&keys.dimid2).unwrap_or(&0) - 1) as usize;

                let dimid2 = match split_record.get(dim2) {
                    Some(value) => {
                        if value == &"0" {
                            0.to_string()
                        } else {
                            value.to_string()
                        }
                    }
                    None => 0.to_string(),
                };

                let dim3 = (*metadata_map.get(&keys.dimid3).unwrap_or(&0) - 1) as usize;

                let dimid3 = match split_record.get(dim3) {
                    Some(value) => {
                        if value == &"0" {
                            0.to_string()
                        } else {
                            value.to_string()
                        }
                    }
                    None => 0.to_string(),
                };

                let dim4 = (*metadata_map.get(&keys.dimid4).unwrap_or(&0) - 1) as usize;

                let dimid4 = match split_record.get(dim4) {
                    Some(value) => {
                        if value == &"0" {
                            0.to_string()
                        } else {
                            value.to_string()
                        }
                    }
                    None => 0.to_string(),
                };

                let dim5 = (*metadata_map.get(&keys.dimid5).unwrap_or(&0) - 1) as usize;

                let dimid5 = match split_record.get(dim5) {
                    Some(value) => {
                        if value == &"0" {
                            0.to_string()
                        } else {
                            value.to_string()
                        }
                    }
                    None => 0.to_string(),
                };

                op_data.asondt = *config_params.as_on_date();
                op_data.llgid = llg.to_string();
                op_data.country = keys.country.to_string();
                op_data.ccy = if currency.is_empty() {
                    "INR".to_string()
                } else {
                    currency.to_string()
                };
                op_data.bktschemeid = config_params.bucket_scheme_id().to_string();
                op_data.dim1itemid = dimid1.to_string();
                op_data.dim2itemid = dimid2.to_string();
                op_data.dim3itemid = dimid3.to_string();
                op_data.dim4itemid = dimid4.to_string();
                op_data.dim5itemid = dimid5.to_string();
                let op_key = op_data.asondt.to_string()
                    + &op_data.llgid
                    + &op_data.ccy
                    + &op_data.bktschemeid
                    + &op_data.dim1itemid
                    + &op_data.dim2itemid
                    + &op_data.dim3itemid
                    + &op_data.dim4itemid
                    + &op_data.dim5itemid;

                let amt_column = match keys.cashflows.parse::<usize>() {
                    Ok(parsed_value) if parsed_value > 0 => parsed_value - 1,
                    _ => {
                        let amt = (*metadata_map.get(&keys.amt_column).unwrap_or(&0) - 1) as usize;
                        match amt {
                            val => val,
                            _ => panic!("Amount and cashflow column is not passed correctly"),
                        }
                    }
                };

                let amount = split_record.get(amt_column).unwrap_or(&"");

                let exrt = conv_map.get(&currency).unwrap_or(&1.0);
                let mut amt_ccy = 0.0;
                let mut amt_hcy = 0.0;
                if config_params.is_consolidated() {
                    amt_ccy = (amount.parse::<f64>().unwrap_or(0.0) * neg_abs) / exrt;
                    amt_hcy = amount.parse::<f64>().unwrap_or(0.0) * neg_abs;
                } else {
                    amt_ccy = amount.parse::<f64>().unwrap_or(0.0) * neg_abs;
                    amt_hcy = amount.parse::<f64>().unwrap_or(0.0) * exrt * neg_abs;
                }

                let mut op = output_map.entry(op_key.clone()).or_insert(op_data.clone());

                store_data_in_bucket(config_params.non_by_bucket_id(), &mut op, amt_ccy, amt_hcy);
            }
            for (_record, value) in output_map.iter() {
                op_str = value.print();
                writeln!(bkt_writer, "{}", op_str).unwrap_or_else(|error| {
                    panic!("Unable to write to the bucketed output file: {}", error);
                });
            }
        } else if config_params.input_file_path().contains(".cf") {
            let rules = AggRules::new_from_path(config_params.rules_file_path(), &input_reader);
            for (_count, mut account) in account_reader.iter().enumerate() {
                let llg = log_measurements!(
                    _diag_logger,
                    ["Type: GetLLG, Identifier".to_string()],
                    llg_for_cf_account(&account, &rules, "".to_string(), config_params, logger)
                );
                let neg_abs = if llg_id_list.contains(&llg.source_code.to_string().as_str()) {
                    -1.0
                } else {
                    1.0
                };

                let mut op_data = OutputData::new();
                let currency = get_field_value(&account, &input_reader, keys.ccy_column.clone())
                    .unwrap_or_else(|_| "".to_string());
                op_data.asondt = *config_params.as_on_date();
                op_data.llgid = llg.to_string();
                op_data.country = keys.country.to_string();
                op_data.ccy = if currency.is_empty() {
                    "INR".to_string()
                } else {
                    currency.to_string()
                };
                op_data.bktschemeid = config_params.bucket_scheme_id().to_string();
                op_data.dim1itemid = get_field_value(&account, &input_reader, keys.dimid1.clone())
                    .unwrap_or_else(|_| 0.to_string());
                op_data.dim2itemid = get_field_value(&account, &input_reader, keys.dimid2.clone())
                    .unwrap_or_else(|_| 0.to_string());
                op_data.dim3itemid = get_field_value(&account, &input_reader, keys.dimid3.clone())
                    .unwrap_or_else(|_| "0".to_string());
                op_data.dim4itemid = get_field_value(&account, &input_reader, keys.dimid4.clone())
                    .unwrap_or_else(|_| "0".to_string());
                op_data.dim5itemid = get_field_value(&account, &input_reader, keys.dimid5.clone())
                    .unwrap_or_else(|_| "0".to_string());

                let op_key = op_data.asondt.to_string()
                    + &op_data.llgid
                    + &op_data.ccy
                    + &op_data.bktschemeid
                    + &op_data.dim1itemid
                    + &op_data.dim2itemid
                    + &op_data.dim3itemid
                    + &op_data.dim4itemid
                    + &op_data.dim5itemid;

                let exrt = conv_map.get(&currency).unwrap_or(&1.0);

                let mut amt_ccy = 0.0;
                let mut amt_hcy = 0.0;
                if keys.cashflows.is_empty() {
                    if keys.amt_column.is_empty() {
                        panic!("amount and cashflow column can not be empty")
                    } else {
                        let amount =
                            get_field_value(&account, &input_reader, keys.amt_column.clone())
                                .unwrap_or_else(|_| "0.0".to_string());
                        if config_params.is_consolidated() {
                            amt_ccy = (amount.parse::<f64>().unwrap_or(0.0) * neg_abs) / exrt;
                            amt_hcy = amount.parse::<f64>().unwrap_or(0.0) * neg_abs;
                        } else {
                            amt_ccy = amount.parse::<f64>().unwrap_or(0.0) * neg_abs;
                            amt_hcy = amount.parse::<f64>().unwrap_or(0.0) * exrt * neg_abs;
                        }

                        let mut op = output_map.entry(op_key.clone()).or_insert(op_data.clone());
                        store_data_in_bucket(
                            config_params.non_by_bucket_id(),
                            &mut op,
                            amt_ccy,
                            amt_hcy,
                        );
                    }
                } else {
                    let cf_val = match account.remove_cfs_for_key(&"cashflows".to_string()) {
                        Ok(value) => value,
                        Err(_error) => panic!("Failed to extract cashflows from the record"),
                    };
                    for cf in cf_val {
                        if keys.cashflows == "interest_amount" {
                            if config_params.is_consolidated() {
                                amt_ccy = (cf.interest_amount * neg_abs) / exrt;
                                amt_hcy = cf.interest_amount * neg_abs;
                            } else {
                                amt_ccy = cf.interest_amount * neg_abs;
                                amt_hcy = (cf.interest_amount * neg_abs) * exrt;
                            }
                            let mut op =
                                output_map.entry(op_key.clone()).or_insert(op_data.clone());
                            store_data_in_bucket(
                                config_params.non_by_bucket_id(),
                                &mut op,
                                amt_ccy,
                                amt_hcy,
                            );
                        } else if keys.cashflows == "principal_amount" {
                            if config_params.is_consolidated() {
                                amt_ccy = (cf.principal_amount * neg_abs) / exrt;
                                amt_hcy = cf.principal_amount * neg_abs;
                            } else {
                                amt_ccy = cf.principal_amount * neg_abs;
                                amt_hcy = (cf.principal_amount * neg_abs) * exrt;
                            }
                            let mut op =
                                output_map.entry(op_key.clone()).or_insert(op_data.clone());

                            store_data_in_bucket(
                                config_params.non_by_bucket_id(),
                                &mut op,
                                amt_ccy,
                                amt_hcy,
                            );
                        } else {
                            panic!("Cashflow column should be in format of either 'principal_amount' or 'interest_amount'");
                        }
                    }
                };
            }
            for (_record, value) in output_map.iter() {
                op_str = value.print();
                writeln!(bkt_writer, "{}", op_str).unwrap_or_else(|error| {
                    panic!("Unable to write to the bucketed output file: {}", error);
                });
            }
        } else {
            panic!(
                "Input file is not in proper format. Input File: `{}`",
                config_params.input_file_path()
            );
        }
    }
}
pub fn get_maturity_bucket_scheme_map(
    maturity_bkt_schm_file: &String,
    bucket_scheme_id: &str,
    as_on_date: &NaiveDate,
) -> HashMap<i64, (usize, usize)> {
    let group_consti = buff_reader(maturity_bkt_schm_file);
    let mut bkt_map: HashMap<i64, (usize, usize)> = HashMap::new();
    for (line_num, lines) in group_consti.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                maturity_bkt_schm_file,
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();

        if fields[0] != bucket_scheme_id {
            continue;
        }

        let bkt_id: i64 = fields[1]
            .to_string()
            .parse()
            .expect("Cannot parse Bucket ID to usize");
        let from_date: usize = fields[3]
            .to_string()
            .parse()
            .expect("Cannot fetch from date");
        let to_date: usize = fields[4].to_string().parse().expect("Cannot fetch to date");
        bkt_map.insert(bkt_id, (from_date, to_date));
    }
    bkt_map
}

pub fn get_exchange_rate_map(cons_curr: String, ccy_path: String) -> HashMap<String, f64> {
    let ccy_file_contents = fs::read_to_string(ccy_path).expect("cannot read currency file");
    let mut currency_map: HashMap<String, f64> = HashMap::new();
    for line in ccy_file_contents.lines() {
        let each_line: Vec<&str> = line.split('|').collect();
        if each_line[1] == cons_curr {
            currency_map.insert(
                each_line[0].to_string(),
                each_line[2]
                    .parse::<f64>()
                    .expect("unable to parse exchange rate"),
            );
        }
    }
    currency_map
}
pub fn store_data_in_bucket(bkt_id: i64, op_data: &mut OutputData, amt_ccy: f64, amt_hcy: f64) {
    match bkt_id {
        1 => {
            op_data.b1amtccy += amt_ccy;
            op_data.b1amthcy += amt_hcy;
        }
        2 => {
            op_data.b2amtccy += amt_ccy;
            op_data.b2amthcy += amt_hcy;
        }
        3 => {
            op_data.b3amtccy += amt_ccy;
            op_data.b3amthcy += amt_hcy;
        }
        4 => {
            op_data.b4amtccy += amt_ccy;
            op_data.b4amthcy += amt_hcy;
        }
        5 => {
            op_data.b5amtccy += amt_ccy;
            op_data.b5amthcy += amt_hcy;
        }
        6 => {
            op_data.b6amtccy += amt_ccy;
            op_data.b6amthcy += amt_hcy;
        }
        7 => {
            op_data.b7amtccy += amt_ccy;
            op_data.b7amthcy += amt_hcy;
        }
        8 => {
            op_data.b8amtccy += amt_ccy;
            op_data.b8amthcy += amt_hcy;
        }
        9 => {
            op_data.b9amtccy += amt_ccy;
            op_data.b9amthcy += amt_hcy;
        }
        10 => {
            op_data.b10amtccy += amt_ccy;
            op_data.b10amthcy += amt_hcy;
        }
        11 => {
            op_data.b11amtccy += amt_ccy;
            op_data.b11amthcy += amt_hcy;
        }
        12 => {
            op_data.b12amtccy += amt_ccy;
            op_data.b12amthcy += amt_hcy;
        }
        13 => {
            op_data.b13amtccy += amt_ccy;
            op_data.b13amthcy += amt_hcy;
        }
        14 => {
            op_data.b14amtccy += amt_ccy;
            op_data.b14amthcy += amt_hcy;
        }
        15 => {
            op_data.b15amtccy += amt_ccy;
            op_data.b15amthcy += amt_hcy;
        }
        16 => {
            op_data.b16amtccy += amt_ccy;
            op_data.b16amthcy += amt_hcy;
        }
        17 => {
            op_data.b17amtccy += amt_ccy;
            op_data.b17amthcy += amt_hcy;
        }
        18 => {
            op_data.b18amtccy += amt_ccy;
            op_data.b18amthcy += amt_hcy;
        }
        19 => {
            op_data.b19amtccy += amt_ccy;
            op_data.b19amthcy += amt_hcy;
        }
        20 => {
            op_data.b20amtccy += amt_ccy;
            op_data.b20amthcy += amt_hcy;
        }
        21 => {
            op_data.b21amtccy += amt_ccy;
            op_data.b21amthcy += amt_hcy;
        }
        22 => {
            op_data.b22amtccy += amt_ccy;
            op_data.b22amthcy += amt_hcy;
        }
        23 => {
            op_data.b23amtccy += amt_ccy;
            op_data.b23amthcy += amt_hcy;
        }
        24 => {
            op_data.b24amtccy += amt_ccy;
            op_data.b24amthcy += amt_hcy;
        }
        25 => {
            op_data.b25amtccy += amt_ccy;
            op_data.b25amthcy += amt_hcy;
        }
        26 => {
            op_data.b26amtccy += amt_ccy;
            op_data.b26amthcy += amt_hcy;
        }
        27 => {
            op_data.b27amtccy += amt_ccy;
            op_data.b27amthcy += amt_hcy;
        }
        28 => {
            op_data.b28amtccy += amt_ccy;
            op_data.b28amthcy += amt_hcy;
        }
        29 => {
            op_data.b29amtccy += amt_ccy;
            op_data.b29amthcy += amt_hcy;
        }
        30 => {
            op_data.b30amtccy += amt_ccy;
            op_data.b30amthcy += amt_hcy;
        }
        _ => {
            op_data.totalamtccy += amt_ccy;
            op_data.totalamthcy += amt_hcy;
        }
    }
    op_data.totalamtccy += amt_ccy;
    op_data.totalamthcy += amt_hcy;
}
