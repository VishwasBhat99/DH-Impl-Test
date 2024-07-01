use colored::Colorize;
use crossterm_cursor::TerminalCursor;
use slog::Logger;
use ssh2::{Session, Sftp};
use terminal_menu::*;
use termion::clear;

use configuration_parameters::ConfigurationParameters;
use process::batches::structs::Stream;
use process::helper::{get_parent_folder, get_width};

use process::programs::programs_menu;

use process::streams::structs::{Program, StreamDetails};

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
    let streamdetails;
    if sftp_client.exist(stream.stream_details_path.as_ref()) {
        streamdetails =
            StreamDetails::from_remote_file(stream.stream_details_path.as_str(), sftp_client);
    } else {
        println!(
            "{}",
            "STream Details Config Not File, please Create one using uat_to_remote application"
                .red()
                .bold()
        );
        return;
    }
    let width = get_width() as u16;
    if let Err(e) = cursor.move_right(width) {
        println!("{}", format!("{}", e).as_str().red().bold());
    }
    println!(
        "Stream Name: {}",
        streamdetails.stream_name.as_str().blue().bold()
    );
    if let Err(e) = cursor.save_position() {
        println!("{}", format!("{}", e).as_str().red().bold());
    }
    if streamdetails.programs.is_empty() {
        println!(
            "{}\n{}",
            "No Programs Found, please add Programs and restart".blue(),
            "Force Exit Stream initiated".red().bold()
        );
        return;
    }
    let mut wish_to_continue = true;
    let _count = 0;
    while wish_to_continue {
        wish_to_continue = print_stream_options(
            config_params,
            logger,
            diag_logger,
            sftp_client,
            &streamdetails,
            stream,
            curr_stream_path.as_str(),
            session,
            &mut cursor,
        );
    }
}
fn print_stream_options(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
    sftp_client: &Sftp,
    streamdetails: &StreamDetails,
    stream: &Stream,
    curr_stream_path: &str,
    session: &Session,
    cursor: &mut TerminalCursor,
) -> bool {
    println!("Below Programs Found \n");
    let mut push_menu: Vec<TerminalMenuItem> = Vec::new();
    push_menu.push(label("Below Programs Found"));
    for each_program in &streamdetails.programs {
        push_menu.push(button(each_program.to_string()));
    }
    push_menu.push(button("Download all Programs"));
    push_menu.push(button("Restore all Programs"));
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
                println!("Wrong Program Id, try again");
                print_stream_options(
                    config_params,
                    logger,
                    diag_logger,
                    sftp_client,
                    &streamdetails,
                    stream,
                    curr_stream_path,
                    session,
                    cursor,
                );
            }
            Some(pg) => {
                if let Err(e) = cursor.restore_position() {
                    println!("{}", format!("{}", e).as_str().red().bold());
                }
                println!("{}", clear::AfterCursor);
                programs_menu(config_params, logger, diag_logger, sftp_client, pg, session);
            }
        }
    } else {
        if chosen_index == programs_len + 1 {
            download_all_programs(&streamdetails, sftp_client, logger);
        } else if chosen_index == programs_len + 3 {
            return_value = false;
        } else if chosen_index == programs_len + 2 {
            restore_all_programs(&streamdetails, sftp_client, logger);
        } else {
            unimplemented!()
        }
    }
    return_value
}

fn get_program_from_number<'a>(programs: &'a Vec<Program>, key: &str) -> Option<&'a Program> {
    for each_stream in programs.clone() {
        if each_stream.to_string().eq(key) {
            return Some(each_stream);
        }
    }
    None
}

pub fn download_all_programs(streamdetails: &StreamDetails, sftp_client: &Sftp, logger: &Logger) {
    println!("\nDownloading Stream `{}`", streamdetails.stream_name);
    for each_program in &streamdetails.programs {
        println!("\nDownloading Program `{}`", each_program.program_name);
        super::programs::download_all_files(
            each_program.program_details_path.as_str(),
            sftp_client,
            logger,
        );
    }
}

pub fn restore_all_programs(streamdetails: &StreamDetails, sftp_client: &Sftp, logger: &Logger) {
    println!("\n\tRestoring Stream `{}`", streamdetails.stream_name);
    for each_program in &streamdetails.programs {
        println!("\n\t\tRestoring Program`{}`", each_program.program_name);
        super::programs::restore_all_files(
            each_program.program_details_path.as_str(),
            sftp_client,
            logger,
        );
    }
}
