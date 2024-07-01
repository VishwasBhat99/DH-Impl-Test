use std::io;
use termion::{clear, cursor};

pub fn display_flow_screen() -> String {
    // Clear terminal screen
    println!("{}", clear::All);
    cursor::Goto(1, 1);
    println!("Select action:");
    println!("1. Display all flows.");
    println!("2. Add a new flow.");
    println!("3. Generate scenario file.");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Error: Invalid selection from user.");
    choice
}

pub fn display_process_screen() -> String {
    // Clear terminal screen
    println!("{}", clear::All);
    println!("Select action:");
    println!("1. Display all Processes.");
    println!("2. Add a new Process.");
    println!("3. Go Back to Flow Screen.");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Error: Invalid selection from user.");
    choice
}
