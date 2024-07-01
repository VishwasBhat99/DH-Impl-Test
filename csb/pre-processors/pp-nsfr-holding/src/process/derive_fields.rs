use super::{InputData, SecDealData, SecDealMap};
use calamine::DataType;
pub use chrono::NaiveDateTime;

pub fn get_op_line(input_data: InputData, sec_data: &mut SecDealMap) -> String {
    let mut op_line = String::new();
    op_line.push_str(&input_data.print());
    let def_sec_data = SecDealData::new();
    let sec_deal_data = sec_data
        .store
        .entry(input_data.isin.to_string())
        .or_insert(def_sec_data);

    if sec_deal_data.mark_val == "NA" {
        op_line.push_str("0");
        op_line.push('|');
        op_line.push_str("0");
        op_line.push('|');
        op_line.push_str("0");
    } else {
        op_line.push_str(&sec_deal_data.mark_val);
        op_line.push('|');
        op_line.push_str(&sec_deal_data.book_val);
        op_line.push('|');
        let book_val: f64 = sec_deal_data.book_val.parse().unwrap();
        let mark_val: f64 = sec_deal_data.mark_val.parse().unwrap();
        let face_val: f64 = input_data.face_value.parse().unwrap();
        let os_amt = face_val * (mark_val / book_val);
        op_line.push_str(&os_amt.to_string());
    }
    op_line.push('\n');
    op_line
}

pub fn get_sec_deal_data(sec_map: &mut SecDealMap, sec_deal_data: SecDealData) {
    sec_map
        .store
        .insert(sec_deal_data.isin_cd.to_string(), sec_deal_data);
}
