use chrono::NaiveDate;
use rbdate;
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
    cust_id: &String,
    acc_no: &String,
    ost_prin_amt: &f64,
    int_rt: &f64,
    mat_dt: &i64,
    acc_start_dt: &i64,
    ccy: &String,
    prod_type: &String,
    as_on_date: &NaiveDate,
    amt_hcy: &f64,
    country_cd: &String,
) {
    write!(
        write,
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
        country_cd,
        as_on_date.format("%d-%m-%Y"),
        cust_id,
        ccy,
        acc_no,
        prod_type,
        prod_type,
        prod_type,
        amt_hcy,
        ost_prin_amt,
        int_rt,
        rbdate::date_from_timestamp(*acc_start_dt).format("%d-%m-%Y"),
        rbdate::date_from_timestamp(*mat_dt).format("%d-%m-%Y")
    )
    .expect(&format!(
        "Error while writing file for cust_id : {} and product : {}",
        cust_id, prod_type
    ));
}
