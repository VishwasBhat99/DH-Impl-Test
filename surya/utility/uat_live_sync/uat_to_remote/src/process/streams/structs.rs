use core::fmt;
use process::batches::structs::Stream;
use process::helper::write_json;
use process::io::read_remote_json;
use serde::{Deserialize, Serialize};
use slog::Logger;
use ssh2::Sftp;
use std::fmt::Formatter;

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
    pub fn display(&self) {
        for each_batch in &self.programs {
            println!("{}", each_batch);
        }
    }
    pub fn update_json(&self, logger: &Logger, sftp_client: &Sftp) {
        write_json(
            self.stream_details_path.as_str(),
            &self,
            logger,
            sftp_client,
        );
    }
    pub fn get_initialised_stream_details(&mut self, curr_stream: &Stream) {
        self.stream_name = curr_stream.stream_name.clone();
        self.stream_id = curr_stream.stream_id;
        self.stream_details_path = curr_stream.stream_details_path.clone();
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

// // StreamDef Struct
// #[derive(Serialize, Deserialize, Debug, Default, Clone)]
// pub struct StreamDef {
//     pub streamName: String,
//     pub streamId: String,
//     pub flows: Vec<FlowDef>,
// }
//
// // FlowDef Struct
// #[derive(Serialize, Deserialize, Debug, Default, Clone)]
// pub struct FlowDef {
//     pub name: String,
//     pub flowId: String,
//     pub flowDependencies: Vec<String>,
//     pub executorID: String,
//     pub process: Vec<ProcDef>,
// }
//
// // ProcDef struct
// #[derive(Serialize, Deserialize, Debug, Default, Clone)]
// pub struct ProcDef {
//     pub processName: String,
//     pub processId: String,
//     pub processBinary: String,
//     pub processArguments: Vec<String>,
//     pub processDependencies: Vec<String>,
//     pub processReport: String,
// }
