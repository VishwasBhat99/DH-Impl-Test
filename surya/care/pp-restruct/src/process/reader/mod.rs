use super::*;

pub fn reader(input_path: &str, logger: &Logger) -> Option<BufReader<File>> {
    let error_msg = format!("{} file not found", input_path);
    return match File::open(input_path) {
        Ok(t) => Some(BufReader::new(t)),
        Err(_) => {
            log_error!(logger, "{}", error_msg);
            None
        }
    };
}
