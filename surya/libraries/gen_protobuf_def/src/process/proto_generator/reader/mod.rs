use std::fs::File;
use std::io::BufReader;

pub fn reader(input_path: &str) -> BufReader<File> {
    let error_msg = format!("{} file not found", input_path);
    let input_file = File::open(input_path).expect(error_msg.as_str());
    BufReader::new(input_file)
}

pub fn read_json(input_path: &str) -> serde_json::Value {
    let error_msg = format!(
        "unable to read {}, file not found/incorrect json format",
        input_path
    );
    let json_value: serde_json::Value =
        serde_json::from_reader(File::open(input_path).expect(error_msg.as_str()))
            .expect("unable to parse json");
    json_value
}
