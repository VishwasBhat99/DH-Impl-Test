use super::root::structs::Root;
use configuration_parameters::ConfigurationParameters;

use process::all_batches::structs::{Batch, BatchesInfo};
use process::helper::{
    create_file_backup, get_current_time, get_folder_name, get_parent_folder, join_paths,
    prompt_continue_msg,
};
use process::io::{read_file, read_input};
use slog::Logger;
use ssh2::{Session, Sftp};

pub mod structs;
use super::batches::batches_menu;
use process::batches::structs::BatchDetails;
use process::batches::{get_streams_from_path, upload_all_streams};
use std::io::{BufRead, Error};
use terminal_menu::*;

//In this modules all the functions related to ALL BATCH Menu is stored
pub fn all_batches_menu(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
    sftp_client: &Sftp,
    root_config: &Root,
    session: &Session,
) {
    let all_batch_path = get_parent_folder(root_config.batch_info_path.as_str());
    let mut batchinfo = BatchesInfo::default();
    if sftp_client.exist(root_config.batch_info_path.as_ref()) {
        batchinfo =
            BatchesInfo::from_remote_file(root_config.batch_info_path.as_str(), sftp_client);
    } else {
        batchinfo.all_batch_path = root_config.batch_info_path.clone();
    }

    let mut wish_to_continue = true;
    while wish_to_continue {
        wish_to_continue = print_root_options(
            config_params,
            logger,
            diag_logger,
            sftp_client,
            &mut batchinfo,
            root_config,
            all_batch_path.as_str(),
            session,
        );
    }
    if sftp_client.exist(batchinfo.all_batch_path.as_ref()) {
        create_file_backup(batchinfo.all_batch_path.as_str(), sftp_client);
    }
    println!("writing below data: ");
    batchinfo.display();
    batchinfo.update_json(logger, sftp_client);
}

//this displays the option regarding all batches
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
    let _choice = "";
    let _input = String::new();
    if batchesinfo.batches.is_empty() {
        println!("\nNo Batches Found, add New Batches!");
        get_fresh_menu(
            config_params,
            logger,
            diag_logger,
            sftp_client,
            root_config,
            all_batches_path,
            &mut batchesinfo,
            true,
        );
        println!("its here");
        return print_root_options(
            config_params,
            logger,
            diag_logger,
            sftp_client,
            batchesinfo,
            root_config,
            all_batches_path,
            session,
        );
    } else {
        let mut push_menu: Vec<TerminalMenuItem> = Vec::new();
        push_menu.push(label("Below Batches Found"));
        for each_batch in &batchesinfo.batches {
            push_menu.push(button(each_batch.to_string()));
        }
        push_menu.push(button("Add a new Batch"));
        push_menu.push(button("Upload all batches"));
        //for adding the option for manual mode
        // push_menu.push(button("Add a new Batch(manual mode)"));
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
                upload_all_batches(&mut batchesinfo, sftp_client, logger)
            } else if chosen_index == batches_len + 3 {
                return_value = false;
            }
            // } else if chosen_index == batches_len + 3 {
            //     get_fresh_menu(
            //         config_params,
            //         logger,
            //         diag_logger,
            //         sftp_client,
            //         root_config,
            //         all_batches_path,
            //         &mut batchesinfo,
            //         false,
            //     );
            // }
            else if chosen_index == batches_len + 1 {
                get_fresh_menu(
                    config_params,
                    logger,
                    diag_logger,
                    sftp_client,
                    root_config,
                    all_batches_path,
                    &mut batchesinfo,
                    true,
                )
            } else {
                batchesinfo.batches.clear();
                get_fresh_menu(
                    config_params,
                    logger,
                    diag_logger,
                    sftp_client,
                    root_config,
                    all_batches_path,
                    &mut batchesinfo,
                    true,
                );
            }
        }
        return_value
    }
}

pub fn get_fresh_menu(
    _config_params: &ConfigurationParameters,
    _logger: &Logger,
    _diag_logger: &Logger,
    sftp_client: &Sftp,
    _root_config: &Root,
    all_batches_path: &str,
    batchesinfo: &mut BatchesInfo,
    from_path: bool,
) {
    let mut batch_count = batchesinfo.num_of_batches;

    loop {
        let curr_batch = match from_path {
            true => get_each_batch_from_path(
                _config_params,
                _logger,
                _diag_logger,
                batch_count,
                all_batches_path,
                sftp_client,
            ),
            false => get_each_batch(batch_count, all_batches_path, sftp_client),
        };
        let filtered_batches: Vec<&Batch> = batchesinfo
            .batches
            .iter()
            .filter(|batch| batch.batch_id == curr_batch.batch_id)
            .collect();
        if !filtered_batches.is_empty() {
            break;
        }
        batch_count += 1;
        batchesinfo.num_of_batches = batch_count;

        batchesinfo.batches.push(curr_batch);
        if from_path || !prompt_continue_msg("Do you wish to add New Batch: ") {
            break;
        }
    }
}

//gets details regarding each batch
fn get_each_batch(id: i32, all_batches_path: &str, sftp_client: &Sftp) -> Batch {
    println!("Enter Batch Details");
    let mut new_batch = Batch::default();
    new_batch.batch_id = id;
    new_batch.batch_name = read_input("batch name: ");
    new_batch.last_modified = get_current_time();
    let batch_path = gen_batch_path(
        all_batches_path,
        get_folder_name(new_batch.batch_name.as_str()).as_str(),
    );
    if let Err(_e) = sftp_client.create_dir_all(batch_path.as_str().as_ref()) {}

    new_batch.batch_details_path = join_paths(batch_path.as_str(), "batch_details.json");
    new_batch
}

//this gets each batch details from streamdef jsons
fn get_each_batch_from_path(
    _config_params: &ConfigurationParameters,
    _logger: &Logger,
    _diag_logger: &Logger,
    id: i32,
    all_batches_path: &str,
    sftp_client: &Sftp,
) -> Batch {
    let selected_batch = select_batch_from_list(_config_params.batch_info_file());
    let mut new_batch = Batch::default();
    new_batch.batch_id = selected_batch.0;
    new_batch.batch_name = selected_batch.1.clone();
    new_batch.last_modified = get_current_time();
    let batch_path = gen_batch_path(
        all_batches_path,
        get_folder_name(new_batch.batch_name.as_str()).as_str(),
    );
    if let Err(_e) = sftp_client.create_dir_all(batch_path.as_str().as_ref()) {}

    new_batch.batch_details_path = join_paths(batch_path.as_str(), "batch_details.json");
    let mut curr_batchdetails = BatchDetails::default();
    get_streams_from_path(
        _config_params,
        _logger,
        _diag_logger,
        sftp_client,
        &new_batch,
        &mut curr_batchdetails,
    );
    new_batch
}

//generates folder pat for each batch
fn gen_batch_path(all_batch_path: &str, batch_name: &str) -> String {
    join_paths(all_batch_path, get_folder_name(batch_name).as_str())
}

//searches for a batch in the vec and returns the vec
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

fn upload_all_batches(batchinfo: &mut BatchesInfo, sftp_client: &Sftp, logger: &Logger) {
    for each_batch in &mut batchinfo.batches {
        if !sftp_client.exist(each_batch.batch_details_path.as_ref()) {
            println!(
                "File `{}` not Found, aborting Reload",
                each_batch.batch_details_path
            );
            continue;
        }
        let mut batchdetails =
            BatchDetails::from_remote_file(each_batch.batch_details_path.as_str(), sftp_client);
        upload_all_streams(&mut batchdetails, sftp_client, logger);
        each_batch.last_modified = get_current_time();
    }
}

//displays all the batches and asks user to select anyone
fn select_batch_from_list(batch_info_path: &str) -> (i32, String) {
    let batch_reader = read_file(batch_info_path);
    let mut batch_info_list: Vec<(i32, String)> = Vec::new();
    for each_line in batch_reader.lines() {
        let line = match each_line {
            Ok(data) => data,
            Err(_) => continue,
        };
        let line_split: Vec<&str> = line.split('|').collect();
        if line_split.len() <= 1 || line.is_empty() {
            continue;
        }
        let batch_id: i32 = line_split[0].parse().unwrap_or(0);
        let batch_name = line_split[1].trim().to_string();
        if batch_id == 0 || batch_name.is_empty() {
            continue;
        }
        batch_info_list.push((batch_id, batch_name));
    }
    batch_info_list.sort_by(|a, b| a.0.cmp(&b.0));
    let mut push_menu: Vec<TerminalMenuItem> = Vec::new();
    push_menu.push(label("Below Batches Found"));
    for each_batch in &batch_info_list {
        push_menu.push(button(format!("{} - {}", each_batch.0, each_batch.1)));
    }
    let batches_len = batch_info_list.len();
    let menu = menu(push_menu);
    match try_run_fancy(&menu) {
        Err(_e) => {}
        Ok(..) => {}
    }
    let chosen_index = selected_item_index(&menu);
    batch_info_list[chosen_index - 1].clone()
}
