use slog::Logger;

use configuration_parameters::ConfigurationParameters;
use process::helper::create_session;

use process::structs::LoginDetails;

mod all_batches;
mod batches;
pub mod helper;
mod io;
pub mod programs;
mod root;
mod streams;
mod structs;

pub fn process_name(config_params: ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let account = LoginDetails::from_file(config_params.remote_account_config_file());
    let session = create_session(&account, logger);
    let sftp = session.sftp().unwrap();
    root::root_menu(&config_params, logger, diag_logger, &sftp, &session);
}
