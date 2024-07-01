use super::*;

pub fn get_writer(file_path: &str) -> BufWriter<File> {
    match buf_file_wrtr(file_path, None) {
        Ok(file) => file,
        Err(error) => panic!("Unable to create file `{}` : `{}`", file_path, error),
    }
}

pub fn read_file(file_path: &str) -> BufReader<File> {
    match new_buf_rdr(file_path) {
        Ok(file) => file,
        Err(error) => panic!("Could not found file `{}` : `{}`.", file_path, error),
    }
}

pub fn open_file(file_path: &str) -> File {
    match open_file_read(file_path) {
        Ok(file) => file,
        Err(error) => panic!("Could not found file `{}` : `{}`.", file_path, error),
    }
}

pub fn extract_lines(
    line_num: usize,
    lines: Result<String, std::io::Error>,
    file_path: &str,
) -> String {
    match lines {
        Ok(line) => line,
        Err(error) => panic!(
            "Unable to read file `{}` at line number: `{}` : {}",
            file_path,
            line_num + 1,
            error
        ),
    }
}

pub fn write_output(static_params: &mut StaticParams, output: String) {
    match static_params.out_writer.write_all(output.as_bytes()) {
        Ok(_) => println!("Successfully processed all records."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`.{}",
            static_params.config_params.output_file_path(),
            error
        ),
    };
}
