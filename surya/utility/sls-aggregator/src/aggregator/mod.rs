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
    let mut prin_amt: f64;
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
        let mut aggregate = AggregateData::new();

        prin_amt = account
            .get_f64_for_key(&keys.prin_amt)
            .expect("Failed to get `principal amount` from required fields.");
        if config_params.is_negative() {
            prin_amt *= -1.0;
        }
        tot_amt += prin_amt;
        aggregate.add_data(prin_amt);

        log_measurements!(
            diag_logger,
            [format!("Type: OrganiseInLLG, Identifier: {:?}", llg)],
            organizer.build_width(&llg, &mut aggregate)
        );
    }
    for (llg, data) in organizer.drain() {
        let conv_data = currency_converter.convert(&llg.currency, &data, true, logger);
        if llg.category != config_params.default_llg_code() {
            if config_params.is_consolidated() {
                write!(
                    output_file,
                    "{}|{}|{}|{}|{}\n",
                    config_params.country(),
                    config_params.as_on_date().format("%d-%m-%Y"),
                    llg.currency,
                    conv_data,
                    data
                )
                .expect("Unable to generate aggregated summary file.");
            } else {
                write!(
                    output_file,
                    "{}|{}|{}|{}|{}\n",
                    config_params.country(),
                    config_params.as_on_date().format("%d-%m-%Y"),
                    llg.currency,
                    data,
                    conv_data
                )
                .expect("Unable to generate aggregated summary file.");
            }
        } else {
            continue;
        }
    }

    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    log_info!(logger, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_params.output_file_path());
}
