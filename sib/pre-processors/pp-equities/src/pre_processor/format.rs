use rbdate::NaiveDate;

pub fn get_op_line(account: &[&str], gl_code: String) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
        NaiveDate::parse_from_str(account[0], "%d-%m-%Y")
            .unwrap_or(NaiveDate::from_ymd(1900, 1, 1))
            .format("%d-%m-%Y"),
        account[1],
        account[2],
        account[3],
        NaiveDate::parse_from_str(account[4], "%d-%m-%Y")
            .unwrap_or(NaiveDate::from_ymd(1900, 1, 1))
            .format("%d-%m-%Y"),
        account[5].parse::<f64>().unwrap_or(0.0),
        account[6].parse::<f64>().unwrap_or(0.0),
        account[7],
        account[8],
        account[9],
        account[10],
        account[11],
        account[12],
        account[13],
        account[14],
        account[15],
        account[16],
        gl_code,
        account[18].parse::<f64>().unwrap_or(0.0),
        NaiveDate::parse_from_str(account[19], "%d-%m-%Y")
            .unwrap_or(NaiveDate::from_ymd(1900, 1, 1))
            .format("%d-%m-%Y"),
        NaiveDate::parse_from_str(account[20], "%d-%m-%Y")
            .unwrap_or(NaiveDate::from_ymd(1900, 1, 1))
            .format("%d-%m-%Y"),
        account[21],
    )
}
