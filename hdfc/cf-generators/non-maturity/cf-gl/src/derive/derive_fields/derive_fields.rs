use derive::output_lines::OutputLines;
use statics::DEFAULT_FLOAT;

pub fn get_output_line(
    fields: &[&str],
    output: &mut OutputLines,
    dr_bal: f64,
    cr_bal: f64,
    temp_concat: &str,
    alm_line: &str,
    c_typ: &str,
    is_acc_gl: &str,
    ccy: &str,
    net_bal: f64,
    ia_line: &str,
    nsfr: &str,
) {
    output.processed_lines.push_str(fields[0]);
    output.processed_lines.push('|');
    output.processed_lines.push_str(fields[1]);
    output.processed_lines.push('|');
    output.processed_lines.push_str(fields[2]);
    output.processed_lines.push('|');
    output.processed_lines.push_str(fields[3]);
    output.processed_lines.push('|');
    output.processed_lines.push_str(fields[4]);
    output.processed_lines.push('|');
    output.processed_lines.push_str(ccy);
    output.processed_lines.push('|');
    output.processed_lines.push_str(fields[6]);
    output.processed_lines.push('|');
    output.processed_lines.push_str(dr_bal.to_string().as_str());
    output.processed_lines.push('|');
    output.processed_lines.push_str(cr_bal.to_string().as_str());
    output.processed_lines.push('|');
    output
        .processed_lines
        .push_str(net_bal.to_string().as_str());
    output.processed_lines.push('|');
    output.processed_lines.push_str(temp_concat);
    output.processed_lines.push('|');
    output.processed_lines.push_str(alm_line);
    output.processed_lines.push('|');
    output
        .processed_lines
        .push_str(DEFAULT_FLOAT.to_string().as_str());
    output.processed_lines.push('|');
    output.processed_lines.push_str(c_typ);
    output.processed_lines.push('|');
    output
        .processed_lines
        .push_str(fields[0].chars().next().unwrap_or('0').to_string().as_str());
    output.processed_lines.push('|');
    output.processed_lines.push_str(is_acc_gl);
    output.processed_lines.push('|');
    output.processed_lines.push_str(ia_line);
    output.processed_lines.push('|');
    output.processed_lines.push_str(nsfr);
    output.processed_lines.push_str("\n");

    if alm_line == "NONE" {
        output
            .concat_lines
            .push(get_concat_line(fields[1], fields[0], temp_concat));
    }
}

fn get_concat_line(acc: &str, gl: &str, alm_concat: &str) -> String {
    let mut op_line = String::new();
    op_line.push_str("GL|");
    op_line.push_str(acc);
    op_line.push('|');
    op_line.push_str(gl);
    op_line.push('|');
    op_line.push_str(alm_concat);
    op_line
}
