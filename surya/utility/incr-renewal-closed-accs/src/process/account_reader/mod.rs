use macros;
use process::account_reader::input_account::InputAccount;
use process::account_reader::input_account_reader_report::InputAccountReaderReport;
use rbdate::DateParser;
use slog::Logger;
use std::fs::File;
use std::io::{BufReader, Lines};

pub mod input_account;
mod input_account_reader_report;

pub struct InputAccountReader {
    file_reader: Lines<BufReader<File>>,
    dmy_date_parser: DateParser,
    report: InputAccountReaderReport,
    log: Logger,
}

impl Iterator for InputAccountReader {
    type Item = InputAccount;

    fn next(&mut self) -> Option<InputAccount> {
        let mut correct_record_found = false;
        let mut next_account: Option<InputAccount> = None;
        while !correct_record_found {
            let line_opt = self.file_reader.next();

            if let Some(line) = line_opt {
                self.report.increment_total_lines_count();
                if let Ok(underlying_string) = line {
                    self.report.increment_well_formed_lines_count();
                    let parse_result =
                        InputAccount::new_from_line(underlying_string, &self.dmy_date_parser);

                    if parse_result.is_err() {
                        self.report.increment_input_accounts_not_parsed_count();
                        log_error!(
                            self.log,
                            "Couldn't parse InputAccount: {}",
                            parse_result
                                .expect_err("Unexpected error occured while reading records.")
                        );
                        continue;
                    }

                    next_account = Some(parse_result.expect("Error while reading next account."));
                    self.report.increment_input_accounts_parsed_count();
                    correct_record_found = true;
                } else {
                    // This line contains an erroneous string.

                    self.report.increment_malformed_lines_count();
                    log_error!(
                        self.log,
                        "Invalid string encountered in line. Value: {:?}",
                        line
                    );
                }
            } else {
                // EOF Reached.
                return None;
            }
        }

        next_account
    }
}
