use rbdate::NaiveDate;

use rbdate::DateParser;

#[derive(Debug, Clone)]
pub struct OutputAccount {
    pub code: String,
    pub description: String,
    pub amount: f64,
    pub currency: String,
    pub cf_date: String,
    pub customer_code : String,
}

pub fn format_output(output_record: OutputAccount) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}",
        output_record.code,
        output_record.description,
        output_record.amount,
        output_record.currency,
        output_record.cf_date,
        output_record.customer_code,
    )
}
pub fn get_writer(file_path: &str) -> std::io::BufWriter<std::fs::File> {
    match sdb_io::buf_file_wrtr(file_path, None) {
        Ok(file) => file,
        Err(error) => panic!("Unable to create file `{}` due to: {}", file_path, error),
    }
}
