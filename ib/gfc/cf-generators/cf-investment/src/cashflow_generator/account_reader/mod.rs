use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_reader::input_account_reader_report::InputAccountReaderReport;
use cashflow_generator::io;
use macros;
use rbdate::DateParser;
use slog::Logger;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;

pub mod input_account;
mod input_account_reader_report;

pub struct InputAccountReader {
    file_reader: Lines<BufReader<File>>,
    dmy_date_parser: DateParser,
    report: InputAccountReaderReport,
    log: Logger,
}

impl InputAccountReader {
    pub fn new(src_file: &str, log: &Logger) -> InputAccountReader {
        let dmy_date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        println!("inside input reader");
        InputAccountReader {
            file_reader: io::buf_reader(src_file).lines(),
            dmy_date_parser,
            report: InputAccountReaderReport::new(),
            log: log.clone(),
        }
    }
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
                            parse_result.err().unwrap()
                        );
                        continue;
                    }

                    next_account = Some(parse_result.unwrap());
                    self.report.increment_input_accounts_parsed_count();
                    correct_record_found = true;
                } else {
                    self.report.increment_malformed_lines_count();
                    log_error!(
                        self.log,
                        "Invalid string encountered in line. Value: {:?}",
                        line
                    );
                }
            } else {
                return None;
            }
        }

        next_account
    }
}
