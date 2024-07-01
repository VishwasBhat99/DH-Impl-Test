use derive::account::Account;
use derive::io;
use macros;
use protobuf::Message;
use slog::Logger;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

mod account_indices;

pub struct AccountWithoutCashflows {
    log: Logger,
    account_w_cf_writer: BufWriter<File>,
}

impl AccountWithoutCashflows {
    pub fn new(path: &str, log: &Logger) -> AccountWithoutCashflows {
        AccountWithoutCashflows {
            account_w_cf_writer: io::buf_file_writer(path, ".cf", None),
            log: log.clone(),
        }
    }
}

impl AccountWithoutCashflows {
    pub fn write(&mut self, record: Account) {
        // Write the account
        let output_bytes = match record.write_length_delimited_to_bytes() {
            Ok(bytes) => bytes,
            Err(err) => {
                log_error!(self.log, "Error while writing: {:?}.", err);
                return;
            }
        };

        let write_result = self.account_w_cf_writer.write(&output_bytes);

        if write_result.is_err() {
            panic!(
                "Couldn't write output bytes : {:?}",
                write_result.err().unwrap()
            );
        }
    }
}

impl AccountWithoutCashflows {
    pub fn close(self) {
        io::flush_contents(self.account_w_cf_writer, "AccountWithoutCashflows")
    }
}
