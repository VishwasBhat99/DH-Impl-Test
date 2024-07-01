use std::env;
use std::env::current_dir;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;

use sdb_io::buf_file_wrtr;
use sdb_io::new_buf_rdr;

pub fn buff_reader(path: &str) -> BufReader<File> {
    match new_buf_rdr(path) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found Maturity Bucket Scheme file: `{}` on location `{}` : {}.",
            path,
            current_dir()
                .unwrap_or_else(|error| {
                    panic!("Error while getting current directory path: {}", error);
                })
                .display(),
            error
        ),
    }
}

pub fn buff_writer(path: &str) -> BufWriter<File> {
    match buf_file_wrtr(path, None) {
        Ok(val) => val,
        Err(error) => {
            panic!(
                "Could not create bucket aggregated file: `{}` on location `{}` : {:?}.",
                path,
                env::current_exe()
                    .unwrap_or_else(|error| {
                        panic!("Unable to find current directory path: {}", error);
                    })
                    .display(),
                error
            );
        }
    }
}
