use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct MethodField {
    pub id: String,
    pub curve_pick_date: String,
    pub tenor_start_date: String,
    pub tenor_end_date: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MethodFields {
    pub method_fields: Vec<MethodField>,
}

impl MethodField {
    pub fn new_from_path(path: &str) -> MethodFields {
        let mut file = open_file(path);
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string");
        let req_fields: MethodFields = serde_json::from_str(&buf[..])
            .expect("Method metadata json file was not well-formatted.");
        req_fields
    }
}
