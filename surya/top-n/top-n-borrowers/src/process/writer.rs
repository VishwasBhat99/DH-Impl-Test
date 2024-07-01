use chrono::NaiveDate;
use process::structs::*;
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

pub fn create_writer_for_path(output_path: &str) -> File {
    let writer = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(output_path)
        .expect("Cannot create Output File");

    writer
}

pub fn write_file(
    write: &mut File,
    as_on_date: &NaiveDate,
    country_cd: &String,
    cust_map: HashMap<String, CustVal>,
    amt_vec: Vec<f64>,
    log: &Logger,
) {
    for (cust_id, value) in cust_map.iter() {
        let amt = value
            .tot_amt_hcy
            .parse::<f64>()
            .expect("cannot convert amt while writing");
        if !amt_vec.contains(&amt) {
            info!(log, "Skipping Record: `{}` with amount: {}.", cust_id, amt);
            continue;
        }
        write!(
            write,
            "{}|{}|{}|{}|{}|{}|{}\n",
            country_cd,
            as_on_date.format("%d-%m-%Y"),
            cust_id,
            value.cust_name,
            value.cust_type,
            value.tot_amt_hcy,
            value.npa_class
        )
        .expect(&format!(
            "Error while writing file for cust_id : {} ",
            cust_id
        ));
    }
}
