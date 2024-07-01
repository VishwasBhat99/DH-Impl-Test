use crate::configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use io::get_writer;
use protobuf::descriptor::FileDescriptorProto;
use protobuf::reflect::FileDescriptor;
use read_metadata::*;
use req_fields::*;
use slog::Logger;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;
use std::time::SystemTime;
use write_output::get_output;

mod acc_field_names;
mod io;
mod read_metadata;
mod req_fields;
mod write_output;

pub fn process(config_params: &mut ConfigurationParameters, _log: &Logger, _diag_log: &Logger) {
    let mut acc_encountered = 0;
    let mut acc_skipped = 0;
    let mut ip_map: HashMap<String, MetaDataFields> = HashMap::new();
    let mut op_map: HashMap<String, i32> = HashMap::new();
    let mut column_count = 0;
    let mut date_fields_vec: Vec<String> = Vec::new();
    let mut proto_str = "".to_string();

    read_input_metadata(
        config_params,
        _log,
        _diag_log,
        &mut ip_map,
        &mut column_count,
        &mut date_fields_vec,
    );

    get_output_fields(
        config_params,
        _log,
        _diag_log,
        &mut ip_map,
        &mut op_map,
        &mut proto_str,
    );

    let proto = format!("syntax = 'proto3'; {}", proto_str);
    let temp_dir = tempfile::tempdir().expect("Unable to get temp directory");
    let tempfile = temp_dir.path().join("temp.proto");
    fs::write(&tempfile, proto).expect("Error writing temp proto file");
    let mut file_descriptor_protos = protobuf_parse::Parser::new()
        .pure()
        .includes(&[temp_dir.path().to_path_buf()])
        .input(&tempfile)
        .parse_and_typecheck()
        .expect("Error getting FileDescriptorProto array")
        .file_descriptors;
    let file_descriptor_proto: FileDescriptorProto = file_descriptor_protos
        .pop()
        .expect("Error getting FileDescriptorProto");
    let file_descriptor: FileDescriptor = FileDescriptor::new_dynamic(file_descriptor_proto, &[])
        .expect("Error getting FileDescriptor");
    let mut message_descriptor = file_descriptor
        .message_by_package_relative_name("Output")
        .expect("Error getting MessageDescriptor");
    let mut cf_message_descriptor = file_descriptor
        .message_by_package_relative_name("Cashflow")
        .expect("Error getting CF MessageDescriptor");

    //Declare output.cf file
    let op_path = format!("{}.cf", config_params.output_file_path());
    let mut output_file = get_writer(&op_path);
    //Read Input-Master file
    let input = File::open(config_params.input_file_path()).expect("Could Not Read File");
    let mut _input_reader = BufReader::new(input);
    let ip_row = fs::read_to_string(config_params.input_file_path()).unwrap();
    let row_count = ip_row.lines().count();
    let mut read_timer = std::time::Duration::new(0, 0);
    let mut process_timer = std::time::Duration::new(0, 0);
    let mut write_timer = std::time::Duration::new(0, 0);
    let acc_field_names =
        acc_field_names::ReqFieldNames::new_from_path(&config_params.required_fields_file());

    for (_line_no, line) in _input_reader.lines().enumerate() {
        acc_encountered += 1;
        let t0 = SystemTime::now();

        let acc_info: String = match line {
            Ok(acc_info) => acc_info,
            Err(_error) => {
                acc_skipped += 1;
                continue;
            }
        };
        if _line_no < *config_params.skip_header_count() {
            debug!(
                _log,
                "Skipped Header `{}` at line_no `{}`",
                acc_info,
                _line_no + 1
            );
            continue;
        }
        if _line_no > row_count - *config_params.skip_footer_count() {
            debug!(
                _log,
                "Skipped Footer `{}` at line_no `{}`",
                acc_info,
                _line_no + 1
            );
            continue;
        }
        let input_fields: Vec<&str> = acc_info.split('|').collect();
        if input_fields.len() != column_count {
            error!(
                _log,
                "Invalid column count `{}` in line_no: `{}`",
                input_fields.len(),
                _line_no + 1
            );
            continue;
        }
        let t1 = SystemTime::now();
        let total_duration1 = t1
            .duration_since(t0)
            .expect("Could not calculate total duration.");
        get_output(
            &mut input_fields.to_owned(),
            &mut message_descriptor,
            &mut cf_message_descriptor,
            config_params,
            &mut op_map,
            &mut date_fields_vec,
            &mut output_file,
            &acc_field_names,
        );
        let t2 = SystemTime::now();
        let total_duration2 = t2
            .duration_since(t1)
            .expect("Could not calculate total duration.");

        let t3 = SystemTime::now();
        let total_duration3 = t3
            .duration_since(t2)
            .expect("Could not calculate total duration.");

        read_timer += total_duration1;
        process_timer += total_duration2;
        write_timer += total_duration3;
    }
    println!("Time to read Input File: {:?}", read_timer);
    println!("Time for Processing: {:?}", process_timer);
    println!("Time to write Output File: {:?}", write_timer);

    let health_report = HealthReport::new(
        acc_encountered as i64,
        acc_encountered - acc_skipped as i64,
        acc_skipped,
        0.0,
        0.0,
        acc_encountered - acc_skipped,
    );
    health_report.gen_health_rpt(config_params.output_file_path());
}
