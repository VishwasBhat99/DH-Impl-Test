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
use rbdate::{NaiveDate, NaiveDateTime};
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::Reader;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;

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
    let num_slabs: Vec<RangeSlab> = get_num_slabs(&files_config.numslab_file_path);
    let prd_slabs: Vec<RangeSlab> =
        get_prd_slabs(&files_config.prdslab_file_path, config_params.as_on_date());
    let map_slabs: Vec<MapSlab> = get_map_slabs(&files_config.srcmapslab_file_path);
    // To store aggregated data
    let mut aggr_data: HashMap<AggrKey, Data> = HashMap::new();
    let mut acc_enc = 0;
    let mut acc_succ = 0;
    let mut ip_amt = 0.0;
    let mut op_amt = 0.0;
    for file in files_config.files {
        // Init local currency converter
        let currency_converter = currency::create_currency_converter(
            config_params.home_currency(),
            &file.exchange_rate_file_path,
        );
        // Read cashflow file
        let mut file_rdr: Reader =
            reader::read_file(&file.input_file_path, &file.metadata_file_path);
        let rules = AggRules::new_from_path(&file.acc_skip_rules_path, &file_rdr);
        for account in file_rdr.iter() {
            acc_enc += 1;
            let acc_open_timestamp = account
                .get_i64_for_key(&file.acc_open_date)
                .expect("Cannot read process field use to determine account open date.");
            let acc_open_dt = naivedate_from_timestamp(acc_open_timestamp);
            if acc_open_dt != *config_params.as_on_date() {
                continue;
            }
            let process_field = account
                .get_string_for_key(&file.process_field)
                .expect("Cannot read process field use to determine WD/NWD.");
            let to_skip = skip_account(&account, &rules);
            if !get_acc_validation(process_wd, process_field, &nwd_prod_codes, to_skip) {
                continue;
            };
            let mut amt = account
                .get_f64_for_key(&file.amt)
                .expect("Cannot read amount field.");
            if file.is_negative {
                amt *= -1.0;
            }
            ip_amt += amt;
            let int_rt = account
                .get_f64_for_key(&file.int_rt)
                .expect("Cannot read interest rate field.");
            let ccy = account
                .get_string_for_key(&file.ccy)
                .expect("Cannot read currency field.");
            // Derive Dim1
            let dim1 = get_dim(
                &file.dim1_fields,
                &file.dim1_type,
                &account,
                &num_slabs,
                &prd_slabs,
                &map_slabs,
            );
            // Derive Dim2
            let dim2 = get_dim(
                &file.dim2_fields,
                &file.dim2_type,
                &account,
                &num_slabs,
                &prd_slabs,
                &map_slabs,
            );
            // Derive Dim3
            let dim3 = get_dim(
                &file.dim3_fields,
                &file.dim3_type,
                &account,
                &num_slabs,
                &prd_slabs,
                &map_slabs,
            );
            // Construct AggrKey for account
            let aggr_key = AggrKey {
                dim1: dim1,
                dim2: dim2,
                dim3: dim3,
                ccy: ccy.to_string(),
            };
            let acc_data = Data {
                tot_prin_amt_org: amt,
                tot_prin_amt_lcy: amt,
                tot_prin_amt_hcy: amt,
                rt_prin_amt_weighted: int_rt * amt,
            };

            let conv_data =
                currency_converter.convert(&aggr_key.ccy, &acc_data, file.is_consolidated, logger);

            // Aggregate data
            aggr_data
                .entry(aggr_key)
                .and_modify(|data| data.append_data(conv_data))
                .or_insert(conv_data);

            acc_succ += 1;
        }
    }
    // Write output
    write_aggr_smry(aggr_data, &mut op_amt, &config_params);

    let health_report = HealthReport::new(acc_enc, acc_succ, acc_enc - acc_succ, ip_amt, op_amt, 0);
    health_report.gen_health_rpt(&config_params.output_file_path());
}

pub fn naivedate_from_timestamp(t: i64) -> NaiveDate {
    let naive_date_time = NaiveDateTime::from_timestamp(t, 0);
    naive_date_time.date()
}
