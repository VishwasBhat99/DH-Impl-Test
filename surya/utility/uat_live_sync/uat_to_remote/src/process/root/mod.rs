use super::all_batches::all_batches_menu;
use configuration_parameters::ConfigurationParameters;

use process::helper::{join_paths, prompt_continue_msg, write_json};
use process::io::read_input;
use process::root::structs::Root;
use slog::Logger;
use ssh2::{Session, Sftp};

pub mod structs;
use colored::Colorize;

//this is start point of the program
//this detects if the program is ran for and takes action on the same
pub fn root_menu(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
    sftp_client: &Sftp,
    session: &Session,
) {
    let root_config_file = join_paths(config_params.remote_folder_path(), "root_info.json");
    let mut root_config_default = Root::default();
    if !sftp_client.exist(root_config_file.as_str().as_ref()) {
        get_fresh_menu(
            config_params,
            logger,
            diag_logger,
            &mut root_config_default,
            sftp_client,
            &root_config_file,
            session,
        );
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

//if this is the fresh start, then this function is called
//Note this will be called only onnce to gather Bank details
pub fn get_fresh_menu(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
    root_config: &mut Root,
    sftp_client: &Sftp,
    _root_config_file: &str,
    session: &Session,
) {
    println!("{}", "Fresh Movement detected".blue().bold());
    root_config.bank_name = read_input("Enter Bank name: ");
    let all_batches_path = get_batches_folder(config_params.remote_folder_path());
    match sftp_client.create_dir_all(all_batches_path.as_str().as_ref()) {
        Err(_e) => {}
        Ok(..) => {}
    }
    let root_config_file = join_paths(config_params.remote_folder_path(), "root_info.json");
    root_config.batch_info_path = join_paths(all_batches_path.as_str(), "batch_info.json");
    write_json(root_config_file.as_str(), &root_config, logger, sftp_client);
    all_batches_menu(
        config_params,
        logger,
        diag_logger,
        sftp_client,
        root_config,
        session,
    );
}

//generates folder path where, all the batches are to be stored
fn get_batches_folder(path: &str) -> String {
    join_paths(path, "all_batches")
}
