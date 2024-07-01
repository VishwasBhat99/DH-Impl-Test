use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::NaiveDate;
use rbdate::NaiveDateTime;
use sdb_dyn_proto_rdr::reader::Reader;
use slog::Logger;

pub mod config;
pub mod reader;

pub fn calc(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let mut prin_cf_amount = 0.0;
    // Read Files Configuration
    let files_config = config::get_files(config_params.cfs_config_path());
    for file in files_config.files {
        // Read cashflow file
        let mut file_rdr: Reader =
            reader::read_file(&file.input_file_path, &file.metadata_file_path);

        for mut account in file_rdr.iter() {
            for cf in account
                .remove_cfs_for_key(&String::from("cashflows"))
                .expect("Error while removing cashflow from the pool of cashflows.")
                .iter_mut()
            {
                let cf_date = naivedate_from_timestamp(cf.get_date());
                if &cf_date > config_params.plan_date() {
                    prin_cf_amount += cf.principal_amount;
                }
            }
        }
    }

    if config_params.target_amt() > &prin_cf_amount {
        let new_bus_val = config_params.target_amt() - &prin_cf_amount;
        println!("New Business Value: {}", new_bus_val);
        log_info!(logger, "New Business Value: {:.2}", new_bus_val);
    } else {
        println!("New Business Value: {}", 0.0);
        log_info!(logger, "New Business Value: {:.2}", 0.0);
    }
}

pub fn naivedate_from_timestamp(t: i64) -> NaiveDate {
    let naive_date_time = NaiveDateTime::from_timestamp(t, 0);
    naive_date_time.date()
}
