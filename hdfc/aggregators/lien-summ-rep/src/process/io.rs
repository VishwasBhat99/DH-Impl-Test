use rbdate::NaiveDate;
use sdb_io::buf_file_wrtr;
use std::io::BufWriter;
use std::{env::current_dir, fs::File};

pub fn get_writer(file_path: &str) -> BufWriter<File> {
    match buf_file_wrtr(file_path, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create file `{}` on location `{}` : {}",
            file_path,
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    }
}

pub fn get_file_path(file_path: String, as_on_date: NaiveDate) -> String {
    if file_path.contains("{ddmmyyyy}") {
        file_path.replace("{ddmmyyyy}", &as_on_date.format("%d%m%Y").to_string())
    } else if file_path.contains("{AsOnDate}") {
        file_path.replace("{AsOnDate}", &as_on_date.format("%d-%m-%Y").to_string())
    } else {
        file_path
    }
}
