use super::InputAccount;

pub fn get_op_line(
    acc: &InputAccount,
    mis2: &str,
    avg_bal: f64,
    is_acc_weaker: &str,
    ews_weaker_value: &str,
    two_point_concat: &str,
    weaker_desc: &String,
    bdp_division: &String,
    bdp_coa: &String,
) -> String {
    let mut op_line = String::new();
    op_line.push_str(&acc.print());
    op_line.push_str(mis2);
    op_line.push('|');
    op_line.push_str(&avg_bal.to_string());
    op_line.push('|');
    op_line.push_str(&is_acc_weaker);
    op_line.push('|');
    op_line.push_str(&ews_weaker_value);
    op_line.push('|');
    op_line.push_str(&acc.alm_concat);
    op_line.push('|');
    op_line.push_str(&two_point_concat);
    op_line.push('|');
    op_line.push_str(&weaker_desc);
    op_line.push('|');
    op_line.push_str(&bdp_division);
    op_line.push('|');
    op_line.push_str(&bdp_coa);
    op_line.push_str("\n");

    op_line
}
