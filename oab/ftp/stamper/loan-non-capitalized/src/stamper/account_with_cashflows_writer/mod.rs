use macros;
use protobuf::Message;
use slog::Logger;
use stamper::account_with_cashflows::AccountWithCashflows;
use stamper::io;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

pub struct AccountWithCashflowsWriter {
    log: Logger,
    account_w_cf_writer: BufWriter<File>,
}

impl AccountWithCashflowsWriter {
    pub fn new(path: &str, log: &Logger) -> AccountWithCashflowsWriter {
        AccountWithCashflowsWriter {
            account_w_cf_writer: io::buf_file_writer(path, ".cf", None),
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
                    record.account_id,
                    e
                );
                return;
            }
        };

        let write_result = self.account_w_cf_writer.write(&output_bytes);

        if write_result.is_err() {
            panic!(
                "Couldn't write output bytes for account: '{}'. Error: {:?}",
                record.account_id,
                write_result.err().unwrap()
            );
        }
    }
}
