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
    ia_line: &str,
    nsfr_map: &str,
) -> String {
    let mut output_acc_info: String = String::new();
    output_acc_info.push_str(&fields[0].to_string());
    output_acc_info.push('|');
    output_acc_info.push_str("source");
    output_acc_info.push('|');
    output_acc_info.push_str("product_code");
    output_acc_info.push('|');
    output_acc_info.push_str("category");
    output_acc_info.push('|');
    output_acc_info.push_str("mis1");
    output_acc_info.push('|');
    output_acc_info.push_str(ccy);
    output_acc_info.push('|');
    output_acc_info.push_str(&fields[6].to_string());
    output_acc_info.push('|');
    output_acc_info.push_str(dr_bal.to_string().as_str());
    output_acc_info.push('|');
    output_acc_info.push_str(cr_bal.to_string().as_str());
    output_acc_info.push('|');
    output_acc_info.push_str(net_bal.to_string().as_str());
    output_acc_info.push('|');
    output_acc_info.push_str("concat");
    output_acc_info.push('|');
    output_acc_info.push_str(&fields[6].to_string());
    output_acc_info.push('|');
    output_acc_info.push_str(DEFAULT_FLOAT.to_string().as_str());
    output_acc_info.push('|');
    output_acc_info.push_str(c_typ);
    output_acc_info.push('|');
    output_acc_info.push_str(
        &fields[0]
            .to_string()
            .chars()
            .next()
            .unwrap_or('0')
            .to_string()
            .as_str(),
    );
    output_acc_info.push('|');
    output_acc_info.push_str(is_acc_gl);
    output_acc_info.push('|');
    output_acc_info.push_str(ia_line);
    output_acc_info.push('|');
    output_acc_info.push_str(nsfr_map);
    output_acc_info.push_str("\n");

    output_acc_info
}
