use crate::cashflow_derivator::convert_datatype::*;
use crate::cashflow_derivator::{Data, Key};
use rbdate::NaiveDate;
use std::collections::HashMap;

pub fn get_output_map(
    file_type: &str,
    mut output_map: HashMap<Key, Data>,
    input_fields: Vec<&str>,
    classid: &i64,
) -> HashMap<Key, Data> {
    match file_type {
        "WD" => {
            let key = Key {
                acc_no: input_fields[0].to_string(),
            };
            let data = Data {
                custid: str_to_int(input_fields[1]),
                classid: *classid,
                curr: input_fields[3].to_string(),
                mat_date: NaiveDate::parse_from_str(input_fields[5], "%d-%m-%Y").unwrap(),
                tot_amt: str_to_flt(input_fields[7]),
                tot_nwd_amt: 0.0,
            };
            output_map.insert(key, data);
        }
        "NWD" => {
            let key = Key {
                acc_no: input_fields[0].to_string(),
            };
            let data = Data {
                custid: str_to_int(input_fields[1]),
                classid: *classid,
                curr: input_fields[3].to_string(),
                mat_date: NaiveDate::parse_from_str(input_fields[5], "%d-%m-%Y").unwrap(),
                tot_amt: str_to_flt(input_fields[7]),
                tot_nwd_amt: str_to_flt(input_fields[7]),
            };
            output_map.insert(key, data);
        }
        _ => {
            println!("Invalid File Type\n");
        }
    }
    output_map
}
