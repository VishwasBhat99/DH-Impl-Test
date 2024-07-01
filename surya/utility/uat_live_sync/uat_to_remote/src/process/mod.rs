use configuration_parameters::ConfigurationParameters;

use process::structs::LoginDetails;
use slog::Logger;

mod all_batches;
mod batches;
mod helper;
mod io;
pub mod programs;
mod root;
mod streams;
mod structs;

use process::all_batches::structs::Batch;
use process::batches::process_each_stream_from_file;
use process::helper::{create_session, get_all_json_from_dir, join_paths};

pub fn process_name(config_params: ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let account = LoginDetails::from_file(config_params.remote_account_config_file());
    let session = create_session(&account, logger);
    let sftp = session.sftp().unwrap();
    let file_folder = join_paths(config_params.remote_folder_path(), "all_files");
    match sftp.create_dir_all(file_folder.as_ref()) {
        Err(_e) => {}
        Ok(..) => {}
    }
    root::root_menu(&config_params, logger, diag_logger, &sftp, &session);
}
