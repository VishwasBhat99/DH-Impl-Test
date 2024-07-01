use calamine::{open_workbook_auto, Reader};
use std::fs;

pub fn read_excel_file(ref_file_path: String, sheet_name: String) -> Vec<String> {
    let mut ref_vec: Vec<Vec<String>> = Vec::new();
    let mut ref_excel =
        open_workbook_auto(ref_file_path).expect("Unable to open Reference Master File.");
    if let Some(Ok(range)) = ref_excel.worksheet_range(&sheet_name) {
        for row in range.rows() {
            let mut curr_row: Vec<String> = Vec::new();
            for words in row {
                curr_row.push(words.to_string());
            }
            ref_vec.push(curr_row);
        }
    }
    ref_vec.remove(0);
    let mut inr_irs_reg: Vec<String> = Vec::new();
    for values in ref_vec {
        if values.len() == 0 {
            continue;
        } else {
            inr_irs_reg.push(values[3].to_string());
        }
    }
    inr_irs_reg
}
// read text file
pub fn read_text_file(inr_irs_infile_path: String) -> Vec<Vec<String>> {
    let input_data = fs::read_to_string(inr_irs_infile_path).expect("Unable to read file");
    let mut inr_irs_input: Vec<Vec<String>> = Vec::new();
    let lines = input_data.split("\n");
    let delimeter_type = '|';
    for line in lines {
        if line.len() == 0 {
            continue;
        }
        let words = line.split(delimeter_type);
        let mut curr_row = Vec::new();
        for w in words {
            curr_row.push(w.to_string());
        }
        inr_irs_input.push(curr_row);
    }
    return inr_irs_input;
}
