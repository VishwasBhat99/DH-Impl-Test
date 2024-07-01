use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct FileData {
    pub input_file_path: String,
    pub output_file_path: String,
    pub cust_id_position: usize,
    pub input_field_delimiter: String,
}

#[derive(Deserialize)]

pub struct Files {
    pub files: Vec<FileData>,
}
