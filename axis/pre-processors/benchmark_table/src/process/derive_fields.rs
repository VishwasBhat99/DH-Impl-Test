pub fn get_op_line(
    acid: String,
    foracid: String,
    pegged_flag: String,
    repricing_plan: String,
    peg_review_date: String,
) -> String {
    let rep_plan = match pegged_flag.as_str() {
        "" => "N".to_string(),
        _ => pegged_flag,
    };
    let float_type = match repricing_plan.as_str() {
        "M" => "MCFLT".to_string(),
        _ => "BFLT".to_string(),
    };

    let op_str = format!(
        "{}|{}|{}|{}|{}\n",
        acid, foracid, rep_plan, float_type, peg_review_date
    );
    op_str
}
