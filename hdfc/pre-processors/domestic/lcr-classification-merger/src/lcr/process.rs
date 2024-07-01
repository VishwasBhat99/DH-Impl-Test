use crate::lcr::*;

pub fn append_data(line: &str,lcr_map:&mut HashMap<String, String>) -> String{
    let input_vec: Vec<&str> = line.split('|').collect();
    let mut output_line: String = String::new();
    if lcr_map.contains_key(&input_vec[0].to_string()){
        let lcr_val = lcr_map.get(&input_vec[0].to_string()).unwrap();               
        output_line.push_str(&input_vec[0].to_string());
        output_line.push('|');
        output_line.push_str(&input_vec[1].to_string());
        output_line.push('|');
        output_line.push_str(&lcr_val.to_string());
        output_line.push('\n');
    }       
    output_line
}
