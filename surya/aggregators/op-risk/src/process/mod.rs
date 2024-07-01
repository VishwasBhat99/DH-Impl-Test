use configuration_parameters::ConfigurationParameters;
use currency;
use health_report::HealthReport;
use llg::get_llg_id;
use macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::Reader;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::collections::HashMap;
use std::io::Write;
use structs::{AccFields, AggrData, AggrKey, SummaryOutputData};

pub fn process_name(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) {
    let summary_op_file = config_params.output_file().replace(".txt", "") + "-summary.txt";
    // Create Summary Output writer
    let mut summary_output_file = match buf_file_wrtr(&summary_op_file, None) {
        Ok(create) => create,
        Err(error) => {
            panic!(
                "Could not create output file: `{}`: {}.",
                config_params.output_file(),
                error
            );
        }
    };
    // Read Req Account fields file
    let keys = AccFields::new_from_path(config_params.req_field_file());
    // Read currency exchange rate file
    let currency_converter = currency::create_currency_converter(
        config_params.base_currency(),
        config_params.exchange_rate_file(),
    );
    // Read cashflow file
    let mut cf_rec_reader: Reader =
        Reader::new_at_path(config_params.metadata_file(), config_params.input_file());
    let mut acc_aggr: HashMap<AggrKey, AggrData> = HashMap::new();
    let mut tot_acc_enc = 0;
    let mut acc_processed = 0;
    let mut total_bal = 0.0;
    let rules = AggRules::new_from_path(config_params.rules_file(), &cf_rec_reader);
    for account in cf_rec_reader.iter() {
        tot_acc_enc += 1;
        let base_currency = config_params.base_currency().to_string();
        let currency = account
            .get_string_for_key(&keys.currency)
            .unwrap_or(&base_currency);
        let bal_1 = account.get_f64_for_key(&keys.bal).unwrap_or(0.0);
        let bal_2 =
            currency_converter.convert(&currency, bal_1, config_params.is_consolidated(), logger);

        let mut op_llg_id = get_llg_id(&account, &rules, config_params, logger);
        let mut bal_ccy;
        let mut bal_lcy;
        if config_params.is_consolidated() {
            bal_ccy = bal_2;
            bal_lcy = bal_1;
        } else {
            bal_ccy = bal_1;
            bal_lcy = bal_2;
        }
        let conv_indicator = op_llg_id / 100000000;
        if conv_indicator != 0 {
            bal_ccy *= -1.0;
            bal_lcy *= -1.0;
            op_llg_id -= 100000000;
        }
        total_bal += bal_lcy;

        let aggr_key = AggrKey {
            llg_id: op_llg_id.to_string(),
            currency: currency.to_string(),
        };

        let acc_aggr_data = AggrData {
            total_bal_lcy: bal_lcy,
            total_bal_ccy: bal_ccy,
        };
        acc_aggr
            .entry(aggr_key)
            .and_modify(|data| data.aggr_data(&acc_aggr_data))
            .or_insert(acc_aggr_data);
        acc_processed += 1;
    }

    for (aggr_key, aggr_data) in acc_aggr.drain() {
        let op = SummaryOutputData {
            as_on_date: config_params.as_on_date().format("%d-%m-%Y").to_string(),
            ops_claim_id: aggr_key.llg_id[4..8].to_string(),
            currency: aggr_key.currency,
            out_bal_ccy: if config_params.is_absolute_flag() {
                aggr_data.total_bal_ccy.abs()
            } else {
                aggr_data.total_bal_ccy
            },
            out_bal_hcy: if config_params.is_absolute_flag() {
                aggr_data.total_bal_lcy.abs()
            } else {
                aggr_data.total_bal_lcy
            },
        };
        match summary_output_file.write(op.to_string().as_bytes()) {
            Ok(val) => {
                log_debug!(diag_logger, "Writing summary data to output file: {}", val);
            }
            Err(error) => {
                log_debug!(
                    diag_logger,
                    "Cannot write summary data to output file: {}",
                    error
                );
            }
        }
    }

    let health_report = HealthReport::new(
        tot_acc_enc,
        acc_processed,
        tot_acc_enc - acc_processed,
        total_bal,
        total_bal,
        0,
    );
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_params.output_file());
}
