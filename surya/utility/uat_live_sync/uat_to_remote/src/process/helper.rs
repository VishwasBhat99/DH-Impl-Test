use macros;

use process::structs::LoginDetails;
use serde::Serialize;
use slog::Logger;
use ssh2::{Session, Sftp};
use std::io::Write;
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fs, io};
use terminal_menu::*;

pub fn create_session(account: &LoginDetails, logger: &Logger) -> Session {
    let tcp_connection =
        TcpStream::connect(account.address.as_str()).expect("unable to create TCP Session");
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp_connection);
    sess.handshake().expect("unable to create handshake");
    sess.userauth_password(account.username.as_str(), account.password.as_str())
        .expect("unable to login, check account login details");
    log_info!(
        logger,
        "connected to {}@{}",
        account.username,
        account.address
    );
    println!("connected to {}@{}", account.username, account.address);
    if let Err(e) = sess.keepalive_send() {
        println!("{}", e);
    }
    sess
}
pub fn join_paths<'a>(first_path: &'a str, second_path: &'a str) -> String {
    let path: PathBuf = Path::new(first_path).join(second_path);
    let error_msg = format!("unable to join {} to {}", first_path, second_path);
    let path_str = path.to_str().expect(error_msg.as_str());
    path_str.to_owned()
}

pub fn get_current_time() -> String {
    chrono::offset::Local::now()
        .naive_local()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}

pub fn get_folder_name(name: &str) -> String {
    name.trim().replace(" ", "_").to_string()
}
pub fn prompt_continue_msg(to_console: &str) -> bool {
    let menu = menu(vec![
        //run the example and try these out
        label("(use arrow keys or wasd)"),
        label(to_console),
        button("Yes"),
        button("No"),
    ]);
    match try_run_fancy(&menu) {
        Err(_e) => {}
        Ok(..) => {}
    }
    let index = selected_item_index(&menu);
    if index == 2usize {
        return true;
    }
    return false;
}

pub fn gen_req_folder_path(curr_folder_path: &str, name: &str) -> String {
    join_paths(curr_folder_path, get_folder_name(name).as_str())
}
pub fn get_parent_folder(path: &str) -> String {
    let current_path = Path::new(path);
    let error_msg = format!("unable to get parent folder of `{}`", path);
    current_path
        .parent()
        .expect(error_msg.as_str())
        .as_os_str()
        .to_str()
        .expect(error_msg.as_str())
        .to_string()
}
// pub fn check_if_exsist(path: &str, sftp_client: &Sftp) -> bool {
//     match sftp_client.open(path.as_ref()) {
//         Ok(_) => true,
//         Err(_) => false,
//     }
// }

pub fn write_json<T>(path: &str, json_struct: &T, logger: &Logger, sftp: &Sftp)
where
    T: ?Sized + Serialize,
{
    let mut output_file = match sftp.create(path.as_ref()) {
        Ok(t) => t,
        Err(t) => {
            log_error!(logger, "error: {:?}", t);
            return;
        }
    };
    let val =
        serde_json::to_string_pretty(json_struct).expect("unable to convert  batchesinfo struct");
    output_file
        .write_all(val.as_bytes())
        .expect("unable to write batchesinfo struct");
}
pub fn get_actual_path(path: &str) -> String {
    let command = format!("echo {}", path);
    let execute = get_path_generic(command.as_str());
    let actual_path = String::from_utf8_lossy(&execute.stdout).to_string();
    actual_path.trim().to_string()
}

pub fn get_path_generic(command: &str) -> std::process::Output {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", command])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("failed to execute process")
    };
    output
}

pub fn get_file_name_from_path(path: &str) -> String {
    let file_path = Path::new(path);
    let file_name = file_path
        .file_name()
        .expect("no file found in path")
        .to_str()
        .expect("unable to get filename");
    file_name.to_string()
}

pub fn create_file_backup(path: &str, sftp_client: &Sftp) {
    let parent_folder = get_parent_folder(path);
    let file_name = get_file_name_from_path(path);
    let file_contents: Vec<&str> = file_name.split('.').collect();
    if file_contents.len() <= 1 {
        println!("invalid file name, skipping `{}`", path);
        return;
    }
    let mut bkp_file_name = file_contents[0].to_string();
    bkp_file_name.push_str("_bkp");
    bkp_file_name.push('.');
    let sufix = file_contents[1];
    bkp_file_name.push_str(sufix);
    let bkp_path = join_paths(parent_folder.as_str(), bkp_file_name.as_str());
    match sftp_client.copy(path, bkp_path.as_str(), true) {
        Ok(_) => {}
        Err(err) => println!("{}", err),
    }
}

// pub fn clear_screen() {
//     print!("\x1B[2J\x1B[1;1H");
// }
pub fn get_width() -> i32 {
    let size = termion::terminal_size().unwrap_or((0, 0));
    size.1 as i32 / 2
}

pub fn get_all_json_from_dir(directory_path: &str, batch_id: i32) -> Option<Vec<String>> {
    let files_entry = match fs::read_dir(directory_path) {
        Ok(entry) => {
            match entry
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, io::Error>>()
            {
                Ok(data) => data,
                Err(err) => {
                    println!("unable to get Dir file,skipping the File: `{}`", err);
                    return None;
                }
            }
        }
        Err(err) => {
            println!("unable to open Dir,skipping the File: `{}`", err);
            return None;
        }
    };
    let mut all_jsons: Vec<String> = Vec::new();
    for each_file in &files_entry {
        if each_file.is_dir() {
            continue;
        }
        let file_path = each_file.to_str().unwrap_or("");
        if file_path.is_empty() || !file_path.ends_with(".json") {
            continue;
        }
        let file_name_with_ext = get_file_name_from_path(file_path);
        let chunks: Vec<&str> = file_name_with_ext.split('.').collect();
        let mut file_name_only = chunks[0].to_string();
        let file_name_len = file_name_only.len();
        file_name_only.truncate(file_name_len - 3);
        let file_batch_id: i32 = file_name_only.parse().unwrap_or(0);
        if file_batch_id == 0 {
            continue;
        }
        if file_batch_id == batch_id {
            all_jsons.push(file_path.to_string());
        }
    }
    Some(all_jsons)
}
