mod currency;
mod implementation;
mod llg_key;
mod organize;
mod structs;

use crate::aggregator::currency::CurrencyExchange;
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
use rbdate::date_from_timestamp;
use std::env;

pub fn aggregate_cashflows(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) {
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
    let mut tot_rec = 0;
    let skp_rec = 0;
    let mut tot_amt = 0.0;
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

    for account in account_reader.iter() {
        tot_rec += 1;
        let llg = log_measurements!(
            diag_logger,
            [format!(
                "Type: GetLLG, Identifier: {:?}",
                account
                    .get_string_for_key(&keys.isin)
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
        tot_amt += account.get_f64_for_key(&keys.nativ_amt).unwrap_or(0.0);
        let ccy = account
            .get_string_for_key(&keys.ccy)
            .unwrap_or(&"NA".to_string())
            .to_string();
        write!(
            output_file,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            tot_rec,
            config_params.as_on_date().format("%d-%m-%Y"),
            config_params.country(),
            llg,
            account
                .get_string_for_key(&keys.short_name)
                .unwrap_or(&"NA".to_string()),
            account
                .get_string_for_key(&keys.long_name)
                .unwrap_or(&"NA".to_string()),
            ccy.to_string(),
            account.get_f64_for_key(&keys.nativ_amt).unwrap_or(0.0),
            if config_params.is_consolidated() {
                account.get_f64_for_key(&keys.cons_amt).unwrap_or(0.0)
            } else {
                account.get_f64_for_key(&keys.cons_amt).unwrap_or(0.0)
                    * currency_converter
                        .exchange_rates
                        .get(
                            &(&CurrencyExchange {
                                from_ccy: ccy.to_string(),
                                to_ccy: config_params.base_currency().to_string(),
                            }),
                        )
                        .unwrap_or_else(|| {
                            panic!(
                                "Unable to get exch-rate for {}->{}",
                                ccy.to_string(),
                                config_params.base_currency().to_string()
                            )
                        })
            },
            account
                .get_string_for_key(&keys.isin)
                .unwrap_or(&"NA".to_string()),
            account
                .get_string_for_key(&keys.inv_type)
                .unwrap_or(&"NA".to_string()),
            account
                .get_string_for_key(&keys.portfolio_type)
                .unwrap_or(&"NA".to_string()),
            date_from_timestamp(account.get_i64_for_key(&keys.value_dt).unwrap_or(0))
                .format("%d-%m-%Y"),
            date_from_timestamp(account.get_i64_for_key(&keys.maturity_dt).unwrap_or(0))
                .format("%d-%m-%Y"),
            account
                .get_string_for_key(&keys.issuer_type)
                .unwrap_or(&"NA".to_string()),
            account
                .get_string_for_key(&keys.issuer_name)
                .unwrap_or(&"NA".to_string()),
            account
                .get_string_for_key(&keys.rating_agency_cd)
                .unwrap_or(&"NA".to_string()),
            account
                .get_string_for_key(&keys.rating_cd)
                .unwrap_or(&"NA".to_string()),
            account.get_f64_for_key(&keys.resid_tenor).unwrap_or(0.0),
            account.get_f64_for_key(&keys.coupon_rate).unwrap_or(0.0),
            account
                .get_string_for_key(&keys.add_class1)
                .unwrap_or(&"NA".to_string()),
            account
                .get_string_for_key(&keys.add_class2)
                .unwrap_or(&"NA".to_string()),
            account
                .get_string_for_key(&keys.add_class3)
                .unwrap_or(&"NA".to_string()),
        )
        .expect("Unable to generate aggregated summary file.");
    }
    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    log_info!(logger, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_params.output_file_path());
}
