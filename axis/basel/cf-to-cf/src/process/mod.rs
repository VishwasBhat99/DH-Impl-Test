use self::io::get_writer;
use self::output_metadata::*;
use self::read_metadata::*;
use self::write_output::get_output;
use crate::configuration_parameters::ConfigurationParameters;

use health_report::HealthReport;
use protobuf::descriptor::FileDescriptorProto;
use protobuf::reflect::FileDescriptor;

use sdb_dyn_proto_rdr::reader;
use slog::Logger;
use std::collections::{BTreeMap, HashMap};
use std::fs;

mod io;
mod output_metadata;
mod read_metadata;
mod write_output;

pub fn process(config_params: &mut ConfigurationParameters, _log: &Logger, _diag_log: &Logger) {
    let mut acc_encountered = 0;
    let mut acc_successful = 0;
    let mut ip_map: BTreeMap<String, MetaDataFields> = BTreeMap::new();
    let mut op_map: BTreeMap<String, MetaDataFields> = BTreeMap::new();
    let mut output_req: HashMap<String, String> = HashMap::new();
    let mut column_count = 0;
    let mut date_fields_vec: Vec<String> = Vec::new();
    let mut proto_str = "".to_string();
    read_metadata(
        config_params.input_metadata_file(),
        _log,
        _diag_log,
        &mut ip_map,
        &mut column_count,
        &mut date_fields_vec,
    );

    read_metadata(
        config_params.output_metadata_file(),
        _log,
        _diag_log,
        &mut op_map,
        &mut column_count,
        &mut date_fields_vec,
    );

    get_req_fields(
        config_params,
        _log,
        _diag_log,
        &mut op_map,
        &mut output_req,
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

    let op_path = format!("{}.cf", config_params.output_file_path());
    let mut output_file = get_writer(&op_path);

    let mut input_reader = reader::Reader::new_at_path(
        config_params.input_metadata_file(),
        config_params.input_file_path(),
    );

    for input in input_reader.iter() {
        acc_encountered += 1;
        get_output(
            config_params,
            input,
            &mut message_descriptor,
            &mut cf_message_descriptor,
            &mut output_req,
            &mut output_file,
        );

        acc_successful += 1;
    }

    let health_report = HealthReport::new(
        acc_encountered,
        acc_successful,
        acc_encountered - acc_successful,
        0.0,
        0.0,
        0,
    );
    health_report.gen_health_rpt(config_params.output_file_path());
}
