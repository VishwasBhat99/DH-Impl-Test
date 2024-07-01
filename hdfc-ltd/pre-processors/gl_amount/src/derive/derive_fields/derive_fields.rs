use derive::output_lines::OutputLines;
use statics::DEFAULT_FLOAT;

pub fn get_output_line(
    fields: &[&str],
    output: &mut OutputLines,
    dr_bal: f64,
    cr_bal: f64,
    alm_line: &str,
    c_typ: &str,
    is_acc_gl: &str,
    ccy: &str,
    net_bal: f64,
    code_desc: &str,
    group_2: &str,
    group_3: &str,
    line: &str,
    prefix: &str,
) {
    output.processed_lines.push_str(fields[0]);
    output.processed_lines.push('|');
    output.processed_lines.push_str(fields[4]);
    output.processed_lines.push('|');
    output.processed_lines.push_str(dr_bal.to_string().as_str());
    output.processed_lines.push('|');
    output.processed_lines.push_str(cr_bal.to_string().as_str());
    output.processed_lines.push('|');
    output
        .processed_lines
        .push_str(net_bal.to_string().as_str());
    output.processed_lines.push('|');
    output.processed_lines.push_str(c_typ);
    output.processed_lines.push('|');
    output.processed_lines.push_str(ccy);
    output.processed_lines.push('|');
    output.processed_lines.push_str(is_acc_gl);
    output.processed_lines.push('|');
    output.processed_lines.push_str(alm_line);
    output.processed_lines.push('|');
    output.processed_lines.push_str(&code_desc);
    output.processed_lines.push('|');
    output.processed_lines.push_str(&group_2);
    output.processed_lines.push('|');
    output.processed_lines.push_str(&group_3);
    output.processed_lines.push('|');
    output.processed_lines.push_str(&line);
    output.processed_lines.push('|');
    output.processed_lines.push_str(&prefix);
    output.processed_lines.push_str("\n");
}
