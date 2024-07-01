use aggregator::account_field_names::AccFieldNames;
use configuration_parameters::ConfigurationParameters;
use macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::env;
use std::io::Write;

mod account_field_names;
mod implementation;
mod reading_pt;

pub fn aggregate(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let keys = AccFieldNames::new_from_path(config_params.req_fields_file_path());
    let mut acc_enc = 0;
    let mut acc_succ = 0;
    let mut account_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );
    let rules = AggRules::new_from_path(config_params.rules_file_path(), &account_reader);

    let mut writer = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(writer) => writer,
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
    let field_type_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );
    for account in account_reader.iter() {
        acc_enc += 1;
        let mut op = String::new();
        let class_type = implementation::class_type_for_account(
            &account,
            &rules,
            config_params.default_code(),
            logger,
        );
        op.push_str(&class_type.to_string());
        op.push('|');
        let pass_throughs = reading_pt::read_pass_through(&account, &keys, &field_type_reader);
        op.push_str(&pass_throughs);
        op.push('\n');
        match writer.write(op.as_bytes()) {
            Ok(_) => {}
            Err(err) => log_error!(logger, "Cannot write output file. Error: {}", err),
        }
        acc_succ += 1;
    }
    let health_stat =
        health_report::HealthReport::new(acc_enc, acc_succ, acc_enc - acc_succ, 0.0, 0.0, 0);
    health_stat.gen_health_rpt(config_params.output_file_path())
}
