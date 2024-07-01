use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::DateParser;
use slog::Logger;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process::Command;
use xml::reader::{EventReader, XmlEvent};

pub fn xml_to_txt(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    // Input file Reader
    let input_file = File::open(config_params.input_file_path()).unwrap();
    let inpur_reader = BufReader::new(input_file);
    let input_parser = EventReader::new(inpur_reader);

    // Output file Writer
    let mut output_file =
        File::open(config_params.log_file_path()).expect("Cannot access output file");
    let mut op_files: Vec<String> = Vec::new();

    // To parse only row tags from xml file
    let mut enable_row_parse = false;
    // To handle empty column in a row
    let mut enable_cell_parse = false;
    // To convert only date fields
    let mut is_date = false;
    let mut tot_acc_encntrd = 0;

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
                    for _ in 1..(attributes[0].value.parse::<i32>().unwrap() - counter - 1) {
                        output_file
                            .write(config_params.field_separator().as_bytes())
                            .expect("Error writing record data to output file.");
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
                    output.push_str(&text.replace("\n", ""));
                }
            }
            // Event: Tag Closing
            Ok(XmlEvent::EndElement { name }) => {
                if enable_cell_parse  {
                    counter += 1;
                    output.push_str(config_params.field_separator());
                    enable_cell_parse = false;
                }
                if name.local_name == "Row" {
                    counter = 0;
                    row_count += 1;
                    tot_acc_encntrd += 1;
                    output.pop(); // Remove last `|`
                    output_file
                        .write_all(output.as_bytes())
                        .expect("Cannot write to output file.");
                    output_file // Add a new line for each row
                        .write(b"\n")
                        .expect("Error adding new line to output file.");
                    enable_row_parse = false;
                    output.clear();
                }
            }
            // Event Error Handling
            Err(err) => {
                log_error!(logger, "Error: {}", err);
            }
            _ => {}
        }
    }

    // remove last empty line record
    tot_acc_encntrd -= 1;
    // if no records are present
    if tot_acc_encntrd < 0 {
        tot_acc_encntrd = 0;
    }

    // Clear output writer buffer
    output_file
        .flush()
        .expect("I/O Error or EOF Reached before flush");

    // Remove empty lines from output file
    for files in op_files {
        let status = Command::new("sed")
            .arg("-i")
            .arg("/^$/d")
            .arg(files)
            .status();
        match status {
            _exit_status => continue,
        }
    }

    let health_report = HealthReport::new(tot_acc_encntrd, tot_acc_encntrd, 0, 0.0, 0.0, 0);
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_params.output_file_path());
}
