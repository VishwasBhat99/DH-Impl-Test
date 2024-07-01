use std::collections::HashMap;

use std::{fs, io};

use colored::Colorize;
use slog::Logger;
use ssh2::{Session, Sftp};
use terminal_menu::*;

use configuration_parameters::ConfigurationParameters;
use process::helper::{
    check_if_exist, create_file_backup, get_file_name_from_path, get_parent_folder, join_paths,
    set_curr_time_stamp, verify_if_different_timestamp,
};
use process::io::read_input;
use process::programs::structs::ProgramDetails;
use process::streams::structs::Program;

pub mod structs;

pub fn programs_menu(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
    sftp_client: &Sftp,
    program: &Program,
    _session: &Session,
) {
    let mut programdetails = get_initialised_program_details(program);
    if sftp_client.exist(programdetails.program_detals_path.as_ref()) {
        programdetails = ProgramDetails::from_remote_file(
            programdetails.program_detals_path.as_str(),
            sftp_client,
        );
    } else {
        println!(
            "{}",
            "Program Details Config Not File, please Create one using uat_to_remote application"
                .red()
                .bold()
        );
        return;
    }
    if programdetails.files.is_empty() || programdetails.script_path.is_empty() {
        println!(
            "{}\n{}",
            "No Files Found or script is empty, please add Files and restart".blue(),
            "Force Exit Program initiated".red().bold()
        );
        return;
    }
    let mut wish_to_continue = true;
    while wish_to_continue {
        wish_to_continue = print_program_options(
            config_params,
            logger,
            diag_logger,
            sftp_client,
            &mut programdetails,
            program,
        );
    }
}

fn print_program_options(
    _config_params: &ConfigurationParameters,
    logger: &Logger,
    _diag_logger: &Logger,
    sftp_client: &Sftp,
    programdetails: &mut ProgramDetails,
    _program: &Program,
) -> bool {
    let _choice = "";
    let _input = String::new();
    println!(
        "The Following Files are present in `{}`\n",
        programdetails.program_name
    );
    let mut push_menu: Vec<TerminalMenuItem> = Vec::new();
    for each_file in &programdetails.files {
        println!("{}", each_file);
    }
    push_menu.push(button("Download all files"));
    push_menu.push(button("Restore all Files"));
    push_menu.push(button("Back"));
    let menu = menu(push_menu);
    let mut return_value = true;
    match try_run_fancy(&menu) {
        Err(_e) => {}
        Ok(..) => {}
    }
    let chosen_index = selected_item_index(&menu) as i32;
    match chosen_index {
        0 => download_all_files(
            programdetails.program_detals_path.as_str(),
            sftp_client,
            logger,
        ),
        1 => restore_all_files(
            programdetails.program_detals_path.as_str(),
            sftp_client,
            logger,
        ),
        2 => return_value = false,
        _ => {}
    }
    return_value
}

fn get_initialised_program_details(curr_program: &Program) -> ProgramDetails {
    let mut programdetails = ProgramDetails::default();
    programdetails.stream_name = curr_program.stream_name.clone();
    programdetails.stream_id = curr_program.stream_id;
    programdetails.flow_name = curr_program.flow_name.clone();
    programdetails.program_id = curr_program.program_id;
    programdetails.program_name = curr_program.program_name.clone();
    programdetails.program_detals_path = curr_program.program_details_path.clone();
    programdetails
}

pub fn download_all_files(program_details_path: &str, sftp_client: &Sftp, logger: &Logger) {
    if !sftp_client.exist(program_details_path.as_ref()) {
        println!(
            "File `{}` not Found, aborting Download",
            program_details_path
        );
        return;
    }
    let mut programdetails = ProgramDetails::from_remote_file(program_details_path, sftp_client);
    programdetails.display();
    let mut existing_files: Vec<&str> = Vec::new();
    println!("Initiating Downloading of all Files");
    for each_file in &programdetails.files {
        println!("{}", each_file);
        if check_if_exist(each_file.destination_file.as_str())
            && verify_if_different_timestamp(each_file.destination_file.as_str())
        {
            existing_files.push(each_file.file_name.as_str());
        }
    }

    println!(
        "{}",
        "checking for existing Files in local to initiate backup"
            .blue()
            .bold()
    );
    for each_file in &programdetails.files {
        if check_if_exist(each_file.destination_file.as_str())
            && verify_if_different_timestamp(each_file.destination_file.as_str())
        {
            create_file_backup(each_file.destination_file.as_str());
        }
    }
    if existing_files.is_empty() {
        println!("No backup created");
    } else {
        println!("\nbackup created for below Files:\n");
        for file in existing_files.iter().enumerate() {
            println!("{}: {}", file.0 + 1, file.1);
        }
    }
    programdetails.initiate_copy(sftp_client, logger);
}

pub fn restore_all_files(program_details_path: &str, sftp_client: &Sftp, _logger: &Logger) {
    if !sftp_client.exist(program_details_path.as_ref()) {
        println!(
            "{}",
            format!(
                "Err: file `{}` not found, skipping the process \n",
                program_details_path
            )
            .as_str()
            .red()
            .bold()
        );
        return;
    }
    let programdetails = ProgramDetails::from_remote_file(program_details_path, sftp_client);
    let mut prev_timestamp = "".to_string();
    for each_file in &programdetails.files {
        if check_if_exist(each_file.destination_file.as_str())
            && !verify_if_different_timestamp(each_file.destination_file.as_str())
        {
            println!(
                "{}",
                format!(
                    "info: file `{}` already restored by other program:\n",
                    each_file.destination_file
                )
                .as_str()
                .blue()
                .bold()
            );
            continue;
        }
        if !check_if_exist(each_file.destination_file.as_str()) {
            println!(
                "{}",
                format!(
                    "info: file `{}` does not exist, skipping restore:\n",
                    each_file.destination_file
                )
                .as_str()
                .blue()
                .bold()
            );
            continue;
        }
        let bkp_filename: String;
        if prev_timestamp.is_empty() {
            prev_timestamp = process_bkp_restore(each_file.destination_file.as_str());
        }
        if prev_timestamp.is_empty() {
            println!(
                "{}",
                format!("Err: backup file not found \n")
                    .as_str()
                    .red()
                    .bold()
            );
            continue;
        }
        println!(
            "{}",
            format!("info: choosing prev time stamp: `{}`", prev_timestamp)
                .as_str()
                .blue()
                .bold()
        );
        bkp_filename =
            gen_req_bkp_name(each_file.destination_file.as_str(), prev_timestamp.as_str());
        println!("Restoring `{}` ", bkp_filename);
        if check_if_exist(bkp_filename.as_str()) {
            if let Err(err) = std::fs::remove_file(each_file.destination_file.as_str()) {
                println!(
                    "{}",
                    format!(
                        "Err: unable to remove existing  file `{}`, `{}` \n",
                        each_file.destination_file.as_str(),
                        err
                    )
                    .as_str()
                    .red()
                    .bold()
                );
            }
        } else {
            println!(
                "{}",
                format!("Err: backup file `{}` not found \n", bkp_filename)
                    .as_str()
                    .red()
                    .bold()
            );
            continue;
        }
        if let Err(err) = std::fs::rename(bkp_filename.clone(), each_file.destination_file.as_str())
        {
            println!("{}", err);
            continue;
        }
        set_curr_time_stamp(each_file.destination_file.as_str());
        println!(
            "{}",
            format!("Restored successfuly to `{}`\n", each_file.destination_file)
                .as_str()
                .green()
                .bold()
        );
    }
}

fn process_bkp_restore(path: &str) -> String {
    let parent_folder = get_parent_folder(path);
    let file_name = get_file_name_from_path(path);
    let file_contents: Vec<&str> = file_name.split('.').collect();
    let mut bkp_file_name = file_contents[0].to_string();
    bkp_file_name.push_str("_bkp_");
    let bkp_path = join_paths(parent_folder.as_str(), bkp_file_name.as_str());
    let al_bkp_files = get_all_bkp_files(bkp_path.as_str());
    match al_bkp_files {
        None => return "".to_string(),
        Some(data) => choose_bkp_file(&data),
    }
}

fn choose_bkp_file(all_bkp_files: &HashMap<i32, (String, String)>) -> String {
    let mut num_of_files = all_bkp_files.keys().clone().collect::<Vec<&i32>>();
    num_of_files.sort();
    if num_of_files.is_empty() {
        println!("Looks like files might be restored");
        return "".to_string();
    }
    println!("\nChoose any one file to restore from below");
    for key in num_of_files.to_owned() {
        match all_bkp_files.get(&key) {
            None => continue,
            Some(data) => {
                println!("{}: {} ", *key, data.0);
            }
        }
    }
    let files_number = read_input("Enter the file number you wish to replace:")
        .parse()
        .unwrap_or(0);
    if files_number == 0 {
        println!("Wrong Choice, please enter again\n");
        choose_bkp_file(all_bkp_files);
    }
    let file_data = match all_bkp_files.get(&files_number) {
        None => {
            println!("invalid choice `{}`, try again", files_number);
            return choose_bkp_file(all_bkp_files);
        }
        Some(data) => data,
    };
    get_time_stamp_from_file(file_data.1.as_str())
}

fn gen_req_bkp_name(path: &str, timestamp: &str) -> String {
    let parent_folder = get_parent_folder(path);
    let file_name = get_file_name_from_path(path);
    let mut file_contents: Vec<&str> = file_name.split('.').collect();
    if file_contents.len() <= 1 {
        file_contents.push("");
    }
    let mut bkp_file_name = file_contents[0].to_string();
    bkp_file_name.push_str(format!("_bkp_{}", timestamp).as_str());
    if !file_contents[1].is_empty() {
        bkp_file_name.push('.');
    }
    let sufix = file_contents[1];
    bkp_file_name.push_str(sufix);
    let bkp_path = join_paths(parent_folder.as_str(), bkp_file_name.as_str());
    bkp_path
}

fn get_time_stamp_from_file(path: &str) -> String {
    let file_contents: Vec<&str> = path.split('.').collect();
    if file_contents.len() < 1 {
        return "".to_string();
    }
    let suffix = file_contents[0];
    let suffix_contents: Vec<&str> = suffix.split("_bkp_").collect();
    println!("{}", suffix_contents[1].to_string());
    suffix_contents[1].to_string()
}

fn get_all_bkp_files(file_name: &str) -> Option<HashMap<i32, (String, String)>> {
    let mut all_files: HashMap<i32, (String, String)> = HashMap::new();
    let parent_dir = get_parent_folder(file_name);
    let actual_file_name = get_file_name_from_path(file_name);
    let files_entry = match fs::read_dir(parent_dir.as_str()) {
        Ok(entry) => {
            match entry
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, io::Error>>()
            {
                Ok(data) => data,
                Err(err) => {
                    println!(
                        "{}",
                        format!("unable to open Dir,skipping the File: `{}`", err)
                            .as_str()
                            .red()
                            .bold()
                    );
                    return None;
                }
            }
        }
        Err(err) => {
            println!(
                "{}",
                format!("unable to open Dir,skipping the File: `{}`", err)
                    .as_str()
                    .red()
                    .bold()
            );
            return None;
        }
    };
    let mut count = 0;
    for each_file in files_entry {
        if each_file.is_dir() {
            continue;
        }
        let file_path = each_file.to_str().unwrap_or("").to_string();
        let file_name = get_file_name_from_path(file_path.as_str());
        if file_name.is_empty() || file_path.is_empty() {
            continue;
        }
        if file_name.starts_with(&actual_file_name) {
            count += 1;
            all_files.insert(count, (file_name, file_path));
        }
    }
    Some(all_files)
}
