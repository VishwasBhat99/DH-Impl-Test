use std::fs::File;
use std::io::Write;

pub fn write_to_file(content:String, output_file: &mut File){
    output_file.write_all(content.as_bytes()).expect("unable to write to file");
}
