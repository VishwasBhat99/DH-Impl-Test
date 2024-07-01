use rbdate::NaiveDate;

#[derive(Debug, Clone, Default)]
pub struct OutputField {
   pub src_code:String,
   pub tot_bal:f64,
   pub asst_class:String,
}

pub fn format_output(output_record: OutputField) -> String {
    format!(
        "{}|{}|{}",
        output_record.src_code,
        output_record.tot_bal,
        output_record.asst_class,
    )
}
pub fn get_writer(file_path: &str) -> std::io::BufWriter<std::fs::File> {
    match sdb_io::buf_file_wrtr(file_path, None) {
        Ok(file) => file,
        Err(error) => panic!("Unable to create file `{}` due to: {}", file_path, error),
    }
}
