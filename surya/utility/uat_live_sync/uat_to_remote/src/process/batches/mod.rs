pub mod structs;
use super::streams::streams_menu;
use colored::Colorize;
use configuration_parameters::ConfigurationParameters;
use crossterm_cursor::TerminalCursor;
use process::all_batches::structs::Batch;
use process::batches::structs::{BatchDetails, Stream, StreamDef};
use process::helper::{
    create_file_backup, gen_req_folder_path, get_all_json_from_dir, get_current_time,
    get_parent_folder, get_width, join_paths, prompt_continue_msg,
};
use process::io::read_input;
use process::streams::structs::StreamDetails;
use process::streams::{get_programs_from_path, upload_all_programs};
use slog::Logger;
use ssh2::{Session, Sftp};

use terminal_menu::*;
use termion::clear;
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
    let mut batchdetails = BatchDetails::default();
    if sftp_client.exist(batch.batch_details_path.as_ref()) {
        batchdetails =
            BatchDetails::from_remote_file(batch.batch_details_path.as_str(), sftp_client);
    } else {
        batchdetails.get_initialised_batch_details(batch);
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
            "Batch Name: {}",
            batchdetails.batch_name.as_str().blue().bold()
        );
        wish_to_continue = print_batches_options(
            config_params,
            logger,
            diag_logger,
            sftp_client,
            &mut batchdetails,
            batch,
            cur_batch_path.as_str(),
            session,
            &mut cursor,
        );
    }
    if sftp_client.exist(batchdetails.batch_details_path.as_ref()) {
        create_file_backup(batchdetails.batch_details_path.as_str(), sftp_client);
    }
    println!("writing below data: ");
    batchdetails.display();
    batchdetails.update_json(logger, sftp_client);
    if let Err(e) = cursor.restore_position() {
        println!("{}", format!("{}", e).as_str().red().bold());
    }
    println!("{}", clear::AfterCursor);
}

fn print_batches_options(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
    sftp_client: &Sftp,
    mut batchdetails: &mut BatchDetails,
    batch: &Batch,
    curr_batch_path: &str,
    session: &Session,
    cursor: &mut TerminalCursor,
) -> bool {
    let _choice = "";
    let _input = String::new();

    if batchdetails.streams.is_empty() {
        println!("\nNo Streams Found, add New Streams!");
        get_fresh_menu(
            config_params,
            logger,
            diag_logger,
            sftp_client,
            curr_batch_path,
            &mut batchdetails,
            &batch,
        );
        return print_batches_options(
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
    } else {
        let mut push_menu: Vec<TerminalMenuItem> = Vec::new();
        push_menu.push(label("Below Streams Found"));
        for each_stream in &batchdetails.streams {
            push_menu.push(button(each_stream.to_string()));
        }
        //for adding a new stream manually
        // push_menu.push(button("Add a new Stream(manual mode)"));
        push_menu.push(button("Upload all Streams"));
        push_menu.push(button("Re-Fetch all Streams"));
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
                    if let Err(e) = cursor.restore_position() {
                        println!("{}", format!("{}", e).as_str().red().bold());
                    }
                    println!("{}", clear::AfterCursor);
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
                upload_all_streams(&mut batchdetails, sftp_client, logger);
            } else if chosen_index == streams_len + 2 {
                let mut curr_batch = Batch::default();
                curr_batch.batch_id = batchdetails.batch_id;
                curr_batch.batch_details_path = batchdetails.batch_details_path.clone();
                curr_batch.batch_name = batchdetails.batch_name.clone();
                get_streams_from_path(
                    config_params,
                    logger,
                    diag_logger,
                    sftp_client,
                    &curr_batch,
                    &mut batchdetails,
                );
            } else if chosen_index == streams_len + 3 {
                return_value = false;
            }
            // else if chosen_index == streams_len + 1 {
            //     get_fresh_menu(
            //         config_params,
            //         logger,
            //         diag_logger,
            //         sftp_client,
            //         curr_batch_path,
            //         &mut batchdetails,
            //         &batch,
            //     );
            // }
            else {
                batchdetails.streams.clear();
                get_fresh_menu(
                    config_params,
                    logger,
                    diag_logger,
                    sftp_client,
                    curr_batch_path,
                    &mut batchdetails,
                    &batch,
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
    curr_batch_path: &str,
    batchdetails: &mut BatchDetails,
    _curr_batch: &Batch,
) {
    let mut streams_count = batchdetails.num_of_streams;
    loop {
        streams_count += 1;
        let cur_stream = get_each_stream(curr_batch_path, sftp_client);
        batchdetails.num_of_streams = streams_count;
        batchdetails.streams.push(cur_stream);
        if !prompt_continue_msg("Do you wish to add a New Stream: ") {
            break;
        }
    }
}
//get new stream(manual mode)
fn get_each_stream(curr_batch_path: &str, sftp_client: &Sftp) -> Stream {
    println!("Enter Stream Details");
    let mut new_stream = Stream::default();
    new_stream.stream_name = read_input("Stream name: ");
    new_stream.stream_id = read_input("Stream ID: ").parse().unwrap_or(0);
    new_stream.last_modified = get_current_time();
    let stream_path = gen_req_folder_path(curr_batch_path, new_stream.stream_name.as_str());
    if let Err(_e) = sftp_client.create_dir_all(stream_path.as_str().as_ref()) {}
    new_stream.stream_details_path = join_paths(stream_path.as_str(), "stream_details.json");
    new_stream
}

fn get_stream_from_number<'a>(streams: &'a Vec<Stream>, key: &str) -> Option<&'a Stream> {
    for each_stream in streams.clone() {
        if each_stream.to_string().eq(key) {
            return Some(each_stream);
        }
    }
    None
}

pub fn upload_all_streams(batchdetails: &mut BatchDetails, sftp_client: &Sftp, logger: &Logger) {
    println!("UPLOADING batch: `{}`", batchdetails.batch_name);
    for each_stream in &mut batchdetails.streams {
        if !sftp_client.exist(each_stream.stream_details_path.as_ref()) {
            println!(
                "{}",
                format!(
                    "File `{}` not Found, aborting Reload",
                    each_stream.stream_details_path
                )
                .as_str()
                .bright_red()
                .bold()
            );
            continue;
        }
        let mut streamdetails =
            StreamDetails::from_remote_file(each_stream.stream_details_path.as_str(), sftp_client);
        upload_all_programs(&mut streamdetails, sftp_client, logger);
        each_stream.last_modified = get_current_time();
    }
}

pub fn get_streams_from_path(
    _config_params: &ConfigurationParameters,
    logger: &Logger,
    _diag_logger: &Logger,
    sftp_client: &Sftp,
    curr_batch: &Batch,
    batchdetails: &mut BatchDetails,
) {
    batchdetails.num_of_streams = 0;
    batchdetails.streams.clear();
    batchdetails.get_initialised_batch_details(curr_batch);
    let all_streams_path =
        match get_all_json_from_dir(_config_params.all_streams_path(), curr_batch.batch_id) {
            None => {
                return;
            }
            Some(data) => data,
        };
    for each_stream_path in all_streams_path {
        let curr_streamdef = StreamDef::from_file(each_stream_path.as_str(), sftp_client, false);
        let curr_stream = process_each_stream_from_file(
            each_stream_path.as_str(),
            sftp_client,
            curr_batch,
            &curr_streamdef,
        );
        batchdetails.num_of_streams += 1;
        get_programs_from_path(
            _config_params,
            logger,
            _diag_logger,
            sftp_client,
            &curr_stream,
            &curr_streamdef,
        );
        batchdetails.streams.push(curr_stream);
    }
    println!("writing below data: ");
    batchdetails.display();
    batchdetails.update_json(logger, sftp_client);
}

pub fn process_each_stream_from_file(
    _stream_path: &str,
    _sftp_client: &Sftp,
    curr_batch: &Batch,
    curr_streamdef: &StreamDef,
) -> Stream {
    let curr_batch_path = get_parent_folder(curr_batch.batch_details_path.as_str());
    let stream_path = gen_req_folder_path(&*curr_batch_path, curr_streamdef.streamName.as_str());
    let mut new_stream = Stream::default();
    new_stream.stream_name = curr_streamdef.streamName.to_lowercase();
    new_stream.stream_id = curr_streamdef.streamId.parse().unwrap_or(0);
    new_stream.last_modified = get_current_time();
    new_stream.stream_details_path = join_paths(stream_path.as_str(), "stream_details.json");
    new_stream.streamdef_file_path = stream_path.clone();
    new_stream
}
