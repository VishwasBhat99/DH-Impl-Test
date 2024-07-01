use core::fmt;
use std::fmt::Formatter;

use serde::{Deserialize, Serialize};

use ssh2::Sftp;
use process::io::read_remote_json;

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct StreamDetails {
    pub stream_name: String,
    pub stream_id: i32,
    pub num_of_programs: i32,
    pub stream_details_path: String,
    pub programs: Vec<Program>,
}
impl StreamDetails {
    pub fn from_remote_file(file_path: &str, sftp_client: &Sftp) -> StreamDetails {
        let json_string = read_remote_json(file_path, sftp_client);
        let error_msg = format!("unable to parse `{}` to StreamDetails", file_path);
        let account: StreamDetails =
            serde_json::from_str(json_string.as_str()).expect(error_msg.as_str());
        account
    }

}
#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Program {
    pub stream_name: String,
    pub stream_id: i32,
    pub program_name: String,
    pub flow_name: String,
    pub program_id: i32,
    pub last_modified: String,
    pub folder_path: String,
    pub program_details_path: String,
}
impl fmt::Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}: {}",
            self.program_id, self.program_name, self.last_modified
        )
    }
}
