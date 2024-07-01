use self::ex_rt::ExKey;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::date_from_timestamp;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_agg_rules_txt::agg_rules::AggRules as AggRules_txt;
use sdb_dyn_proto_rdr::reader::{self, Reader};
use slog::Logger;
use splitter::account_field_names::AccFieldNames;
use splitter::implementation::{llg_for_cf_account, llg_for_txt_account};
use splitter::reader::account_with_cfs::get_field_value;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::time::SystemTime;

mod account_field_names;
mod ex_rt;
mod implementation;
mod llg_key;
mod writers;

pub fn call_splitter(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) {
    // Initialize a pool of writers.
    let mut writers_pool: HashMap<String, BufWriter<File>> = HashMap::new();
    let mut prin_writer_pool: HashMap<String, BufWriter<File>> = HashMap::new();
    let mut int_writer_pool: HashMap<String, BufWriter<File>> = HashMap::new();
    let mut account_reader = reader::Reader::new_at_path(
        config_params.metadata_file_path(),
        config_params.input_file_path(),
    );
    let input_reader = reader::Reader::new_at_path(
        config_params.metadata_file_path(),
        config_params.input_file_path(),
    );

    let mut field_names = Vec::new();
    let metadata_reader = fs::read_to_string(&config_params.metadata_file_path())
        .expect("Failed to read metadata file!");
    //Fetch the names from the metadata file.
    for line in metadata_reader.lines() {
        if line.contains("name") {
            let fields: Vec<&str> = line.split(':').collect();
            let mut name = fields[1].to_string();
            name.pop();
            name.pop();
            name = name[2..].to_string();
            field_names.push(name);
        }
    }
    let mut source_map: HashMap<String, String> = HashMap::new();
    let source_map_reader = fs::read_to_string(&config_params.source_map_file_path())
        .expect("Failed to read source map file!");

    for line in source_map_reader.lines() {
        let source_fields = line.split('|').collect::<Vec<&str>>();
        if source_fields.len() == 2 && config_params.input_file_path().contains(".txt") {
            source_map.insert(source_fields[0].to_string(), source_fields[1].to_string());
            let new_writer = writers::get_new_writer(
                source_fields[1].to_string(),
                config_params.output_file_path(),
            );
            writers_pool.insert(source_fields[0].to_string(), new_writer);
        } else if source_fields.len() == 3 && config_params.input_file_path().contains(".cf") {
            let prin_writer = writers::get_new_writer(
                source_fields[1].to_string(),
                config_params.output_file_path(),
            );
            prin_writer_pool.insert(source_fields[0].to_string(), prin_writer);
            let int_writer = writers::get_new_writer(
                source_fields[2].to_string(),
                config_params.output_file_path(),
            );
            int_writer_pool.insert(source_fields[0].to_string(), int_writer);
        } else {
            log_error!(logger, "Line not proper:{}", line);
        }
    }

    if config_params.input_file_path().contains(".txt") {
        text_split_files(
            &mut writers_pool,
            input_reader,
            config_params,
            logger,
            diag_logger,
        );
        log_info!(logger, "Input file is text File");
    } else if config_params.input_file_path().contains(".cf") {
        cashflow_split_files(
            &mut prin_writer_pool,
            &mut int_writer_pool,
            &mut account_reader,
            input_reader,
            field_names,
            config_params,
            logger,
            diag_logger,
        );
        log_info!(logger, "Input file is CF File");
    } else {
        panic!("Input File extension is not well formatted. Use '.txt' or '.cf' extensions");
    }
}

pub fn cashflow_split_files(
    prin_writer_pool: &mut HashMap<String, BufWriter<File>>,
    int_writer_pool: &mut HashMap<String, BufWriter<File>>,
    account_reader: &mut Reader,
    input_reader: Reader,
    field_names: Vec<String>,
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) {
    let rules = AggRules::new_from_path(config_params.rule_file_path(), &input_reader);
    let keys = AccFieldNames::new_from_path(config_params.req_fields_file_path());
    let _as_on_date = config_params.as_on_date();
    for (_count, mut account) in account_reader.iter().enumerate() {
        let llg = log_measurements!(
            diag_logger,
            ["Type: GetLLG, Identifier".to_string()],
            llg_for_cf_account(
                &account,
                &rules,
                field_names[0].to_string(),
                config_params,
                logger
            )
        );
        let llg_int = llg.to_string().parse().unwrap_or(0);
        let prefix = llg_int / 10000;
        let flow;
        if prefix == 1 {
            flow = "I";
        } else {
            flow = "O";
        }
        let llg_act = (llg_int % 10000).to_string();
        let mut out_acc = AccFieldNames {
            FlowID: get_field_value(&account, &input_reader, keys.FlowID.to_string())
                .unwrap_or_else(|_| "NA".to_string())
                .trim()
                .to_string(),
            GrpID: keys.GrpID.to_string(),
            LLGID: get_field_value(&account, &input_reader, keys.LLGID.to_string())
                .unwrap_or_else(|_| "NA".to_string())
                .trim()
                .to_string(),
            Amount: get_field_value(&account, &input_reader, keys.Amount.to_string())
                .unwrap_or_else(|_| "NA".to_string())
                .trim()
                .to_string(),
            CcyID: get_field_value(&account, &input_reader, keys.CcyID.to_string())
                .unwrap_or_else(|_| "NA".to_string())
                .trim()
                .to_string(),
            IntrRate: get_field_value(&account, &input_reader, keys.IntrRate.to_string())
                .unwrap_or_else(|_| "NA".to_string())
                .trim()
                .to_string(),
            RepriceFreq: get_field_value(&account, &input_reader, keys.RepriceFreq.to_string())
                .unwrap_or_else(|_| "NA".to_string())
                .trim()
                .to_string(),
            RepriceDt: date_from_timestamp(
                get_field_value(&account, &input_reader, keys.RepriceDt.to_string())
                    .unwrap_or_else(|_| "0".to_string())
                    .trim()
                    .parse()
                    .unwrap_or(0),
            )
            .format("%d-%m-%Y")
            .to_string(),
            MatuDt: get_field_value(&account, &input_reader, keys.MatuDt.to_string())
                .unwrap_or_else(|_| "NA".to_string())
                .trim()
                .to_string(),
            AcctNum: get_field_value(&account, &input_reader, keys.AcctNum.to_string())
                .unwrap_or_else(|_| "NA".to_string())
                .trim()
                .to_string(),
            StrtDt: date_from_timestamp(
                get_field_value(&account, &input_reader, keys.StrtDt.to_string())
                    .unwrap_or_else(|_| "0".to_string())
                    .trim()
                    .parse()
                    .unwrap_or(0),
            )
            .format("%d-%m-%Y")
            .to_string(),
            IntrCalFreq: get_field_value(&account, &input_reader, keys.IntrCalFreq.to_string())
                .unwrap_or_else(|_| "NA".to_string())
                .trim()
                .to_string(),
            IsFlotRate: get_field_value(&account, &input_reader, keys.IsFlotRate.to_string())
                .unwrap_or_else(|_| "NA".to_string())
                .trim()
                .to_string(),
            FlotRateBM: get_field_value(&account, &input_reader, keys.FlotRateBM.to_string())
                .unwrap_or_else(|_| "NA".to_string())
                .trim()
                .to_string(),
            BUID: get_field_value(&account, &input_reader, keys.BUID.to_string())
                .unwrap_or_else(|_| "NA".to_string())
                .trim()
                .to_string(),
            CustID: get_field_value(&account, &input_reader, keys.CustID.to_string())
                .unwrap_or_else(|_| "NA".to_string())
                .trim()
                .to_string(),
            CustName: get_field_value(&account, &input_reader, keys.CustName.to_string())
                .unwrap_or_else(|_| "NA".to_string())
                .trim()
                .to_string(),
            Sprd: get_field_value(&account, &input_reader, keys.Sprd.to_string())
                .unwrap_or_else(|_| "NA".to_string())
                .trim()
                .to_string(),
            SchmCode: get_field_value(&account, &input_reader, keys.SchmCode.to_string())
                .unwrap_or_else(|_| "NA".to_string())
                .trim()
                .to_string(),
            MinIR: get_field_value(&account, &input_reader, keys.MinIR.to_string())
                .unwrap_or_else(|_| "NA".to_string())
                .trim()
                .to_string(),
            MaxIR: get_field_value(&account, &input_reader, keys.MaxIR.to_string())
                .unwrap_or_else(|_| "NA".to_string())
                .trim()
                .to_string(),
            DepAmount: get_field_value(&account, &input_reader, keys.DepAmount.to_string())
                .unwrap_or_else(|_| "NA".to_string())
                .trim()
                .to_string(),
            MatuAmt: get_field_value(&account, &input_reader, keys.MatuAmt.to_string())
                .unwrap_or_else(|_| "NA".to_string())
                .trim()
                .to_string(),
            ExchRate: get_field_value(&account, &input_reader, keys.ExchRate.to_string())
                .unwrap_or_else(|_| "NA".to_string())
                .trim()
                .to_string(),
            CustCtryCode: get_field_value(&account, &input_reader, keys.CustCtryCode.to_string())
                .unwrap_or_else(|_| "NA".to_string())
                .trim()
                .to_string(),
            CustCrdtRtng: get_field_value(&account, &input_reader, keys.CustCrdtRtng.to_string())
                .unwrap_or_else(|_| "NA".to_string())
                .trim()
                .to_string(),
            CustSectCode: get_field_value(&account, &input_reader, keys.CustSectCode.to_string())
                .unwrap_or_else(|_| "NA".to_string())
                .trim()
                .to_string(),
            CustIndtCode: get_field_value(&account, &input_reader, keys.CustIndtCode.to_string())
                .unwrap_or_else(|_| "NA".to_string())
                .trim()
                .to_string(),
            Custom1: get_field_value(&account, &input_reader, keys.Custom1.to_string())
                .unwrap_or_else(|_| "NA".to_string())
                .trim()
                .to_string(),
            Custom2: get_field_value(&account, &input_reader, keys.Custom2.to_string())
                .unwrap_or_else(|_| "NA".to_string())
                .trim()
                .to_string(),
            cashflows: "".to_string(),
        };
        if out_acc.FlowID.is_empty() || out_acc.FlowID == "NA" || out_acc.FlowID == " " {
            out_acc.FlowID = flow.to_string();
        }
        if out_acc.LLGID.is_empty() || out_acc.LLGID == "NA" || out_acc.LLGID == " " {
            out_acc.LLGID = llg_act.to_string();
        }
        let mut cashflows = match account.remove_cfs_for_key(&keys.cashflows) {
            Ok(value) => value,
            Err(_error) => continue,
        };

        for cashflow in &cashflows {
            let dt = &date_from_timestamp(cashflow.date)
                .format("%d-%m-%Y")
                .to_string();
            if !config_params.overdue_llg_code().to_string().is_empty() {
                let llg_id = config_params.overdue_llg_code();
                let mat_date = date_from_timestamp(cashflow.date);
                if mat_date <= config_params.as_on_date() {
                    if (cashflow.principal_amount != 0.0) {
                        let over_prin_writer = match prin_writer_pool.get_mut(&llg_id.to_string()) {
                            Some(writer) => writer,
                            None => {
                                //If the source id could not be found the output is written to a default file: "NA.txt".
                                let new_writer = writers::get_new_writer(
                                    config_params.default_file_name().to_string(),
                                    config_params.output_file_path(),
                                );
                                prin_writer_pool.insert(llg_id.to_string(), new_writer);
                                // cannot return new writer as ownership of that writer is assigned to writers_pool in prev step
                                prin_writer_pool.get_mut(&llg_id.to_string()).unwrap()
                            }
                        };
                        let mut overdue_prin_op = format!("{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",out_acc.FlowID,
                    out_acc.GrpID,
                    llg_id,
                    format!("{:.9}",cashflow.principal_amount),
                    out_acc.CcyID,
                    format!("{:.9}",out_acc.IntrRate),
                    out_acc.RepriceFreq,
                    out_acc.RepriceDt,
                    dt,
                    out_acc.AcctNum,
                    out_acc.StrtDt,
                    out_acc.IntrCalFreq,
                    out_acc.IsFlotRate,
                    out_acc.FlotRateBM,
                    out_acc.BUID,
                    out_acc.CustID,
                    out_acc.CustName,
                    out_acc.Sprd,
                    out_acc.SchmCode,
                    out_acc.MinIR,
                    out_acc.MaxIR,
                    out_acc.DepAmount,
                    out_acc.MatuAmt,
                    out_acc.ExchRate,
                    out_acc.CustCtryCode,
                    out_acc.CustCrdtRtng,
                    out_acc.CustSectCode,
                    out_acc.CustIndtCode,
                    out_acc.Custom1,
                    out_acc.Custom2);
                        overdue_prin_op.push_str("\r\n");
                        writers::write_data(over_prin_writer, overdue_prin_op, logger);
                    }
                    if (cashflow.interest_amount != 0.0) {
                        let over_int_writer = match int_writer_pool.get_mut(&llg_id.to_string()) {
                            Some(writer) => writer,
                            None => {
                                //If the source id could not be found the output is written to a default file: "NA.txt".
                                let new_writer = writers::get_new_writer(
                                    config_params.default_file_name().to_string(),
                                    config_params.output_file_path(),
                                );
                                int_writer_pool.insert(llg_id.to_string(), new_writer);
                                // cannot return new writer as ownership of that writer is assigned to writers_pool in prev step
                                int_writer_pool.get_mut(&llg_id.to_string()).unwrap()
                            }
                        };
                        let mut overdue_int_op = format!("{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",out_acc.FlowID,
                    out_acc.GrpID,
                    llg_id,
                    format!("{:.9}",cashflow.interest_amount),
                    out_acc.CcyID,
                    format!("{:.9}",out_acc.IntrRate),
                    out_acc.RepriceFreq,
                    out_acc.RepriceDt,
                    dt,
                    out_acc.AcctNum,
                    out_acc.StrtDt,
                    out_acc.IntrCalFreq,
                    out_acc.IsFlotRate,
                    out_acc.FlotRateBM,
                    out_acc.BUID,
                    out_acc.CustID,
                    out_acc.CustName,
                    out_acc.Sprd,
                    out_acc.SchmCode,
                    out_acc.MinIR,
                    out_acc.MaxIR,
                    out_acc.DepAmount,
                    out_acc.MatuAmt,
                    out_acc.ExchRate,
                    out_acc.CustCtryCode,
                    out_acc.CustCrdtRtng,
                    out_acc.CustSectCode,
                    out_acc.CustIndtCode,
                    out_acc.Custom1,
                    out_acc.Custom2);
                        overdue_int_op.push_str("\r\n");
                        writers::write_data(over_int_writer, overdue_int_op, logger);
                    }
                } else {
                    if (cashflow.principal_amount != 0.0) {
                        let prin_writer = match prin_writer_pool.get_mut(&llg.to_string()) {
                            Some(writer) => writer,
                            None => {
                                //If the source id could not be found the output is written to a default file: "NA.txt".
                                let new_writer = writers::get_new_writer(
                                    config_params.default_file_name().to_string(),
                                    config_params.output_file_path(),
                                );
                                prin_writer_pool.insert(llg.source_code.to_string(), new_writer);
                                // cannot return new writer as ownership of that writer is assigned to writers_pool in prev step
                                prin_writer_pool
                                    .get_mut(&llg.source_code.to_string())
                                    .unwrap()
                            }
                        };
                        let mut prin_op = format!("{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",out_acc.FlowID,
                            out_acc.GrpID,
                            out_acc.LLGID,
                            format!("{:.9}",cashflow.principal_amount),
                            out_acc.CcyID,
                            format!("{:.9}",out_acc.IntrRate),
                            out_acc.RepriceFreq,
                            out_acc.RepriceDt,
                            dt,
                            out_acc.AcctNum,
                            out_acc.StrtDt,
                            out_acc.IntrCalFreq,
                            out_acc.IsFlotRate,
                            out_acc.FlotRateBM,
                            out_acc.BUID,
                            out_acc.CustID,
                            out_acc.CustName,
                            out_acc.Sprd,
                            out_acc.SchmCode,
                            out_acc.MinIR,
                            out_acc.MaxIR,
                            out_acc.DepAmount,
                            out_acc.MatuAmt,
                            out_acc.ExchRate,
                            out_acc.CustCtryCode,
                            out_acc.CustCrdtRtng,
                            out_acc.CustSectCode,
                            out_acc.CustIndtCode,
                            out_acc.Custom1,
                            out_acc.Custom2);
                        prin_op.push_str("\r\n");
                        writers::write_data(prin_writer, prin_op, logger);
                    }
                    if (cashflow.interest_amount != 0.0) {
                        let int_writer = match int_writer_pool.get_mut(&llg.to_string()) {
                            Some(writer) => writer,
                            None => {
                                //If the source id could not be found the output is written to a default file: "NA.txt".
                                let new_writer = writers::get_new_writer(
                                    config_params.default_file_name().to_string(),
                                    config_params.output_file_path(),
                                );
                                int_writer_pool.insert(llg.source_code.to_string(), new_writer);
                                // cannot return new writer as ownership of that writer is assigned to writers_pool in prev step
                                int_writer_pool
                                    .get_mut(&llg.source_code.to_string())
                                    .unwrap()
                            }
                        };
                        let mut int_op = format!("{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",out_acc.FlowID,
                            out_acc.GrpID,
                            out_acc.LLGID,
                            format!("{:.9}",cashflow.interest_amount),
                            out_acc.CcyID,
                            format!("{:.9}",out_acc.IntrRate),
                            out_acc.RepriceFreq,
                            out_acc.RepriceDt,
                            dt,
                            out_acc.AcctNum,
                            out_acc.StrtDt,
                            out_acc.IntrCalFreq,
                            out_acc.IsFlotRate,
                            out_acc.FlotRateBM,
                            out_acc.BUID,
                            out_acc.CustID,
                            out_acc.CustName,
                            out_acc.Sprd,
                            out_acc.SchmCode,
                            out_acc.MinIR,
                            out_acc.MaxIR,
                            out_acc.DepAmount,
                            out_acc.MatuAmt,
                            out_acc.ExchRate,
                            out_acc.CustCtryCode,
                            out_acc.CustCrdtRtng,
                            out_acc.CustSectCode,
                            out_acc.CustIndtCode,
                            out_acc.Custom1,
                            out_acc.Custom2);
                        int_op.push_str("\r\n");
                        writers::write_data(int_writer, int_op, logger);
                    }
                }
            } else {
                if (cashflow.principal_amount != 0.0) {
                    let prin_writer = match prin_writer_pool.get_mut(&llg.to_string()) {
                        Some(writer) => writer,
                        None => {
                            //If the source id could not be found the output is written to a default file: "NA.txt".
                            let new_writer = writers::get_new_writer(
                                config_params.default_file_name().to_string(),
                                config_params.output_file_path(),
                            );
                            prin_writer_pool.insert(llg.source_code.to_string(), new_writer);
                            // cannot return new writer as ownership of that writer is assigned to writers_pool in prev step
                            prin_writer_pool
                                .get_mut(&llg.source_code.to_string())
                                .unwrap()
                        }
                    };
                    let mut prin_op = format!("{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",out_acc.FlowID,
                        out_acc.GrpID,
                        out_acc.LLGID,
                        format!("{:.9}",cashflow.principal_amount),
                        out_acc.CcyID,
                        format!("{:.9}",out_acc.IntrRate),
                        out_acc.RepriceFreq,
                        out_acc.RepriceDt,
                        dt,
                        out_acc.AcctNum,
                        out_acc.StrtDt,
                        out_acc.IntrCalFreq,
                        out_acc.IsFlotRate,
                        out_acc.FlotRateBM,
                        out_acc.BUID,
                        out_acc.CustID,
                        out_acc.CustName,
                        out_acc.Sprd,
                        out_acc.SchmCode,
                        out_acc.MinIR,
                        out_acc.MaxIR,
                        out_acc.DepAmount,
                        out_acc.MatuAmt,
                        out_acc.ExchRate,
                        out_acc.CustCtryCode,
                        out_acc.CustCrdtRtng,
                        out_acc.CustSectCode,
                        out_acc.CustIndtCode,
                        out_acc.Custom1,
                        out_acc.Custom2);
                    prin_op.push_str("\r\n");
                    writers::write_data(prin_writer, prin_op, logger);
                }

                if (cashflow.interest_amount != 0.0) {
                    let int_writer = match int_writer_pool.get_mut(&llg.to_string()) {
                        Some(writer) => writer,
                        None => {
                            //If the source id could not be found the output is written to a default file: "NA.txt".
                            let new_writer = writers::get_new_writer(
                                config_params.default_file_name().to_string(),
                                config_params.output_file_path(),
                            );
                            int_writer_pool.insert(llg.source_code.to_string(), new_writer);
                            // cannot return new writer as ownership of that writer is assigned to writers_pool in prev step
                            int_writer_pool
                                .get_mut(&llg.source_code.to_string())
                                .unwrap()
                        }
                    };
                    let mut int_op = format!("{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",out_acc.FlowID,
                        out_acc.GrpID,
                        out_acc.LLGID,
                        format!("{:.9}",cashflow.interest_amount),
                        out_acc.CcyID,
                        format!("{:.9}",out_acc.IntrRate),
                        out_acc.RepriceFreq,
                        out_acc.RepriceDt,
                        dt,
                        out_acc.AcctNum,
                        out_acc.StrtDt,
                        out_acc.IntrCalFreq,
                        out_acc.IsFlotRate,
                        out_acc.FlotRateBM,
                        out_acc.BUID,
                        out_acc.CustID,
                        out_acc.CustName,
                        out_acc.Sprd,
                        out_acc.SchmCode,
                        out_acc.MinIR,
                        out_acc.MaxIR,
                        out_acc.DepAmount,
                        out_acc.MatuAmt,
                        out_acc.ExchRate,
                        out_acc.CustCtryCode,
                        out_acc.CustCrdtRtng,
                        out_acc.CustSectCode,
                        out_acc.CustIndtCode,
                        out_acc.Custom1,
                        out_acc.Custom2);
                    int_op.push_str("\r\n");
                    writers::write_data(int_writer, int_op, logger);
                }
            }
        }
        cashflows.clear();
        // writer
    }
}

pub fn text_split_files(
    writers_pool: &mut HashMap<String, BufWriter<File>>,
    input_reader: Reader,
    config_params: &ConfigurationParameters,
    logger: &Logger,
    _diag_logger: &Logger,
) {
    let rules = AggRules_txt::new_from_path(config_params.rule_file_path(), &input_reader);
    let mut ex_map: HashMap<ExKey, f64> = HashMap::new();
    let exch_reader = fs::read_to_string(&config_params.exchange_file_path())
        .expect("Failed to read exchange rate file!");
    for line in exch_reader.lines() {
        let ex_fields = line.split('|').collect::<Vec<&str>>();
        let ex_key = ExKey::new(ex_fields[0], ex_fields[1]);
        let ex_rt = ex_fields[2].to_string().parse::<f64>().unwrap();
        ex_map.insert(ex_key, ex_rt);
    }
    let input_file_reader = fs::read_to_string(&config_params.input_file_path())
        .expect("Failed to read metadata file!");
    for line in input_file_reader.lines() {
        let llg = llg_for_txt_account(&line.to_string(), &rules, &input_reader, config_params);
        let prefix = llg / 10000;
        let flow = if prefix == 1 { "I" } else { "O" };
        let llg_act = (llg % 10000).to_string();
        // writer
        let writer = match writers_pool.get_mut(&llg_act.to_string()) {
            Some(writer) => writer,
            None => {
                //If the source id could not be found the output is written to a default file: "NA.txt".
                let new_writer = writers::get_new_writer(
                    config_params.default_file_name().to_string(),
                    config_params.output_file_path(),
                );
                writers_pool.insert(llg.to_string(), new_writer);
                // cannot return new writer as ownership of that writer is assigned to writers_pool in prev step
                writers_pool.get_mut(&llg.to_string()).unwrap()
            }
        };
        let mut temp_data = line.split('|').collect::<Vec<&str>>();
        let mut data = Vec::new();
        for val in temp_data {
            data.push(val.trim());
        }
        let amt = format!("{:.9}", data[3].parse().unwrap_or(0.0)).to_string();
        let int_rt = format!("{:.9}", data[5].parse().unwrap_or(0.0)).to_string();
        if data[0] == " "
            || data[0].to_string().is_empty()
            || data[0] == "NULL"
            || data[0] == "null"
        {
            data[0] = flow;
        }
        if data[2] == " "
            || data[2].to_string().is_empty()
            || data[2] == "NULL"
            || data[2] == "null"
        {
            data[2] = llg_act.as_str();
        }
        let ccy = data[4];
        let exchange_rate;
        if config_params.is_consolidated() == "true" {
            let exch_key = ExKey::new(config_params.base_currency(), ccy);
            exchange_rate = (ex_map.get(&exch_key).unwrap_or(&1.0)).to_string();
        } else {
            let exch_key = ExKey::new(ccy, config_params.base_currency());
            exchange_rate = (ex_map.get(&exch_key).unwrap_or(&1.0)).to_string();
        }
        data[3] = amt.as_str();
        data[5] = int_rt.as_str();
        data[23] = exchange_rate.as_str();
        let out = data.join("|");
        let op_line = format!("{}\r\n", out);
        writers::write_data(writer, op_line, logger);
    }
}
