use super::structs::{AggrKey, AggrValues};
use chrono::NaiveDate;
use std::io::prelude::*;
use std::{collections::HashMap, fs::File, io::BufWriter};

pub enum OutputType {
    Weighted,
    Average,
}

pub fn write_output(
    aggr_map: &HashMap<AggrKey, AggrValues>,
    as_on_date: &NaiveDate,
    dimid: String,
    rlgid: String,
    writer: &mut BufWriter<File>,
    output_type: OutputType,
) {
    let ason_str = as_on_date.format("%d-%m-%Y");
    for (key, value) in aggr_map {
        let (int_rate, ftp_rate) = match output_type {
            OutputType::Weighted => (
                calculate_portion(value.sum_prod_int_rt_bal_amt, value.average_balance),
                calculate_portion(value.sum_prod_ftp_rt_bal_amt, value.average_balance),
            ),
            OutputType::Average => (
                calculate_portion(value.interest_amount, value.average_balance) * 1200.0,
                calculate_portion(value.ftp_amount, value.average_balance) * 1200.0,
            ),
        };

        let op_line = format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            ason_str,
            dimid,
            key.dim_item_id,
            rlgid,
            key.rlg_item_id,
            key.aorl,
            value.average_balance,
            int_rate,
            value.interest_amount,
            ftp_rate,
            value.ftp_amount
        );
        writeln!(writer, "{}", op_line).unwrap_or_else(|error| {
            panic!("Unable to write to the output file: {}", error);
        });
    }
}

fn calculate_portion(numerator: f64, denominator: f64) -> f64 {
    if denominator == 0.0 {
        0.0
    } else {
        numerator / denominator
    }
}
