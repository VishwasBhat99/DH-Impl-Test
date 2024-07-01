#[derive(Debug, Clone, Default)]
pub struct OutputField {
    pub gl_code: String,
    pub ccy: String,
    pub description: String,
    pub classification: String,
    pub group: String,
    pub llg: String,
    pub recon_amt: f64,
    pub gstt_amt: f64,
    pub diff_amt: f64,
}

pub fn format_output(output_record: OutputField) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}",
        output_record.gl_code,
        output_record.ccy,
        output_record.description,
        output_record.classification,
        output_record.group,
        output_record.llg,
        output_record.recon_amt,
        output_record.gstt_amt,
        output_record.diff_amt
    )
}
pub fn get_writer(file_path: &str) -> std::io::BufWriter<std::fs::File> {
    match sdb_io::buf_file_wrtr(file_path, None) {
        Ok(file) => file,
        Err(error) => panic!("Unable to create file `{}` due to: {}", file_path, error),
    }
}
