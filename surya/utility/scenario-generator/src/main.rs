extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate sdb_io;
extern crate serde_json;
extern crate termion;

use std::io::Write;
use stream::get_stream_info;
use structs::StreamDef;
use termion::{clear, cursor};

mod flow;
mod process;
mod screens;
mod stream;
mod structs;
fn main() {
    // Clear terminal screen
    println!("{}", clear::All);
    cursor::Goto(1, 1);
    let welcome_message = "Scenario Generator 1.0";
    println!("{:^130}", welcome_message);

    let scenario: StreamDef = get_stream_info();

    let output = serde_json::to_string_pretty(&scenario).expect("Cannot parse scenario as JSON.");
    let op_path = "scenario.json";
    let mut writer = match sdb_io::buf_file_wrtr(&op_path, None) {
        Ok(writer) => writer,
        Err(error) => panic!(format!(
            "Cannot write to file at path: '{}'. Error: {}",
            op_path, error
        )),
    };
    writer.write(&output.as_bytes()).expect("writer error");
    println!("Scenario JSON created successfully!!");
}
