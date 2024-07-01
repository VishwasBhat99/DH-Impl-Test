use std::net::TcpStream;
use std::path::{Path, PathBuf};

use std::time::SystemTime;

use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use filetime::FileTime;

use slog::Logger;
use ssh2::Session;
use terminal_menu::*;

use macros;

use colored::Colorize;
use process::structs::LoginDetails;

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
    chrono::offset::Utc::now()
        .format("%Y-%m-%dT%H:%M:%S")
        .to_string()
}
pub fn set_curr_time_stamp(path: &str) {
    let curr_time = NaiveDateTime::parse_from_str(
        super::super::CURR_DATE_TIME.to_string().as_str(),
        "%Y-%m-%dT%H:%M:%S",
    )
    .unwrap();
    let dt = DateTime::<Utc>::from_utc(curr_time, Utc);
    let dtime = FileTime::from(SystemTime::from(dt));
    if let Err(e) = filetime::set_file_mtime(path, dtime) {
        println!(
            "{}",
            format!("Err:unable to set timestamp, `{}`", e)
                .as_str()
                .red()
        )
    }
    if let Err(e) = filetime::set_file_atime(path, dtime) {
        println!(
            "{}",
            format!("Err:unable to set timestamp, `{}`", e)
                .as_str()
                .red()
        )
    }
}
pub fn verify_if_different_timestamp(path: &str) -> bool {
    let filemetadata = std::fs::metadata(path).unwrap();
    let mtime = FileTime::from_last_modification_time(&filemetadata);
    let file_datetime = NaiveDateTime::from_timestamp(mtime.seconds(), 0);
    let curr_time = NaiveDateTime::parse_from_str(
        super::super::CURR_DATE_TIME.to_string().as_str(),
        "%Y-%m-%dT%H:%M:%S",
    )
    .unwrap();
    if curr_time.signed_duration_since(file_datetime) == Duration::seconds(0) {
        return false;
    } else {
        return true;
    }
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
pub fn check_if_exist(path: &str) -> bool {
    Path::new(path).exists()
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

pub fn create_file_backup(path: &str) {
    let parent_folder = get_parent_folder(path);
    let file_name = get_file_name_from_path(path);
    let mut file_contents: Vec<&str> = file_name.split('.').collect();
    if file_contents.len() <= 1 {
        file_contents.push("");
    }
    let mut bkp_file_name = file_contents[0].to_string();
    bkp_file_name.push_str(format!("_bkp_{}", super::super::CURR_DATE_TIME.to_string()).as_str());
    if !file_contents[1].is_empty() {
        bkp_file_name.push('.');
    }
    let sufix = file_contents[1];
    bkp_file_name.push_str(sufix);
    let bkp_path = join_paths(parent_folder.as_str(), bkp_file_name.as_str());
    if let Err(err) = std::fs::rename(path, bkp_path.as_str()) {
        println!("{}", err)
    }
}
pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
pub fn get_width() -> i32 {
    let size = termion::terminal_size().unwrap_or((0, 0));
    size.1 as i32 / 2
}
