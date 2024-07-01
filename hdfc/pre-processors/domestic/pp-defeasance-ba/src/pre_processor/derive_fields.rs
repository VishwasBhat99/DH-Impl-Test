use calamine::DataType;
use rbdate::{datevalue_to_naive_date, NaiveDate};

pub fn get_op_line(rec: &[DataType], trade_src: &str) -> String {
    let mut out_lines = String::new();
    out_lines.push_str(
        &get_date(&rec[0].to_string(), &rec[1].to_string())
            .format("%d-%m-%Y")
            .to_string(),
    );
    for index in 1..5 {
        out_lines.push('|');
        out_lines.push_str(&rec[index].to_string());
    }
    out_lines.push('|');
    out_lines.push_str(
        &get_date(&rec[5].to_string(), &rec[1].to_string())
            .format("%d-%m-%Y")
            .to_string(),
    );
    for index in 6..8 {
        out_lines.push('|');
        out_lines.push_str(&rec[index].to_string());
    }
    out_lines.push('|');
    out_lines.push_str(trade_src);
    out_lines.push('\n');

    out_lines
}

fn get_date(date: &str, isin: &str) -> NaiveDate {
    match NaiveDate::parse_from_str(date, "%d-%m-%Y") {
        Ok(dt) => dt,
        Err(_) => match datevalue_to_naive_date(&date) {
            Ok(dt) => dt,
            Err(error) => panic!("{} for isin: `{}`.", error, isin),
        },
    }
}
