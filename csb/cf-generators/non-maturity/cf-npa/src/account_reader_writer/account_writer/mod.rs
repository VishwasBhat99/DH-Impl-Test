use account_reader_writer::account_without_cashflows::OutputAccount;
use account_reader_writer::account_writer::account_indices::Index;
use macros;
use protobuf::Message;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::fs::File;
use std::io::{BufWriter, Write};

mod account_indices;

pub struct AccountWithoutCashflows {
    log: Logger,
    account_writer: BufWriter<File>,
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

impl AccountWithoutCashflows {
    pub fn new(path: &str, log: &Logger) -> AccountWithoutCashflows {
        AccountWithoutCashflows {
            account_writer: buf_file_wrtr(&get_full_path(path, ".cf"), None)
                .expect("Unable to create `.cf` file."),
            account_indices_writer: buf_file_wrtr(&get_full_path(path, ".idx"), None)
                .expect("Unable to create `.idx` file."),
            account_index: Index::new(),
            current_account_offset: 0,
            log: log.clone(),
        }
    }
}

impl AccountWithoutCashflows {
    pub fn write(&mut self, record: OutputAccount) {
        // Write the account
        let output_bytes = match record.write_length_delimited_to_bytes() {
            Ok(bytes) => bytes,
            Err(e) => {
                log_error!(
                    self.log,
                    "Couldn't parse to bytes. Aborting execution. Account: '{}' Error: {:?}.",
                    record.acc_no,
                    e
                );
                return;
            }
        };

        let write_result = self.account_writer.write(&output_bytes);

        if write_result.is_err() {
            panic!(
                "Couldn't write output bytes for account: '{}'. Error: {:?}",
                record.acc_no,
                write_result.err().expect("Unexpected Error occured.")
            );
        }

        // Write the index for this account:
        self.account_index.set_key(record.acc_no.to_string());
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

impl AccountWithoutCashflows {
    pub fn close(self) {
        flush_contents(self.account_writer, "AccountWithoutCashflows")
    }
}

fn flush_contents(mut writer: BufWriter<File>, writer_name: &str) {
    let flush_result = writer.flush();
    if flush_result.is_err() {
        panic!(
            "Program failed when flushing contents of writer: '{}'. Error: {}. Aborting execution.",
            writer_name,
            flush_result
                .err()
                .expect("Unexpected error occured while flushing the buffer contents.")
        );
    }
}
