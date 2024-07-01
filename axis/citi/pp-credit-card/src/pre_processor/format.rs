use rbdate::NaiveDate;
pub fn get_op_line(
    input_account: Vec<&str>,
    npa_classification: String,
    npa_amt: f64,
    cust_health_code: String,
    cust_npa_class: String,
    final_npa_class: String,
) -> String {
    let outstanding_bal_inr = if final_npa_class != *"0" {
        npa_amt
    } else {
        input_account[3].parse::<f64>().unwrap_or(0.0)
    };
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
        input_account[0],
        input_account[1],
        NaiveDate::parse_from_str(input_account[2], "%d-%m-%Y")
            .unwrap_or(NaiveDate::from_ymd(1900, 1, 1))
            .format("%d-%m-%Y"),
        outstanding_bal_inr,
        input_account[4],
        input_account[5],
        input_account[6],
        input_account[7],
        input_account[8],
        input_account[9],
        input_account[10],
        NaiveDate::parse_from_str(input_account[11], "%d-%m-%Y")
            .unwrap_or(NaiveDate::from_ymd(1900, 1, 1))
            .format("%d-%m-%Y"),
        NaiveDate::parse_from_str(input_account[12], "%d-%m-%Y")
            .unwrap_or(NaiveDate::from_ymd(1900, 1, 1))
            .format("%d-%m-%Y"),
        input_account[13],
        input_account[14],
        npa_classification,
        npa_amt,
        cust_health_code,
        cust_npa_class,
        final_npa_class,
    )
}
