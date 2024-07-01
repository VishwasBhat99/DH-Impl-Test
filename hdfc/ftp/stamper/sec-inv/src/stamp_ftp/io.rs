use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{BufReader, BufWriter};

pub fn buf_reader(file_path: &str) -> BufReader<File> {
    let file = match OpenOptions::new().read(true).open(file_path) {
        Ok(f) => f,
        Err(e) => {
            panic!("Unable to open file at path '{}'. Error: {}", file_path, e);
        }
    };

    BufReader::new(file)
}

pub fn buf_file_writer(path: &str, suffix: &str, buffer_size: Option<usize>) -> BufWriter<File> {
    let mut full_path = path.to_string();
    full_path.push_str(suffix);

    let file_open_result = OpenOptions::new()
        .write(true)
        .create(true)
        .open(full_path.clone());

    if file_open_result.is_err() {
        panic!(
            "Unable to open file '{}'. Error: {}",
            full_path,
            file_open_result.err().unwrap()
        );
    }

    if buffer_size.is_some() {
        return BufWriter::with_capacity(buffer_size.unwrap(), file_open_result.unwrap());
    } else {
        return BufWriter::new(file_open_result.unwrap());
    }
}

pub fn flush_contents(mut writer: BufWriter<File>, writer_name: &str) {
    let flush_result = writer.flush();
    if flush_result.is_err() {
        // TODO: Also include the filepath of the writer if possible.
        panic!(
            "Program failed when flushing contents of writer: '{}'. Error: {}. Aborting execution.",
            writer_name,
            flush_result.err().unwrap()
        );
    }
}
