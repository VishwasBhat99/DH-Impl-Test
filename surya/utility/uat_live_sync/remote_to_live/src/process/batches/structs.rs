use core::fmt;
use std::fmt::Formatter;

use serde::{Deserialize, Serialize};

use ssh2::Sftp;
use process::io::read_remote_json;

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct BatchDetails {
    pub batch_name: String,
    pub batch_id: i32,
    pub num_of_streams: i32,
    pub batch_details_path: String,
    pub streams: Vec<Stream>,
}
impl BatchDetails {
    pub fn from_remote_file(file_path: &str, sftp_client: &Sftp) -> BatchDetails {
        let json_string = read_remote_json(file_path, sftp_client);
        let error_msg = format!("unable to parse `{}` to BatchDetails", file_path);
        let account: BatchDetails =
            serde_json::from_str(json_string.as_str()).expect(error_msg.as_str());
        account
    }
}
#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Stream {
    pub stream_name: String,
    pub stream_id: i32,
    pub last_modified: String,
    pub stream_details_path: String,
}
impl fmt::Display for Stream {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}: {}",
            self.stream_id, self.stream_name, self.last_modified
        )
    }
}
