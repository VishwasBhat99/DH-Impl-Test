mod acc_validation;
pub mod accured;
pub mod aggr_data;
pub mod aggr_key;
pub mod config;
mod currency;
mod dimensions;
pub mod reader;
mod writer;
use self::acc_validation::{get_acc_validation, skip_account};
use self::accured::add_int_accured;
use self::aggr_data::Data;
use self::aggr_key::AggrKey;
use self::dimensions::{get_dim, get_map_slabs, get_num_slabs, get_prd_slabs};
use self::dimensions::{MapSlab, RangeSlab};
use self::writer::write_aggr_smry;
use calamine::{open_workbook_auto, Reader as ex_reader, Sheets};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use rbdate::date_from_timestamp;
use rbdate::num_days_start_to_end;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::account_with_cfs::get_field_value;
use sdb_dyn_proto_rdr::reader::Reader;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::ffi::OsStr;
use std::io::prelude::*;
use std::path::Path;

pub fn aggregate(config_params: ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    // Read Files Configuration
    let files_config = config::get_files(config_params.config_file_path());
    let process_wd = if files_config.process_type == "WD" {
        true
    } else {
        false
    };
    // init NWD product code file
    let nwd_file = match new_buf_rdr(&files_config.nwd_codes_file_path) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            files_config.nwd_codes_file_path,
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut nwd_prod_codes: Vec<String> = Vec::new();
    for (line_num, lines) in nwd_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.output_file_path(),
                line_num + 1,
                error
            ),
        };
        nwd_prod_codes.push(line.to_string());
    }

    // Read slabs config from file
    let mut map_slabs1: Vec<MapSlab> = Vec::new();
    let mut map_slabs2: Vec<MapSlab> = Vec::new();
    let mut map_slabs3: Vec<MapSlab> = Vec::new();
    if &files_config.files[0].dim1_type == "SRCMAP" {
        map_slabs1 = get_map_slabs(&files_config.dim1_file_path);
    }
    if &files_config.files[0].dim2_type == "SRCMAP" {
        map_slabs2 = get_map_slabs(&files_config.dim2_file_path);
    }
    if &files_config.files[0].dim3_type == "SRCMAP" {
        map_slabs3 = get_map_slabs(&files_config.dim3_file_path);
    }
    let mut dim1_slabs: Vec<RangeSlab> = Vec::new();
    let mut dim2_slabs: Vec<RangeSlab> = Vec::new();
    let mut dim3_slabs: Vec<RangeSlab> = Vec::new();
    if &files_config.files[0].dim1_type == "NUMSLAB" {
        dim1_slabs = get_num_slabs(&files_config.dim1_file_path)
    }
    if &files_config.files[0].dim1_type == "PRDSLAB" {
        dim1_slabs = get_prd_slabs(&files_config.dim1_file_path, config_params.as_on_date())
    }
    if &files_config.files[0].dim2_type == "NUMSLAB" {
        dim2_slabs = get_num_slabs(&files_config.dim2_file_path)
    }
    if &files_config.files[0].dim2_type == "PRDSLAB" {
        dim2_slabs = get_prd_slabs(&files_config.dim2_file_path, config_params.as_on_date())
    };
    if &files_config.files[0].dim3_type == "NUMSLAB" {
        dim3_slabs = get_num_slabs(&files_config.dim3_file_path)
    }
    if &files_config.files[0].dim3_type == "PRDSLAB" {
        dim3_slabs = get_prd_slabs(&files_config.dim3_file_path, config_params.as_on_date())
    };
    let numslab_file_extension = Path::new(&files_config.files[0].numslab_file_path)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("txt");
    let mut numslab_data: Vec<Vec<String>> = Vec::new();
    if numslab_file_extension == "xlsx"
        || numslab_file_extension == "xls"
        || numslab_file_extension == "xlsb"
    {
        let mut numslab_excel = open_workbook_auto(&files_config.files[0].numslab_file_path)
            .expect(&format!(
                "Unable to open Tcfsl File. on path {}",
                &files_config.files[0].numslab_file_path
            ));
        check_sheet_name(
            files_config.files[0].numslab_file_path.to_string(),
            &files_config.files[0].numslab_sheet_name,
            &numslab_excel,
        );
        if let Some(Ok(reader)) =
            numslab_excel.worksheet_range(&files_config.files[0].numslab_sheet_name)
        {
            for row in reader.rows().skip(1) {
                let mut numslab_vec: Vec<String> = Vec::new();
                for data in row {
                    numslab_vec.push(data.to_string().trim().to_string());
                }
                numslab_data.push(numslab_vec);
            }
        }
    } else {
        let numslab_file = match new_buf_rdr(&files_config.files[0].numslab_file_path) {
            Ok(file) => file,
            Err(error) => panic!(
                "Could not found numslab_file: `{}` :{}",
                &files_config.files[0].numslab_file_path, error,
            ),
        };

        for (line_num, lines) in numslab_file.lines().enumerate().skip(1) {
            let numslab_line = match lines {
                Ok(numslab_line) => numslab_line,
                Err(error) => panic!(
                    "Unable to read file `{}` at line number: `{}` : {}",
                    &files_config.files[0].numslab_file_path,
                    line_num + 1,
                    error
                ),
            };
            let numslab_fields: Vec<String> = numslab_line
                .split('|')
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>();
            numslab_data.push(numslab_fields);
        }
    }

    // To store aggregated data
    let mut aggr_data: HashMap<AggrKey, Data> = HashMap::new();
    let mut ex_rt: f64 = 1.0;
    let mut acc_enc = 0;
    let mut acc_succ = 0;
    let mut ip_amt = 0.0;
    let mut op_amt = 0.0;
    for file in files_config.files {
        // Init currency converter
        let currency_converter = currency::create_currency_converter(
            config_params.home_currency(),
            &file.exchange_rate_file_path,
        );
        // Read cashflow file
        let mut file_rdr: Reader =
            reader::read_file(&file.input_file_path, &file.metadata_file_path);
        let method_reader: Reader =
            reader::read_file(&file.input_file_path, &file.metadata_file_path);
        let rules = AggRules::new_from_path(&file.acc_skip_rules_path, &file_rdr);
        let display_ccy = &config_params.display_currency().to_string();
        for account in file_rdr.iter() {
            acc_enc += 1;
            let process_field =
                get_field_value(&account, &method_reader, file.process_field.to_owned())
                    .expect("Cannot read process field use to determine WD/NWD.");
            let to_skip = skip_account(&account, &rules);
            if !get_acc_validation(process_wd, &process_field, &nwd_prod_codes, to_skip) {
                continue;
            };
            let mut ccy = &get_field_value(&account, &method_reader, file.ccy.to_owned())
                .expect("Cannot read currency field.");

            let mut amt = get_field_value(&account, &method_reader, file.amt.to_owned())
                .expect("Cannot read amount field.")
                .parse()
                .unwrap_or(0.0);

            let account_open_date = date_from_timestamp(
                get_field_value(&account, &method_reader, file.account_open_date.to_owned())
                    .expect("Cannot read account open date.")
                    .parse()
                    .unwrap_or(0),
            );

            let mat_date = date_from_timestamp(
                get_field_value(&account, &method_reader, file.mat_date.to_owned())
                    .expect("Cannot read maturity_date.")
                    .parse()
                    .unwrap_or(0),
            );

            if file.is_negative {
                amt *= -1.0;
            }
            ip_amt += amt;
            let conv_amt = currency_converter.convert(
                ccy,
                amt,
                file.is_consolidated,
                file.is_account_level_exchange_rate,
                ex_rt,
                logger,
            );
            let acc_data;
            let int_rt = account
                .get_f64_for_key(&file.int_rt)
                .expect("Cannot read interest rate field.");

            if file.is_account_level_exchange_rate {
                ex_rt = account
                    .get_f64_for_key(&file.exchange_rate)
                    .expect("Cannot get exchange rate from account.");
                if ex_rt == 0.0 {
                    continue;
                }
            }
            // Derive Dim1
            let dim1 = get_dim(
                &file.dim1_fields,
                &file.dim1_type,
                &account,
                &method_reader,
                &dim1_slabs,
                &map_slabs1,
            );
            // Derive Dim2
            let dim2 = get_dim(
                &file.dim2_fields,
                &file.dim2_type,
                &account,
                &method_reader,
                &dim2_slabs,
                &map_slabs2,
            );
            // Derive Dim3
            let dim3 = get_dim(
                &file.dim3_fields,
                &file.dim3_type,
                &account,
                &method_reader,
                &dim3_slabs,
                &map_slabs3,
            );
            if display_ccy != "NA"
                && ccy.trim().to_lowercase() == config_params.home_currency().trim().to_lowercase()
            {
                ccy = &display_ccy;
            }
            // Construct AggrKey for account
            let aggr_key = AggrKey {
                dim1: dim1.to_owned(),
                dim2: dim2.to_owned(),
                dim3: dim3.to_owned(),
                ccy: ccy.to_string(),
            };
            let consol_aggr_key = AggrKey {
                dim1,
                dim2,
                dim3,
                ccy: config_params.consol_currency().to_string(),
            };
            let account_cont_days = num_days_start_to_end(account_open_date, mat_date);
            let account_residual_days = if config_params.as_on_date() < &mat_date {
                num_days_start_to_end(*config_params.as_on_date(), mat_date)
            } else {
                num_days_start_to_end(mat_date, *config_params.as_on_date()) * (-1)
            };

            if file.is_consolidated {
                acc_data = Data {
                    tot_prin_amt: conv_amt,
                    tot_prin_amt_lcy: amt,
                    rt_prin_amt_weighted: int_rt * amt,
                    min_amount_ccy: amt,
                    min_amount_hcy: conv_amt,
                    max_amount_ccy: amt,
                    max_amount_hcy: conv_amt,
                    int_accured: 0.0,
                    min_int_rate: int_rt,
                    max_int_rate: int_rt,
                    avg_days_contract_mat_sum: (account_cont_days as f64 * amt),
                    avg_days_residual_mat_sum: (account_residual_days as f64 * amt),
                    no_of_depositers: 1,
                    total_bal: amt,
                };
            } else {
                acc_data = Data {
                    tot_prin_amt: amt,
                    tot_prin_amt_lcy: conv_amt,
                    rt_prin_amt_weighted: int_rt * conv_amt,
                    min_amount_ccy: conv_amt,
                    min_amount_hcy: amt,
                    max_amount_ccy: amt,
                    max_amount_hcy: conv_amt,
                    int_accured: 0.0,
                    min_int_rate: int_rt,
                    max_int_rate: int_rt,
                    avg_days_contract_mat_sum: (account_cont_days as f64 * amt),
                    avg_days_residual_mat_sum: (account_residual_days as f64 * amt),
                    no_of_depositers: 1,
                    total_bal: amt,
                };
            }
            // Aggregate data

            aggr_data
                .entry(aggr_key)
                .and_modify(|data| data.append_data(acc_data))
                .or_insert(acc_data);
            aggr_data
                .entry(consol_aggr_key)
                .and_modify(|data| data.append_data(acc_data))
                .or_insert(acc_data);
            acc_succ += 1;
        }
    }
    add_int_accured(&mut aggr_data, numslab_data, logger);
    // Write output
    write_aggr_smry(aggr_data, &mut op_amt, &config_params);

    let health_report = HealthReport::new(acc_enc, acc_succ, acc_enc - acc_succ, ip_amt, op_amt, 0);
    health_report.gen_health_rpt(&config_params.output_file_path());
}
fn check_sheet_name(file_name: String, sheet_name: &String, excel_sheets: &Sheets) {
    if !excel_sheets.sheet_names().contains(&sheet_name.to_string()) {
        panic!(
            "sheet name {} is not present in {} : Available sheet names :{:?}",
            sheet_name,
            file_name,
            excel_sheets.sheet_names()
        )
    }
}
