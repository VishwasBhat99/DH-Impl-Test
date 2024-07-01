use rbdate::NaiveDate;

use rbdate::DateParser;

#[derive(Debug, Clone)]
pub struct OutputAccount {
    pub product_type: String,
    pub isin: String,
    pub first_holder_pan: String,
    pub first_holder_name: String,
    pub category: String,
    pub amount: String,
    pub principal_os: String,
    pub mat_date: String,
    pub portfolio: String,
    pub ccy: String,
}

pub fn format_output(output_record: OutputAccount) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        output_record.product_type,
        output_record.isin,
        output_record.first_holder_pan,
        output_record.first_holder_name,
        output_record.category,
        output_record.amount,
        output_record.principal_os,
        output_record.mat_date,
        output_record.portfolio,
        output_record.ccy,
    )
}
pub fn get_writer(file_path: &str) -> std::io::BufWriter<std::fs::File> {
    match sdb_io::buf_file_wrtr(file_path, None) {
        Ok(file) => file,
        Err(error) => panic!("Unable to create file `{}` due to: {}", file_path, error),
    }
}
