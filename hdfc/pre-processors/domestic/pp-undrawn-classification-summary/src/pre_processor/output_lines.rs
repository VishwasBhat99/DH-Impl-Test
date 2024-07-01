pub fn get_output_line(
    fields: Vec<&str>,
    lcr_category: &str,
    asset_class_desc: &str,
    final_mapping_lcr: &str,
    sls_amount: f64,
    lcr_amount: f64,
) -> String {
    let mut output_line = String::new();
    output_line.push_str(fields[0].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[6].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[16].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[17].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[26].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[28].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[31].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(lcr_category);
    output_line.push_str("|");
    output_line.push_str(asset_class_desc);
    output_line.push_str("|");
    output_line.push_str(final_mapping_lcr);
    output_line.push_str("|");
    output_line.push_str(fields[3].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(&sls_amount.to_string());
    output_line.push_str("|");
    output_line.push_str(&lcr_amount.to_string());
    output_line.push_str("|");
    output_line.push_str(fields[1].trim_matches('"'));
    output_line.push_str("\n");

    output_line
}
