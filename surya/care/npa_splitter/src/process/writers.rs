use sdb_io::buf_file_wrtr;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

pub fn get_new_writer(file_id: i32, output_file_path: &str) -> BufWriter<File> {
    let full_path = format!("{}-{}.cf", output_file_path, file_id);
    match buf_file_wrtr(&full_path, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file '{}'. Error: {:?}.",
            full_path, error
        ),
    }
}

pub fn get_recon_writer(file_path: &str) -> BufWriter<File> {
    let full_path = format!("{}-recon.txt", file_path);
    match buf_file_wrtr(&full_path, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file '{}'. Error: {:?}.",
            full_path, error
        ),
    }
}

pub fn write_data(writer: &mut BufWriter<File>, hdr_bytes: Vec<u8>, data_bytes: Vec<u8>) {
    // Write the account data
    // TODO: Use match for error handling
    let _ = writer.write(&hdr_bytes);
    let _ = writer.write(&data_bytes);
}

pub fn write_recon_data(writer: &mut BufWriter<File>, op_data: String) {
    // TODO: Use match for error handling
    write!(writer, "{}\n", op_data);
}
