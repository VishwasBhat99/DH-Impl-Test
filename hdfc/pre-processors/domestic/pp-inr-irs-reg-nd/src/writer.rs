use super::configuration_parameters::ConfigurationParameters;
use std::fs::File;
use std::io::Write;
//write txt_file
pub fn write_txt_file(file_path: String, data: Vec<Vec<String>>) -> std::io::Result<()> {
    let mut file = File::create(file_path).expect("file can not be write");
    let seperator = "|";
    for value in data {
        for val in value {
            file.write(val.as_bytes())
                .expect("As on and Customer Id can not be written");
            file.write(seperator.as_bytes())
                .expect("Seperator can not be written");
        }
        file.write("\n".as_bytes())
            .expect("new line can not be written");
    }
    Ok(())
}
