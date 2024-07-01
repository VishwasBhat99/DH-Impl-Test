pub fn get_output_line(fields: &[&str], balm_llg: &str, ia_llg: &str) -> String {
    let mut output_acc_info: String = String::new();

    for field in fields.iter() {
        output_acc_info.push_str(field);
        output_acc_info.push('|');
    }
    output_acc_info.push_str(balm_llg);
    output_acc_info.push('|');
    output_acc_info.push_str(ia_llg);
    output_acc_info.push_str("\n");

    output_acc_info
}
