use std::process::exit;

use colored::Colorize;
use slog::Logger;
use ssh2::{Session, Sftp};
use terminal_menu::*;

use configuration_parameters::ConfigurationParameters;

use process::all_batches::structs::{Batch, BatchesInfo};
use process::batches::structs::BatchDetails;
use process::batches::{download_all_streams, restore_all_streams};
use process::helper::{get_parent_folder, prompt_continue_msg};

use super::batches::batches_menu;
use super::root::structs::Root;

pub mod structs;

pub fn all_batches_menu(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
    sftp_client: &Sftp,
    root_config: &Root,
    session: &Session,
) {
    let all_batch_path = get_parent_folder(root_config.batch_info_path.as_str());
    let mut batchinfo;
    if sftp_client.exist(root_config.batch_info_path.as_ref()) {
        batchinfo =
            BatchesInfo::from_remote_file(root_config.batch_info_path.as_str(), sftp_client);
    } else {
        println!(
            "{}",
            "Batch Info Config Not File, please Create one using uat_to_remote application"
                .red()
                .bold()
        );
        exit(0);
    }
    if batchinfo.batches.is_empty() {
        println!(
            "{}\n{}",
            "No batches Found, please add batches and restart".blue(),
            "Force Exit initiated".red().bold()
        );
        exit(0)
    }
    let mut wish_to_continue = true;
    while wish_to_continue {
        print_root_options(
            config_params,
            logger,
            diag_logger,
            sftp_client,
            &mut batchinfo,
            root_config,
            all_batch_path.as_str(),
            session,
        );
        wish_to_continue = prompt_continue_msg("Do you wish to see This Roots's menu Again :");
    }
}
fn print_root_options(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
    sftp_client: &Sftp,
    mut batchesinfo: &mut BatchesInfo,
    root_config: &Root,
    all_batches_path: &str,
    session: &Session,
) -> bool {
    let mut push_menu: Vec<TerminalMenuItem> = Vec::new();
    push_menu.push(label("Below Batches Found"));
    for each_batch in &batchesinfo.batches {
        push_menu.push(button(each_batch.to_string()));
    }
    push_menu.push(button("Download all batches"));
    push_menu.push(button("Restore all batches"));
    push_menu.push(button("Exit"));
    let batches_len = *&batchesinfo.batches.len();
    let menu = menu(push_menu);
    match try_run_fancy(&menu) {
        Err(_e) => {}
        Ok(..) => {}
    }
    let chosen_index = selected_item_index(&menu);
    let chosen_name = selected_item_name(&menu);
    let mut return_value = true;
    if chosen_index <= batches_len {
        let _number = chosen_index;
        match get_batch_from_number(&batchesinfo.batches, chosen_name.as_str()) {
            None => {
                println!("Wrong Choice, try again");
                print_root_options(
                    config_params,
                    logger,
                    diag_logger,
                    sftp_client,
                    batchesinfo,
                    root_config,
                    all_batches_path,
                    session,
                );
            }
            Some(bt) => {
                batches_menu(config_params, logger, diag_logger, sftp_client, bt, session);
            }
        }
    } else {
        if chosen_index == batches_len + 2 {
            restore_all_batches(&mut batchesinfo, sftp_client, logger)
        } else if chosen_index == batches_len + 3 {
            return_value = false;
        } else if chosen_index == batches_len + 1 {
            download_all_batches(&mut batchesinfo, sftp_client, logger)
        } else {
            unimplemented!()
        }
    }
    return_value
}

fn get_batch_from_number<'a>(batchs: &'a Vec<Batch>, key: &str) -> Option<&'a Batch> {
    for each_batch in batchs.clone() {
        if each_batch.to_string().clone().eq(key) {
            println!("found");
            return Some(&each_batch);
        }
    }
    println!("didnt find it");
    None
}

fn download_all_batches(batchinfo: &BatchesInfo, sftp_client: &Sftp, logger: &Logger) {
    for each_batch in &batchinfo.batches {
        if !sftp_client.exist(each_batch.batch_details_path.as_ref()) {
            println!(
                "File `{}` not Found, aborting Reload",
                each_batch.batch_details_path
            );
            continue;
        }
        let batchdetails =
            BatchDetails::from_remote_file(each_batch.batch_details_path.as_str(), sftp_client);
        download_all_streams(&batchdetails, sftp_client, logger);
    }
}

fn restore_all_batches(batchinfo: &BatchesInfo, sftp_client: &Sftp, logger: &Logger) {
    for each_batch in &batchinfo.batches {
        if !sftp_client.exist(each_batch.batch_details_path.as_ref()) {
            println!(
                "File `{}` not Found, aborting Reload",
                each_batch.batch_details_path
            );
            continue;
        }
        let batchdetails =
            BatchDetails::from_remote_file(each_batch.batch_details_path.as_str(), sftp_client);
        restore_all_streams(&batchdetails, sftp_client, logger);
    }
}
