use super::{AggregateData, HashMap};
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn get_op_line(
    no_of_days: f64,
    bal_org: &mut HashMap<String, AggregateData>,
    mut ttl_amt: &mut f64,
    writer: &mut BufWriter<File>,
) {
    let mut op_line = String::new();
    append_to_string(&mut op_line, no_of_days, &mut ttl_amt, bal_org);
    write!(writer, "{}", op_line).expect("Error while writing to the output file.");
}

fn append_to_string(
    op_line: &mut String,
    no_of_days: f64,
    ttl_amt: &mut f64,
    bal_org: &mut HashMap<String, AggregateData>,
) {
    for aggr_data in bal_org.values_mut() {
        aggr_data.average(no_of_days);
        *ttl_amt += aggr_data.balance_ccy;
        op_line.push_str(&aggr_data.to_string())
    }
}
