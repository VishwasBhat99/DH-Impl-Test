use self::derive_open_acc_fields::*;
use self::io::*;
use self::structs::NormalizeData;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use normalize::account_field_names::AccFieldNames;
use rbdate::date_from_timestamp;
use sdb_dyn_proto_rdr::reader;
use slog::Logger;
use statics::DEFAULT_FLOAT;
use std::io::Write;
use std::time::SystemTime;

mod account_field_names;
mod derive_open_acc_fields;
mod io;
mod structs;

pub fn normalizing(config_params: ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let start_time = SystemTime::now();
    let mut tot_acc_encntrd = 0;
    let skp_acc = 0;
    let mut ttl_amt: f64 = 0.0;
    let keys = AccFieldNames::new_from_path(config_params.known_fields_file_path());

    let mut account_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );

    let as_on_date = *config_params.as_on_date();
    let mut open_accs = String::new();
    let mut store: Vec<NormalizeData> = Vec::new();
    for account in account_reader.iter() {
        tot_acc_encntrd += 1;
        let acc_no = account
            .get_string_for_key(&keys.account_number)
            .expect("Cannot get 'account number` field.");
        let amt = account
            .get_f64_for_key(&keys.amount)
            .expect("Cannot get 'amount` field.");
        let int_rt = account
            .get_f64_for_key(&keys.interest_rate)
            .unwrap_or(DEFAULT_FLOAT);
        ttl_amt += amt;
        let mut aggr_data = NormalizeData::new();
        aggr_data.insert(acc_no.to_string(), as_on_date, amt, int_rt);
        store.push(aggr_data);
        let acc_open_dt = account
            .get_i64_for_key(&keys.account_open_date)
            .expect("Error while getting `account_open_date`.");
        if date_from_timestamp(acc_open_dt) == as_on_date.pred()
            || is_first_day_of_month(as_on_date)
        {
            open_accs.push_str(acc_no);
            open_accs.push('\n');
        }
    }

    let mut op_writer = get_writer(config_params.output_file_path());
    for data in store.iter() {
        write!(op_writer, "{}", data).expect("Unable to generate summary file.");
    }

    let mut open_file_writer = get_append_writer(config_params.open_accs_file_path());
    output_writer(
        &mut open_file_writer,
        open_accs,
        config_params.open_accs_file_path(),
    );

    let total_duration = print_return_time_since!(start_time);
    log_info!(logger, "Total time for aggregation: {:?}", total_duration);
    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - skp_acc,
        skp_acc,
        ttl_amt,
        ttl_amt,
        0,
    );
    health_report.gen_health_rpt(&config_params.output_file_path());
}
