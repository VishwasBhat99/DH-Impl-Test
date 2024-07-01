use rbdate::NaiveDate;

pub fn get_acc_level_data(
    op_line: &mut String,
    as_on_date: &NaiveDate,
    llg: String,
    acc_no: String,
    ccy: String,
    ccy_amt: &f64,
    hcy_amt: &f64,
    duration: &f64,
) {
    op_line.push_str(&as_on_date.to_string());
    op_line.push('|');
    op_line.push_str(&llg);
    op_line.push('|');
    op_line.push_str(&acc_no);
    op_line.push('|');
    op_line.push_str(&ccy);
    op_line.push('|');
    op_line.push_str(&ccy_amt.to_string());
    op_line.push('|');
    op_line.push_str(&hcy_amt.to_string());
    op_line.push('|');
    op_line.push_str(&duration.to_string());
    op_line.push('\n');
}
