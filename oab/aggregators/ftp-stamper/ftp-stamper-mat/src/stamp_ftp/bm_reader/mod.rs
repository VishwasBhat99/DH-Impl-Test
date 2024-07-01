use chrono::NaiveDate;
use macros;
use rbdate::{increment_date_by_months, num_days_start_to_end};
use slog::Logger;
use stamp_ftp::bm_reader;
use stamp_ftp::bm_reader::bm_structure::BmMaster;
use stamp_ftp::io;
use statics::DEFAULT_INT;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;

pub mod bm_structure;
pub mod yieldrate_calc;

pub struct BMReader {
    file_reader: Lines<BufReader<File>>,
    log: Logger,
}

impl BMReader {
    pub fn new(src_file: &str, log: &Logger) -> bm_reader::BMReader {
        bm_reader::BMReader {
            file_reader: io::buf_reader(src_file).lines(),
            log: log.clone(),
        }
    }
}

impl Iterator for BMReader {
    type Item = BmMaster;

    fn next(&mut self) -> Option<BmMaster> {
        let mut correct_record_found = false;
        let mut next_bm: Option<BmMaster> = None;

        while !correct_record_found {
            let line_opt = self.file_reader.next();

            if let Some(line) = line_opt {
                if let Ok(underlying_string) = line {
                    let parse_result = BmMaster::new(underlying_string);

                    if parse_result.is_err() {
                        log_error!(
                            self.log,
                            "Couldn't parse benchmark: {}",
                            parse_result.err().unwrap()
                        );
                        continue;
                    }

                    next_bm = Some(parse_result.expect("Error in parsing next bm"));
                    correct_record_found = true;
                } else {
                    // This line contains an erroneous string.
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

        next_bm
    }
}

pub fn get_bm_points(
    bm_file_path: &str,
    cpd: NaiveDate,
    log: &Logger,
) -> Vec<IntermediateBmPoints> {
    let reader = BMReader::new(bm_file_path, log);
    let mut lst_bm: Vec<BmMaster> = Vec::new();

    let mut reader_iter = reader.into_iter();
    loop {
        let bm = reader_iter.next();

        if bm.is_none() {
            break;
        }

        lst_bm.push(bm.expect("Error in reading lst_bm"));
    }

    IntermediateBmPoints::get_intermediate_bm_points(&mut lst_bm, cpd, log)
}

#[derive(Debug)]
pub struct IntermediateBmPoints {
    pub vertex: i64,
    pub uom: String,
    pub rate: f64,
    pub days_diff: i64,
    pub month: i64,
}

impl IntermediateBmPoints {
    pub fn get_intermediate_bm_points(
        lst_bm: &mut Vec<bm_structure::BmMaster>,
        cpd: NaiveDate,
        log: &Logger,
    ) -> Vec<IntermediateBmPoints> {
        let mut lst_bm_inter: Vec<IntermediateBmPoints> = Vec::new();

        for index in lst_bm.iter_mut() {
            let inter_uom = &index.uom;
            let inter_bm = IntermediateBmPoints {
                vertex: index.vertex,
                uom: inter_uom.to_string(),
                rate: index.rate,
                days_diff: get_days_diff(cpd, inter_uom.to_string(), index.vertex, log),
                month: (get_days_diff(cpd, inter_uom.to_string(), index.vertex, log) as f64 / 365.0
                    * 12.0)
                    .round() as i64,
            };
            lst_bm_inter.push(inter_bm);
        }

        lst_bm_inter
    }
}

pub fn get_days_diff(date: NaiveDate, uom: String, vertex: i64, log: &Logger) -> i64 {
    match uom.as_str() {
        "D" => vertex,
        "M" => {
            let end_date = increment_date_by_months(date, vertex as u16);
            num_days_start_to_end(date, end_date)
        }
        "Y" => {
            let end_date = increment_date_by_months(date, (vertex * 12) as u16);
            num_days_start_to_end(date, end_date)
        }
        _ => {
            log_error!(log, "Invalid UOM. Value: {:?}", uom);
            DEFAULT_INT
        }
    }
}
