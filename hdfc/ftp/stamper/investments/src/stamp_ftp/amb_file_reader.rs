use calamine::{open_workbook_auto, Reader};
use statics::DEFAULT_FLOAT;
use std::collections::HashMap;

#[derive(Debug)]
pub struct AvgBalances {
    pub avg_balance: f64,
    pub accr_int: f64,
}

pub fn read_avg_bal(amb_file_path: &str, amb_sheet: &str) -> HashMap<String, AvgBalances> {
    let mut avg_bal: HashMap<String, AvgBalances> = HashMap::new();
    let mut lcr_master_excel =
        open_workbook_auto(amb_file_path).expect("Unable to open lcr Master File.");
    if let Some(Ok(reader)) = lcr_master_excel.worksheet_range(amb_sheet) {
        for row in reader.rows().skip(1) {
            let avg_balances = AvgBalances {
                avg_balance: row[13].to_string().parse::<f64>().unwrap_or(DEFAULT_FLOAT),
                accr_int: row[15].to_string().parse::<f64>().unwrap_or(DEFAULT_FLOAT),
            };
            avg_bal.insert(row[1].to_string(), avg_balances);
        }
    }
    avg_bal
}
