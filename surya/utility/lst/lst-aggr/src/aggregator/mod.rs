use aggregator::account_field_names::AccFieldNames;
use configuration_parameters::ConfigurationParameters;
use macros;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::time::SystemTime;

mod account_field_names;
mod duration_extensions;
mod exchange_rate;
mod grouping_cfs;
mod reading_pt;

pub fn aggregate_cashflows(
    config_params: ConfigurationParameters,
    logger: &Logger,
    _diag_logger: &Logger,
) {
    let start_time = SystemTime::now();
    let mut writer = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(wrtr) => wrtr,
        Err(error) => {
            panic!(
                "Could not create file: `{}` on location `{}` : {:?}.",
                config_params.output_file_path(),
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    let keys = AccFieldNames::new_from_path(config_params.known_fields_file_path());
    let exchange_rate_map =
        exchange_rate::read_exchange_rate(config_params.currency_conversion_file_path());
    let mut account_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );
    let field_type_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );

    for account in account_reader.iter() {
        let pass_throughs = reading_pt::read_pass_through(&account, &keys, &field_type_reader);
        let currency = read_currency(&account, &keys);
        let aggr_data = grouping_cfs::group_cfs(account, &keys, config_params.as_on_date());
        let acc_exchange_rate = exchange_rate::get_exch_rate(
            currency,
            config_params.base_currency(),
            &exchange_rate_map,
        );
        let consol_aggr_data = get_consol_grped_cfs(&aggr_data, acc_exchange_rate);
        write_aggr_data(&mut writer, pass_throughs, aggr_data, consol_aggr_data);
    }

    let total_duration = print_return_time_since!(start_time);
    log_info!(logger, "Total time for aggregation: {:?}", total_duration);
}

fn read_currency(account: &AccountWithCFs, keys: &AccFieldNames) -> String {
    let currency = account
        .get_string_for_key(&keys.currency)
        .expect("Error while reading currency.");
    currency.to_string()
}

pub fn get_consol_grped_cfs(grped_cfs: &Vec<f64>, exchange_rate: f64) -> Vec<f64> {
    let mut consol_data: Vec<f64> = vec![0.0, 0.0, 0.0];
    let mut index = 0;
    for amount in grped_cfs {
        consol_data[index] = amount * exchange_rate;
        index += 1;
    }
    consol_data
}

pub fn write_aggr_data(
    writer: &mut BufWriter<File>,
    pt: String,
    aggr_data: Vec<f64>,
    consol_aggr_data: Vec<f64>,
) {
    let output = format!(
        "{}{}|{}|{}|{}|{}|{}\n",
        &pt,
        aggr_data[0],
        aggr_data[1],
        aggr_data[2],
        consol_aggr_data[0],
        consol_aggr_data[1],
        consol_aggr_data[2]
    );
    let output_as_bytes = output.as_bytes();
    match writer.write(output_as_bytes) {
        Ok(_val) => {}
        Err(err) => println!("Error writing to output file. Error: {}", err),
    }
}
