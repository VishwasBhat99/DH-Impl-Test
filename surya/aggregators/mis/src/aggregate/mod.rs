mod acc_validation;
pub mod aggr_data;
pub mod aggr_key;
pub mod config;
mod currency;
mod dimensions;
pub mod reader;
mod writer;

use self::acc_validation::{get_acc_validation, skip_account};
use self::aggr_data::Data;
use self::aggr_key::AggrKey;
use self::dimensions::{get_dim, get_map_slabs, get_num_slabs, get_prd_slabs};
use self::dimensions::{MapSlab, RangeSlab};
use self::writer::write_aggr_smry;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use rbdate::{timestamp_to_naivedate, DateParser};
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::Reader;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn aggregate(config_params: ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    //date parser
    let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
    // Read Files Configuration
    let files_config = config::get_files(config_params.config_file_path());
    let process_wd = if files_config.process_type == "WD" {
        true
    } else {
        false
    };

    //period slab reader
    let period_file = match File::open(files_config.prdslab_file_path) {
        Ok(input_file) => input_file,
        Err(error) => panic!("{}", error),
    };
    let period_slab_reader = BufReader::new(period_file);

    let mut period_slab_vec: Vec<Vec<String>> = Vec::new();

    for line in period_slab_reader.lines() {
        let line = line.expect("Could not read period slab line!");
        let info: Vec<String> = line.split('|').map(|s| s.to_string()).collect();
        period_slab_vec.push(info);
    }

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
    let num_slabs: Vec<RangeSlab> = get_num_slabs(&files_config.numslab_file_path);
    let mut prd_slabs: Vec<RangeSlab> = Vec::new();
    if !config_params.is_act_open_dt_flag() {
        prd_slabs = get_prd_slabs(&period_slab_vec, config_params.as_on_date());
    }

    let map_slabs: Vec<MapSlab> = get_map_slabs(&files_config.srcmapslab_file_path);
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
        let is_numslab_consol_req = if file.is_numslab_consol_required.is_some() {
            if file.is_numslab_consol_required.unwrap() == true {
                true
            } else {
                false
            }
        } else {
            false
        };
        let mut acct_open_date_field = String::new();
        if config_params.is_act_open_dt_flag() {
            acct_open_date_field = file
                .acc_open_dt_field
                .expect("Account open date field is not passed in config files");
        }
        // Read cashflow file
        let mut file_rdr: Reader =
            reader::read_file(&file.input_file_path, &file.metadata_file_path);
        let method_reader: Reader =
            reader::read_file(&file.input_file_path, &file.metadata_file_path);
        let rules = AggRules::new_from_path(&file.acc_skip_rules_path, &file_rdr);
        let display_ccy = &config_params.display_currency().to_string();
        for account in file_rdr.iter() {
            acc_enc += 1;
            if config_params.is_act_open_dt_flag() {
                prd_slabs.clear();
                let acct_op_dt = account
                    .get_i64_for_key(&acct_open_date_field)
                    .expect("Cannot read account open date field");
                prd_slabs = get_prd_slabs(&period_slab_vec, &timestamp_to_naivedate(acct_op_dt));
            }

            let process_field = account
                .get_string_for_key(&file.process_field)
                .expect("Cannot read process field use to determine WD/NWD.");
            let to_skip = skip_account(&account, &rules);
            if !get_acc_validation(process_wd, process_field, &nwd_prod_codes, to_skip) {
                continue;
            };
            let mut ccy = account
                .get_string_for_key(&file.ccy)
                .expect("Cannot read currency field.");
            let mut amt = account
                .get_f64_for_key(&file.amt)
                .expect("Cannot read amount field.");
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
                &num_slabs,
                &prd_slabs,
                &map_slabs,
                ccy,
                currency_converter.clone(),
                is_numslab_consol_req,
                file.is_consolidated,
                file.is_account_level_exchange_rate,
                ex_rt,
                logger,
            );
            // Derive Dim2
            let dim2 = get_dim(
                &file.dim2_fields,
                &file.dim2_type,
                &account,
                &method_reader,
                &num_slabs,
                &prd_slabs,
                &map_slabs,
                ccy,
                currency_converter.clone(),
                is_numslab_consol_req,
                file.is_consolidated,
                file.is_account_level_exchange_rate,
                ex_rt,
                logger,
            );
            // Derive Dim3
            let dim3 = get_dim(
                &file.dim3_fields,
                &file.dim3_type,
                &account,
                &method_reader,
                &num_slabs,
                &prd_slabs,
                &map_slabs,
                ccy,
                currency_converter.clone(),
                is_numslab_consol_req,
                file.is_consolidated,
                file.is_account_level_exchange_rate,
                ex_rt,
                logger,
            );
            if display_ccy != "NA"
                && ccy.trim().to_lowercase() == config_params.home_currency().trim().to_lowercase()
            {
                ccy = display_ccy;
            }
            // Construct AggrKey for account
            let aggr_key = AggrKey {
                dim1,
                dim2,
                dim3,
                ccy: ccy.to_string(),
            };
            if file.is_consolidated {
                acc_data = Data {
                    tot_prin_amt: conv_amt,
                    tot_prin_amt_lcy: amt,
                    rt_prin_amt_weighted: int_rt * amt,
                };
            } else {
                acc_data = Data {
                    tot_prin_amt: amt,
                    tot_prin_amt_lcy: conv_amt,
                    rt_prin_amt_weighted: int_rt * conv_amt,
                };
            }
            // Aggregate data
            aggr_data
                .entry(aggr_key)
                .and_modify(|data| data.append_data(acc_data))
                .or_insert(acc_data);

            acc_succ += 1;
        }
    }
    // Write output
    write_aggr_smry(aggr_data, &mut op_amt, &config_params);

    let health_report = HealthReport::new(acc_enc, acc_succ, acc_enc - acc_succ, ip_amt, op_amt, 0);
    health_report.gen_health_rpt(&config_params.output_file_path());
}
