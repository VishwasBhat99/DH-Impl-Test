use self::llg_key::*;
use aggregator::account_field_names::AccFieldNames;
use aggregator::tenor::*;
use calamine::{open_workbook, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use sdb_dyn_proto_rdr::reader::types::Type;
use sdb_io::{buf_file_wrtr, new_buf_rdr, open_file_read};
use slog::Logger;
use std::collections::HashMap;
use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Read;
use std::io::Write;
use std::time::SystemTime;

mod account_field_names;
pub mod config;
mod llg_key;
mod tenor;
mod util;

pub fn aggregate(config_params: ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let start_time = SystemTime::now();
    let mut output_file = match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&config_params.output_file_path())
    {
        Ok(create) => create,
        Err(error) => {
            panic!(
                "Could not create file: `{}` on location `{}` : {:?}.",
                config_params.output_file_path(),
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };

    let mut tenor_excel: Xlsx<_> = open_workbook(config_params.org_tenor_file_path())
        .expect("Unable to open original tenor file.");
    let mut tenor_map: HashMap<Tenor, String> = HashMap::new();
    if let Some(Ok(reader)) = tenor_excel.worksheet_range(config_params.org_tenor_sheet_name()) {
        for row in reader.rows() {
            let key: Tenor = Tenor::new(row[0].to_string(), row[1].to_string());
            let val: String = row[2].to_string();
            tenor_map.insert(key, val);
        }
    }

    let mut alco_excel: Xlsx<_> = open_workbook(config_params.alco_map_file_path())
        .expect("Unable to open original tenor file.");
    let mut alco_map: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = alco_excel.worksheet_range(config_params.alco_map_sheet_name()) {
        for row in reader.rows() {
            alco_map.insert(row[0].to_string(), row[1].to_string());
        }
    }

    let mut psl_excel: Xlsx<_> = open_workbook(config_params.psl_map_file_path())
        .expect("Unable to open original tenor file.");
    let mut psl_map: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = psl_excel.worksheet_range(config_params.psl_map_sheet_name()) {
        for row in reader.rows() {
            psl_map.insert(row[0].to_string(), row[1].to_string());
        }
    }

    let mut acc_skip_map: HashMap<String, String> = HashMap::new();
    let mut acc_lookup_nos: HashMap<String, String> = HashMap::new();
    let acc_skip_file = match new_buf_rdr(&config_params.incr_acc_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not find account skip file: `{}`  : {}.",
            config_params.incr_acc_file_path(),
            error
        ),
    };
    for (line_num, lines) in acc_skip_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.incr_acc_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split("~#~").collect();
        if fields.len() != 2 {
            continue;
        }
        acc_skip_map.insert(fields[0].to_string(), fields[1].to_string());
        let (acc_no, _) = fields[0].rsplit_once("|").unwrap();
        acc_lookup_nos.insert(acc_no.to_string(), fields[1].to_string());
    }
    let mut acc_enc = 0;
    let mut acc_succ = 0;
    let mut ip_amt = 0.0;
    let mut op_amt = 0.0;
    let mut aggr_data: HashMap<LLGKey, LLGVal> = HashMap::new();

    let files_config = config::get_files(config_params.config_file_path());
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
        let keys = AccFieldNames::new_from_path(&file.required_fields_file_path);
        let mut account_reader =
            reader::Reader::new_at_path(&file.metadata_file_path, &file.input_file_path);
        let reader_for_calling_method =
            reader::Reader::new_at_path(&file.metadata_file_path, &file.input_file_path);
        let mut required_fields_file = open_file_read(&file.required_fields_file_path)
            .expect("Cannot open the required fields file.");

        let mut required_fields_buffer = String::new();
        required_fields_file
            .read_to_string(&mut required_fields_buffer)
            .expect("Cannot read the required fields file.");

        let rf: AccFieldNames = serde_json::from_str(&required_fields_buffer[..])
            .expect("Unable to parse the required fields file.");

        let def = "NA".to_string();
        let rules = AggRules::new_from_path(&file.acc_skip_rules_path, &account_reader);
        for mut account in account_reader.iter() {
            let acc_no = get_required_data(&rf.acc_no, &reader_for_calling_method, &mut account);
            let acc_open_dt_read = account.get_i64_for_key(&keys.acc_open_dt).unwrap_or(0);
            let acc_open_dt = naivedate_from_timestamp(acc_open_dt_read).to_string();
            let as_on_date_str = config_params.as_on_date().to_string();
            let acc_open_vec: Vec<&str> = acc_open_dt.split('-').collect();
            let ason_vec: Vec<&str> = as_on_date_str.split('-').collect();
            let acc_skip_line = acc_no.to_string()
                + &"|".to_string()
                + &file.source.to_string()
                + &"~#~".to_string()
                + &config_params.as_on_date().to_string()
                + &"|".to_string()
                + &acc_open_dt.to_string();
            let acc_skip_lookup = acc_no.to_string();
            if file.is_exclusion_rules_required && skip_account(&account, &rules) {
                continue;
            }
            acc_enc += 1;
            if acc_lookup_nos.contains_key(&acc_skip_lookup)
                || acc_open_vec[1].to_string() + &acc_open_vec[0].to_string()
                    != ason_vec[1].to_string() + &ason_vec[0].to_string()
            {
                continue;
            }
            acc_succ += 1;
            let skip: Vec<&str> = acc_skip_line.split("~#~").collect();
            acc_skip_map.insert(skip[0].to_string(), skip[1].to_string());
            acc_lookup_nos.insert(acc_no.to_string(), skip[1].to_string());
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
            let is_cf_passed = !keys.cashflows.is_empty();
            let mut tot_amt = if is_cf_passed {
                let cashflows = match account.remove_cfs_for_key(&keys.cashflows) {
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
                account.get_f64_for_key(&keys.tot_amt).unwrap_or(0.0)
            };
            let psl_code = psl_map.get(&psl_code_read).unwrap_or(&def);
            let alco_code = alco_map.get(&alco_code_read).unwrap_or(&def);
            let custom1 = get_required_data(&rf.custom1, &reader_for_calling_method, &mut account);
            let custom2 = get_required_data(&rf.custom2, &reader_for_calling_method, &mut account);
            let mut src_yield =
                get_required_data(&rf.inr_rate, &reader_for_calling_method, &mut account);
            if src_yield == "NA" {
                src_yield = "0.0".to_string()
            };
            let mut exrt = 1.0;
            if !file.is_consolidated {
                let key = ccy.to_string() + &"|".to_string() + &config_params.consol_ccy();
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
            let llg_key = LLGKey::new(
                acc_no.to_string(),
                config_params.as_on_date().to_string(),
                file.source.to_string(),
                acc_open_dt.to_string(),
                ccy.to_string(),
                prod_code.to_string(),
                scheme_id.to_string(),
                mis_1.to_string(),
                mis_2.to_string(),
                mis_3.to_string(),
                raw_bm.to_string(),
                final_bm.to_string(),
                concat.to_string(),
                npa_flag.to_string(),
                division.to_string(),
                alm_line.to_string(),
                ia_line.to_string(),
                org_tenor.to_string(),
                alco_code.to_string(),
                psl_code.to_string(),
                custom1.to_string(),
                custom2.to_string(),
            );
            let llg_val = LLGVal {
                tot_amt: tot_amt.to_string(),
                src_yield: src_yield.to_string(),
            };
            aggr_data.insert(llg_key, llg_val);
        }
    }

    let mut acc_skip_writer = match buf_file_wrtr(config_params.incr_acc_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create acc skip file `{}` : {}",
            config_params.incr_acc_file_path(),
            error
        ),
    };

    let mut acc_skip_op_line = String::new();
    for (key, val) in acc_skip_map {
        let op = format!("{}~#~{}", key, val);
        acc_skip_op_line.push_str(&op[..]);
        acc_skip_op_line.push_str("\n");
    }
    match acc_skip_writer.write_all(acc_skip_op_line.as_bytes()) {
        Ok(_) => println!("Successfully written acc skip file."),
        Err(error) => panic!(
            "Unable to write reconcilation lines on file `{}`: {}.",
            config_params.incr_acc_file_path(),
            error,
        ),
    };
    for (key, data) in aggr_data.drain() {
        write!(output_file, "{}|", key).expect("Unable to write key to summary file.");
        let mut data_op = String::new();

        data_op.push_str(&data.tot_amt.to_string());
        data_op.push_str("|");
        data_op.push_str(&data.src_yield.to_string());
        data_op.push_str("\n");
        write!(output_file, "{}", data_op).expect("Unable to write data to summary file.");
    }
    let health_report = HealthReport::new(acc_enc, acc_succ, acc_enc - acc_succ, ip_amt, op_amt, 0);
    health_report.gen_health_rpt(&config_params.output_file_path());
    let total_duration = print_return_time_since!(start_time);
    log_info!(logger, "Total time for aggregation: {:?}", total_duration);
}

pub fn naivedate_from_timestamp(t: i64) -> rbdate::NaiveDate {
    let naive_date_time = rbdate::NaiveDateTime::from_timestamp(t, 0);
    naive_date_time.date()
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

pub fn skip_account(account: &AccountWithCFs, rules: &AggRules) -> bool {
    let skip_field = match rules.llg_for_acc(account) {
        Some(_) => true,
        None => false,
    };
    skip_field
}
