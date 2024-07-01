use aggregator::account_field_names::AccFieldNames;
use aggregator::structs::{DrilldownOpTrailingFields, OpLeadingFields, SummaryOpTrailingFields};
use calamine::{open_workbook_auto, Reader};
use chrono::Local;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::{num_days_start_to_end, NaiveDate};
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::types::Type;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
mod account_field_names;
mod structs;

struct AccDet {
    acc_no: String,
    amt_booked: f64,
}

struct Data {
    pdt_code: String,
    ccy: String,
    cust_name: String,
    account_det: Vec<AccDet>,
    agg_booking_amt: f64,
    mat_dt: NaiveDate,
    mis1: String,
    division: String,
    source_gl: String,
    bucket_val: String,
    tot_int_amt: f64,
    alm_line: String,
    ia_line: String,
    concat: String,
    lcr_cat: String,
}

pub fn aggregate_cashflows(
    config_params: ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) {
    let mut acc_enc = 0;
    let mut acc_succ = 0;
    let tot_amt = 0.0;
    let tot_cfs = 0;
    let mut summary_rows = 0;
    let mut drilldown_rows = 0;

    //Output file for summary report.
    let mut summary_report_path = String::new();
    summary_report_path.push_str(config_params.summary_file_path());
    let mut summary_writer = match buf_file_wrtr(&summary_report_path, None) {
        Ok(wrtr) => wrtr,
        Err(error) => {
            panic!(
                "Could not create file: `{}` at location `{}` : {:?}.",
                config_params.summary_file_path(),
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };

    //Output file for drilldown report.
    let mut drilldown_report_path = String::new();
    drilldown_report_path.push_str(config_params.drilldown_file_path());
    let mut drilldown_writer = match buf_file_wrtr(&drilldown_report_path, None) {
        Ok(wrtr) => wrtr,
        Err(error) => {
            panic!(
                "Could not create file: `{}` at location `{}` : {:?}.",
                config_params.drilldown_file_path(),
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };

    let mut tenor_map: HashMap<String, String> = HashMap::new();
    let mut tenor_master_excel =
        open_workbook_auto(config_params.master_file()).expect("Unable to open Tenor Master File.");
    if let Some(Ok(reader)) = tenor_master_excel.worksheet_range(config_params.sheet_name()) {
        for row in reader.rows().skip(1) {
            tenor_map.insert(row[0].to_string(), row[3].to_string());
        }
    }

    let mut lcr_map_ret: HashMap<String, String> = HashMap::new();
    let mut lcr_map_non_ret: HashMap<String, String> = HashMap::new();
    let lcr_reader_ret = fs::read_to_string(&config_params.ret_cust_aggr_lcy_file())
        .expect("Failed to read ret_cust_aggr_lcy_file file!");
    for line in lcr_reader_ret.lines().skip(1) {
        let lcr_fields = line.split('|').collect::<Vec<&str>>();
        if lcr_fields.len() >= 2 {
            lcr_map_ret.insert(lcr_fields[1].to_string(), lcr_fields[0].to_string());
        }
    }
    let lcr_reader_non_ret = fs::read_to_string(&config_params.non_ret_cust_aggr_lcy_file())
        .expect(
            "Failed to read non_ret_cust_aggr_lcy_file: String,
            file!",
        );
    for line in lcr_reader_non_ret.lines().skip(1) {
        let lcr_fields = line.split('|').collect::<Vec<&str>>();
        if lcr_fields.len() >= 2 {
            lcr_map_non_ret.insert(lcr_fields[1].to_string(), lcr_fields[0].to_string());
        }
    }

    let keys = AccFieldNames::new_from_path(config_params.req_fields_file_path());

    let mut cur_mth_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.cur_mth_src_file(),
    );

    let field_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.cur_mth_src_file(),
    );
    //Hashmap to hold fields in the order:HashMap<cust_id, HashMap<(value_dt, org_tenor), Data>>
    let mut data_map: HashMap<String, HashMap<(NaiveDate, i64), Data>> = HashMap::new();

    let summary_header = "Customer Id|Product Code|CCY|Customer Name|Value Date|Aggregate Booking |Maturity Date|MIS 1|Division|Source GL|Org Tenor|Bucket|Effective Rate|ALM Line|IA Line|Concat|Count of TDs|LCR Category\n";
    let drilldown_header = "Customer Id|Product Code|CCY|Account Number|Customer Name|Value Date|O/S Amount |Maturity Date|MIS 1|Division|Source GL|Org Tenor|Bucket|Effective Rate|ALM Line|IA Line|Concat|LCR Category\n";
    write_data(&mut summary_writer, summary_header.to_string(), logger);
    write_data(&mut drilldown_writer, drilldown_header.to_string(), logger);

    for account in cur_mth_reader.iter() {
        acc_enc += 1;
        //Find customer ID for the account.
        let cust_id = match field_reader.get_field_type(&keys.cust_id) {
            Some(typ) => match typ {
                Type::I64 => account
                    .get_i64_for_key(&keys.cust_id)
                    .unwrap_or(0)
                    .to_string(),
                Type::F64 => account
                    .get_f64_for_key(&keys.cust_id)
                    .unwrap_or(0.0)
                    .to_string(),
                Type::String => account
                    .get_string_for_key(&keys.cust_id)
                    .unwrap_or(&"NA".to_string())
                    .to_string(),
                _ => "NA".to_string(),
            },
            None => "NA".to_string(),
        };

        // Check if customer id is already a key in hashmap.
        match data_map.get_mut(&cust_id) {
            Some(data_vec) => {
                //Calculate value date and tenor
                let value_dt =
                    naivedate_from_timestamp(account.get_i64_for_key(&keys.value_dt).unwrap_or(0))
                        .format("%d-%m-%Y")
                        .to_string();
                let value_dt = NaiveDate::parse_from_str(&value_dt, "%d-%m-%Y")
                    .expect("Could not parse value date");
                let mat_dt =
                    naivedate_from_timestamp(account.get_i64_for_key(&keys.mat_dt).unwrap_or(0))
                        .format("%d-%m-%Y")
                        .to_string();
                let mat_dt = NaiveDate::parse_from_str(&mat_dt, "%d-%m-%Y")
                    .expect("Could not parse maturity date");
                let org_tenor = num_days_start_to_end(value_dt, mat_dt);
                let acc_no = account
                    .get_string_for_key(&keys.acc_no)
                    .unwrap_or(&"NA".to_string())
                    .to_string();
                let amt_booked = account.get_f64_for_key(&keys.amt_booked).unwrap_or(0.00);
                let acc_det = AccDet {
                    acc_no: acc_no.to_string(),
                    amt_booked,
                };
                let int_rt = account.get_f64_for_key(&keys.int_rt).unwrap_or(0.00);
                let int_amt = amt_booked * int_rt / 100.0;
                //Check if value date and tenor are found in hashmap.
                match data_vec.get_mut(&(value_dt, org_tenor)) {
                    Some(data) => {
                        data.account_det.push(acc_det);
                        data.agg_booking_amt += amt_booked;
                        data.tot_int_amt += int_amt;
                    }
                    None => {
                        //TODO: Create a function for inserting data to data_vec
                        let pdt_code = account
                            .get_string_for_key(&keys.pdt_code)
                            .unwrap_or(&"NA".to_string())
                            .to_string();
                        let ccy = account
                            .get_string_for_key(&keys.ccy)
                            .unwrap_or(&"NA".to_string())
                            .to_string();
                        let cust_name = account
                            .get_string_for_key(&keys.cust_name)
                            .unwrap_or(&"NA".to_string())
                            .to_string();
                        let mis1 = match field_reader.get_field_type(&keys.mis1) {
                            Some(typ) => match typ {
                                Type::I64 => {
                                    account.get_i64_for_key(&keys.mis1).unwrap_or(0).to_string()
                                }
                                Type::F64 => account
                                    .get_f64_for_key(&keys.mis1)
                                    .unwrap_or(0.0)
                                    .to_string(),
                                Type::String => account
                                    .get_string_for_key(&keys.mis1)
                                    .unwrap_or(&"NA".to_string())
                                    .to_string(),
                                _ => "NA".to_string(),
                            },
                            None => "NA".to_string(),
                        };
                        let division = match field_reader.get_field_type(&keys.division) {
                            Some(typ) => match typ {
                                Type::I64 => account
                                    .get_i64_for_key(&keys.division)
                                    .unwrap_or(0)
                                    .to_string(),
                                Type::F64 => account
                                    .get_f64_for_key(&keys.division)
                                    .unwrap_or(0.0)
                                    .to_string(),
                                Type::String => account
                                    .get_string_for_key(&keys.division)
                                    .unwrap_or(&"NA".to_string())
                                    .to_string(),
                                _ => "NA".to_string(),
                            },
                            None => "NA".to_string(),
                        };
                        let source_gl = match field_reader.get_field_type(&keys.source_gl) {
                            Some(typ) => match typ {
                                Type::I64 => account
                                    .get_i64_for_key(&keys.source_gl)
                                    .unwrap_or(0)
                                    .to_string(),
                                Type::F64 => account
                                    .get_f64_for_key(&keys.source_gl)
                                    .unwrap_or(0.0)
                                    .to_string(),
                                Type::String => account
                                    .get_string_for_key(&keys.source_gl)
                                    .unwrap_or(&"NA".to_string())
                                    .to_string(),
                                _ => "NA".to_string(),
                            },
                            None => "NA".to_string(),
                        };
                        let alm_line = account
                            .get_string_for_key(&keys.alm_line)
                            .unwrap_or(&"NA".to_string())
                            .to_string();
                        let ia_line = account
                            .get_string_for_key(&keys.ia_line)
                            .unwrap_or(&"NA".to_string())
                            .to_string();
                        let concat = account
                            .get_string_for_key(&keys.concat)
                            .unwrap_or(&"NA".to_string())
                            .to_string();
                        let lcr_cat = match lcr_map_ret.get(&cust_id) {
                            Some(val) => val.to_string(),
                            None => match lcr_map_non_ret.get(&cust_id) {
                                Some(val) => val.to_string(),
                                None => "NA".to_string(),
                            },
                        };
                        let def_str_val = String::from("NA");
                        let bucket_val = tenor_map
                            .get(&org_tenor.to_string())
                            .unwrap_or(&def_str_val);
                        let acc_det = vec![AccDet { acc_no, amt_booked }];
                        data_vec.insert(
                            (value_dt, org_tenor),
                            Data {
                                pdt_code,
                                ccy,
                                cust_name,
                                mat_dt,
                                mis1,
                                division,
                                source_gl,
                                alm_line,
                                ia_line,
                                concat,
                                lcr_cat,
                                account_det: acc_det,
                                agg_booking_amt: amt_booked,
                                tot_int_amt: int_amt,
                                bucket_val: bucket_val.to_string(),
                            },
                        );
                    }
                };
            }
            None => {
                let acc_no = account
                    .get_string_for_key(&keys.acc_no)
                    .unwrap_or(&"NA".to_string())
                    .to_string();
                let cust_id = match field_reader.get_field_type(&keys.cust_id) {
                    Some(typ) => match typ {
                        Type::I64 => account
                            .get_i64_for_key(&keys.cust_id)
                            .unwrap_or(0)
                            .to_string(),
                        Type::F64 => account
                            .get_f64_for_key(&keys.cust_id)
                            .unwrap_or(0.0)
                            .to_string(),
                        Type::String => account
                            .get_string_for_key(&keys.cust_id)
                            .unwrap_or(&"NA".to_string())
                            .to_string(),
                        _ => "NA".to_string(),
                    },
                    None => "NA".to_string(),
                };

                let pdt_code = account
                    .get_string_for_key(&keys.pdt_code)
                    .unwrap_or(&"NA".to_string())
                    .to_string();

                let ccy = account
                    .get_string_for_key(&keys.ccy)
                    .unwrap_or(&"NA".to_string())
                    .to_string();

                let cust_name = account
                    .get_string_for_key(&keys.cust_name)
                    .unwrap_or(&"NA".to_string())
                    .to_string();

                let value_dt =
                    naivedate_from_timestamp(account.get_i64_for_key(&keys.value_dt).unwrap_or(0))
                        .format("%d-%m-%Y")
                        .to_string();
                let value_dt = NaiveDate::parse_from_str(&value_dt, "%d-%m-%Y")
                    .expect("Could not parse value date");

                let mat_dt =
                    naivedate_from_timestamp(account.get_i64_for_key(&keys.mat_dt).unwrap_or(0))
                        .format("%d-%m-%Y")
                        .to_string();
                let mat_dt = NaiveDate::parse_from_str(&mat_dt, "%d-%m-%Y")
                    .expect("Could not parse mat date");

                let org_tenor = num_days_start_to_end(value_dt, mat_dt);

                let mis1 = match field_reader.get_field_type(&keys.mis1) {
                    Some(typ) => match typ {
                        Type::I64 => account.get_i64_for_key(&keys.mis1).unwrap_or(0).to_string(),
                        Type::F64 => account
                            .get_f64_for_key(&keys.mis1)
                            .unwrap_or(0.0)
                            .to_string(),
                        Type::String => account
                            .get_string_for_key(&keys.mis1)
                            .unwrap_or(&"NA".to_string())
                            .to_string(),
                        _ => "NA".to_string(),
                    },
                    None => "NA".to_string(),
                };
                let division = match field_reader.get_field_type(&keys.division) {
                    Some(typ) => match typ {
                        Type::I64 => account
                            .get_i64_for_key(&keys.division)
                            .unwrap_or(0)
                            .to_string(),
                        Type::F64 => account
                            .get_f64_for_key(&keys.division)
                            .unwrap_or(0.0)
                            .to_string(),
                        Type::String => account
                            .get_string_for_key(&keys.division)
                            .unwrap_or(&"NA".to_string())
                            .to_string(),
                        _ => "NA".to_string(),
                    },
                    None => "NA".to_string(),
                };
                let source_gl = match field_reader.get_field_type(&keys.source_gl) {
                    Some(typ) => match typ {
                        Type::I64 => account
                            .get_i64_for_key(&keys.source_gl)
                            .unwrap_or(0)
                            .to_string(),
                        Type::F64 => account
                            .get_f64_for_key(&keys.source_gl)
                            .unwrap_or(0.0)
                            .to_string(),
                        Type::String => account
                            .get_string_for_key(&keys.source_gl)
                            .unwrap_or(&"NA".to_string())
                            .to_string(),
                        _ => "NA".to_string(),
                    },
                    None => "NA".to_string(),
                };
                let alm_line = account
                    .get_string_for_key(&keys.alm_line)
                    .unwrap_or(&"NA".to_string())
                    .to_string();
                let ia_line = account
                    .get_string_for_key(&keys.ia_line)
                    .unwrap_or(&"NA".to_string())
                    .to_string();
                let concat = account
                    .get_string_for_key(&keys.concat)
                    .unwrap_or(&"NA".to_string())
                    .to_string();
                let lcr_cat = match lcr_map_ret.get(&cust_id) {
                    Some(val) => val.to_string(),
                    None => match lcr_map_non_ret.get(&cust_id) {
                        Some(val) => val.to_string(),
                        None => "NA".to_string(),
                    },
                };
                let def_str_val = String::from("NA");
                let bucket_val = tenor_map
                    .get(&org_tenor.to_string())
                    .unwrap_or(&def_str_val);

                let amt_booked = account.get_f64_for_key(&keys.amt_booked).unwrap_or(0.00);
                let int_rt = account.get_f64_for_key(&keys.int_rt).unwrap_or(0.00);
                let int_amt = amt_booked * int_rt / 100.0;
                let mut hashmap2 = HashMap::new();
                let acc_det = vec![AccDet { acc_no, amt_booked }];

                //TODO:Create a function for inseting data to hashmap to avoid code repetition.

                hashmap2.insert(
                    (value_dt, org_tenor),
                    Data {
                        pdt_code,
                        ccy,
                        cust_name,
                        mat_dt,
                        mis1,
                        division,
                        source_gl,
                        alm_line,
                        ia_line,
                        concat,
                        lcr_cat,
                        account_det: acc_det,
                        agg_booking_amt: amt_booked,
                        tot_int_amt: int_amt,
                        bucket_val: bucket_val.to_string(),
                    },
                );
                data_map.insert(cust_id, hashmap2);
            }
        }
    }

    for (key, val) in data_map.iter() {
        let cust_id = key;
        for (key2, val2) in val.iter() {
            let (value_dt, org_tenor) = key2;
            let tot_int_amt = val2.tot_int_amt;
            let tot_booking_amt = &val2.agg_booking_amt;
            //Effective interest rate.
            let effective_rate = tot_int_amt / tot_booking_amt * 100.0;
            let leading_fields = OpLeadingFields {
                cust_id: cust_id.to_string(),
                pdt_code: val2.pdt_code.to_string(),
                ccy: val2.ccy.to_string(),
            };
            let summary_trailing_fields = SummaryOpTrailingFields {
                cust_name: val2.cust_name.to_string(),
                value_dt: value_dt.format("%d-%m-%Y").to_string(),
                agg_booking: tot_booking_amt.to_string(),
                mat_dt: val2.mat_dt.format("%d-%m-%Y").to_string(),
                mis1: val2.mis1.to_string(),
                division: val2.division.to_string(),
                source_gl: val2.source_gl.to_string(),
                org_tenor: org_tenor.to_string(),
                bucket: val2.bucket_val.to_string(),
                effective_rate: effective_rate.to_string(),
                alm_line: val2.alm_line.to_string(),
                ia_line: val2.ia_line.to_string(),
                concat: val2.concat.to_string(),
                count_td: val2.account_det.len().to_string(),
                lcr_cat: val2.lcr_cat.to_string(),
            };
            //write to summary report
            let mut summary_op: String = leading_fields.print().to_owned();
            summary_op.push_str(&summary_trailing_fields.print());
            write_data(&mut summary_writer, summary_op, logger);
            summary_rows += 1;
            for account_det in &val2.account_det {
                acc_succ += 1;
                let mut drilldown_op: String = leading_fields.print().to_owned();
                let drilldown_trailing_fields = DrilldownOpTrailingFields {
                    acc_no: account_det.acc_no.to_string(),
                    cust_name: val2.cust_name.to_string(),
                    value_dt: value_dt.format("%d-%m-%Y").to_string(),
                    os_amt: account_det.amt_booked.to_string(),
                    mat_dt: val2.mat_dt.format("%d-%m-%Y").to_string(),
                    mis1: val2.mis1.to_string(),
                    division: val2.division.to_string(),
                    source_gl: val2.source_gl.to_string(),
                    org_tenor: org_tenor.to_string(),
                    bucket: val2.bucket_val.to_string(),
                    effective_rate: effective_rate.to_string(),
                    alm_line: val2.alm_line.to_string(),
                    ia_line: val2.ia_line.to_string(),
                    concat: val2.concat.to_string(),
                    lcr_cat: val2.lcr_cat.to_string(),
                };
                drilldown_op.push_str(&drilldown_trailing_fields.print());
                write_data(&mut drilldown_writer, drilldown_op, logger);
                drilldown_rows += 1;
            }
        }
    }
    let timestamp = Local::now().naive_local().to_string();
    let as_on_date: NaiveDate = NaiveDate::parse_from_str(&config_params.as_on_date(), "%Y%m%d")
        .expect("Could not parse as on date");
    let footer_summary = format!(
        "FTR|{}|{}|{}\n",
        as_on_date.format("%d-%m-%Y"),
        timestamp,
        summary_rows
    );
    let footer_drilldown = format!(
        "FTR|{}|{}|{}\n",
        as_on_date.format("%d-%m-%Y"),
        timestamp,
        drilldown_rows
    );
    write_data(&mut summary_writer, footer_summary, logger);
    write_data(&mut drilldown_writer, footer_drilldown, logger);

    let health_report = HealthReport::new(
        acc_enc,
        acc_succ,
        acc_enc - acc_succ,
        tot_amt,
        tot_amt,
        tot_cfs,
    );
    health_report.gen_health_rpt(&drilldown_report_path);

    pub fn write_data(writer: &mut BufWriter<File>, op: String, logger: &Logger) {
        let output_as_bytes = op.as_bytes();
        match writer.write(output_as_bytes) {
            Ok(_val) => {}
            Err(err) => {
                log_info!(logger, "Error writing to output file. Error: {}", err);
            }
        }
    }

    fn naivedate_from_timestamp(t: i64) -> NaiveDate {
        let naive_date_time = rbdate::NaiveDateTime::from_timestamp(t, 0);
        naive_date_time.date()
    }
}
