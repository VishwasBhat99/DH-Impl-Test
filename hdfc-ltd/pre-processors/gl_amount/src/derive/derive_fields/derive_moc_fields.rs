use calamine::DataType;
use statics::DEFAULT_FLOAT;

pub fn get_moc_output_line(
    fields: &[DataType],
    dr_bal: f64,
    cr_bal: f64,
    ccy: &str,
    c_typ: &str,
    is_acc_gl: &str,
    net_bal: f64,
    alm_line: &str,
    code_desc: &str,
    group_2: &str,
    group_3: &str,
    line: &str,
    prefix: &str,
) -> String {
    let mut output: String = String::new();
    output.push_str(&fields[0].to_string());
    output.push('|');
    output.push_str(&fields[1].to_string());
    output.push('|');
    output.push_str(dr_bal.to_string().as_str());
    output.push('|');
    output.push_str(cr_bal.to_string().as_str());
    output.push('|');
    output.push_str(net_bal.to_string().as_str());
    output.push('|');
    output.push_str(c_typ);
    output.push('|');
    output.push_str(ccy);
    output.push('|');
    output.push_str(is_acc_gl);
    output.push('|');
    output.push_str(alm_line);
    output.push('|');
    output.push_str(code_desc.to_string().as_str());
    output.push('|');
    output.push_str(group_2.to_string().as_str());
    output.push('|');
    output.push_str(group_3.to_string().as_str());
    output.push('|');
    output.push_str(line.to_string().as_str());
    output.push('|');
    output.push_str(prefix.to_string().as_str());
    output.push_str("\n");

    output
}
