use rbdate::NaiveDate;

pub fn split_by_defesance(
    fields: &Vec<&str>,
    acc_num: String,
    amt: f64,
    mat_date: String,
    nxt_rep_date: String,
    op_line: &mut String,
    as_on_date: NaiveDate,
) {
    let mut field_num = 1;
    let mut maturity_date = NaiveDate::parse_from_str(&mat_date, "%d-%m-%Y")
        .expect("Cannot parse maturity date in defesance.");
    let base_date = match NaiveDate::parse_from_str(fields[28], "%d-%m-%Y") {
        Ok(val) => val,
        Err(_err) => as_on_date,
    };
    let max_mat_date: NaiveDate = base_date + chrono::Duration::days(90);
    if maturity_date > max_mat_date {
        maturity_date = max_mat_date;
    }
    for field in fields {
        if field_num == 48 {
            op_line.push_str(field);
        } else if field_num == 17 {
            op_line.push_str(&amt.to_string());
            op_line.push('|');
        } else if field_num == 22 {
            op_line.push_str(&maturity_date.format("%d-%m-%Y").to_string());
            op_line.push('|');
        } else if field_num == 3 {
            op_line.push_str(&nxt_rep_date);
            op_line.push('|');
        } else if field_num == 1 {
            op_line.push_str(&acc_num);
            op_line.push('|');
        } else {
            op_line.push_str(field);
            op_line.push('|');
        }
        field_num += 1;
    }
}
