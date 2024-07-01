use aggregator::account_field_names::AccFieldNames;
use aggregator::bucket::*;
use aggregator::structs::*;
use aggregator::tenor::*;
use calamine::{open_workbook_auto, Reader};
use chrono::Local;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::{num_days_start_to_end, NaiveDate};
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::types::Type;
use sdb_io::buf_file_wrtr;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
mod account_field_names;
mod bucket;
mod structs;
mod tenor;

pub fn aggregate(config_params: ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let mut acc_enc = 0;
    let mut acc_succ = 0;
    let mut tot_amt = 0.0;
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

    //Create hashmaps for the master files.
    let mut clubbed_tenor_map: HashMap<String, String> = HashMap::new();
    let mut tenor_desc_map: HashMap<Tenor, String> = HashMap::new();
    let mut tenor_master_excel = open_workbook_auto(config_params.club_ten_master())
        .expect("Unable to open Clubbed Tenor Master File.");
    if let Some(Ok(reader)) = tenor_master_excel.worksheet_range(config_params.club_ten_sheet()) {
        for row in reader.rows().skip(1) {
            let tenor = Tenor::new(row[1].to_string(), row[2].to_string());
            tenor_desc_map.insert(tenor, row[3].to_string());
            clubbed_tenor_map.insert(row[3].to_string(), row[4].to_string());
        }
    } else {
        log_error!(
            logger,
            "1.Unable to read master file:{}, sheet:{}",
            config_params.club_ten_master(),
            config_params.club_ten_sheet()
        );
    }

    let mut cust_type_staff: Vec<String> = Vec::new();
    let mut cust_type_senior: Vec<String> = Vec::new();
    let mut cust_type_excel = open_workbook_auto(config_params.cust_type_master())
        .expect("Unable to open Customer type Master File.");
    if let Some(Ok(reader)) = cust_type_excel.worksheet_range(config_params.cust_type_staff_sheet())
    {
        for row in reader.rows() {
            cust_type_staff.push(row[0].to_string());
        }
    } else {
        log_error!(
            logger,
            "2.Unable to read master file:{}, sheet:{}",
            config_params.cust_type_master(),
            config_params.cust_type_staff_sheet()
        );
    }
    if let Some(Ok(reader)) =
        cust_type_excel.worksheet_range(config_params.cust_type_senior_sheet())
    {
        for row in reader.rows() {
            cust_type_senior.push(row[0].to_string());
        }
    } else {
        log_error!(
            logger,
            "3.Unable to read master file:{}, sheet:{}",
            config_params.cust_type_master(),
            config_params.cust_type_senior_sheet()
        );
    }

    let mut tenor_rt_staff_map: HashMap<String, String> = HashMap::new();
    let mut tenor_rt_senior_map: HashMap<String, String> = HashMap::new();
    let mut tenor_rt_others_map: HashMap<String, String> = HashMap::new();
    let mut tenor_rate_excel = open_workbook_auto(config_params.club_ten_rate_master())
        .expect("Unable to open Clubbed tenor rate Master File.");
    if let Some(Ok(reader)) = tenor_rate_excel.worksheet_range(config_params.tenor_rate_staff()) {
        for row in reader.rows().skip(1) {
            tenor_rt_staff_map.insert(row[0].to_string(), row[1].to_string());
        }
    } else {
        log_error!(
            logger,
            "4.Unable to read from master file:{}, sheet:{}",
            config_params.club_ten_rate_master(),
            config_params.tenor_rate_staff()
        );
    }
    if let Some(Ok(reader)) = tenor_rate_excel.worksheet_range(config_params.tenor_rate_senior()) {
        for row in reader.rows().skip(1) {
            tenor_rt_senior_map.insert(row[0].to_string(), row[1].to_string());
        }
    } else {
        log_error!(
            logger,
            "5.Unable to read from master file:{}, sheet:{}",
            config_params.club_ten_rate_master(),
            config_params.tenor_rate_senior()
        );
    }
    if let Some(Ok(reader)) = tenor_rate_excel.worksheet_range(config_params.tenor_rate_others()) {
        for row in reader.rows().skip(1) {
            tenor_rt_others_map.insert(row[0].to_string(), row[1].to_string());
        }
    } else {
        log_error!(
            logger,
            "6.Unable to read from master file:{}, sheet:{}",
            config_params.club_ten_rate_master(),
            config_params.tenor_rate_others()
        );
    }
    //Log error if hashmap is empty.
    if tenor_rt_staff_map.is_empty() {
        log_error!(
            logger,
            "No clubbed tenor rate found for sheet: `{}` in file `{}`",
            config_params.tenor_rate_staff(),
            config_params.club_ten_rate_master()
        );
    }
    if tenor_rt_senior_map.is_empty() {
        log_error!(
            logger,
            "No clubbed tenor rate found for sheet: `{}` in file `{}`",
            config_params.tenor_rate_senior(),
            config_params.club_ten_rate_master()
        );
    }
    if tenor_rt_others_map.is_empty() {
        log_error!(
            logger,
            "No clubbed tenor rate found for sheet: `{}` in file `{}`",
            config_params.tenor_rate_others(),
            config_params.club_ten_rate_master()
        );
    }

    let amb_reader = match new_buf_rdr(config_params.amb_master()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.amb_master(),
            env::current_exe()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut amb_map: HashMap<String, String> = HashMap::new();
    for (line_num, lines) in amb_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    logger,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_params.amb_master(),
                    line_num + 1,
                    error
                );
                "".to_string()
            }
        };

        let fields: Vec<&str> = line.split("~#~").collect();
        if fields.len() >= 7 {
            amb_map.insert(fields[1].to_string(), fields[6].to_string());
        }
    }

    let mut bucket_map: HashMap<Bucket, String> = HashMap::new();
    let mut bucket_master_excel = open_workbook_auto(config_params.bucket_master())
        .expect("Unable to open Bucket Master File.");
    if let Some(Ok(reader)) = bucket_master_excel.worksheet_range(config_params.bucket_sheet()) {
        for row in reader.rows().skip(1) {
            let bucket = Bucket::new(row[0].to_string(), row[1].to_string());
            bucket_map.insert(bucket, row[2].to_string());
        }
    } else {
        log_error!(
            logger,
            "7.Unable to read from master file:{}, sheet:{}",
            config_params.bucket_master(),
            config_params.bucket_sheet()
        );
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

    let mut input_file_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );

    let field_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );

    let summary_header = "Customer Id|Product Code|Customer Name|Aggregate Booking |Count of TDs|Division|Weighted Tenor days|Clubbed Tenor|Org Contract weight rate|Clubbed Tenor Rate|MIS 1|CCY|Amount Bucket|LCR Category|Average Balance\n";
    let drilldown_header = "Customer Id|Product Code|Account Number|Customer Name|O/S balance |Division|Org Tenor|Weighted Avg Tenor|Original Tenor Description|Clubbed Tenor|Org Contract weight rate|Clubbed Tenor Rate|MIS 1|CCY|Amount Bucket|LCR Category\n";
    write_data(&mut summary_writer, summary_header.to_string(), logger);
    write_data(&mut drilldown_writer, drilldown_header.to_string(), logger);

    //Hashmap to hold fields in the order: HashMap<cust_id, HashMap<Key, Value>>
    let mut output_map: HashMap<String, HashMap<Key, Value>> = HashMap::new();
    for account in input_file_reader.iter() {
        acc_enc += 1;
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
        let op_keys: Key = get_key(&account, &keys);
        let acc_no = account
            .get_string_for_key(&keys.acc_no)
            .unwrap_or(&"NA".to_string())
            .to_string();
        let int_rt = account.get_f64_for_key(&keys.int_rt).unwrap_or(0.00);
        let os_amt = account.get_f64_for_key(&keys.os_amt).unwrap_or(0.00);
        let weighted_int_rt = os_amt * int_rt / 100.0;
        let weighted_tenor_rt = os_amt * op_keys.tenor as f64;
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
        //Note: op_keys.tenor = maturity_date - value_date.
        let mat_dt = naivedate_from_timestamp(account.get_i64_for_key(&keys.mat_dt).unwrap_or(0))
            .format("%d-%m-%Y")
            .to_string();
        let mat_dt =
            NaiveDate::parse_from_str(&mat_dt, "%d-%m-%Y").expect("Could not parse maturity date");
        //Check if customer ID is present as key in hashmap.
        match output_map.get_mut(&cust_id) {
            Some(data_vec) => {
                //Search if account with similar tenor and value month exists.
                match data_vec.get_mut(&op_keys) {
                    Some(data) => {
                        //Append values to existing fields.
                        data.account_det.push(AccDetail { acc_no, os_amt });
                        data.agg_booking += os_amt;
                        data.weighted_int_rt += weighted_int_rt;
                        data.weighted_tenor_rt += weighted_tenor_rt;
                    }
                    None => {
                        //Insert a new key-value pair in output hashmap.
                        let op_values: Value = get_value(
                            &account,
                            &keys,
                            acc_no,
                            os_amt,
                            weighted_tenor_rt,
                            weighted_int_rt,
                            mat_dt,
                            division,
                        );
                        data_vec.insert(op_keys, op_values);
                    }
                }
            }
            None => {
                //Make a new entry in the hashmap.
                let op_values: Value = get_value(
                    &account,
                    &keys,
                    acc_no,
                    os_amt,
                    weighted_tenor_rt,
                    weighted_int_rt,
                    mat_dt,
                    division,
                );
                let mut data_map = HashMap::new();
                data_map.insert(op_keys, op_values);
                output_map.insert(cust_id, data_map);
            }
        };
    }
    //Calculate all derived fields before writing to output report.
    for (cust_id, value1) in output_map.iter() {
        //Ignoring customer IDs which have a single account number.
        for (key, value) in value1.iter() {
            if value.account_det.len() > 1 {
                let count_of_tds = value.account_det.len().to_string();
                let wt_tenor_days = value.weighted_tenor_rt / value.agg_booking;
                let org_wt_rt = value.weighted_int_rt / value.agg_booking * 100.0;
                let org_tenor_desc = get_tenor(value.org_tenor.to_string(), &tenor_desc_map);
                //Note: value.org_tenor = maturity date - account open date.
                let clubbed_tenor = match clubbed_tenor_map.get(&org_tenor_desc) {
                    Some(val) => val.to_string(),
                    None => "NA".to_string(),
                };

                //Get clubbed tenor rate based on customer type:
                let club_ten_rate = get_clubbed_tenor_rate(
                    &key.pdt_code,
                    &cust_type_staff,
                    &cust_type_senior,
                    &clubbed_tenor,
                    &tenor_rt_staff_map,
                    &tenor_rt_senior_map,
                    &tenor_rt_others_map,
                );

                let mis1 = match amb_map.get(cust_id) {
                    Some(val) => val.to_string(),
                    None => "NA".to_string(),
                };
                let bucket = get_amt_cat(value.agg_booking.to_string(), &bucket_map);
                let lcr_cat = match lcr_map_ret.get(cust_id) {
                    Some(val) => val.to_string(),
                    None => match lcr_map_non_ret.get(cust_id) {
                        Some(val) => val.to_string(),
                        None => "NA".to_string(),
                    },
                };
                tot_amt += &value.agg_booking;
                let avg_bal = value.agg_booking / value.account_det.len() as f64;

                let summary_op_line = SummaryOp {
                    cust_id: cust_id.to_string(),
                    pdt_code: key.pdt_code.to_string(),
                    cust_name: value.customer_name.to_string(),
                    agg_booking: value.agg_booking.to_string(),
                    count_of_tds: count_of_tds.to_string(),
                    division: value.division.to_string(),
                    wt_tenor_days: wt_tenor_days.to_string(),
                    clubbed_tenor: clubbed_tenor.to_string(),
                    org_wt_rt: org_wt_rt.to_string(),
                    club_ten_rate: club_ten_rate.to_string(),
                    mis1: mis1.to_string(),
                    ccy: value.ccy.to_string(),
                    bucket: bucket.to_string(),
                    lcr_cat: lcr_cat.to_string(),
                    avg_bal: avg_bal.to_string(),
                };
                //Write to summary report.
                write_data(&mut summary_writer, summary_op_line.print(), logger);
                summary_rows += 1;

                for acc_det in &value.account_det {
                    acc_succ += 1;
                    let drilldown_op_line = DrilldownOp {
                        cust_id: cust_id.to_string(),
                        pdt_code: key.pdt_code.to_string(),
                        account_no: acc_det.acc_no.to_string(),
                        customer_name: value.customer_name.to_string(),
                        os_amt: acc_det.os_amt.to_string(),
                        division: value.division.to_string(),
                        tenor: key.tenor.to_string(),
                        wt_tenor_days: wt_tenor_days.to_string(),
                        org_tenor_desc: org_tenor_desc.to_string(),
                        clubbed_tenor: clubbed_tenor.to_string(),
                        org_wt_rt: org_wt_rt.to_string(),
                        club_ten_rate: club_ten_rate.to_string(),
                        mis1: mis1.to_string(),
                        ccy: value.ccy.to_string(),
                        bucket: bucket.to_string(),
                        lcr_cat: lcr_cat.to_string(),
                    };
                    //Write to drilldown report.
                    write_data(&mut drilldown_writer, drilldown_op_line.print(), logger);
                    drilldown_rows += 1;
                }
            }
        }
    }
    let as_on_date: NaiveDate = NaiveDate::parse_from_str(&config_params.as_on_date(), "%Y%m%d")
        .expect("Could not parse as on date");
    let timestamp = Local::now().naive_local().to_string();
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
        tot_cfs as i64,
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
}
