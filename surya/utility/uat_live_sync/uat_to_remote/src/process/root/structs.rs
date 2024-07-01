use process::io::{read_remote_json};
use serde::{Deserialize, Serialize};
use ssh2::Sftp;

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct Root {
    pub bank_name: String,
    pub batch_info_path: String,
}

impl Root {
    pub fn from_remote_file(file_path: &str, sftp_client: &Sftp) -> Root {
        let json_string = read_remote_json(file_path, sftp_client);
        let error_msg = format!("unable to parse `{}` to LoginDetails", file_path);
        let account: Root = serde_json::from_str(json_string.as_str()).expect(error_msg.as_str());
        account
    }
}
