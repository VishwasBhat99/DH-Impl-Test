use aggregator::llg_key::LLGKey;
use aggregator::organize::Cashflow;
use chrono::NaiveDate;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
#[derive(Debug)]
pub struct Writers {
    pub writer_smry: File,
    pub writer_aggr_pa: File,
    pub writer_aggr_rate: File,
}

pub fn create_writer_for_path(output_path: &str) -> Writers {
    let output_path_smry = output_path.to_string() + "_smry.txt";
    let output_path_pa = output_path.to_string() + "_principal_amt.txt";
    let output_path_rate = output_path.to_string() + "_rate.txt";

    let writer_smry = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(output_path_smry)
        .expect("Cannot create Summary Output File");

    let writer_aggr_pa = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(output_path_pa)
        .expect("Cannot create Principal Output File");

    let writer_aggr_rate = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(output_path_rate)
        .expect("Cannot create Rate Output File");

    Writers {
        writer_smry: writer_smry,
        writer_aggr_pa: writer_aggr_pa,
        writer_aggr_rate: writer_aggr_rate,
    }
}

pub fn write_to_file(
    as_on: &NaiveDate,
    mut smry_file: &File,
    mut aggr_pa: &File,
    mut aggr_rate: &File,
    llg_key: &LLGKey,
    value_to_write: Cashflow,
    exrt: &f64,
) {
    let mut count = 0;
    let outstandig_amt = value_to_write.amt[0];
    let spread: f64 = if outstandig_amt == 0.0 {
        0.0
    } else {
        (value_to_write.spread) / outstandig_amt
    };

    let len = value_to_write.amt.len() - 1;
    let mut summary_rate = 0.0;
    if value_to_write.outstanding_amt != 0.0 {
        summary_rate = value_to_write.outstanding_rate / value_to_write.outstanding_amt
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
        value_to_write.outstanding_amt,
        summary_rate
    )
    .expect("summary file writing error");
    write!(
        aggr_pa,
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|",
        as_on.format("%d-%m-%Y"),
        llg_key.llg_id,
        llg_key.curr_code,
        exrt,
        llg_key.bm_id,
        llg_key.tenor,
        spread,
        llg_key.next_repricing_date,
        llg_key.rep_freq
    )
    .expect("aggregate principal amount writing error");

    for amt in value_to_write.amt.clone() {
        if count == 0 {
            count += 1;
            continue;
        }
        if count == len {
            write!(aggr_pa, "{}\n", amt).expect("aggregate amount file maount writing error");
        } else {
            write!(aggr_pa, "{}|", amt).expect("aggregate amount file maount writing error");
        }
        count += 1;
    }
    write!(
        aggr_rate,
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|",
        as_on.format("%d-%m-%Y"),
        llg_key.llg_id,
        llg_key.curr_code,
        exrt,
        llg_key.bm_id,
        llg_key.tenor,
        spread,
        llg_key.next_repricing_date,
        llg_key.rep_freq,
    )
    .expect("weighted average interest rate writing error");
    count = 0;
    for rt in value_to_write.weighted_rate {
        let rate = if value_to_write.amt[count] == 0.0 {
            0.0
        } else {
            rt / value_to_write.amt[count].clone()
        };
        if count == 0 {
            count += 1;
            continue;
        }
        if count == len {
            write!(aggr_rate, "{}\n", rate).expect("interest file interest rate writing error");
        } else {
            write!(aggr_rate, "{}|", rate).expect("interest file interest rate writing error");
        }
        count += 1;
    }
}
