pub fn get_output_line(
    fields: Vec<&str>,
    branch_code: &str,
    country_code: &str,
    lcr_category: &str,
    asset_class_desc: &str,
    ccod_flag: &str,
    fb_nfp: &str,
    final_mapping_lcr: &str,
) -> String {
    let mut output_line = String::new();
    output_line.push_str(fields[2].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[4].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[11].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[26].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[30].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[31].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[46].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(fields[49].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(branch_code);
    output_line.push_str("|");
    output_line.push_str(country_code);
    output_line.push_str("|");
    output_line.push_str(lcr_category);
    output_line.push_str("|");
    output_line.push_str(asset_class_desc);
    output_line.push_str("|");
    output_line.push_str(final_mapping_lcr);
    output_line.push_str("|");
    output_line.push_str(fields[8].trim_matches('"'));
    output_line.push_str("|");
    output_line.push_str(ccod_flag);
    output_line.push_str("|");
    output_line.push_str(fb_nfp);
    output_line.push_str("\n");

    output_line
}
