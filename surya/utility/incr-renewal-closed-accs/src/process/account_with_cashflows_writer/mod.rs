use macros;
use process::account_with_cashflows::AccountWithCashflows;
use protobuf::Message;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct AccountWithCashflowsWriter {
    log: Logger,
    account_w_cf_writer: BufWriter<File>,
}

fn get_full_path(path: &str, suffix: &str) -> String {
    let mut path_cf_ext = String::new();
    path_cf_ext.push_str(path);
    path_cf_ext.push_str(suffix);
    path_cf_ext
}

impl AccountWithCashflowsWriter {
    pub fn new(
        file_path: &str,
        file_name: &str,
        ason: String,
        log: &Logger,
    ) -> AccountWithCashflowsWriter {
        let path = format!("{}-{}_{}", file_path, file_name, ason);
        AccountWithCashflowsWriter {
            account_w_cf_writer: buf_file_wrtr(&get_full_path(path.as_str(), ".cf"), None)
                .expect("Unable to create `.cf` file."),
            log: log.clone(),
        }
    }
}

impl AccountWithCashflowsWriter {
    pub fn write(&mut self, record: AccountWithCashflows) {
        // Write the account
        let output_bytes = match record.write_length_delimited_to_bytes() {
            Ok(bytes) => bytes,
            Err(e) => {
                log_error!(
                    self.log,
                    "Couldn't parse to bytes. Aborting execution. Account: '{}' Error: {:?}.",
                    record.account_number,
                    e
                );
                return;
            }
        };

        let write_result = self.account_w_cf_writer.write(&output_bytes);

        if write_result.is_err() {
            panic!(
                "Couldn't write output bytes for account: '{}'. Error: {:?}",
                record.account_number,
                write_result.expect_err("Unexpected error occured while writing .cf file.")
            );
        }
    }
}
