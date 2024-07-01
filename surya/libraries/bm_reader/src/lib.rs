mod calculations;
mod structs;
mod tests;

pub use calculations::*;
pub use structs::*;

use rbdate::*;
use sdb_io::new_buf_rdr;
use std::{
    collections::HashMap,
    convert::AsRef,
    fs::File,
    io::{BufRead, BufReader, Error, Lines},
};

pub struct BMReader {
    file_reader: Lines<BufReader<File>>,
}

impl BMReader {
    pub fn new(bm_file_path: &str) -> BMReader {
        BMReader {
            file_reader: new_buf_rdr(bm_file_path)
                .unwrap_or_else(|_| {
                    panic!("Error while opening benchmark file: `{}`.", bm_file_path)
                })
                .lines(),
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
                    if let Ok(result) = BmMaster::new(underlying_string) {
                        next_bm = Some(result);
                        correct_record_found = true;
                    } else {
                        continue;
                    }
                }
            } else {
                return None;
            }
        }
        next_bm
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct BMKey {
    pub date: String,
    pub base_curve_id: i32,
}

pub type IntermediateBMPoints = Vec<IntermediateBMPoint>;
pub type IntermediateBMPointsMap = HashMap<BMKey, IntermediateBMPoints>;

pub fn get_bm_points(
    bm_file_path: &str,
    curve_pick_date: NaiveDate,
    mut lst_bm_inter: &mut IntermediateBMPoints,
) {
    let reader = BMReader::new(bm_file_path);
    let mut lst_bm: Vec<BmMaster> = Vec::new();

    for bm in reader {
        lst_bm.push(bm);
    }

    IntermediateBMPoint::get_intermediate_bm_points(
        &mut lst_bm,
        curve_pick_date,
        &mut lst_bm_inter,
    );
}
