use std::{collections::HashMap, path::Path};
pub fn get_file_id(
    input_file: String,
    master_map: &HashMap<String, String>,
    cust_id: &String,
) -> i32 {
    let path = Path::new(&input_file);
    let file_name = path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .replace(".cf", "");
    let input_file_class_id: i32 = file_name.split("-").collect::<Vec<&str>>()[1]
        .parse()
        .unwrap_or(99);
    let default_claim_id = (input_file_class_id%100).to_string();
    let claim_class_id = master_map.get(cust_id).unwrap_or(&default_claim_id);
    let mut new_file_id = (input_file_class_id / 100).to_string();
    new_file_id.push_str(&claim_class_id);
    new_file_id.parse::<i32>().unwrap_or(99)
}
