use std::fmt;

use serde::export::Formatter;
use serde::{Deserialize, Serialize};

use ssh2::Sftp;

use process::io::read_remote_json;

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct BatchesInfo {
    pub num_of_batches: i32,
    pub all_batch_path: String,
    pub batches: Vec<Batch>,
}
impl BatchesInfo {
    pub fn from_remote_file(file_path: &str, sftp_client: &Sftp) -> BatchesInfo {
        let json_string = read_remote_json(file_path, sftp_client);
        let error_msg = format!("unable to parse `{}` to BatchesInfo", file_path);
        let account: BatchesInfo =
            serde_json::from_str(json_string.as_str()).expect(error_msg.as_str());
        account
    }
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Batch {
    pub batch_name: String,
    pub batch_id: i32,
    pub last_modified: String,
    pub batch_details_path: String,
}
impl fmt::Display for Batch {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}: {}",
            self.batch_id, self.batch_name, self.last_modified
        )
    }
}
