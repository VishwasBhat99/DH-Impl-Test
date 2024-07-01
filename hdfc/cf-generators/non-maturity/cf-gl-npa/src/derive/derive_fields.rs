pub fn get_output_line(fields: &[&str], bal: f64, ccy: &str) -> String {
    let mut output_acc_info: String = String::new();

    output_acc_info.push_str(fields[0]);
    output_acc_info.push('|');
    output_acc_info.push_str(&bal.to_string());
    output_acc_info.push('|');
    output_acc_info.push_str(fields[2]);
    output_acc_info.push('|');
    output_acc_info.push_str(ccy);
    output_acc_info.push_str("\n");

    output_acc_info
}
