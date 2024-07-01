use super::io::*;
use crate::macros;
use slog::Logger;
use std::{fs::metadata, io::BufRead};

pub fn reading_scripts(file_path: &str, log: &Logger) -> Vec<String> {
    let mut op_line: Vec<String> = Vec::new();
    if metadata(file_path).is_err() {
        log_error!(log, "File: `{}` not present.", file_path);
        return vec![String::default()];
    }
    let reader = read_file(file_path);
    for (line_num, lines) in reader.lines().enumerate() {
        let line = extract_lines(line_num, lines, file_path);
        if line.contains("programs")
            && !line.contains(".sh")
            && !line.contains("#")
            && !line.contains("@")
        {
            let prog_path: Vec<&str> = line.split(" ").collect();
            let pos = prog_path
                .iter()
                .position(|&prog| prog.to_lowercase().contains("programs"))
                .expect("Error while getting program's path index.");
            op_line.push(prog_path[pos].to_string());
        } else if line.contains(".sh") && !line.contains("#") && !line.contains("@") {
            let script_path: Vec<&str> = line.split(" ").collect();
            let pos = script_path
                .iter()
                .position(|&script| script.to_lowercase().contains(".sh"))
                .expect("Error while getting script's path index.");
            op_line.push(script_path[pos].to_string());
            op_line.append(&mut reading_scripts(script_path[pos], &log));
        }
    }
    op_line
}
