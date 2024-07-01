use account::Account;
use macros;
use protobuf::Message;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::fs::File;
use std::io::{BufWriter, Write};

mod account_indices;

pub struct AccountWithCashflows {
    log: Logger,
    account_w_cf_writer: BufWriter<File>,
}

impl AccountWithCashflows {
    pub fn new(path: &str, log: &Logger) -> AccountWithCashflows {
        let mut full_file_path: String = String::new();
        full_file_path.push_str(path);
        full_file_path.push_str(".cf");
        AccountWithCashflows {
            account_w_cf_writer: buf_file_wrtr(&full_file_path, None)
                .expect("Unable to create `.cf` file."),
            log: log.clone(),
        }
    }
}

impl AccountWithCashflows {
    pub fn write(&mut self, record: Account) {
        // Write the account
        let output_bytes = match record.write_length_delimited_to_bytes() {
            Ok(bytes) => bytes,
            Err(e) => {
                log_error!(
                    self.log,
                    "Couldn't parse to bytes for `source` : `{}`. Error: {:?}.",
                    record.source,
                    e
                );
                return;
            }
        };

        let write_result = self.account_w_cf_writer.write(&output_bytes);

        if write_result.is_err() {
            panic!(
                "Couldn't write output bytes for `source` : {}. Error: {:?}",
                record.source,
                write_result
                    .err()
                    .expect("Unexpected error occured while writing output bytes for `source`.")
            );
        }
    }
}
