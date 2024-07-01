use rbdate::NaiveDate;

#[derive(Debug, Clone, Default)]
pub struct OutputField {
    pub llg_id: String,
    pub as_on_date: NaiveDate,
    pub ccy: String,
    pub sls_irs: String,
    pub source: String,
    pub flow_type: String,
    pub amt: f64,
    pub coupon_rate: f64,
}

pub fn format_output(output_record: OutputField) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{:.2}",
        output_record.llg_id,
        output_record.as_on_date.format("%d-%m-%Y"),
        output_record.ccy,
        output_record.sls_irs,
        output_record.source,
        output_record.flow_type,
        output_record.amt,
        output_record.coupon_rate,
    )
}
pub fn get_writer(file_path: &str) -> std::io::BufWriter<std::fs::File> {
    match sdb_io::buf_file_wrtr(file_path, None) {
        Ok(file) => file,
        Err(error) => panic!("Unable to create file `{}` due to: {}", file_path, error),
    }
}
