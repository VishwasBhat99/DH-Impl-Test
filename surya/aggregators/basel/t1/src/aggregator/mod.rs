mod currency;
mod implementation;
mod llg_key;
mod organize;
mod structs;

use self::structs::AggregateData;
use aggregator::account_field_names::AccFieldNames;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::io::Write;
use std::time::SystemTime;
mod account_field_names;
use self::organize::Organizer;
use aggregator::currency::CurrencyExchange;
use std::env;
pub fn aggregate_cashflows(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) {
    let llg_negative: Vec<&str> = config_params.llg_neg().split(',').collect();
    let neg_llg: Vec<String> = llg_negative.iter().map(|&id| id.to_string()).collect();

    let llg_absolute: Vec<&str> = config_params.llg_abs().split(',').collect();
    let abs_llg: Vec<String> = llg_absolute.iter().map(|&id| id.to_string()).collect();

    for id in &neg_llg {
        if abs_llg.contains(id) {
            log_error!(
                logger,
                "Same LLG encountered in both negative and absolute llg list."
            );
            break;
        }
    }
    let currency_converter = currency::create_currency_converter(
        config_params.base_currency(),
        config_params.currency_conversion_file_path(),
    );
    let mut organizer = Organizer::new(currency_converter.clone());
    let keys = AccFieldNames::new_from_path(config_params.req_fields_file_path());

    let mut account_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );
    let rules = AggRules::new_from_path(config_params.rules_file_path(), &account_reader);
    let mut prin_amt: f64;
    let mut prin_amt_lcy;
    let mut tot_rec = 0;
    let skp_rec = 0;
    let tot_amt = 0.0;
    let mut ex_rt = 1.0;
    let mut output_file = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(create) => create,
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
    let mut target_currency;

    for account in account_reader.iter() {
        tot_rec += 1;
        let llg = log_measurements!(
            diag_logger,
            [format!(
                "Type: GetLLG, Identifier: {:?}",
                account
                    .get_string_for_key(&keys.acc_no)
                    .unwrap_or(&"NA".to_string())
            )],
            implementation::llg_for_account(
                &account,
                &keys,
                &rules,
                config_params.default_llg_code(),
                config_params,
                logger
            )
        );
        let exrt_key = keys.exchange_rate.clone();
        ex_rt = account
            .get_f64_for_key(&exrt_key.unwrap_or_default())
            .expect("Cannot get exchange rate from account.");

        let mut aggregate = AggregateData::new();

        prin_amt = account
            .get_f64_for_key(&keys.prin_amt)
            .expect("Failed to get `principal amount` from required fields.");
        prin_amt_lcy = account
            .get_f64_for_key(&keys.prin_amt)
            .expect("Failed to get `principal amount for local amount` from required fields.");
        if config_params.is_account_level_exchange_rate() {
            if config_params.is_consolidated() {
                prin_amt /= ex_rt;
            } else {
                prin_amt_lcy *= ex_rt;
            }
            aggregate.add_data(prin_amt, prin_amt_lcy);
        } else {
            if config_params.is_consolidated() {
                target_currency = CurrencyExchange {
                    from_ccy: config_params.base_currency().to_string(),
                    to_ccy: llg.currency.to_string(),
                };
                let ex_rate = currency_converter.convert(&target_currency, logger);
                prin_amt *= ex_rate;
            } else {
                target_currency = CurrencyExchange {
                    from_ccy: llg.currency.to_string(),
                    to_ccy: config_params.base_currency().to_string(),
                };
                let ex_rate = currency_converter.convert(&target_currency, logger);
                prin_amt_lcy *= ex_rate;
            }
            aggregate.add_data(prin_amt, prin_amt_lcy);
        }
        if config_params.is_negative() {
            prin_amt *= -1.0;
        }
        log_measurements!(
            diag_logger,
            [format!("Type: OrganiseInLLG, Identifier: {:?}", llg)],
            organizer.build_width(&llg, &mut aggregate)
        );
    }
    for (llg, mut data) in organizer.drain() {
        if neg_llg.contains(&llg.category.to_string()) {
            data.tot_prin_amt *= -1.0;
            data.tot_prin_amt_ccy *= -1.0;

        }
        if abs_llg.contains(&llg.category.to_string()) {
            data.tot_prin_amt = data.tot_prin_amt.abs();
            data.tot_prin_amt_ccy = data.tot_prin_amt_ccy.abs();

        }
        writeln!(
            output_file,
            "{}|{}|{}|{}|{}|{}",
            config_params.as_on_date().format("%d-%m-%Y"),
            config_params.country(),
            llg.currency,
            llg.category,
            data.tot_prin_amt,
            data.tot_prin_amt_ccy
        )
        .expect("Unable to generate aggregated summary file.");
    }

    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    log_info!(logger, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file_path());
}
