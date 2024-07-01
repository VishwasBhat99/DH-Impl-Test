use super::configuration_parameters::ConfigurationParameters;
use super::reader;
use super::writer;
// process the data
pub fn process(config_params: ConfigurationParameters) {
    // only taking the 4th column in ref_data
    let ref_data = reader::read_excel_file(config_params.ref_file_path, config_params.sheet_name);
    let inr_irs_input = reader::read_text_file(config_params.inr_irs_infile_path);
    let mut inr_irs_nd: Vec<Vec<String>> = Vec::new();
    let mut inr_irs_prev: Vec<Vec<String>> = Vec::new();
    for curr_row in inr_irs_input {
        if ref_data.contains(&curr_row[25]) {
            inr_irs_nd.push(curr_row);
        } else {
            inr_irs_prev.push(curr_row);
        }
    }
    writer::write_txt_file(config_params.inr_irs_outfile_path, inr_irs_prev)
        .expect("inr_irs file can not be written");
    writer::write_txt_file(config_params.inr_irs_nd_file_path, inr_irs_nd)
        .expect("inr_irs_nd file can not be written");
}
