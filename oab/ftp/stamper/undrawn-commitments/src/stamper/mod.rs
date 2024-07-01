mod account_reader;
mod account_without_cashflows;
mod account_writer;
mod cashflow_appender;
mod llg_finder;

use configuration_parameters::ConfigurationParameters;
use macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use slog::Logger;
use stamper::account_reader::AccFieldNames;
use stamper::account_writer::AccountWithoutCashflows;
use stamper::cashflow_appender::append_cashflow;

pub fn stamp_llg(config_params: ConfigurationParameters, log: &Logger) {
    let input_field_names = AccFieldNames::get_input_fields_names();

    let mut input_data = reader::Reader::new_at_path(
        &config_params.metadata_file_path(),
        &config_params.input_file_path(),
    );

    let rules = &AggRules::new_from_path(&config_params.rule_file_path(), &input_data);
    let default_rl1 = config_params.default_rl1();

    let mut total_accounts = 0;
    let mut output_writer = create_io_workers(&config_params.output_file_path(), &log);

    for mut account_input in input_data.iter() {
        total_accounts += 1;
        // Read Cashflow
        let stmp_records = append_cashflow(
            &mut account_input,
            &input_field_names,
            rules,
            default_rl1,
            &log,
        );
        // Write Output
        output_writer.write(stmp_records);
    }

    let report_string = format!("Total Accounts: {}", total_accounts);

    // Log the output and exit
    log_info!(log, "{}", report_string);
    println!("{}", report_string);
}

fn create_io_workers(output_path: &str, log: &Logger) -> (AccountWithoutCashflows) {
    let writer = AccountWithoutCashflows::new(output_path, log);

    return writer;
}
