#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub input_sheet_name: String,
    pub validate_flag: String,
    pub percent: f64,
    pub duration: String,
}
