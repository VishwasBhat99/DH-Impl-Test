use self::bucket::write_bucket;
use crate::aggregator::account_field_names::AccFieldNames;
use crate::aggregator::organize::Organizer;
use crate::aggregator::structs::*;
use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use health_report::HealthReport;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::env;
use std::io::Write;
use std::time::SystemTime;

mod account_field_names;
mod bucket;
mod currency;
mod implementation;
mod llg_key;
mod organize;
mod structs;

pub fn aggregate(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let keys = AccFieldNames::new_from_path(config_params.req_fields_file_path());
    let mut account_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );

    let currency_converter = currency::create_currency_converter(
        config_params.base_currency(),
        config_params.currency_conversion_file_path(),
    );
    let mut organizer = Organizer::new(currency_converter);
    let mut amt: f64;
    let mut accounts_encountered = 0;
    let mut tot_amt_in_ip: f64 = 0.0;

    let rules = AggRules::new_from_path(config_params.rules_file_path(), &account_reader);
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

    let as_on_date = config_params.as_on_date();
    for account in account_reader.iter() {
        let llg = log_measurements!(
            diag_logger,
            [format!(
                "Type: GetLLG, Identifier: {:?}",
                account
                    .get_string_for_key(&keys.account_number)
                    .expect("Error getting `account_number`.")
            )],
            implementation::llg_for_account(&account, &keys, &rules, config_params, logger)
        );

        let mut data = AggregateData::new();
        amt = account
            .get_f64_for_key(&keys.amount)
            .expect("Failed to get `amount` from required fields.");

        if config_params.is_amt_abs() {
            amt = amt.abs();
        }
        tot_amt_in_ip += amt;
        data.add_amount(amt);
        accounts_encountered += 1;

        log_measurements!(
            diag_logger,
            [format!("Type: OrganiseCFsInLLG, Identifier: {:?}", llg)],
            organizer.organize(&llg, data, config_params, logger)
        );
    }
    for (llg_key, data) in organizer.drain() {
        let op_line = format!(
            "{}|{}|{}|{}|",
            as_on_date,
            config_params.country(),
            llg_key.currency,
            llg_key.category
        );
        writeln!(
            output_file,
            "{}",
            write_bucket(logger, data, config_params, op_line)
        )
        .expect("Unable to generate aggregated summary file.");
    }
    let health_report = HealthReport::new(
        accounts_encountered,
        accounts_encountered,
        0,
        tot_amt_in_ip,
        tot_amt_in_ip,
        0,
    );
    health_report.gen_health_rpt(config_params.output_file_path());
}
