#[derive(Debug, Clone, Default)]
pub struct OutputField {
    pub acid: String,
    pub avg_bal: f64,
    pub wt_int_rate: f64,
    pub int_amt: String,
}

pub fn format_output(output_record: OutputField) -> String {
    format!(
        "{}|{:.3}|{}|{}",
        output_record.acid, output_record.avg_bal, output_record.wt_int_rate, output_record.int_amt,
    )
}
pub fn get_writer(file_path: &str) -> std::io::BufWriter<std::fs::File> {
    match sdb_io::buf_file_wrtr(file_path, None) {
        Ok(file) => file,
        Err(error) => panic!("Unable to create file `{}` due to: {}", file_path, error),
    }
}
