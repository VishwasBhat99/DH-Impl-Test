use self::output::Output;
use self::structs::*;
use super::configuration_parameters::ConfigurationParameters;
use crate::macros;
use health_report::HealthReport;
use protobuf::Message;
use sdb_dyn_proto_rdr::compound_types::Cashflow;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::account_with_cfs::get_field_value;
use sdb_dyn_proto_rdr::reader::types::*;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::collections::HashMap;
use std::io::{prelude::BufRead, Write};

mod config;
mod output;
mod structs;

pub fn classify(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    //Initialize HCR Data
    let mut acc_encountered = 0;
    let mut cf_encountered = 0;
    let mut acc_success = 0;
    let mut ip_amt = 0.0;
    let mut op_amt = 0.0;

    let mut txt_writer = buf_file_wrtr(
        &format!(
            "{}.txt",
            config_params
                .output_file_path()
                .replace(".txt", "")
                .replace(".cf", "")
        ),
        None,
    )
    .expect("Unable to Create Writer for Output File.");
    let mut cf_output_file = std::fs::File::create(&format!(
        "{}.cf",
        config_params
            .output_file_path()
            .replace(".txt", "")
            .replace(".cf", "")
    ))
    .expect("unable to create output file");

    let mut exch_rate_map: HashMap<String, f64> = HashMap::new();
    let exch_rate_input = std::fs::File::open(config_params.currency_conversion_file_path())
        .expect("Could Not Read Exchange Rate File");
    let exch_rate_file = std::io::BufReader::new(exch_rate_input);
    for (line_num, lines) in exch_rate_file.lines().enumerate() {
        let exch_rate_line = match lines {
            Ok(input_line) => input_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.currency_conversion_file_path(),
                line_num + 1,
                error
            ),
        };
        let exch_rate_fields = exch_rate_line.split('|').collect::<Vec<&str>>();
        if exch_rate_fields[1].to_string().trim() == config_params.base_currency() {
            exch_rate_map.insert(
                exch_rate_fields[0].to_string(),
                exch_rate_fields[2]
                    .to_string()
                    .parse::<f64>()
                    .unwrap_or(1.0),
            );
        }
    }

    //Granularity Weight Data Reader
    let mut granularity_weight = 0.0;
    let mut gran_perc = String::new();
    let gran_wt_input = std::fs::File::open(config_params.granularity_weight_file())
        .expect("Could Not Read Granularity File");
    let gran_wt_file = std::io::BufReader::new(gran_wt_input);
    for (line_num, lines) in gran_wt_file.lines().enumerate() {
        let gran_wt_line = match lines {
            Ok(input_line) => input_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.granularity_weight_file(),
                line_num + 1,
                error
            ),
        };
        let gran_wt_fields = gran_wt_line.split('|').collect::<Vec<&str>>();
        if gran_wt_fields.len() < 2 {
            panic!("Error Reading Granularity Weight Data\nMinimum Two Columns Expected (Gran-Perc|Gran-Amt)");
        }
        gran_perc = gran_wt_fields[0].to_string();
        granularity_weight = if config_params.is_granularity_perc() {
            gran_wt_fields[0]
        } else {
            gran_wt_fields[1]
        }
        .to_string()
        .parse::<f64>()
        .unwrap_or(1.0);
    }

    let mut classified_data: HashMap<String, AggrVal> = HashMap::new();
    let files_config = config::get_files(config_params.config_file_path());
    let mut tot_out_bal: f64 = 0.0;
    for file in files_config.files {
        //Get Required Data
        let mut file_reader =
            reader::Reader::new_at_path(&file.metadata_file_path, &file.input_file_path);
        let method_reader =
            reader::Reader::new_at_path(&file.metadata_file_path, &file.input_file_path);

        //Get Output Data
        for mut record in file_reader.iter() {
            acc_encountered += 1;
            cf_encountered += 1;
            let cust_id = match get_field_value(&record, &method_reader, file.cust_id.to_string()) {
                Ok(value) => value.to_string(),
                Err(_error) => panic!("Could not get Cust-ID for Record: {:?}", record),
            };
            let currency = match get_field_value(&record, &method_reader, file.currency.to_string())
            {
                Ok(value) => value.to_string(),
                Err(_error) => panic!("Could not get Currency for Cust-ID: {:?}", cust_id),
            };

            let mut out_bal = if !file.out_bal.clone().unwrap_or("".to_string()).is_empty() {
                match get_field_value(
                    &record,
                    &method_reader,
                    file.out_bal.clone().unwrap_or("".to_string()).to_string(),
                ) {
                    Ok(value) => value.parse::<f64>().unwrap_or(0.0),
                    Err(_error) => 0.0,
                }
            } else {
                let mut cf_amt: f64 = 0.0;
                if let Type::Cashflows = method_reader
                    .get_field_type(&file.cashflows.clone().unwrap_or("cashflows".to_string()))
                    .unwrap_or_else(|| {
                        panic!(
                            "Could not get Cashflows for amount field for the Customer: {:?}",
                            cust_id
                        )
                    })
                {
                    let default_cf_vec: Vec<Cashflow> = Vec::new();
                    let cashflow_vec = record
                        .remove_cfs_for_key(
                            &file.cashflows.clone().unwrap_or("cashflows".to_string()),
                        )
                        .unwrap_or(default_cf_vec);
                    for cf in cashflow_vec {
                        cf_encountered += 1;
                        cf_amt += cf.principal_amount;
                    }
                }
                cf_encountered -= 1;
                cf_amt
            };
            let limit_bal = if file.limit_bal.clone().unwrap_or("".to_string()).is_empty() {
                out_bal
            } else {
                match get_field_value(
                    &record,
                    &method_reader,
                    file.limit_bal.clone().unwrap_or("".to_string()).to_string(),
                ) {
                    Ok(value) => value.parse::<f64>().unwrap_or(0.0),
                    Err(_error) => 0.0,
                }
            };

            ip_amt += out_bal;
            acc_success += 1;
            if !file.is_consolidated.unwrap_or(false) {
                out_bal *= exch_rate_map.get(&currency).unwrap_or(&1.0);
            }
            tot_out_bal += out_bal;
            let aggr_val = AggrVal::new(1, out_bal, limit_bal);
            classified_data
                .entry(cust_id)
                .and_modify(|data| data.aggregateamount(aggr_val.clone()))
                .or_insert(aggr_val.clone());
            op_amt += tot_out_bal;
        }
    }

    let exposure_amt = if !config_params.is_granularity_perc() {
        granularity_weight
    } else {
        (if config_params.total_out_bal() == 0.0 {
            log_info!(
                logger,
                "Total Outstanding Balance (Aggregated): {}",
                tot_out_bal
            );
            tot_out_bal
        } else {
            log_info!(
                logger,
                "Total Outstanding Balance (Config-Params): {}",
                config_params.total_out_bal()
            );
            config_params.total_out_bal()
        } * granularity_weight)
            / 100.00
    };
    if config_params.is_granularity_perc() {
        log_info!(
            logger,
            "{} % of Total-Out-Bal Considered for Comparison: {}",
            granularity_weight,
            exposure_amt
        );
    } else {
        log_info!(
            logger,
            "{} Considered for Comparison with Exposure-Amt: {} as is_granularity_perc is false",
            granularity_weight,
            exposure_amt
        );
    }

    for (cust_id, aggr_data) in classified_data.iter() {
        let amt_to_be_compared = match config_params.compare_condition() {
            "MAX" => {
                if aggr_data.out_bal > aggr_data.limit_bal {
                    aggr_data.out_bal
                } else {
                    aggr_data.limit_bal
                }
            }
            "MIN" => {
                if aggr_data.out_bal > aggr_data.limit_bal {
                    aggr_data.limit_bal
                } else {
                    aggr_data.out_bal
                }
            }
            "TTL_BAL" => aggr_data.out_bal,
            "TTL_LIM_BAL" => aggr_data.limit_bal,
            _ => panic!(
                "Invalid Comparator passed: {}",
                config_params.compare_condition()
            ),
        };
        let status = if amt_to_be_compared <= exposure_amt {
            "ELIGIBLE"
        } else {
            "NON-ELIGIBLE"
        };
        writeln!(
            txt_writer,
            "{}|{}|{}|{}|{}|{}",
            cust_id,
            aggr_data.count,
            aggr_data.out_bal,
            aggr_data.limit_bal,
            amt_to_be_compared,
            status
        )
        .expect("Unable to generate aggregated summary file.");
        let mut output_data = Output::new();
        output_data.set_cust_id(cust_id.to_string());
        output_data.set_count(aggr_data.count);
        output_data.set_out_bal(aggr_data.out_bal);
        output_data.set_limit_bal(aggr_data.limit_bal);
        output_data.set_amt_considered(amt_to_be_compared);
        output_data.set_status(status.to_string());
        let account_byte_info = output_data
            .write_length_delimited_to_bytes()
            .expect("unable convert into bytes");
        cf_output_file
            .write_all(&account_byte_info)
            .expect("unable to write to output file");
    }

    //Init Granularity Writer
    let mut granularity_writer = buf_file_wrtr(
        &format!(
            "{}-output.txt",
            config_params.granularity_weight_file().replace(".txt", "")
        ),
        None,
    )
    .expect("Unable to Create Writer for Granularity Output File.");

    //Writing Granularity Weight Data
    write!(granularity_writer, "{}|{}", gran_perc, exposure_amt,)
        .expect("Unable to generate granularity weight file.");

    //Write Health Check Report
    let health_report = HealthReport::new(
        acc_encountered,
        acc_success,
        acc_encountered - acc_success,
        ip_amt,
        op_amt,
        cf_encountered,
    );
    log_info!(logger, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file_path());
}
