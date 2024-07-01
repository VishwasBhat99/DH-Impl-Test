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
    let mut organizer = Organizer::new(currency_converter);
    let keys = AccFieldNames::new_from_path(config_params.req_fields_file_path());

    let mut account_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );
    let rules = AggRules::new_from_path(config_params.rules_file_path(), &account_reader);
    let mut prin_amt: f64;
    let mut rt: f64;
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
        let mut llg = log_measurements!(
            diag_logger,
            [format!(
                "Type: GetLLG, Identifier: {:?}",
                account.get_string_for_key(&keys.concat).expect("fail")
            )],
            implementation::llg_for_account(
                &account,
                &keys,
                &rules,
                config_params.consolidated_currency(),
                config_params.local_consolidation_currency(),
                config_params.default_llg_code(),
                logger
            )
        );
        let mut build = AggregateData::new();

        prin_amt = account
            .get_f64_for_key(&keys.prin_amt)
            .expect("Failed to get `principal amount` from required fields.");

        rt = account
            .get_f64_for_key(&keys.int_rt)
            .expect("Failed to get `interest amount` from required fields.");

        prin_amt = prin_amt.abs();
        rt = rt.abs();

        build.add_data(prin_amt, rt);

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
            "{}|{}|{}|ALL|MASTER|{}|{}\n",
            llg.category,
            config_params.as_on_date().format("%d-%m-%Y"),
            llg.currency,
            llg.cf_type,
            data
        )
        .expect("Unable to generate summary file.");
    }
}
