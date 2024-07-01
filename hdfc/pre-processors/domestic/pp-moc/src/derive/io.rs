use sdb_io::buf_file_wrtr;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

pub fn buf_file_writer(path: &str, suffix: &str, buffer_size: Option<usize>) -> BufWriter<File> {
    let mut full_path = path.to_string();
    full_path.push_str(suffix);

    match buf_file_wrtr(&full_path, buffer_size) {
        Ok(file) => file,
        Err(error) => panic!("Unable to create file: `{}`. : `{}`.", full_path, error),
    }
}

pub fn flush_contents(mut writer: BufWriter<File>, writer_name: &str) {
    let flush_result = writer.flush();
    if flush_result.is_err() {
        // TODO: Also include the filepath of the writer if possible.
        panic!(
            "Program failed when flushing contents of writer: '{}'. Error: {}. Aborting execution.",
            writer_name,
            flush_result
                .err()
                .expect("Unexpected error occured while flushing buffer.")
        );
    }
}
