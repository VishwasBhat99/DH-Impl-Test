


use colored::Colorize;

use slog::Logger;
use ssh2::{Session, Sftp};

use configuration_parameters::ConfigurationParameters;

use process::helper::join_paths;

use process::root::structs::Root;

use super::all_batches::all_batches_menu;

pub mod structs;

pub fn root_menu(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
    sftp_client: &Sftp,
    session: &Session,
) {
    let root_config_file = join_paths(config_params.remote_folder_path(), "root_info.json");
    if !sftp_client.exist(root_config_file.as_str().as_ref()) {
        println!(
            "{}",
            "No Root Config File Found, please Use uat_to_remote application and create one"
                .red()
                .bold()
        );
        return;
    } else {
        println!("Root Config File found");
        let root_config = Root::from_remote_file(root_config_file.as_str(), sftp_client);
        all_batches_menu(
            config_params,
            logger,
            diag_logger,
            sftp_client,
            &root_config,
            session,
        );
    }
}
