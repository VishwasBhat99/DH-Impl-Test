use colored::Colorize;
use configuration_parameters::ConfigurationParameters;
use crossterm_cursor::TerminalCursor;
use process::batches::structs::{ProcDef, Stream, StreamDef};
use process::helper::{
    create_file_backup, gen_req_folder_path, get_current_time, get_parent_folder, get_width,
    join_paths, prompt_continue_msg,
};
use process::io::read_input;
use process::programs::{get_files_from_path, programs_menu};

use process::streams::structs::{Program, StreamDetails};
use slog::Logger;
use ssh2::{Session, Sftp};

use terminal_menu::*;
use termion::clear;
pub mod structs;
pub fn streams_menu(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
    sftp_client: &Sftp,
    stream: &Stream,
    session: &Session,
) {
    let mut cursor = crossterm_cursor::cursor();
    let curr_stream_path = get_parent_folder(stream.stream_details_path.as_str());
    let mut streamdetails = StreamDetails::default();
    if sftp_client.exist(stream.stream_details_path.as_ref()) {
        streamdetails =
            StreamDetails::from_remote_file(stream.stream_details_path.as_str(), sftp_client);
    } else {
        streamdetails.get_initialised_stream_details(stream);
    }
    if let Err(e) = cursor.save_position() {
        println!("{}", format!("{}", e).as_str().red().bold());
    }
    let mut wish_to_continue = true;
    let _count = 0;
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
            "Stream Name: {}",
            streamdetails.stream_name.as_str().blue().bold()
        );

        wish_to_continue = print_stream_options(
            config_params,
            logger,
            diag_logger,
            sftp_client,
            &mut streamdetails,
            stream,
            curr_stream_path.as_str(),
            session,
            &mut cursor,
        );
    }
    if sftp_client.exist(streamdetails.stream_details_path.as_ref()) {
        create_file_backup(streamdetails.stream_details_path.as_str(), sftp_client);
    }
    println!("writing below data: ");
    streamdetails.display();
    streamdetails.update_json(logger, sftp_client);
    if let Err(e) = cursor.restore_position() {
        println!("{}", format!("{}", e).as_str().red().bold());
    }
    println!("{}", clear::AfterCursor);
}
fn print_stream_options(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
    sftp_client: &Sftp,
    mut streamdetails: &mut StreamDetails,
    stream: &Stream,
    curr_stream_path: &str,
    session: &Session,
    cursor: &mut TerminalCursor,
) -> bool {
    // let mut empty_stream_details = StreamDetails::default();
    let _choice = "";
    let _input = String::new();
    if streamdetails.programs.is_empty() {
        println!("No programs Found, add New Programs!");
        get_fresh_menu(
            config_params,
            logger,
            diag_logger,
            sftp_client,
            curr_stream_path,
            &mut streamdetails,
            &stream,
        );
        return print_stream_options(
            config_params,
            logger,
            diag_logger,
            sftp_client,
            &mut streamdetails,
            stream,
            curr_stream_path,
            session,
            cursor,
        );
    } else {
        let mut push_menu: Vec<TerminalMenuItem> = Vec::new();
        push_menu.push(label("Below Programs Found"));
        for each_program in &streamdetails.programs {
            push_menu.push(button(each_program.to_string()));
        }
        // push_menu.push(button("Add a new Program"));
        push_menu.push(button("Upload all Programs"));
        push_menu.push(button("Back"));
        let programs_len = *&streamdetails.programs.len();
        let menu = menu(push_menu);
        match try_run_fancy(&menu) {
            Err(_e) => {}
            Ok(..) => {}
        }
        let chosen_index = selected_item_index(&menu);
        let chosen_name = selected_item_name(&menu);
        let mut return_value = true;
        if chosen_index <= programs_len {
            let _number = chosen_index;
            match get_program_from_number(&streamdetails.programs, chosen_name.as_str()) {
                None => {
                    println!("Wrong Stream Id, try again");
                    if let Err(e) = cursor.restore_position() {
                        println!("{}", format!("{}", e).as_str().red().bold());
                    }
                    println!("{}", clear::AfterCursor);
                    print_stream_options(
                        config_params,
                        logger,
                        diag_logger,
                        sftp_client,
                        &mut streamdetails,
                        stream,
                        curr_stream_path,
                        session,
                        cursor,
                    );
                }
                Some(pg) => {
                    cursor.restore_position();
                    println!("{}", clear::AfterCursor);
                    programs_menu(config_params, logger, diag_logger, sftp_client, pg, session);
                }
            }
        } else {
            if chosen_index == programs_len + 1 {
                upload_all_programs(&mut streamdetails, sftp_client, logger);
            } else if chosen_index == programs_len + 2 {
                return_value = false;
            }
            // else if chosen_index == programs_len + 1 {
            //     get_fresh_menu(
            //         config_params,
            //         logger,
            //         diag_logger,
            //         sftp_client,
            //         curr_stream_path,
            //         &mut streamdetails,
            //         &stream,
            //     );
            // }
            else {
                streamdetails.programs.clear();
                get_fresh_menu(
                    config_params,
                    logger,
                    diag_logger,
                    sftp_client,
                    curr_stream_path,
                    &mut streamdetails,
                    &stream,
                );
            }
        }
        return_value
    }
}
//to add a fresh Stream(mannual mode)
fn get_fresh_menu(
    _config_params: &ConfigurationParameters,
    _logger: &Logger,
    _diag_logger: &Logger,
    sftp_client: &Sftp,
    curr_stream_path: &str,
    streamdetails: &mut StreamDetails,
    curr_stream: &Stream,
) {
    streamdetails.get_initialised_stream_details(curr_stream);
    let mut prev_flow_name = String::new();
    loop {
        streamdetails.num_of_programs += 1;
        let mut cur_program = get_each_program(
            streamdetails.num_of_programs,
            curr_stream_path,
            sftp_client,
            &prev_flow_name,
            streamdetails,
        );
        cur_program.stream_name = streamdetails.stream_name.clone();
        cur_program.stream_id = streamdetails.stream_id;
        prev_flow_name = cur_program.flow_name.clone();
        streamdetails.programs.push(cur_program);
        if !prompt_continue_msg("Do you wish to add New Program: ") {
            break;
        }
    }
}
fn get_each_program(
    id: i32,
    curr_stream_path: &str,
    sftp_client: &Sftp,
    flow_name: &str,
    _streamdetails: &StreamDetails,
) -> Program {
    println!("Enter Program Details");
    let mut new_program = Program::default();
    new_program.program_name = read_input("Program name: ");
    if !flow_name.is_empty()
        && prompt_continue_msg(
            format!("Do you wish to use previous Flow name `{}`?", flow_name).as_str(),
        )
    {
        new_program.flow_name = flow_name.to_string();
    } else {
        new_program.flow_name = read_input("Flow name: ");
    }
    new_program.program_id = id;
    new_program.last_modified = get_current_time();
    new_program.folder_path = gen_req_folder_path(
        curr_stream_path,
        gen_program_folder_name(
            new_program.flow_name.as_str(),
            new_program.program_name.as_str(),
        )
        .as_str(),
    );
    if let Err(_e) = sftp_client.create_dir_all(new_program.folder_path.as_str().as_ref()) {}
    new_program.program_details_path =
        join_paths(new_program.folder_path.as_str(), "program_details.json");
    new_program
}
fn gen_program_folder_name(flow_name: &str, program_name: &str) -> String {
    let mut new_flow = flow_name.trim().replace(" ", "_");
    new_flow.push_str(program_name.trim().replace(" ", "_").as_str());
    new_flow
}

fn get_program_from_number<'a>(programs: &'a Vec<Program>, key: &str) -> Option<&'a Program> {
    for each_stream in programs.clone() {
        if each_stream.to_string().eq(key) {
            return Some(each_stream);
        }
    }
    None
}

pub fn upload_all_programs(streamdetails: &mut StreamDetails, sftp_client: &Sftp, logger: &Logger) {
    println!("\nRELOADING `{}`", streamdetails.stream_name);
    for each_program in &mut streamdetails.programs {
        println!("\nReloading `{}`", each_program.program_name);
        super::programs::upload_all_files(
            each_program.program_details_path.as_str(),
            sftp_client,
            logger,
        );
        each_program.last_modified = get_current_time();
    }
}
pub fn get_programs_from_path(
    _config_params: &ConfigurationParameters,
    logger: &Logger,
    _diag_logger: &Logger,
    sftp_client: &Sftp,
    curr_stream: &Stream,
    curr_streamdef: &StreamDef,
) {
    let curr_stream_path = get_parent_folder(curr_stream.stream_details_path.as_str());
    let mut streamdetails = StreamDetails::default();
    streamdetails.get_initialised_stream_details(curr_stream);
    let mut count = 0;
    for each_flow in &curr_streamdef.flows {
        let flow_name = each_flow.name.as_str();
        for each_process in &each_flow.process {
            count += 1;
            let mut curr_program = process_each_program_from_file(
                count,
                curr_stream_path.as_str(),
                sftp_client,
                flow_name,
                each_process,
            );
            curr_program.stream_name = curr_stream.stream_name.clone();
            curr_program.stream_id = curr_stream.stream_id.clone();
            get_files_from_path(
                _config_params,
                logger,
                _diag_logger,
                sftp_client,
                &curr_program,
                each_process.processBinary.as_str(),
            );
            streamdetails.programs.push(curr_program);
        }
    }
    streamdetails.update_json(logger, sftp_client);
}

pub fn process_each_program_from_file(
    id: i32,
    curr_stream_path: &str,
    sftp_client: &Sftp,
    flow_name: &str,
    processdef: &ProcDef,
) -> Program {
    let mut new_program = Program::default();
    new_program.program_name = processdef.processName.clone();
    new_program.program_id = id;
    new_program.flow_name = flow_name.to_string();
    new_program.last_modified = get_current_time();
    new_program.folder_path = gen_req_folder_path(
        curr_stream_path,
        gen_program_folder_name(
            new_program.flow_name.as_str(),
            new_program.program_name.as_str(),
        )
        .as_str(),
    );
    if let Err(_e) = sftp_client.create_dir_all(new_program.folder_path.as_str().as_ref()) {}

    new_program.program_details_path =
        join_paths(new_program.folder_path.as_str(), "program_details.json");
    new_program
}
