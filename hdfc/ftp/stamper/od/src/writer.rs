use slog::Logger;
use stamp_ftp::io;
use std::fs::File;
use std::io::BufWriter;

pub struct AccountWithCashflowsWriter {
    log: Logger,
    account_w_cf_writer: BufWriter<File>,
}

impl AccountWithCashflowsWriter {
    pub fn new(path: &str, log: &Logger) -> AccountWithCashflowsWriter {
        AccountWithCashflowsWriter {
            account_w_cf_writer: io::buf_file_writer(path, ".FTPcf", None),
            log: log.clone(),
        }
    }
}
