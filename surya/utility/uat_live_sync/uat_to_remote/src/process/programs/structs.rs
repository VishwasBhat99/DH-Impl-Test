use colored::Colorize;
use core::fmt;
use process::helper::write_json;
use process::io::read_remote_json;
use serde::{Deserialize, Serialize};
use slog::Logger;
use ssh2::Sftp;
use std::fmt::Formatter;

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct ProgramDetails {
    pub stream_name: String,
    pub stream_id: i32,
    pub flow_name: String,
    pub program_name: String,
    pub program_id: i32,
    pub script_path: String,
    pub num_of_files: i32,
    pub program_detals_path: String,
    pub files: Vec<ResFile>,
}
impl ProgramDetails {
    pub fn display(&self) {
        println!("Stream name: {}", self.stream_name);
        println!("Flow name: {}", self.flow_name);
        println!("Program name: {}", self.program_name);
    }
    pub fn initiate_copy(&mut self, sftp_client: &Sftp, logger: &Logger) {
        for each_file in &mut self.files {
            each_file.upload_status = false
        }
        self.update_json(logger, sftp_client);

        for each_file in &mut self.files {
            each_file.initiate_copy(sftp_client)
        }
        self.update_json(logger, sftp_client);
    }
    pub fn from_remote_file(file_path: &str, sftp_client: &Sftp) -> ProgramDetails {
        let json_string = read_remote_json(file_path, sftp_client);
        let error_msg = format!("unable to parse `{}` to StreamDetails", file_path);
        let account: ProgramDetails =
            serde_json::from_str(json_string.as_str()).expect(error_msg.as_str());
        account
    }
    pub fn update_json(&self, logger: &Logger, sftp_client: &Sftp) {
        write_json(
            self.program_detals_path.as_str(),
            &self,
            logger,
            sftp_client,
        );
    }
}
#[derive(Deserialize, Serialize, Default, Debug)]
pub struct ResFile {
    pub file_name: String,
    pub file_id: i32,
    pub source_file: String,
    pub destination_file: String,
    pub upload_status: bool,
}
impl ResFile {
    pub fn initiate_copy(&mut self, sftp_client: &Sftp) {
        println!("Uploading {}", self.file_name);
        match sftp_client.copy_to_remote(
            self.source_file.as_str(),
            self.destination_file.as_str(),
            true,
        ) {
            Ok(_) => {
                println!(
                    "{}",
                    format!("Uploaded `{}` Successfully\n", self.file_name)
                        .as_str()
                        .green()
                        .bold()
                );
                self.upload_status = true;
            }
            Err(err) => {
                println!(
                    "{}",
                    format!("Unable to upload: `{}`", err).as_str().red().bold()
                );
                self.upload_status = false;
            }
        }
    }
}

impl fmt::Display for ResFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}  upload status: {}",
            self.file_id,
            self.file_name,
            self.upload_status.to_string().as_str().bold()
        )
    }
}
#[derive(Deserialize, Serialize, Default, Debug)]
pub struct EachFile {
    pub variable_name: String,
    pub file_location: String,
}
impl EachFile {
    pub fn from_another(each_file: &EachFile) -> EachFile {
        EachFile {
            variable_name: each_file.variable_name.clone(),
            file_location: each_file.file_location.clone(),
        }
    }
}
