use rbdate::NaiveDate;
pub fn get_op_line(
    input_account: Vec<&str>,
    acid: String,
    cust_id: String,
    overdue_amt: f64,
    overdue_date: NaiveDate,
    duedate: NaiveDate,
    closing_bal: f64,
    npa_classification: String,
    cust_hlth_code: String,
    cust_npa_class: String,
    final_npa_class: String,
    description_type: String,
) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
        acid,
        cust_id,
        overdue_amt,
        overdue_date.format("%d-%m-%Y"),
        input_account[4],
        input_account[5],
        input_account[6],
        duedate.format("%d-%m-%Y"),
        input_account[8].parse::<f64>().unwrap_or(0.0),
        closing_bal,
        input_account[10].parse::<f64>().unwrap_or(0.0),
        input_account[11],
        input_account[12],
        npa_classification,
        cust_hlth_code,
        cust_npa_class,
        final_npa_class,
        description_type
    )
}
