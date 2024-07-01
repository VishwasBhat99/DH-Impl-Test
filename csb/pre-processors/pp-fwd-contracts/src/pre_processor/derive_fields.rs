use calamine::DataType;
use chrono::NaiveDate;
use rbdate::num_days_start_to_end;
use sdb_agg_rules_txt::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::Reader;
use std::fs::File;
use std::io::{BufWriter, Write};

//Reads method rules and produce the matched method for an account passed
#[allow(dead_code, unused_imports)]
pub fn get_value(account: &String, rules: &AggRules, reader: &Reader) -> String {
    let llg_id = match rules.llg_for_acc(account, reader) {
        Some(c) => c.llg,
        None => 0,
    };
    llg_id.to_string()
}

pub fn get_output(
    row: &[DataType],
    writer: &mut BufWriter<BufWriter<File>>,
    ason: NaiveDate,
    def_date: NaiveDate,
) {
    let mut op_line = String::new();
    for i in 0..26 {
        op_line.push_str(&row[i].to_string());
        op_line.push_str("|");
    }
    op_line.push_str("|");
    let val_dt = NaiveDate::parse_from_str(&row[9].to_string(), "%d-%b-%Y").unwrap_or(def_date);
    let resi_maturity_days = if val_dt > ason {
        num_days_start_to_end(ason, val_dt)
    } else {
        0
    };
    op_line.push_str(&resi_maturity_days.to_string());
    op_line.push_str(&"||||\n".to_string());
    writer.write_all(op_line.as_bytes()).unwrap();
}
