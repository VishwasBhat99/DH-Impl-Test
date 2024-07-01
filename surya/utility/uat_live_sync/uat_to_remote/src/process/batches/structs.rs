use core::fmt;
use process::all_batches::structs::Batch;
use process::helper::write_json;
use process::io::{read_local_json, read_remote_json};
use serde::{Deserialize, Serialize};
use slog::Logger;
use ssh2::Sftp;
use std::fmt::Formatter;

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct BatchDetails {
    pub batch_name: String,
    pub batch_id: i32,
    pub num_of_streams: i32,
    pub batch_folder_path: String,
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
    pub fn display(&self) {
        for each_batch in &self.streams {
            println!("{}", each_batch);
        }
    }
    pub fn get_initialised_batch_details(&mut self, curr_batch: &Batch) {
        self.batch_name = curr_batch.batch_name.clone();
        self.batch_id = curr_batch.batch_id;
        self.batch_details_path = curr_batch.batch_details_path.clone();
    }
    pub fn update_json(&self, logger: &Logger, sftp_client: &Sftp) {
        write_json(self.batch_details_path.as_str(), &self, logger, sftp_client);
    }
}
#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Stream {
    pub stream_name: String,
    pub stream_id: i32,
    pub last_modified: String,
    pub stream_details_path: String,
    pub streamdef_file_path: String,
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

// StreamDef Struct
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct StreamDef {
    pub streamName: String,
    pub streamId: String,
    pub flows: Vec<FlowDef>,
}
impl StreamDef {
    pub fn from_file(file_path: &str, sftp_client: &Sftp, use_remote: bool) -> StreamDef {
        let json_string = match use_remote {
            true => read_remote_json(file_path, sftp_client),
            false => read_local_json(file_path),
        };

        let error_msg = format!("unable to parse `{}` to BatchDetails", file_path);
        let account: StreamDef =
            serde_json::from_str(json_string.as_str()).expect(error_msg.as_str());
        account
    }
}
// FlowDef Struct
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FlowDef {
    pub name: String,
    pub flowId: String,
    pub flowDependencies: Vec<String>,
    pub executorID: String,
    pub process: Vec<ProcDef>,
}

// ProcDef struct
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProcDef {
    pub processName: String,
    pub processId: String,
    pub processBinary: String,
    pub processArguments: Vec<String>,
    pub processDependencies: Vec<String>,
    pub processReport: String,
}
