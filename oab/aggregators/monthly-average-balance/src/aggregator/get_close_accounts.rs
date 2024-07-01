use super::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn get_closed_acc_nos(
    account_pool: &mut HashMap<String, Vec<f64>>,
    closed_acc_nos: &mut Vec<String>,
) {
    for (acc_no, mut bals) in account_pool.drain() {
        bals.dedup();
        let is_already_closed = bals.iter().all(|&bal| bal == 0.0);
        if let Some(last) = bals.last() {
            if *last == 0.0 && !is_already_closed {
                closed_acc_nos.push(acc_no);
            }
        }
    }
}

pub fn write_closed_accounts_data(
    closed_acc_nos: &mut Vec<String>,
    closed_acc_writer: &mut BufWriter<File>,
) {
    for acc_no in closed_acc_nos.iter() {
        write!(closed_acc_writer, "{}\n", acc_no)
            .expect("Error while writing accounts to the closed account file.");
    }
}
