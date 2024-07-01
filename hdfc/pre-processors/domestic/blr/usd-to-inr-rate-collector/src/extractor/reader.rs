use super::{
    extract_lines, macros, read_file, ConfigurationParameters, Dates, ExchangeRate, HashMap,
    Logger, NaiveDate,
};
use std::path::Path;

pub fn get_data(
    dates: &Dates,
    config_params: &ConfigurationParameters,
    rates_data: &mut HashMap<NaiveDate, f64>,
    log: &Logger,
) {
    let mut next_date = dates.start_date;
    let date_format = "%d%m%Y";
    let as_on_dt = config_params.as_on_date().format(date_format).to_string();
    while next_date <= dates.end_date {
        let date_folder = next_date.format(date_format).to_string();
        let inp_file_path = config_params.input_file().replace(&as_on_dt, &date_folder);
        if !Path::new(&inp_file_path).exists() {
            log_error!(
                log,
                "Exchange Rate file: `{}` not available for date: `{}`.",
                inp_file_path,
                next_date.format("%d-%m-%Y"),
            );
            panic!(
                "Exchange Rate file: {} not available for date: {}.",
                inp_file_path,
                next_date.format("%d-%m-%Y")
            );
        }
        let mut reader = read_file(&inp_file_path);
        for (line_num, lines) in reader.deserialize().enumerate() {
            let exchange_rate: ExchangeRate = extract_lines(line_num, lines, &inp_file_path, log);
            if exchange_rate.source_ccy == "USD" && exchange_rate.target_ccy == "INR" {
                rates_data.insert(next_date, exchange_rate.rate);
            }
        }
        next_date = next_date.succ();
    }
}
