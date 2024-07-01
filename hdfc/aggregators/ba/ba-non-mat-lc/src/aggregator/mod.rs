mod currency;
mod implementation;
mod llg_key;
mod organize;
mod structs;

use self::structs::AggregateData;
use aggregator::account_field_names::AccFieldNames;
use configuration_parameters::ConfigurationParameters;
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
        config_params.consolidated_currency(),
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
        let currency = account
            .get_string_for_key(&keys.curr)
            .expect("Could not read `currency` field from .cf file.");
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
                currency,
                config_params,
                logger
            )
        );
        let mut build = AggregateData::new();

        prin_amt = account
            .get_f64_for_key(&keys.prin_amt)
            .expect("Failed to get `principal amount` from required fields.");

        if config_params.is_consolidated() {
            let ex_rt = *currency_converter.exchange_rate(currency);
            if ex_rt != 0.0 {
                prin_amt = prin_amt / ex_rt;
            } else {
                prin_amt = 0.0
            }
        }

        if llg.cf_type == "O" {
            prin_amt = prin_amt * -1.0;
        }

        build.add_data(prin_amt);

        log_measurements!(
            diag_logger,
            [format!("Type: OrganiseInLLG, Identifier: {:?}", llg)],
            organizer.build_width(
                &llg,
                &mut build,
                config_params.foreign_consolidation_currency(),
                config_params.local_consolidation_currency(),
            )
        );
    }
    for (llg, data) in organizer.drain() {
        write!(
            output_file,
            "{}|{}|{}|DIM1|DIM2|DIM3|DIM4|DIM5|{}\n",
            llg.category,
            date_from_timestamp(llg.date).format("%d-%m-%Y"),
            llg.currency,
            data
        )
        .expect("Unable to generate summary file.");
    }
}
