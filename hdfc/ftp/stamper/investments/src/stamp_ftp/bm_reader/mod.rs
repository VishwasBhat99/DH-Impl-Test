use chrono::NaiveDate;
use macros;
use rbdate;
use slog::Logger;
use stamp_ftp::bm_reader;
use stamp_ftp::bm_reader::bm_structure::BmMaster;
use stamp_ftp::io;
use std::collections::HashMap;
use std::convert::AsRef;
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

                    next_bm = Some(parse_result.unwrap());
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

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct BmKey {
    pub date: String,
    pub base_curve_id: i32,
}

pub fn get_new_bm_points(
    saved_bm_rates: &mut HashMap<BmKey, Vec<IntermediateBmPoints>>,
    bm_key: BmKey,
) -> &Vec<IntermediateBmPoints> {
    saved_bm_rates.get(&bm_key).unwrap()
}

pub fn get_bm_points<'a>(
    bm_file_path: &'a str,
    cpd: NaiveDate,
    log: &Logger,
    mut lst_bm_inter: &mut Vec<IntermediateBmPoints>,
) {
    let reader = BMReader::new(bm_file_path, log);
    let mut lst_bm: Vec<BmMaster> = Vec::new();

    let mut reader_iter = reader.into_iter();
    loop {
        let bm = reader_iter.next();

        if bm.is_none() {
            break;
        }

        let unwrapped_bm = bm.unwrap();

        lst_bm.push(unwrapped_bm);
    }

    IntermediateBmPoints::get_intermediate_bm_points(&mut lst_bm, cpd, log, &mut lst_bm_inter);
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntermediateBmPoints {
    pub vertex: i64,
    pub uom: String,
    pub rate: f64,
    pub days_diff: i64,
    pub month: i64,
}

impl<'a> IntermediateBmPoints {
    pub fn get_intermediate_bm_points(
        lst_bm: &mut Vec<bm_structure::BmMaster>,
        cpd: NaiveDate,
        log: &Logger,
        lst_bm_inter: &mut Vec<IntermediateBmPoints>,
    ) {
        for i in lst_bm.iter_mut() {
            let inter_uom = &i.uom;
            let inter_bm = IntermediateBmPoints {
                vertex: i.vertex,
                uom: inter_uom.to_string(),
                rate: i.rate,
                days_diff: get_days_diff(cpd, inter_uom.to_string(), i.vertex),
                month: (get_days_diff(cpd, inter_uom.to_string(), i.vertex) as f64 / 365.0 * 12.0)
                    .round() as i64,
            };

            lst_bm_inter.push(inter_bm);
        }
    }
}

pub fn get_days_diff(date: NaiveDate, uom: String, vertex: i64) -> i64 {
    let days_diff;

    match uom.as_ref() {
        "D" => {
            days_diff = vertex;
        }
        "M" => {
            let end_date = rbdate::increment_date_by_months(date, vertex as u16);
            days_diff = rbdate::num_days_start_to_end(date, end_date);
        }
        "Y" => {
            let end_date = rbdate::increment_date_by_months(date, (vertex * 12) as u16);
            days_diff = rbdate::num_days_start_to_end(date, end_date);
        }
        _ => days_diff = 0,
    }

    days_diff
}
