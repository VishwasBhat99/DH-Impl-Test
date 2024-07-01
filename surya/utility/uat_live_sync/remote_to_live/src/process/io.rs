use ssh2::Sftp;
use std::fs::File;
use std::io::Read;

pub fn read_json(path: &str) -> serde_json::Value {
    let error_msg = format!(
        "unable to read {}, file not found/incorrect json format",
        path
    );
    let config_json: serde_json::Value =
        serde_json::from_reader(File::open(path).expect(error_msg.as_str()))
            .expect("unable to parse json");
    config_json
}

pub fn read_input(console: &str) -> String {
    let buffer = rprompt::prompt_reply_stdout(console).expect("unable to get input");
    let choice = buffer.trim().to_string();
    choice
}

pub fn read_remote_json(file_path: &str, sftp_client: &Sftp) -> String {
    let error_msg = format!("unable to read `{}`", file_path);
    let mut reader = sftp_client
        .open(file_path.as_ref())
        .expect(error_msg.as_str());
    let mut json_string = String::new();
    let _bytes = reader
        .read_to_string(&mut json_string)
        .expect(error_msg.as_str());
    json_string
}
