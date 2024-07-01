use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use health_report::HealthReport;
use io::get_writer;
use protobuf::descriptor::FileDescriptorProto;
use protobuf::reflect::FileDescriptor;
use rbdate::DateParser;
use read_metadata::*;
use req_fields::*;
use sdb_dyn_proto_rdr::compound_types::Cashflow;
use sdb_dyn_proto_rdr::reader;
use slog::Logger;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use write_output::get_output;
//mod get_cf_fields;
mod acc_field_names;
mod io;
mod read_metadata;
mod req_fields;
mod write_output;

pub fn process(config_params: &mut ConfigurationParameters, _log: &Logger, _diag_log: &Logger) {
    let mut acc_encountered = 0;
    let mut acc_successful = 0;
    let mut ip_map: HashMap<String, MetaDataFields> = HashMap::new();
    let mut op_map: HashMap<String, String> = HashMap::new();
    let mut column_count = 0;
    let mut date_fields_vec: Vec<String> = Vec::new();
    let mut proto_str = "".to_string();
    let date_parser = DateParser::new("%d-%m-%Y".to_string(), true);
    read_input_metadata(
        config_params,
        _log,
        _diag_log,
        &mut ip_map,
        &mut column_count,
        &mut date_fields_vec,
    );

    //get_cf_fields(config_params, _log, _diag_log, &mut ip_map, &mut cf_vec);

    get_req_fields(
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
    //Read Input-Cashflow file
    let input = File::open(config_params.input_file_path()).expect("Could Not Read File");
    let mut _input_reader = BufReader::new(input);

    let mut cf_reader = reader::Reader::new_at_path(
        config_params.input_metadata_file(),
        config_params.input_file_path(),
    );
    let mut cf_reader_cf_lvl = reader::Reader::new_at_path(
        config_params.input_metadata_file(),
        config_params.input_file_path(),
    );

    let acc_field_names =
        acc_field_names::ReqFieldNames::new_from_path(&config_params.required_fields_file);

    let record_reader = cf_reader.iter();
    let record_reader_cf_lvl = cf_reader_cf_lvl.iter();

    let mut cfs_map: HashMap<_, Vec<Cashflow>> = HashMap::new();
    if config_params.is_cf_lvl_data() {
        for mut record in record_reader_cf_lvl {
            let cf_val = match record.remove_cfs_for_key(&"cashflows".to_string()) {
                Ok(value) => value,
                Err(_error) => panic!("Failed to extract cashflows from the record"),
            };
            let acc_id = record
                .get_string_for_key(&acc_field_names.acc_id.to_string())
                .expect("Could not get Acc-ID")
                .clone();
            cfs_map
                .entry(acc_id)
                .and_modify(|data| data.push(cf_val[0].clone()))
                .or_insert(cf_val);
        }
    }
    for record in record_reader {
        acc_encountered += 1;
        let acc_id = record
            .get_string_for_key(&acc_field_names.acc_id.to_string())
            .expect("Could not get Acc-ID")
            .clone();
        if config_params.is_cf_lvl_data() && !cfs_map.contains_key(&acc_id.to_string()) {
            log_warn!(
                _log,
                "`is-cf-lvl-data` flag  is true, Skipping account: {:?} which occurred more than once",
                acc_id
            );
            continue;
        }

        get_output(
            record,
            &mut message_descriptor,
            &mut cf_message_descriptor,
            config_params,
            &mut op_map,
            &mut output_file,
            &acc_field_names,
            &date_parser,
            &mut cfs_map,
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
