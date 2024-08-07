use crate::configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use io::get_writer;
use protobuf::descriptor::FileDescriptorProto;
use protobuf::reflect::FileDescriptor;
use rbdate::{dcr_dt_by_days, get_days_from_month, is_month_end_date, DateParser};
use read_metadata::*;
use req_fields::*;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::account_with_cfs::get_field_value;
use slog::Logger;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
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
    let mut eomonth_key_set: HashSet<String> = HashSet::new();
    let mut op_map: HashMap<String, String> = HashMap::new();
    let mut column_count = 0;
    let mut date_fields_vec: Vec<String> = Vec::new();
    let mut proto_str = "".to_string();
    let date_parser = DateParser::new("%d-%m-%Y".to_string(), true);

    //Check the input data is for EOMONTH:
    if !is_month_end_date(config_params.as_on_date) {
        panic!("Entered Date is not a EOMONTH date");
    }
    //read input_metadata & req_fields:
    read_input_metadata(
        config_params,
        _log,
        _diag_log,
        &mut ip_map,
        &mut column_count,
        &mut date_fields_vec,
    );
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

        let acc_field_names =
        acc_field_names::ReqFieldNames::new_from_path(&config_params.required_fields_file);
   
    //Declare output.cf file
    let op_path = format!("{}.cf", config_params.output_file_path());
    let mut output_file = get_writer(&op_path);

    //reading EOMONTH date .cf:
    let mut input_reader = reader::Reader::new_at_path(
        config_params.input_metadata_file(),
        config_params.input_file_path(),
    );
    let input_method_reader = reader::Reader::new_at_path(
        config_params.input_metadata_file(),
        config_params.input_file_path(),
    );
    //adding existing accounts to the HashSet:
    for input in input_reader.iter() {
        let lookup_val = get_field_value(
            &input,
            &input_method_reader,
            acc_field_names.lookup_value.to_string(),
        )
        .unwrap_or("NA".to_string());
        eomonth_key_set.insert(lookup_val.clone());

        get_output(
            input,
            &mut message_descriptor,
            &mut cf_message_descriptor,
            &mut op_map,
            &mut output_file,
            &acc_field_names,
            &date_parser,
            false
        );
    }
    //Back-track -> prev_date to 1st of month:
    let no_of_days = get_days_from_month(config_params.as_on_date);
    for i in 1..no_of_days {
        let curr_date = dcr_dt_by_days(config_params.as_on_date, i);
        let prefix_str = &config_params.date_prefix;
        let suffix_str = &config_params.date_suffix;

        let curr_date_str = curr_date.format("%d%m%Y");
        let new_input_file_path = format!("{}{}{}", prefix_str, curr_date_str, suffix_str);

        //read input_file path:
        let path_exists = Path::new(&new_input_file_path).exists();
        if !path_exists {
            info!(
                _log,
                "Missing file for date: {}", curr_date_str
            );       
            continue;
        }
        //read_file for new_date:
        let mut account_reader =
            reader::Reader::new_at_path(config_params.input_metadata_file(), &new_input_file_path);
        let method_reader =
            reader::Reader::new_at_path(config_params.input_metadata_file(), &new_input_file_path);

        for account in account_reader.iter() {
            acc_encountered += 1;
            let lookup_val = get_field_value(
                &account,
                &method_reader,
                acc_field_names.lookup_value.to_string(),
            )
            .unwrap_or("NA".to_string());

            if eomonth_key_set.contains(&lookup_val) {
                continue;
            }
            get_output(
                account,
                &mut message_descriptor,
                &mut cf_message_descriptor,
                &mut op_map,
                &mut output_file,
                &acc_field_names,
                &date_parser,
                true,
            );
            eomonth_key_set.insert(lookup_val);
            acc_successful += 1;
        }
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
