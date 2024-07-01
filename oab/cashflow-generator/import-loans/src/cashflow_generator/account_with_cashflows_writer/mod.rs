use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows_writer::account_indices::Index;
use cashflow_generator::io;
use macros;
use protobuf::Message;
use slog::Logger;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

mod account_indices;

pub struct AccountWithCashflowsWriter {
    log: Logger,
    account_w_cf_writer: BufWriter<File>,
    account_indices_writer: BufWriter<File>,
    account_index: Index,
    current_account_offset: i64,
}

impl AccountWithCashflowsWriter {
    pub fn new(path: &str, log: &Logger) -> AccountWithCashflowsWriter {
        AccountWithCashflowsWriter {
            account_w_cf_writer: io::buf_file_writer(path, ".cf", None),
            account_indices_writer: io::buf_file_writer(path, ".idx", None),
            account_index: Index::new(),
            current_account_offset: 0,
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

        // Write the index for this account:
        self.account_index.set_key(record.account_id);
        self.current_account_offset += output_bytes.len() as i64;

        match self
            .account_index
            .write_length_delimited_to_writer(&mut self.account_indices_writer)
        {
            Ok(_) => {}
            Err(e) => {
                panic!("Failed to write index. Aborting execution. Error: {}.", e);
            }
        }
    }
}

impl AccountWithCashflowsWriter {
    pub fn close(self) {
        io::flush_contents(self.account_w_cf_writer, "AccountWithCashflowsWriter")
    }
}
