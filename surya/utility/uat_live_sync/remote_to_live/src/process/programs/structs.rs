use core::fmt;
use std::fmt::Formatter;

use colored::Colorize;
use serde::{Deserialize, Serialize};
use slog::Logger;
use ssh2::Sftp;

use process::helper::{check_if_exist, set_curr_time_stamp, verify_if_different_timestamp};
use process::io::read_remote_json;

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
    pub fn initiate_copy(&mut self, sftp_client: &Sftp, _logger: &Logger) {
        for each_file in &mut self.files {
            each_file.initiate_copy(sftp_client)
        }
    }
    pub fn from_remote_file(file_path: &str, sftp_client: &Sftp) -> ProgramDetails {
        let json_string = read_remote_json(file_path, sftp_client);
        let error_msg = format!("unable to parse `{}` to StreamDetails", file_path)
            .as_str()
            .red()
            .bold()
            .to_string();
        let account: ProgramDetails =
            serde_json::from_str(json_string.as_str()).expect(error_msg.as_str());
        account
    }

}
#[derive(Deserialize, Serialize, Default, Debug)]
pub struct ResFile {
    pub file_name: String,
    pub file_id: i32,
    #[serde(rename(serialize = "destination_file", deserialize = "destination_file"))]
    pub source_file: String,
    #[serde(rename(serialize = "source_file", deserialize = "source_file"))]
    pub destination_file: String,
    pub upload_status: bool,
}
impl ResFile {
    pub fn initiate_copy(&mut self, sftp_client: &Sftp) {
        println!(
            "{}",
            format!("Downloading {}", self.file_name).as_str().bold()
        );
        if check_if_exist(self.destination_file.as_str())
            && !verify_if_different_timestamp(self.destination_file.as_str())
        {
            println!(
                "{}\n",
                format!(
                    "info: Skipping File `{}` as it already exists\n",
                    self.file_name
                )
                .as_str()
                .blue()
                .bold()
            );
            return;
        }
        match sftp_client.copy_from_remote(
            self.source_file.as_str(),
            self.destination_file.as_str(),
            true,
        ) {
            Ok(_) => {
                println!(
                    "{}",
                    format!("Downloaded `{}` Successfully\n", self.file_name)
                        .as_str()
                        .green()
                        .bold()
                );
                set_curr_time_stamp(self.destination_file.as_str());
            }
            Err(err) => {
                println!(
                    "{}",
                    format!("Unable to Download: `{}`", err)
                        .as_str()
                        .red()
                        .bold()
                );
            }
        }
    }
}

impl fmt::Display for ResFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.file_id, self.file_name)
    }
}
#[derive(Deserialize, Serialize, Default, Debug)]
pub struct EachFile {
    pub variable_name: String,
    pub file_location: String,
}
