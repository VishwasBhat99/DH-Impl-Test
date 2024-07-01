use super::proto::{Feild, Message};
use super::reader::read_json;
use super::writer::*;
use super::*;
use serde_json::Value;
use std::fs::File;

pub fn generate_protofile(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) {
    let json_value: Value = read_json(config_params.input_json_path());
    let message: Message =
        serde_json::from_value(json_value).expect("unable to get fields, check json format");
    write_protobuf(config_params, logger, diag_logger, &message);
}
pub fn write_protobuf(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
    message: &Message,
) {
    let mut output_file: File =
        File::create(config_params.output_proto_path()).expect("unable to create proto file");
    write_proto_structure(message.name.as_str(), &mut output_file);
    write_fields(message, &mut output_file);
    let close_braces: char = '}';
    write_to_file(format!("{}", close_braces), &mut output_file);
}
pub fn write_proto_structure(message_name: &str, output_file: &mut File) {
    write_to_file(get_proto_beginning(message_name), output_file);
}
pub fn get_proto_beginning(message_name: &str) -> String {
    let open_braces: char = '{';
    format!(
        "syntax = \"proto3\"; \nmessage {}{}\n",
        message_name, open_braces
    )
}
pub fn write_fields(message: &Message, output_file: &mut File) {
    for each_field in message.fields.iter() {
        let content = get_fields_stringify(each_field);
        write_to_file(content, output_file);
    }
}
pub fn get_fields_stringify(each_field: &Feild) -> String {
    format!(
        "\t{} {} = {};\n",
        each_field.typ.to_lowercase(),
        each_field.name,
        each_field.position
    )
}
