use macros;
use process::account_with_cashflows::Account;
use process::account_with_cashflows_writer::account_indices::Index;
use protobuf::Message;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::fs::File;
use std::io::{BufWriter, Write};

mod account_indices;

pub struct AccountWithCashflowsWriter {
    log: Logger,
    account_w_cf_writer: BufWriter<File>,
    account_indices_writer: BufWriter<File>,
    account_index: Index,
    current_account_offset: i64,
}

fn get_full_path(path: &str, suffix: &str) -> String {
    let mut path_cf_ext = String::new();
    path_cf_ext.push_str(path);
    path_cf_ext.push_str(suffix);
    path_cf_ext
}

impl AccountWithCashflowsWriter {
    pub fn new(path: &str, log: &Logger) -> AccountWithCashflowsWriter {
        AccountWithCashflowsWriter {
            account_w_cf_writer: buf_file_wrtr(&get_full_path(path, ".cf"), None)
                .expect("Unable to create `.cf` file."),
            account_indices_writer: buf_file_wrtr(&get_full_path(path, ".idx"), None)
                .expect("Unable to create `.idx` file."),
            account_index: Index::new(),
            current_account_offset: 0,
            log: log.clone(),
        }
    }
}

impl AccountWithCashflowsWriter {
    pub fn write(&mut self, record: Account) {
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
                write_result
                    .err()
                    .expect("Unexpected error occured while writing .cf file.")
            );
        }

        // Write the index for this account:
        self.account_index.set_key(record.account_number);
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
        flush_contents(self.account_w_cf_writer, "AccountWithCashflowsWriter")
    }
}

pub fn flush_contents(mut writer: BufWriter<File>, writer_name: &str) {
    let flush_result = writer.flush();
    if flush_result.is_err() {
        panic!(
            "Program failed when flushing contents of writer: '{}'. Error: {}. Aborting execution.",
            writer_name,
            flush_result
                .err()
                .expect("Unexpected error occured while flushing buffer!")
        );
    }
}
