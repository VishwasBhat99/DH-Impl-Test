use chrono::Datelike;
use chrono::{NaiveDateTime, NaiveDate};

pub fn get_premat_op_line(output_line: &mut String, as_on_date: NaiveDate, rec: Vec<&str>) -> String {
    format!("{}|{}|{}|{}|||{}|{}|{}|{}|{}|{}|{}|||||||||||",
        &as_on_date.format("%d-%m-%Y").to_string(),
        &rec[0],
        &rec[3],
        &rec[5],
        &rec[6],
        &rec[7],
        &rec[8],
        &rec[9],
        &rec[10],
        &rec[11],
        &rec[4],

    )
}

pub fn get_renewal_op_line(output_line: &mut String, as_on_date: NaiveDate, rec: Vec<&str>) -> String {
    format!("{}|{}|{}|{}|||{}|{}|{}|{}||{}||||||||||||",
    &as_on_date.format("%d-%m-%Y").to_string(),
    &rec[0],
    &rec[1],
    &rec[3],
    &rec[6],
    &rec[8],
    &rec[7],
    &rec[4],
    &rec[5],
)
}
