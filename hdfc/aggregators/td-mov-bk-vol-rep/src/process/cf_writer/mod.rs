use super::cf_writer::account::Account;
use super::macros;
use protobuf::Message;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::fs::File;
use std::io::{BufWriter, Write};

pub mod account;
pub mod create_account;

pub struct CFwrite {
    log: Logger,
    account_w_cf_writer: BufWriter<File>,
}

impl CFwrite {
    pub fn new(path: &str, log: &Logger) -> CFwrite {
        let mut full_file_path: String = String::new();
        full_file_path.push_str(path);
        full_file_path.push_str(".cf");
        CFwrite {
            account_w_cf_writer: buf_file_wrtr(&full_file_path, None)
                .expect("Unable to create `.cf` file."),
            log: log.clone(),
        }
    }
}

impl CFwrite {
    pub fn write(&mut self, record: Account) {
        // Write the account
        let output_bytes = match record.write_length_delimited_to_bytes() {
            Ok(bytes) => bytes,
            Err(e) => {
                log_error!(
                    self.log,
                    "Couldn't parse to bytes for `acc_no` : `{}`. Error: {:?}.",
                    record.acc_no,
                    e
                );
                return;
            }
        };

        let write_result = self.account_w_cf_writer.write(&output_bytes);

        if write_result.is_err() {
            panic!(
                "Couldn't write output bytes for `acc_no` : {}. Error: {:?}",
                record.acc_no,
                write_result
                    .err()
                    .expect("Unexpected error occured while writing output bytes for `alm_line`.")
            );
        }
    }
}
