use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::DateParser;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process::Command;
use xlsxwriter::Workbook;
use xml::reader::{EventReader, XmlEvent};

pub fn xml_to_xlsx(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    _diag_logger: &Logger,
) {
    // Input file Reader
    let input_file = File::open(config_params.input_file_path()).unwrap();
    let input_reader = BufReader::new(input_file);
    let input_parser = EventReader::new(input_reader);

    // Output file Writer
    let mut output_file =
        File::open(config_params.input_file_path()).expect("Cannot access output file");
    let mut op_files: Vec<String> = Vec::new();

    // To parse only row tags from xml file
    let mut enable_row_parse = false;
    // To handle empty column in a row
    let mut enable_cell_parse = false;
    // To exclude conversion of empty rows
    let mut is_valid_data = false;
    // To convert only date fields
    let mut is_date = false;

    let mut output = String::new();

    // Parse XML file
    let mut counter = 0;
    let mut row_count = 1;

    for event in input_parser {
        match event {
            // Event: Tag Opening
            Ok(XmlEvent::StartElement {
                name,
                attributes,
                namespace: _,
            }) => {
                if attributes.len() >= 1 && attributes[0].name.local_name == "Index" {
                    counter += 1;
                    if config_params.is_header_present() && row_count == 1 {
                        continue;
                    }
                    // handle empty cells
                    for _ in 0..(attributes[0].value.parse::<i32>().unwrap() - counter) {
                        output.push_str(config_params.field_separator());
                    }
                }

                // Extract data into separate files as per sheet names
                if name.local_name == "Worksheet" {
                    for attr in attributes {
                        let op_file =
                            format!("{}_{}.txt", config_params.output_file_path(), attr.value);
                        op_files.push(op_file.to_string());
                        output_file = File::create(op_file).expect("Cannot access output file");
                    }
                } else if name.local_name == "Row" {
                    enable_row_parse = true;
                } else if name.local_name == "Data" {
                    // A <Data/> Tag indicates a non-empty row
                    is_valid_data = true;
                    if attributes[0].value == "DateTime" {
                        is_date = true;
                    }
                }

                // To handle empty column within a <Row/> Tag
                if enable_row_parse && name.local_name == "Cell" {
                    enable_cell_parse = true;
                }
            }
            // Extract Value within a Tag
            Ok(XmlEvent::Characters(mut text)) => {
                if enable_row_parse {
                    if is_date {
                        // Convert DateTime type to required date format
                        let old_dt = text.replace("T00:00:00.000", "");
                        let date_parser = DateParser::new("%Y-%m-%d".to_string(), false);
                        let new_date = date_parser
                            .parse(&old_dt)
                            .format(config_params.date_format());
                        text = new_date.to_string();
                        is_date = false;
                    }
                    output.push_str(&text);
                }
            }
            // Event: Tag Closing
            Ok(XmlEvent::EndElement { name }) => {
                if enable_cell_parse && is_valid_data {
                    counter += 1;
                    output.push_str(config_params.field_separator());
                    enable_cell_parse = false;
                }
                if name.local_name == "Row" {
                    counter = 0;
                    row_count += 1;
                    output.pop(); // Remove last `|`
                    output_file
                        .write_all(output.as_bytes())
                        .expect("Cannot write to output file.");
                    output_file // Add a new line for each row
                        .write(b"\n")
                        .expect("Error adding new line to output file.");
                    enable_row_parse = false;
                    is_valid_data = false;
                    output.clear();
                }
            }
            // Event Error Handling
            Err(err) => {
                panic!("Invalid Event Error: {}", err);
            }
            _ => {}
        }
    }

    // Clear output writer buffer
    output_file
        .flush()
        .expect("I/O Error or EOF Reached before flush");

    let (output_path, filename) = config_params.output_file_path().rsplit_once('/').unwrap();

    let mut acc_enc = 0;
    let full_output_path = output_path.to_string() + "/" + &filename + ".xlsx";
    let workbook = Workbook::new(&full_output_path);

    for files in op_files.iter() {
        let input_file = match new_buf_rdr(&files) {
            Ok(file) => file,
            Err(error) => panic!("Could not find file `{}`: {}.", files, error),
        };
        let (_, sheet_name_ext) = files.rsplit_once("_").unwrap();
        let (sheet_name, _) = sheet_name_ext.split_once(".txt").unwrap();
        let mut sheet = workbook
            .add_worksheet(Some(sheet_name))
            .expect("Could not add sheet to excel.");
        for (line_num, lines) in input_file.lines().enumerate() {
            let mut y = 0;
            acc_enc += 1;
            let line = match lines {
                Ok(line) => line,
                Err(error) => {
                    error!(
                        logger,
                        "Unable to read file `{}` at line number: `{}` : `{}`",
                        files,
                        line_num + 1,
                        error
                    );
                    continue;
                }
            };
            let fields: Vec<&str> = line.split(config_params.field_separator()).collect();
            for input in fields.iter() {
                sheet
                    .write_string(line_num as u32, y, input, None)
                    .expect("Unable to write into sheet.");
                y += 1;
            }
        }
        if config_params.is_header_present() {
            acc_enc -= 1;
        }
        log_info!(
            logger,
            "Total accounts encountered in sheet `{}`: {}",
            sheet_name,
            acc_enc
        );

        // Remove intermediate text file created
        let status = Command::new("rm").arg(files).status();
        match status {
            _exit_status => continue,
        }
    }
    workbook.close().expect("Failed to close workbook.");
}
