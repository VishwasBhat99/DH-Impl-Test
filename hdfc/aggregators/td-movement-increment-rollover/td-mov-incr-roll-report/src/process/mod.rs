use calamine::{open_workbook_auto, Reader};
use chrono::Local;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use process::account::*;
use process::account_field_names::AccFieldNames;
use process::bucket::*;
use process::convert::*;
use process::io::*;
use process::llg_key::LLGKey;
use process::tenor::*;
use rbdate::date_from_timestamp;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::account_with_cfs::get_field_value;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::Write;

mod account;
mod account_field_names;
mod bucket;
mod config;
mod convert;
mod grp_key;
mod io;
mod llg_key;
mod tenor;

pub fn process(config_params: ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let mut acc_enc = 0;
    let mut acc_succ = 0;
    let mut tot_amt = 0.0;
    let tot_cfs: usize = 0;

    //Output file.
    let mut output_report_path = String::new();
    output_report_path.push_str(config_params.output_file_path());
    let mut output_writer = match buf_file_wrtr(&output_report_path, None) {
        Ok(wrtr) => wrtr,
        Err(error) => {
            panic!(
                "Could not create file: `{}` at location `{}` : {:?}.",
                config_params.output_file_path(),
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };

    let mut alco_map: HashMap<String, String> = HashMap::new();
    let mut alco_master_excel =
        open_workbook_auto(config_params.alco_master()).expect("Unable to open ALCO Master File.");
    if let Some(Ok(reader)) = alco_master_excel.worksheet_range(config_params.alco_sheet()) {
        for row in reader.rows().skip(1) {
            alco_map.insert(row[0].to_string(), row[1].to_string());
        }
    }
    let mut org_tenor_map: HashMap<Tenor, String> = HashMap::new();
    let mut res_tenor_map: HashMap<Tenor, String> = HashMap::new();
    let mut ia_tenor_map: HashMap<Tenor, String> = HashMap::new();
    let mut tenor_master_excel = open_workbook_auto(config_params.tenor_master())
        .expect("Unable to open Tenor Master File.");
    if let Some(Ok(reader)) = tenor_master_excel.worksheet_range(config_params.tenor_sheet()) {
        for row in reader.rows().skip(1) {
            let ia_tenor = Tenor::new(row[0].to_string(), row[1].to_string());
            let org_tenor = Tenor::new(row[0].to_string(), row[1].to_string());
            let res_tenor = Tenor::new(row[0].to_string(), row[1].to_string());
            ia_tenor_map.insert(ia_tenor, row[2].to_string());
            org_tenor_map.insert(org_tenor, row[3].to_string());
            res_tenor_map.insert(res_tenor, row[4].to_string());
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
    }
    let mut cat_map: HashMap<String, String> = HashMap::new();
    let mut cat_master_excel = open_workbook_auto(config_params.cat_master())
        .expect("Unable to open Category Master File.");
    if let Some(Ok(reader)) = cat_master_excel.worksheet_range(config_params.cat_sheet()) {
        for row in reader.rows().skip(1) {
            cat_map.insert(row[0].to_string(), row[1].to_string());
        }
    }

    let mut lcr_map: HashMap<String, String> = HashMap::new();
    let lcr_reader =
        fs::read_to_string(&config_params.lcr_master()).expect("Failed to read LCR file!");
    for line in lcr_reader.lines() {
        let lcr_fields = line.split('|').collect::<Vec<&str>>();
        lcr_map.insert(lcr_fields[1].to_string(), lcr_fields[2].to_string());
    }

    let mut wd_nwd_map: HashMap<String, String> = HashMap::new();
    let mut wd_nwd_master_excel = open_workbook_auto(config_params.wd_nwd_master())
        .expect("Unable to open WD/NWD Master File.");
    if let Some(Ok(reader)) = wd_nwd_master_excel.worksheet_range(config_params.wd_nwd_sheet()) {
        for row in reader.rows().skip(1) {
            wd_nwd_map.insert(row[0].to_string(), row[1].to_string());
        }
    }

    // Read the incremental accounts skip file
    pub struct IncrValue {
        val_date: String,
        asondate: String,
    }
    let mut acc_skip_map: HashMap<String, IncrValue> = HashMap::new();
    let acc_skip_file = match new_buf_rdr(&config_params.incremental_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not find incremental account skip file: `{}`  : {}.",
            config_params.incremental_file_path(),
            error
        ),
    };

    for (line_num, lines) in acc_skip_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.incremental_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split("|").collect();
        // acc_no and val_dt combined together is the key to lookup. Asondate is used to differentiate present date accounts.
        let key = fields[0].to_string() + "|" + fields[3];
        acc_skip_map.insert(
            key,
            IncrValue {
                val_date: fields[3].to_string(),
                asondate: fields[2].to_string(),
            },
        );
    }

    let mut inc_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(config_params.incremental_file_path().clone())
        .unwrap();
    let files_config = config::get_files(config_params.config_file_path());
    let ason = config_params.as_on_date().format("%d-%m-%Y").to_string();
    for file in files_config.input_files {
        let mut exrt_map: HashMap<String, f64> = HashMap::new();
        let exrt_file_path = get_file_path(file.exrt_file_path, *config_params.as_on_date());
        let exrt_file = read_file(&exrt_file_path);
        let mut matured_acc_map: HashMap<String, String> = HashMap::new();
        let matured_reader = fs::read_to_string(file.matured_accounts_file)
            .expect("Failed to read Matured Accounts File!");
        for line in matured_reader.lines() {
            let matured_fields = line.split('|').collect::<Vec<&str>>();
            matured_acc_map.insert(matured_fields[0].to_string(), "".to_string());
        }

        for (line_num, lines) in exrt_file.lines().enumerate() {
            let line = extract_lines(line_num, lines, &exrt_file_path);
            let fields: Vec<&str> = line.split('|').collect();
            let key: String = fields[0].to_string() + &"|".to_string() + &fields[1].to_string();
            let val: f64 = fields[2]
                .to_string()
                .parse::<f64>()
                .expect("could not read exchange rate");
            exrt_map.insert(key, val);
        }
        let keys = AccFieldNames::new_from_path(&file.req_fields_file_path);
        let mut account_reader =
            reader::Reader::new_at_path(&file.metadata_file_path, &file.input_file);
        let mut input_reader =
            reader::Reader::new_at_path(&file.metadata_file_path, &file.input_file);
        let mut exrt = 1.0;

        for account in account_reader.iter() {
            acc_enc += 1;
            let ccy = match get_field_value(&account, &input_reader, keys.ccy.to_string()) {
                Ok(val) => val,
                Err(_err) => panic!("{}", _err),
            };
            if !file.is_consolidated {
                let key = ccy.to_string() + &"|".to_string() + config_params.consol_ccy();
                exrt = *exrt_map.get(&key).unwrap_or(&1.0);
            }
            let mut val_dt = match get_field_value(&account, &input_reader, keys.val_dt.to_string())
            {
                Ok(val) => to_i64(val),
                Err(_err) => panic!("{}", _err),
            };
            let acc_no = match get_field_value(&account, &input_reader, keys.acc_no.to_string()) {
                Ok(val) => val,
                Err(_err) => panic!("{}", _err),
            };

            let value_dt = date_from_timestamp(val_dt);
            if value_dt.format("%d-%m-%Y").to_string()[3..] == ason[3..] {
                acc_succ += 1;

                let acc_data = grp_key::fetch_acc_data(
                    &mut alco_map,
                    &mut org_tenor_map,
                    &mut res_tenor_map,
                    &mut ia_tenor_map,
                    &mut bucket_map,
                    &mut cat_map,
                    &mut lcr_map,
                    &mut wd_nwd_map,
                    &mut matured_acc_map,
                    &exrt,
                    &file.source,
                    &account,
                    &mut input_reader,
                    &keys,
                    &config_params,
                    &acc_no,
                    &mut val_dt,
                );

                let acc_skip_lookup = acc_no.to_string() + "|" + &value_dt.to_string();
                if acc_skip_map.contains_key(&acc_skip_lookup) {
                    let skip_data = acc_skip_map
                        .get(&acc_skip_lookup)
                        .expect(("Cannot get data for account from skipp accounts map."));
                    if skip_data.asondate == config_params.as_on_date().to_string() {
                        write!(
                            output_writer,
                            "{}|{}|{}",
                            acc_no, acc_data.grp_key, acc_data.data
                        )
                        .expect("Unable to write1 summary file.");
                    }
                    continue;
                }
                acc_skip_map.insert(
                    acc_skip_lookup,
                    IncrValue {
                        val_date: value_dt.to_string(),
                        asondate: config_params.as_on_date().to_string(),
                    },
                );
                let incremental_op_line = acc_no.to_string()
                    + "|"
                    + &file.source
                    + "|"
                    + &config_params.as_on_date().to_string()
                    + "|"
                    + &value_dt.to_string()
                    + "\n";
                inc_file.write_all(incremental_op_line.as_bytes()).unwrap();
                tot_amt += acc_data.data.amt_initl_dep;
                write!(
                    output_writer,
                    "{}|{}|{}",
                    acc_no, acc_data.grp_key, acc_data.data
                )
                .expect("Unable to write summary file.");
            }
        }
    }
    let timestamp = Local::now().naive_local().to_string();
    let as_on_date = config_params.as_on_date();
    let footer_summary = format!(
        "FTR|{}|{}|{}\n",
        as_on_date.format("%d-%m-%Y"),
        timestamp,
        acc_succ
    );
    write!(output_writer, "{}", footer_summary).expect("Unable to write footer to summary file.");
    println!("Total accounts encountered: {}", acc_enc);
    println!("Total accounts successful: {}", acc_succ);
    let health_report = HealthReport::new(
        acc_enc,
        acc_succ,
        acc_enc - acc_succ,
        tot_amt,
        tot_amt,
        tot_cfs as i64,
    );
    health_report.gen_health_rpt(&output_report_path);
}
