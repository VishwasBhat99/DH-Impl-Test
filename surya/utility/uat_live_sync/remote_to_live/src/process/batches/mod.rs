use colored::Colorize;
use crossterm_cursor::TerminalCursor;
use slog::Logger;
use ssh2::{Session, Sftp};
use terminal_menu::*;
use termion::clear;

use configuration_parameters::ConfigurationParameters;
use process::all_batches::structs::Batch;
use process::batches::structs::{BatchDetails, Stream};
use process::helper::{clear_screen, get_parent_folder, get_width};

use process::streams::structs::StreamDetails;
use process::streams::{download_all_programs, restore_all_programs};

use super::streams::streams_menu;

pub mod structs;

pub fn batches_menu(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
    sftp_client: &Sftp,
    batch: &Batch,
    session: &Session,
) {
    let mut cursor = crossterm_cursor::cursor();
    let cur_batch_path = get_parent_folder(batch.batch_details_path.as_str());
    let batchdetails;
    if sftp_client.exist(batch.batch_details_path.as_ref()) {
        batchdetails =
            BatchDetails::from_remote_file(batch.batch_details_path.as_str(), sftp_client);
    } else {
        println!(
            "{}",
            "Batch Details Config Not File, please Create one using uat_to_remote application"
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
        "Batch Name: {}",
        batchdetails.batch_name.as_str().blue().bold()
    );
    if let Err(e) = cursor.save_position() {
        println!("{}", format!("{}", e).as_str().red().bold());
    }
    if batchdetails.streams.is_empty() {
        println!(
            "{}\n{}",
            "No Streams Found, please add Streams and restart".blue(),
            "Force Exit for Batch initiated".red().bold()
        );
        return;
    }
    let mut wish_to_continue = true;
    while wish_to_continue {
        wish_to_continue = print_batches_options(
            config_params,
            logger,
            diag_logger,
            sftp_client,
            &batchdetails,
            batch,
            cur_batch_path.as_str(),
            session,
            &mut cursor,
        );
    }
}

fn print_batches_options(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
    sftp_client: &Sftp,
    batchdetails: &BatchDetails,
    batch: &Batch,
    curr_batch_path: &str,
    session: &Session,
    cursor: &mut TerminalCursor,
) -> bool {
    let mut push_menu: Vec<TerminalMenuItem> = Vec::new();
    push_menu.push(label("Below Streams Found"));
    for each_stream in &batchdetails.streams {
        push_menu.push(button(each_stream.to_string()));
    }
    push_menu.push(button("Download all Streams"));
    push_menu.push(button("Restore all Streams"));
    push_menu.push(button("Back"));
    let streams_len = *&batchdetails.streams.len();
    let menu = menu(push_menu);
    match try_run_fancy(&menu) {
        Err(_e) => {}
        Ok(..) => {}
    }
    let chosen_index = selected_item_index(&menu);
    let chosen_name = selected_item_name(&menu);
    let mut return_value = true;
    if chosen_index <= streams_len {
        let _number = chosen_index;
        match get_stream_from_number(&batchdetails.streams, chosen_name.as_str()) {
            None => {
                println!("Wrong Stream Id, try again");
                print_batches_options(
                    config_params,
                    logger,
                    diag_logger,
                    sftp_client,
                    batchdetails,
                    batch,
                    curr_batch_path,
                    session,
                    cursor,
                );
            }
            Some(st) => {
                if let Err(e) = cursor.restore_position() {
                    println!("{}", format!("{}", e).as_str().red().bold());
                }
                println!("{}", clear::AfterCursor);
                streams_menu(config_params, logger, diag_logger, sftp_client, st, session);
            }
        }
    } else {
        if chosen_index == streams_len + 1 {
            download_all_streams(&batchdetails, sftp_client, logger);
        } else if chosen_index == streams_len + 3 {
            return_value = false;
        } else if chosen_index == streams_len + 2 {
            restore_all_streams(&batchdetails, sftp_client, logger);
        } else {
            unimplemented!()
        }
    }
    return_value
}

fn get_stream_from_number<'a>(streams: &'a Vec<Stream>, key: &str) -> Option<&'a Stream> {
    for each_stream in streams.clone() {
        if each_stream.to_string().eq(key) {
            return Some(each_stream);
        }
    }
    None
}

pub fn download_all_streams(batchdetails: &BatchDetails, sftp_client: &Sftp, logger: &Logger) {
    println!("Downloading Batch `{}`", batchdetails.batch_name);
    for each_stream in &batchdetails.streams {
        if !sftp_client.exist(each_stream.stream_details_path.as_ref()) {
            println!(
                "File `{}` not Found, aborting Download",
                each_stream.stream_details_path
            );
            continue;
        }
        let mut streamdetails =
            StreamDetails::from_remote_file(each_stream.stream_details_path.as_str(), sftp_client);
        download_all_programs(&mut streamdetails, sftp_client, logger);
    }
}

pub fn restore_all_streams(batchdetails: &BatchDetails, sftp_client: &Sftp, logger: &Logger) {
    println!("Restoring Batch `{}`", batchdetails.batch_name);
    for each_stream in &batchdetails.streams {
        if !sftp_client.exist(each_stream.stream_details_path.as_ref()) {
            println!(
                "File `{}` not Found, aborting Download",
                each_stream.stream_details_path
            );
            continue;
        }
        let mut streamdetails =
            StreamDetails::from_remote_file(each_stream.stream_details_path.as_str(), sftp_client);
        restore_all_programs(&mut streamdetails, sftp_client, logger);
    }
}
