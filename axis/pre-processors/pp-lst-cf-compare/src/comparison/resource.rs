use macros;
use rbdate::NaiveDate;
use slog::Logger;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

pub fn write_data(writer: &mut BufWriter<File>, op: String, logger: &Logger) {
    let output_as_bytes = op.as_bytes();
    match writer.write(output_as_bytes) {
        Ok(_val) => {}
        Err(err) => {
            log_info!(logger, "Error writing to output file. Error: {}", err);
        }
    }
}
pub fn get_file_name(base_path: String, name: String) -> String {
    format!("{}{}.txt", base_path, name)
}

pub fn naivedate_from_timestamp(t: i64) -> NaiveDate {
    let naive_date_time = rbdate::NaiveDateTime::from_timestamp(t, 0);
    naive_date_time.date()
}

pub fn get_key_format(cf_keys: &Vec<String>) -> String {
    let mut cf_key_format: String = String::new();
    for key in cf_keys {
        cf_key_format.push_str(key);
        cf_key_format.push('|');
    }
    cf_key_format.pop();
    cf_key_format
}
