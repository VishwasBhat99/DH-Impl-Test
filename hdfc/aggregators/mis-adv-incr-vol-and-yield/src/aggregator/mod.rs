use crate::aggregator::account_field_names::{PrevData, RatingData};
use crate::aggregator::output_struct::{format_output, get_writer, OutputData};
use aggregator::account_field_names::AccFieldNames;
use aggregator::tenor::*;
use calamine::open_workbook_auto;
use calamine::Reader;
use chrono::{Datelike, Duration};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::{date_from_timestamp, NaiveDate};
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use sdb_dyn_proto_rdr::reader::types::Type;
use sdb_io::{buf_file_wrtr, new_buf_rdr, open_file_read};
use slog::Logger;
use std::collections::HashMap;
use std::env::{self, current_dir};
use std::fs;
use std::io::prelude::*;
use std::io::Read;
use std::io::{prelude::*, BufWriter};
use std::path::PathBuf;
use std::time::SystemTime;
use std::{fs::File, io::Write, path::Path};

mod account_field_names;
pub mod config;
mod output_struct;
mod tenor;

pub fn aggregate(config_params: ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let start_time = SystemTime::now();
    let output_file = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => {
            panic!(
                "Could not create output file: `{}` on location `{}`: {}.",
                config_params.output_file_path(),
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    //Reading tenor excel file
    let mut tenor_excel = open_workbook_auto(config_params.org_tenor_file_path())
        .expect("Unable to open original tenor file.");
    let mut tenor_map: HashMap<Tenor, String> = HashMap::new();
    if let Some(Ok(reader)) = tenor_excel.worksheet_range(config_params.org_tenor_sheet_name()) {
        for row in reader.rows() {
            let key: Tenor = Tenor::new(row[0].to_string(), row[1].to_string());
            let val: String = row[2].to_string();
            tenor_map.insert(key, val);
        }
    }
    //Reading alco excel file
    let mut alco_excel = open_workbook_auto(config_params.alco_map_file_path())
        .expect("Unable to open alco excel file.");
    let mut alco_map: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = alco_excel.worksheet_range(config_params.alco_map_sheet_name()) {
        for row in reader.rows() {
            alco_map.insert(row[0].to_string(), row[1].to_string());
        }
    }
    //Reading psl excel file
    let mut psl_excel = open_workbook_auto(config_params.psl_map_file_path())
        .expect("Unable to open psl excel file.");
    let mut psl_map: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = psl_excel.worksheet_range(config_params.psl_map_sheet_name()) {
        for row in reader.rows() {
            psl_map.insert(row[0].to_string(), row[1].to_string());
        }
    }
    let mut rating_excel = open_workbook_auto(config_params.rate_bucket_file_path())
        .expect("Unable to open Rate bucket file.");

    let mut acc_enc = 0;
    let mut acc_succ = 0;
    let mut ip_amt = 0.0;
    let mut op_amt = 0.0;
    let mut writer = BufWriter::new(output_file);
    let files_config = config::get_files(config_params.config_file_path());
    let mut output_map: HashMap<String, OutputData> = HashMap::new();
    let as_on_date: &NaiveDate = config_params.as_on_date();
    let first_day_of_curr_month = NaiveDate::from_ymd(as_on_date.year(), as_on_date.month(), 1);
    let prev_month_end_date = first_day_of_curr_month - chrono::Duration::days(1);

    //Finding the latest available output file
    let as_on_date_no_hyphen = as_on_date.format("%d%m%Y").to_string();
    let mut prev_date = *as_on_date - Duration::days(1);
    let prev_date_no_hyphen = prev_date.format("%d%m%Y").to_string();
    let mut prev_day_output_file_path = config_params
        .output_file_path()
        .replace(&as_on_date_no_hyphen, &prev_date_no_hyphen);
    let mut path_exists: bool = Path::new(&prev_day_output_file_path).exists();
    let mut counter = 1;
    while path_exists == false {
        prev_date = *as_on_date - Duration::days(counter);
        if prev_date == prev_month_end_date {
            break;
        }
        let prev_date_no_hyphen_curr = prev_date.format("%d%m%Y").to_string();
        prev_day_output_file_path = config_params
            .output_file_path()
            .replace(&as_on_date_no_hyphen, &prev_date_no_hyphen_curr);
        path_exists = Path::new(&prev_day_output_file_path).exists();
        counter += 1;
    }
    if path_exists == true {
        let prev_date_output_file = match new_buf_rdr(&prev_day_output_file_path) {
            Ok(file) => file,
            Err(error) => panic!(
                "Could not found output file: `{}`  {}.",
                prev_day_output_file_path, error
            ),
        };

        for (line_num, lines) in prev_date_output_file.lines().enumerate() {
            let output_line = match lines {
                Ok(output_line) => output_line,
                Err(error) => panic!(
                    "Unable to read file `{}` at line number: `{}` : {}",
                    prev_day_output_file_path,
                    line_num + 1,
                    error
                ),
            };
            let output_fields = output_line.split('|').collect::<Vec<&str>>();
            let output_data: OutputData = OutputData::new(
                &config_params,
                &prev_day_output_file_path,
                &output_fields,
                line_num + 1,
            );
            output_map.insert(output_data.acc_no.to_string(), output_data);
        }
    }
    let mut op_writer = get_writer(config_params.output_file_path());
    for file in files_config.files {
        let mut exrt_map: HashMap<String, f64> = HashMap::new();
        let exrt_file = match new_buf_rdr(&file.exrt_file_path) {
            Ok(file) => file,
            Err(error) => panic!(
                "Could not found exrt file: `{}`  : {}.",
                file.exrt_file_path, error
            ),
        };
        for (line_num, lines) in exrt_file.lines().enumerate() {
            let line = match lines {
                Ok(line) => line,
                Err(error) => panic!(
                    "Unable to read file `{}` at line number: `{}` : {}",
                    file.exrt_file_path,
                    line_num + 1,
                    error
                ),
            };
            let fields: Vec<&str> = line.split('|').collect();
            let key: String = fields[0].to_string() + &"|".to_string() + &fields[1].to_string();
            let val: f64 = fields[2]
                .to_string()
                .parse::<f64>()
                .expect("could not read exchange rate");
            exrt_map.insert(key, val);
        }

        let curr_input_file_path = file.input_file_path;
        let prev_month_metadata_path = &file.account_metadata_file_path;
        let prev_as_on_date_no_hyphen = prev_month_end_date.format("%d%m%Y").to_string();

        let prev_input_file_path =
            curr_input_file_path.replace(&as_on_date_no_hyphen, &prev_as_on_date_no_hyphen);
        let prev_month_metadata_path =
            prev_month_metadata_path.replace(&as_on_date_no_hyphen, &prev_as_on_date_no_hyphen);

        let mut prev_account_reader =
            reader::Reader::new_at_path(&prev_month_metadata_path, &prev_input_file_path);
        let keys = AccFieldNames::new_from_path(&file.required_fields_path);

        let mut account_reader =
            reader::Reader::new_at_path(&file.account_metadata_file_path, &curr_input_file_path);
        let reader_for_calling_method =
            reader::Reader::new_at_path(&file.account_metadata_file_path, &curr_input_file_path);
        let mut required_fields_file = open_file_read(&file.required_fields_path)
            .expect("Cannot open the required fields file.");

        let mut required_fields_buffer = String::new();
        required_fields_file
            .read_to_string(&mut required_fields_buffer)
            .expect("Cannot read the required fields file.");

        let rf: AccFieldNames = serde_json::from_str(&required_fields_buffer[..])
            .expect("Unable to parse the required fields file.");

        let def = "NA".to_string();
        let mut acct_prev_map: HashMap<String, PrevData> = HashMap::new();
        for mut value in prev_account_reader.iter() {
            let acc_no = get_required_data(&rf.acc_no, &reader_for_calling_method, &mut value);
            let value_date = value
                .get_i64_for_key(&keys.value_dt)
                .expect("Cannot get acc open date for account.");
            let tot_amount = value
                .get_f64_for_key(&keys.tot_amt)
                .expect("Cannot get acc open date for account.");
            let is_cf_passed = !keys.cashflows.is_empty();
            let keys_cashflows = keys.cashflows.clone();
            let tot_amt = if is_cf_passed {
                let cashflows = match value.remove_cfs_for_key(&keys_cashflows) {
                    Ok(value) => value,
                    Err(err) => {
                        log_info!(
                            logger,
                            "Account: {} \n Error while removing cashflow from the pool of cashflows.{:#?}",
                            value
                                .get_string_for_key(&keys.acc_no)
                                .unwrap_or(&"".to_string()),
                            err
                        );
                        Vec::new()
                    }
                };
                let mut cf_amt = 0.0;
                for cf in cashflows {
                    cf_amt += cf.get_principal_amount();
                }
                cf_amt
            } else {
                tot_amount
            };
            acct_prev_map.insert(
                acc_no.to_string(),
                PrevData {
                    value_date,
                    tot_amount: tot_amt,
                    acc_no: acc_no.to_string(),
                },
            );
        }
        for mut account in account_reader.iter() {
            let acc_no = get_required_data(&rf.acc_no, &reader_for_calling_method, &mut account);
            let as_on_date = config_params.as_on_date();
            let def_1: PrevData = Default::default();
            let prev_month_data: &PrevData =
                acct_prev_map.get(&acc_no.to_string()).unwrap_or(&def_1);
            let aval_bal: f64 = account.get_f64_for_key(&keys.tot_amt).unwrap_or(0.0);
            let value_dt = account.get_i64_for_key(&keys.value_dt).unwrap_or(0);
            let prev_aval_bal = prev_month_data.tot_amount;
            let curr_date = date_from_timestamp(value_dt);
            let curr_date_month = curr_date.month();
            let as_on_month = as_on_date.month();
            let curr_date_year = curr_date.year();
            let as_on_year = as_on_date.year();
            let is_cf_passed = !keys.cashflows.is_empty();
            let mut tot_amt = if is_cf_passed {
                let cashflows = match account.remove_cfs_for_key(&keys.cashflows.to_string()) {
                    Ok(value) => value,
                    Err(err) => {
                        log_info!(
                            logger,
                            "Account: {} \n Error while removing cashflow from the pool of cashflows.{:#?}",
                            account
                                .get_string_for_key(&keys.acc_no)
                                .unwrap_or(&"".to_string()),
                            err
                        );
                        Vec::new()
                    }
                };
                let mut cf_amt = 0.0;
                for cf in cashflows {
                    cf_amt += cf.get_principal_amount();
                }
                cf_amt
            } else {
                aval_bal
            };

            if acct_prev_map.contains_key(&acc_no.to_string()) {
                if tot_amt <= prev_aval_bal {
                    continue;
                }
            } else {
                if curr_date_month != as_on_month || curr_date_year < as_on_year {
                    continue;
                }
            }

            let acc_open_dt_read = account.get_i64_for_key(&keys.acct_open_dt).unwrap_or(0);
            let src = file.source.to_string();
            let ccy = get_required_data(&rf.ccy, &reader_for_calling_method, &mut account);
            let prod_code =
                get_required_data(&rf.prod_code, &reader_for_calling_method, &mut account);
            let scheme_id =
                get_required_data(&rf.scheme_id, &reader_for_calling_method, &mut account);
            let mis_1 = get_required_data(&rf.mis_1, &reader_for_calling_method, &mut account);
            let mis_2 = get_required_data(&rf.mis_2, &reader_for_calling_method, &mut account);
            let mis_3 = get_required_data(&rf.mis_3, &reader_for_calling_method, &mut account);
            let raw_bm = get_required_data(&rf.raw_bm, &reader_for_calling_method, &mut account);
            let final_bm =
                get_required_data(&rf.final_bm, &reader_for_calling_method, &mut account);
            let concat = get_required_data(&rf.concat, &reader_for_calling_method, &mut account);
            let npa_flag =
                get_required_data(&rf.npa_flag, &reader_for_calling_method, &mut account);
            let division =
                get_required_data(&rf.division, &reader_for_calling_method, &mut account);
            let alm_line =
                get_required_data(&rf.alm_line, &reader_for_calling_method, &mut account);
            let ia_line = get_required_data(&rf.ia_line, &reader_for_calling_method, &mut account);
            let alco_code_read =
                get_required_data(&rf.alco_map, &reader_for_calling_method, &mut account);
            let psl_code_read =
                get_required_data(&rf.psl_code, &reader_for_calling_method, &mut account);
            let mat_dt = account.get_i64_for_key(&keys.mat_dt).unwrap_or(0);

            let psl_code = psl_map.get(&psl_code_read).unwrap_or(&def);
            let alco_code = alco_map.get(&alco_code_read).unwrap_or(&def);
            let custom1 = get_required_data(&rf.custom1, &reader_for_calling_method, &mut account);
            let custom2 = get_required_data(&rf.custom2, &reader_for_calling_method, &mut account);
            let mut src_yield =
                get_required_data(&rf.inr_rate, &reader_for_calling_method, &mut account);
            let def_2 = "".to_string();
            if src_yield == "NA" {
                src_yield = def_2;
            };
            let mut exrt = 1.0;
            if !file.is_consolidated {
                let key = ccy.to_string() + &"|".to_string() + &config_params.base_ccy();
                exrt = *exrt_map.get(&key).unwrap_or(&1.0);
            }
            tot_amt *= exrt;
            if file.is_negative {
                tot_amt *= -1.0;
            }
            acc_enc += 1;
            let days = ((mat_dt - acc_open_dt_read) / 3600) / 24;
            let mut org_tenor = "NA".to_string();
            for (key, value) in &tenor_map {
                if days >= key.from_days && days <= key.to_days {
                    org_tenor = value.to_string();
                    break;
                }
            }
            let mut rate_bucket = "".to_string();
            let yield_rate = src_yield.parse::<f64>().unwrap_or(0.0);
            if let Some(Ok(reader)) =
                rating_excel.worksheet_range(config_params.rate_bucket_sheet_name())
            {
                for row in reader.rows() {
                    let rate_data = RatingData::new_from_excel(row);
                    if yield_rate > rate_data.start_bound && yield_rate <= rate_data.end_bound {
                        rate_bucket = rate_data.rate_bucket;
                    }
                }
            }
            let output_data: OutputData = OutputData {
                acc_no: acc_no.to_string(),
                as_on_dt: as_on_date.format("%d-%m-%Y").to_string(),
                src: src.to_string(),
                ccy: ccy.to_string(),
                value_dt: date_from_timestamp(value_dt).format("%d-%m-%Y").to_string(),
                prod_code: prod_code.to_string(),
                scheme_id: scheme_id.to_string(),
                mis_1: mis_1.to_string(),
                mis_2: mis_2.to_string(),
                mis_3: mis_3.to_string(),
                raw_bm: raw_bm.to_string(),
                final_bm: final_bm.to_string(),
                concat: concat.to_string(),
                npa_flag: npa_flag.to_string(),
                division: division.to_string(),
                alm_line: alm_line.to_string(),
                ia_line: ia_line.to_string(),
                org_tenor,
                alco_map: alco_code.to_string(),
                psl_code: psl_code.to_string(),
                custom1: custom1.to_string(),
                rate_bucket,
                tot_amt: tot_amt.to_string(),
                yield_rate: yield_rate.to_string(),
            };
            let temp_data: OutputData = output_data.clone();
            output_map
                .entry(acc_no.to_string())
                .and_modify(|prev_output_data| {
                    let prev_amount = &prev_output_data.tot_amt.parse::<f64>().unwrap_or(0.0);
                    if *prev_amount < tot_amt {
                        *prev_output_data = output_data;
                    }
                })
                .or_insert(temp_data);
        }
    }
    for (key, value) in output_map {
        let output_data: OutputData = OutputData {
            acc_no: value.acc_no,
            as_on_dt: value.as_on_dt,
            src: value.src,
            ccy: value.ccy,
            value_dt: value.value_dt,
            prod_code: value.prod_code,
            scheme_id: value.scheme_id,
            mis_1: value.mis_1,
            mis_2: value.mis_2,
            mis_3: value.mis_3,
            raw_bm: value.raw_bm,
            final_bm: value.final_bm,
            concat: value.concat,
            npa_flag: value.npa_flag,
            division: value.division,
            alm_line: value.alm_line,
            ia_line: value.ia_line,
            org_tenor: value.org_tenor,
            alco_map: value.alco_map,
            psl_code: value.psl_code,
            custom1: value.custom1,
            rate_bucket: value.rate_bucket,
            tot_amt: value.tot_amt,
            yield_rate: value.yield_rate,
        };
        writeln!(op_writer, "{}", format_output(output_data)).expect("Error in Writing Output");
    }

    let health_report = HealthReport::new(acc_enc, acc_succ, acc_enc - acc_succ, ip_amt, op_amt, 0);
    health_report.gen_health_rpt(&config_params.output_file_path());
    let total_duration = print_return_time_since!(start_time);
    log_info!(logger, "Total time for aggregation: {:?}", total_duration);
}

pub fn get_required_data(
    field_name: &str,
    reader_for_calling_method: &reader::Reader,
    record: &mut AccountWithCFs,
) -> String {
    let mut req_val = "NA".to_string();
    let def = "NA".to_string();
    if field_name == "" {
        return def;
    }
    match reader_for_calling_method
        .get_field_type(&field_name.to_string())
        .unwrap_or(Type::Cashflows)
    {
        Type::I32 => {
            let val = record.get_i32_for_key(&field_name.to_string()).unwrap_or(0);
            req_val = val.to_string();
        }

        Type::I64 => {
            let val = record.get_i64_for_key(&field_name.to_string()).unwrap_or(0);
            req_val = val.to_string();
        }

        Type::F32 => {
            let val = record
                .get_f32_for_key(&field_name.to_string())
                .unwrap_or(0.0);
            req_val = val.to_string();
        }

        Type::F64 => {
            let val = record
                .get_f64_for_key(&field_name.to_string())
                .unwrap_or(0.0);
            req_val = val.to_string();
        }

        Type::String => {
            let val = record
                .get_string_for_key(&field_name.to_string())
                .unwrap_or(&def);
            req_val = val.to_string();
        }
        Type::Cashflows => {
            req_val = "NA".to_string();
        }
        _ => {
            req_val = "NA".to_string();
        }
    };
    return req_val;
}
