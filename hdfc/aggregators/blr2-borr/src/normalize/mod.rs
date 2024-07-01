use self::io::*;
use self::structs::*;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use normalize::account_field_names::AccFieldNames;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use slog::Logger;
use std::collections::HashMap;
use std::default::Default;
use std::io::{BufRead, Write};
use std::time::SystemTime;

mod account_field_names;
mod config;
mod currency;
mod io;
mod structs;

pub fn normalizing(config_params: ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let start_time = SystemTime::now();
    let mut tot_acc_encntrd = 0;
    let mut skp_acc = 0;
    let mut ttl_amt: f64 = 0.0;

    let currency_converter = currency::create_currency_converter(
        config_params.base_ccy(),
        config_params.exchange_rate_file_path(),
    );

    let mut cust_master: HashMap<String, Vec<CustMasterOutput>> = HashMap::new();
    let cust_master_reader = read_manual_file(config_params.cust_code_master());
    for (line_num, lines) in cust_master_reader.lines().enumerate().skip(1) {
        let line = extract_manual_lines(line_num, lines, config_params.cust_code_master());
        let fields: Vec<&str> = line.split(config_params.delimiter()).collect();
        if fields.len() != 4 {
            continue;
        }
        let cust_rec = CustMaster::new(&fields);
        let cust_master_data = CustMasterOutput::new(&cust_rec);
        cust_master
            .entry(cust_rec.ucc_id.to_string())
            .and_modify(|val| val.push(cust_master_data.clone()))
            .or_insert_with(|| vec![cust_master_data]);
    }

    // Read Files Configuration
    let files_config = config::get_files(config_params.config_file_path());
    let mut normalize: HashMap<String, NormalizeData> = HashMap::new();
    for file in files_config.files {
        let keys = AccFieldNames::new_from_path(&file.req_fields_file_path);
        let input_file_path = file.input_file_path.replace(
            "{ddmmyyyy}",
            config_params
                .as_on_date()
                .format("%d%m%Y")
                .to_string()
                .as_str(),
        );
        // Read Cashflows and organise them.
        let mut account_reader =
            reader::Reader::new_at_path(&file.metadata_file_path, &input_file_path);
        let rules = AggRules::new_from_path(&file.rules_file_path, &account_reader);
        for account in account_reader.iter() {
            tot_acc_encntrd += 1;
            let acc_no = account
                .get_string_for_key(&keys.account_number)
                .expect("Cannot get 'account number` field.");
            let amt = match account.get_f64_for_key(&keys.amount) {
                Ok(val) => val,
                Err(err) => panic!(
                    "Cannot get 'amount` field for account: `{}`: `{:?}`.",
                    acc_no, err
                ),
            };
            // Ignoring records whose mapping is not available in rules file.
            // For files having entire borrowings recods `is_rule_required` flag will be `false` otherwise `true`.
            if rules.llg_for_acc(&account).is_none() && file.is_rule_required {
                skp_acc += 1;
                continue;
            }
            let def_str = String::new();
            let cust_id = account
                .get_string_for_key(&keys.cust_id)
                .unwrap_or(&def_str);
            let ccy = match account.get_string_for_key(&keys.ccy) {
                Ok(val) => val,
                Err(err) => panic!(
                    "Cannot get 'currency` field for account: `{}`: `{:?}`.",
                    acc_no, err
                ),
            };

            let ex_rt = *currency_converter.exchange_rates.get(ccy).unwrap_or_else(|| panic!(
                    "The target exchange rate requested '{}' was not found in the conversion rates file.",
                    ccy
                ));
            let (lcy_amt, fcy_amt) =
                if currency_converter.consolidated_currency != *ccy && !file.is_consolidated {
                    (amt * ex_rt, amt)
                } else if currency_converter.consolidated_currency != *ccy && file.is_consolidated {
                    (amt, amt / ex_rt)
                } else {
                    (amt, amt)
                };
            ttl_amt += lcy_amt;
            let mut norm_data = NormalizeData::new();
            let def_str = String::default();
            norm_data.insert(
                ccy.to_string(),
                def_str.to_string(),
                def_str,
                lcy_amt,
                fcy_amt,
            );
            normalize
                .entry(cust_id.to_string())
                .and_modify(|data| data.add(norm_data.clone()))
                .or_insert(norm_data);
        }
    }

    let mut op_writer = get_writer(config_params.output_file_path());
    let mut aggregates: HashMap<AggregatedKey, NormalizeData> = HashMap::new();
    for (ucc_id, cust_data) in cust_master.drain() {
        for cust_dat in cust_data.iter() {
            if let Some(mut data) = normalize.remove(&cust_dat.cust_id.to_string()) {
                data.add_cust_det(ucc_id.to_string(), cust_dat.cust_name.to_string());
                aggregates
                    .entry(AggregatedKey::new(
                        cust_dat.cust_id.to_string(),
                        data.ccy_id.to_string(),
                    ))
                    .and_modify(|val| val.add(data.clone()))
                    .or_insert(data);
            }
        }
    }

    for (_, data) in normalize.drain() {
        write!(
            op_writer,
            "{}|{}|{}",
            config_params.country_id(),
            config_params.as_on_date().format("%d-%m-%Y"),
            data
        )
        .expect("Unable to generate summary file.");
    }

    for (_, data) in aggregates.drain() {
        write!(
            op_writer,
            "{}|{}|{}",
            config_params.country_id(),
            config_params.as_on_date().format("%d-%m-%Y"),
            data
        )
        .expect("Unable to generate summary file.");
    }

    let total_duration = print_return_time_since!(start_time);
    log_info!(logger, "Total time for aggregation: {:?}", total_duration);
    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - skp_acc,
        skp_acc,
        ttl_amt,
        ttl_amt,
        0,
    );
    health_report.gen_health_rpt(config_params.output_file_path());
}
