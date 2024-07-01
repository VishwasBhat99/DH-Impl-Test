use colored::Colorize;
use configuration_parameters::ConfigurationParameters;
use crossterm_cursor::TerminalCursor;
use process::helper::{
    create_file_backup, get_actual_path, get_file_name_from_path, get_parent_folder, get_width,
    join_paths, prompt_continue_msg, write_json,
};
use process::io::{read_file, read_input};
use process::programs::structs::{EachFile, ProgramDetails, ResFile};
use process::streams::structs::Program;

use slog::Logger;
use ssh2::{Session, Sftp};
use std::collections::HashMap;

use std::io::BufRead;

use std::{fs, io};
use terminal_menu::*;
use termion::clear;
pub mod structs;

pub fn programs_menu(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
    sftp_client: &Sftp,
    program: &Program,
    _session: &Session,
) {
    let mut cursor = crossterm_cursor::cursor();
    let mut programdetails = get_initialised_program_details(program);
    if sftp_client.exist(programdetails.program_detals_path.as_ref()) {
        programdetails = ProgramDetails::from_remote_file(
            programdetails.program_detals_path.as_str(),
            sftp_client,
        );
    } else {
        programdetails = get_initialised_program_details(program);
    }
    if let Err(e) = cursor.save_position() {
        println!("{}", format!("{}", e).as_str().red().bold());
    }
    let mut wish_to_continue = true;
    while wish_to_continue {
        if let Err(e) = cursor.restore_position() {
            println!("{}", format!("{}", e).as_str().red().bold());
        }
        println!("{}", clear::AfterCursor);
        let width = get_width() as u16;
        if let Err(e) = cursor.move_right(width) {
            println!("{}", format!("{}", e).as_str().red().bold());
        }
        println!(
            "Program Name: {}",
            programdetails.program_name.as_str().green().bold()
        );
        wish_to_continue = print_program_options(
            config_params,
            logger,
            diag_logger,
            sftp_client,
            &mut programdetails,
            program,
            &mut cursor,
        );
    }
    if let Err(e) = cursor.restore_position() {
        println!("{}", format!("{}", e).as_str().red().bold());
    }
    println!("{}", clear::AfterCursor);
}
fn print_program_options(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
    sftp_client: &Sftp,
    mut programdetails: &mut ProgramDetails,
    program: &Program,
    cursor: &mut TerminalCursor,
) -> bool {
    let _choice = "";
    let _input = String::new();
    if programdetails.script_path.is_empty() {
        println!("No Script Found, add New Script!");
        get_fresh_menu(
            config_params,
            logger,
            diag_logger,
            sftp_client,
            &mut programdetails,
            program,
        );
        return print_program_options(
            config_params,
            logger,
            diag_logger,
            sftp_client,
            programdetails,
            program,
            cursor,
        );
    } else {
        println!("Script  Found \n");
        let mut push_menu: Vec<TerminalMenuItem> = Vec::new();
        for each_file in &programdetails.files {
            println!("{}", each_file);
        }
        push_menu.push(button("modify files"));
        push_menu.push(button("Upload all Files"));
        push_menu.push(button("Back"));
        let menu = menu(push_menu);
        let mut return_value = true;
        match try_run_fancy(&menu) {
            Err(_e) => {}
            Ok(..) => {}
        }
        let chosen_index = selected_item_index(&menu) as i32;
        match chosen_index {
            0 => get_fresh_menu(
                config_params,
                logger,
                diag_logger,
                sftp_client,
                &mut programdetails,
                program,
            ),
            1 => {
                upload_all_files(
                    programdetails.program_detals_path.as_str(),
                    sftp_client,
                    logger,
                );
                return_value = false;
            }
            2 => return_value = false,
            _ => {}
        }
        return_value
    }
}
fn get_fresh_menu(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    _diag_logger: &Logger,
    sftp_client: &Sftp,
    mut programdetails: &mut ProgramDetails,
    _curr_program: &Program,
) {
    programdetails.display();
    if programdetails.script_path.is_empty() {
        let script_path = read_input("Enter the program script path: ");
        programdetails.script_path = script_path.clone();
    }
    programdetails.files.clear();
    programdetails.num_of_files = 0;
    let all_files_data = get_all_files(programdetails.script_path.as_str(), logger, sftp_client);
    let mut filtered_files_data = intelligent_filter_files(&all_files_data);
    println!("The below Files are automatically selected\n");
    print_all_files(&filtered_files_data);
    if prompt_continue_msg("Do you wish to add/remove files:\n") {
        add_files_to_list(&all_files_data, &mut filtered_files_data);
        remove_files_from_list(&all_files_data, &mut filtered_files_data);
    }
    println!("\nThe below Files will be moved");
    print_all_files(&filtered_files_data);
    for each_file in filtered_files_data {
        let variable_name = each_file.1.variable_name.clone();
        let file_location = each_file.1.file_location.clone();
        let mut resfile = ResFile::default();
        let file_name = get_file_name_from_path(file_location.as_str());
        if !variable_name.to_lowercase().contains("binary") && !file_name.contains('.') {
            get_file_starting_with(
                &mut programdetails,
                &each_file.1,
                sftp_client,
                config_params.remote_folder_path(),
            );
            continue;
        }
        programdetails.num_of_files += 1;
        resfile.file_name = file_name.clone();
        resfile.file_id = programdetails.num_of_files;
        resfile.source_file = get_actual_path(file_location.as_str());
        resfile.destination_file =
            gen_target_location(config_params.remote_folder_path(), file_location.as_str());
        programdetails.files.push(resfile);
    }
    println!("Initiating Upload of Files");
    println!("{:#?}", programdetails.files);
    programdetails.initiate_copy(sftp_client, logger);
    println!("Writing the Below Details");
    for each_file in &programdetails.files {
        println!("{}: {}", each_file.file_id, each_file.file_name);
    }
    if sftp_client.exist(programdetails.program_detals_path.as_ref()) {
        create_file_backup(programdetails.program_detals_path.as_str(), sftp_client);
    }
    write_json(
        programdetails.program_detals_path.as_str(),
        &programdetails,
        logger,
        sftp_client,
    );
}

fn get_file_starting_with(
    programdetails: &mut ProgramDetails,
    curr_file: &EachFile,
    _sftp_client: &Sftp,
    root_location: &str,
) {
    println!(
        "Multiple Files Detected for {}\n Please choose the files from below",
        curr_file.variable_name
    );
    let parent_dir = get_parent_folder(curr_file.file_location.as_str());
    let files_entry = match fs::read_dir(parent_dir.as_str()) {
        Ok(entry) => {
            match entry
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, io::Error>>()
            {
                Ok(data) => data,
                Err(err) => {
                    println!("unable to get Dir file,skipping the File: `{}`", err);
                    return;
                }
            }
        }
        Err(err) => {
            println!("unable to open Dir,skipping the File: `{}`", err);
            return;
        }
    };
    let pattern = get_file_name_from_path(curr_file.file_location.as_str());
    let mut selected_file: HashMap<i32, (String, String)> = HashMap::new();
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
        if file_name.starts_with(pattern.as_str()) {
            count += 1;
            selected_file.insert(count, (file_name, file_path));
        }
    }
    let mut num_of_files = selected_file.keys().clone().collect::<Vec<&i32>>();
    num_of_files.sort();
    for key in num_of_files {
        match selected_file.get(&key) {
            None => continue,
            Some(data) => {
                println!("{}: {}: {} ", *key, data.0, data.1);
            }
        }
    }
    let files_number =
        read_input("Enter the file numbers you wish to remove(`,` separate)\nPress enter to Skip");
    if files_number.is_empty() {
        return;
    }
    let choices: Vec<&str> = files_number.trim_matches(',').split(',').collect();
    for choice in choices {
        let num = choice.parse().unwrap_or(0);
        if num.eq(&0) {
            continue;
        }
        let file_data = match selected_file.get(&num) {
            None => {
                println!("invalid choice `{}`, skipping choice", num);
                continue;
            }
            Some(data) => data,
        };
        let mut res_file = ResFile::default();
        res_file.file_name = file_data.0.clone();
        res_file.source_file = file_data.1.clone();
        programdetails.num_of_files += 1;
        res_file.file_id = programdetails.num_of_files;
        res_file.destination_file =
            gen_target_location(root_location, res_file.source_file.as_str());
        programdetails.files.push(res_file);
    }
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

fn get_all_files(
    script_path: &str,
    _logger: &Logger,
    _sftp_client: &Sftp,
) -> HashMap<i32, EachFile> {
    let mut files_data: HashMap<i32, EachFile> = HashMap::new();
    let mut count = 0;
    let script_reader = read_file(script_path);
    if !script_path.contains("pfc") {
        for each_line in script_reader.lines() {
            let line = match each_line {
                Ok(data) => data,
                Err(_) => {
                    continue;
                }
            };
            if line.starts_with("--") {
                continue;
            }
            if check_if_binary(line.as_str()) {
                let binary_location = get_binary_path(line.as_str());
                count += 1;
                files_data.insert(
                    count,
                    EachFile {
                        variable_name: "Binary".to_string(),
                        file_location: binary_location.clone(),
                    },
                );
                continue;
            }
            let data: Vec<&str> = line.trim().split('=').collect();
            let variable_name = data.first().unwrap_or(&"").to_string();
            let location = data
                .last()
                .unwrap_or(&"")
                .to_string()
                .trim_start_matches('$')
                .trim_matches('\"')
                .to_string();
            if variable_name.is_empty() || location.is_empty() || data.len() <= 1 {
                continue;
            }
            count += 1;
            files_data.insert(
                count,
                EachFile {
                    variable_name: variable_name.clone(),
                    file_location: location.to_string(),
                },
            );
        }
    }
    count += 1;
    files_data.insert(
        count,
        EachFile {
            variable_name: "Script".to_string(),
            file_location: script_path.to_string(),
        },
    );
    files_data
}

fn get_binary_path(line: &str) -> String {
    let pat: &[_] = &[' ', '\\'];
    let trimmed_line = line.trim_matches(pat);
    let real_path = get_actual_path(trimmed_line);
    real_path
}
fn check_if_binary(line: &str) -> bool {
    if line.ends_with('\\') {
        return true;
    }
    false
}

fn print_all_files(files_data: &HashMap<i32, EachFile>) {
    let mut num_of_files = files_data.keys().clone().collect::<Vec<&i32>>();
    num_of_files.sort();
    for key in num_of_files {
        match files_data.get(&key) {
            None => continue,
            Some(data) => {
                println!("{}: {} , {}", key, data.variable_name, data.file_location);
            }
        }
    }
    println!("\n");
}

fn intelligent_filter_files(files_data: &HashMap<i32, EachFile>) -> HashMap<i32, EachFile> {
    let mut filtered_data: HashMap<i32, EachFile> = HashMap::new();
    for each_file in files_data {
        if check_files_to_ignore(&*each_file.1.variable_name)
            || check_files_to_ignore(&*each_file.1.file_location)
        {
            continue;
        }
        filtered_data.insert(*each_file.0, EachFile::from_another(each_file.1));
    }
    filtered_data
}

fn check_files_to_ignore(file: &str) -> bool {
    let patterns = vec!["inp", "out", "log", "diag", "concat", "rec"];
    for each_pattern in patterns {
        if file.to_lowercase().contains(each_pattern) {
            return true;
        }
    }
    false
}

fn add_files_to_list(
    total_data: &HashMap<i32, EachFile>,
    filtered_data: &mut HashMap<i32, EachFile>,
) {
    println!("Files List for Adding Files\n");
    print_all_files(total_data);
    let files_number =
        read_input("\nEnter the file numbers you wish to add(`,` separate)\nPress enter to Skip:");
    if files_number.is_empty() {
        return;
    }
    let choices: Vec<&str> = files_number.trim_matches(',').split(',').collect();
    for choice in choices {
        let num = choice.parse().unwrap_or(0);
        if num.eq(&0) {
            continue;
        }
        let req_eachfile = total_data.get(&num);
        match req_eachfile {
            None => {
                println!("No File found with num {}, ignoring choice", num);
                continue;
            }
            Some(data) => {
                filtered_data
                    .entry(num)
                    .or_insert(EachFile::from_another(data));
            }
        }
    }
}

fn remove_files_from_list(
    total_data: &HashMap<i32, EachFile>,
    filtered_data: &mut HashMap<i32, EachFile>,
) {
    println!("Files List for Removing Files\n");
    print_all_files(total_data);
    let files_number = read_input(
        "\nEnter the file numbers you wish to remove(`,` separate)\nPress enter to Skip:",
    );
    if files_number.is_empty() {
        return;
    }
    let choices: Vec<&str> = files_number.trim_matches(',').split(',').collect();
    for choice in choices {
        let num = choice.parse().unwrap_or(0);
        if num.eq(&0) {
            continue;
        }

        match filtered_data.remove_entry(&num) {
            None => println!("No File found with num {}, ignoring choice", num),
            Some(_) => continue,
        }
    }
}
pub fn gen_target_location(root_location: &str, source_location: &str) -> String {
    let prefix = join_paths(root_location, "all_files");
    let target = join_paths(prefix.as_str(), source_location.trim_start_matches('/'));
    target
}
pub fn upload_all_files(program_details_path: &str, sftp_client: &Sftp, logger: &Logger) {
    if !sftp_client.exist(program_details_path.as_ref()) {
        println!(
            "{}",
            format!("File `{}` not Found, aborting Reload", program_details_path)
                .as_str()
                .bright_red()
                .bold()
        );
        return;
    }
    let mut programdetails = ProgramDetails::from_remote_file(program_details_path, sftp_client);
    println!("Initiating Reloading of all Files");
    for each_file in &programdetails.files {
        println!("{}", each_file);
    }
    programdetails.initiate_copy(sftp_client, logger);
}

pub fn get_files_from_path(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    _diag_logger: &Logger,
    sftp_client: &Sftp,
    curr_program: &Program,
    script_path: &str,
) {
    let mut programdetails = get_initialised_program_details(curr_program);
    if sftp_client.exist(programdetails.program_detals_path.as_ref()) {
        programdetails = ProgramDetails::from_remote_file(
            programdetails.program_detals_path.as_str(),
            sftp_client,
        );
    } else {
        programdetails = get_initialised_program_details(curr_program);
    }
    if !programdetails.files.is_empty() {
        programdetails.script_path = script_path.to_string();
        programdetails.update_json(logger, sftp_client);
        return;
    }
    programdetails = get_initialised_program_details(curr_program);
    programdetails.script_path = script_path.to_string();
    programdetails.files.clear();
    programdetails.num_of_files = 0;
    let all_files_data = get_all_files(script_path, logger, sftp_client);
    let mut filtered_files_data = intelligent_filter_files(&all_files_data);

    for each_file in filtered_files_data {
        let variable_name = each_file.1.variable_name.clone();
        let file_location = each_file.1.file_location.clone();
        let mut resfile = ResFile::default();
        let file_name = get_file_name_from_path(file_location.as_str());
        if !variable_name.to_lowercase().contains("binary") && !file_name.contains('.') {
            continue;
        }
        programdetails.num_of_files += 1;
        resfile.file_name = file_name.clone();
        resfile.file_id = programdetails.num_of_files;
        resfile.source_file = get_actual_path(file_location.as_str());
        resfile.destination_file =
            gen_target_location(config_params.remote_folder_path(), file_location.as_str());
        programdetails.files.push(resfile);
    }
    programdetails.update_json(logger, sftp_client);
}
