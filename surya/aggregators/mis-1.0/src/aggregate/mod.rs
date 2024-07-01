pub mod aggr_data;
pub mod aggr_key;
pub mod config;
mod currency;
mod dimensions;
pub mod reader;
mod writer;

use self::aggr_data::Data;
use self::aggr_key::AggrKey;
use self::dimensions::{get_dim, get_map_slabs, get_num_slabs, get_prd_slabs};
use self::dimensions::{MapSlab, RangeSlab};
use self::writer::write_aggr_smry;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::account_with_cfs::get_field_value;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use sdb_dyn_proto_rdr::reader::Reader;
use slog::Logger;
use std::collections::HashMap;

pub fn aggregate(config_params: ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    // Read Files Configuration
    let files_config = config::get_files(config_params.config_file_path());

    // Read slabs config from file
    let num_slabs: Vec<RangeSlab> = get_num_slabs(&files_config.numslab_file_path);
    let prd_slabs: Vec<RangeSlab> =
        get_prd_slabs(&files_config.prdslab_file_path, config_params.as_on_date());
    let map_slabs: Vec<MapSlab> = get_map_slabs(&files_config.srcmapslab_file_path);
    // To store aggregated data
    let mut aggr_data: HashMap<AggrKey, Data> = HashMap::new();
    let mut ex_rt: f64 = 1.0;
    let mut acc_enc = 0;
    let mut acc_succ = 0;
    let mut ip_amt = 0.0;
    let mut op_amt = 0.0;

    //Creating Default Rule File 
    std::fs::File::create(format!(
        "{}-rule.txt",
        config_params.output_file_path().replace(".txt", "")
    ))
    .expect("Error in Creating Def-Rule File");
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
        let mut is_rules_passed = false;
        let mut rules = AggRules::new_from_path(
            &format!(
                "{}-rule.txt",
                config_params.output_file_path().replace(".txt", "")
            ),
            &file_rdr,
        );
        if file.acc_skip_rules_path.is_some() {
            rules = AggRules::new_from_path(
                &file
                    .acc_skip_rules_path
                    .expect("Could Not read Acc-Skip-Rules File"),
                &file_rdr,
            );
            is_rules_passed = true;
        };

        for account in file_rdr.iter() {
            acc_enc += 1;
            if skip_account(&account, &rules) && is_rules_passed {
                let acc_id = file.acc_id.clone();
                log_warn!(
                    logger,
                    "Skipping Account: {} from Input based on a rule",
                    account
                        .get_string_for_key(&acc_id.unwrap_or_else(|| panic!(
                            "Acc-ID Not configured, Acc-ID is mandatory if Acc-Skip-Rules is Passed/Read"
                        )))
                        .expect("Cannot read Account-ID field.")
                );
                continue;
            }
            let ccy = account
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
            let mut dims: Vec<String> = Vec::new();
            let stamp_input_vec = &file.is_dim_input_stamped;
            for val in 0..=file.dims_fields.len() - 1 {
                let dim: String = if stamp_input_vec[val] {
                    match get_field_value(
                        &account,
                        &method_reader,
                        file.dims_fields[val][0].to_string(),
                    ) {
                        Ok(value) => value,
                        Err(_error) => panic!("{}", _error),
                    }
                } else {
                    get_dim(
                        &file.dims_fields[val],
                        &file.dims_type[val],
                        &account,
                        &method_reader,
                        &num_slabs,
                        &prd_slabs,
                        &map_slabs,
                    )
                };
                dims.push(dim);
            }

            // Construct AggrKey for account
            let aggr_key = AggrKey {
                cust_id: account
                    .get_string_for_key(&file.process_field)
                    .expect("Cannot get customer id from account.")
                    .to_string(),
                dims,
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
    health_report.gen_health_rpt(config_params.output_file_path());

    //Deleting Default Rule File 
    std::fs::remove_file(format!(
        "{}-rule.txt",
        config_params.output_file_path().replace(".txt", "")
    ))
    .expect("Error in Deleting Def-Rule File");
}

pub fn skip_account(account: &AccountWithCFs, rules: &AggRules) -> bool {
    let skip_field = match rules.llg_for_acc(account) {
        Some(_) => true,
        None => false,
    };
    skip_field
}
