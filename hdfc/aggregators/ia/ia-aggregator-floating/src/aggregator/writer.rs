use aggregator::llg_key::LLGKey;
use aggregator::organize::Cashflow;
use chrono::NaiveDate;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

pub fn create_writer_for_path(output_path: &str) -> File {
    let output_path_smry = output_path.to_string() + "_smry.txt";

    let writer_smry = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(output_path_smry)
        .expect("error");

    writer_smry
}

pub fn write_to_file(
    as_on: &NaiveDate,
    mut smry_file: &File,
    llg_key: &LLGKey,
    value_to_write: Cashflow,
    exrt: &f64,
) {
    let outstandig_amt = value_to_write.amt;
    let weighted_avg_rate: f64;
    let spread: f64;
    if outstandig_amt == 0.0 {
        spread = 0.0;
    } else {
        spread = (value_to_write.spread) / outstandig_amt;
    }
    if value_to_write.amt == 0.0 {
        weighted_avg_rate = 0.0;
    } else {
        weighted_avg_rate = value_to_write.weighted_rate / (value_to_write.amt * exrt);
    }

    write!(
        smry_file,
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
        as_on.format("%d-%m-%Y"),
        llg_key.llg_id,
        llg_key.curr_code,
        exrt,
        llg_key.bm_id,
        llg_key.tenor,
        spread,
        llg_key.next_repricing_date,
        llg_key.rep_freq,
        value_to_write.amt,
        weighted_avg_rate
    )
    .expect("summary file writing error");
}
