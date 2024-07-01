pub fn get_output_line(
    fields: Vec<&str>,
    fb_nfp: &str,
    ccod_flag: &str,
    lcr_category: &str,
    asset_class_desc: &str,
    final_mapping_lcr: &str,
    sls_amount: f64,
    lcr_amount: f64,
) -> String {
    let mut output_line = String::new();
    output_line.push_str(fields[0].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[1].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[2].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[4].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[6].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[7].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[11].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[20].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[21].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[24].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[26].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[28].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[29].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[30].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[31].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[46].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[47].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[48].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[49].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[50].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[52].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fb_nfp);
    output_line.push_str("|");
    output_line.push_str(ccod_flag);
    output_line.push_str("|");
    output_line.push_str(lcr_category);
    output_line.push_str("|");
    output_line.push_str(asset_class_desc);
    output_line.push_str("|");
    output_line.push_str(final_mapping_lcr);
    output_line.push_str("|");
    output_line.push_str(fields[8].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(&sls_amount.to_string());
    output_line.push_str("|");
    output_line.push_str(&lcr_amount.to_string());
    output_line.push_str("\n");

    output_line
}
