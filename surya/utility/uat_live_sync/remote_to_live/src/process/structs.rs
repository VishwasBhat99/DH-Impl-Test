use serde::{Deserialize, Serialize};


use process::io::read_json;

#[derive(Deserialize, Debug, Serialize)]
pub struct LoginDetails {
    pub username: String,
    pub password: String,
    pub address: String,
}

impl LoginDetails {
    pub fn from_file(file_path: &str) -> LoginDetails {
        let json_value = read_json(file_path);
        let error_msg = format!("unable to parse `{}` to LoginDetails", file_path);
        let account: LoginDetails = serde_json::from_value(json_value).expect(error_msg.as_str());
        account
    }
}
