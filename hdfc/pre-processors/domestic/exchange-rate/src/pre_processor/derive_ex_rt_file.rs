pub fn get_ex_rt_lines(val: &[&str]) -> String {
    let mut output_line = String::new();
    output_line.push_str(val[0]);
    output_line.push('|');
    output_line.push_str(val[1]);
    output_line.push('|');
    output_line.push_str(val[2]);
    output_line.push('\n');

    output_line
}

pub fn append_ccy(lines: &mut String, ccy: &str, lcy: &str, fcy: &str) {
    lines.push_str(&format!("{}|{}|{}\n", ccy, lcy, "1.0"));
    lines.push_str(&format!("{}|{}|{}\n", lcy, ccy, "1.0"));
    lines.push_str(&format!("{}|{}|{}\n", ccy, ccy, "1.0"));
    lines.push_str(&format!("{}|{}|{}\n", fcy, ccy, "1.0"));
    lines.push_str(&format!("{}|{}|{}\n", ccy, fcy, "1.0"));
}
