use crate::configuration_parameters::ConfigurationParameters;
use protobuf::reflect::ReflectValueBox;
use protobuf::MessageDyn;
use sdb_dyn_proto_rdr::compound_types::Cashflow;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use std::io::BufWriter;

use std::fs::File;

use std::io::Write;

pub fn get_output(
    config_params: &mut ConfigurationParameters,
    mut cf_record: AccountWithCFs,
    message_descriptor: &mut protobuf::reflect::MessageDescriptor,
    cf_message_descriptor: &mut protobuf::reflect::MessageDescriptor,
    output_req: &mut std::collections::HashMap<std::string::String, String>,
    output_file: &mut BufWriter<File>,
) {
    let mut msg_desc_inst = message_descriptor.new_instance();
    let mut cf_msg_desc_inst = cf_message_descriptor.new_instance();
    let mut def_cf = Cashflow::new();
    def_cf.principal_amount = 0.00000001;
    def_cf.interest_amount = 0.0;
    def_cf.date = 0;
    let cf_val = match cf_record.remove_cfs_for_key(&"cashflows".to_string()) {
        Ok(value) => value,
        _ => vec![def_cf],
    };

    let mut cf_vec: Vec<Box<dyn MessageDyn>> = Vec::new();
    for val in cf_val.iter() {
        let cf_data_field: protobuf::reflect::FieldDescriptor = cf_message_descriptor
            .field_by_name("principal_amount")
            .expect("error getting field from CF MessageDescriptor");
        cf_data_field.set_singular_field(
            &mut *cf_msg_desc_inst,
            ReflectValueBox::F64(val.principal_amount),
        );
        let cf_data_field: protobuf::reflect::FieldDescriptor = cf_message_descriptor
            .field_by_name("interest_amount")
            .expect("error getting field from CF MessageDescriptor");
        cf_data_field.set_singular_field(
            &mut *cf_msg_desc_inst,
            ReflectValueBox::F64(val.interest_amount),
        );

        let cf_data_field: protobuf::reflect::FieldDescriptor = cf_message_descriptor
            .field_by_name("date")
            .expect("error getting field from CF MessageDescriptor");
        cf_data_field.set_singular_field(&mut *cf_msg_desc_inst, ReflectValueBox::I64(val.date));
        if val.principal_amount > 0.0 {
            cf_vec.push(cf_msg_desc_inst.to_owned());
        }
    }

    for val in cf_vec.iter() {
        let field_desc = message_descriptor.fields();
        for field in field_desc {
            
            let data_field = message_descriptor
                .field_by_name(field.name())
                .expect("error getting field from MessageDescriptor");
                if field.name() == "source_name" {
                    data_field.set_singular_field(&mut *msg_desc_inst, ReflectValueBox::String(config_params.source_name().to_string()));
                    continue;
                }

            let data_type = format!(
                "{:?}",
                field
                    .proto()
                    .type_
                    .expect("error getting type from FileDescriptor")
            );
            match data_type.as_str() {
                "TYPE_INT64" => {
                    data_field.set_singular_field(
                        &mut *msg_desc_inst,
                        ReflectValueBox::I64(
                            cf_record
                                .get_i64_for_key(
                                    output_req
                                        .get(&data_field.to_string())
                                        .unwrap_or(&"0".to_string()),
                                )
                                .unwrap_or(0),
                        ),
                    );
                }
                "TYPE_INT32" => {
                    data_field.set_singular_field(
                        &mut *msg_desc_inst,
                        ReflectValueBox::I32(
                            cf_record
                                .get_i32_for_key(
                                    output_req
                                        .get(&data_field.to_string())
                                        .unwrap_or(&"0".to_string()),
                                )
                                .unwrap_or(0),
                        ),
                    );
                }
                "TYPE_DOUBLE" => {
                    data_field.set_singular_field(
                        &mut *msg_desc_inst,
                        ReflectValueBox::F64(
                            cf_record
                                .get_f64_for_key(
                                    output_req
                                        .get(&data_field.to_string())
                                        .unwrap_or(&"0.0".to_string()),
                                )
                                .unwrap_or(0.0),
                        ),
                    );
                }
                "TYPE_FLOAT" => {
                    data_field.set_singular_field(
                        &mut *msg_desc_inst,
                        ReflectValueBox::F32(
                            cf_record
                                .get_f32_for_key(
                                    output_req
                                        .get(&data_field.to_string())
                                        .unwrap_or(&"0.0".to_string()),
                                )
                                .unwrap_or(0.0),
                        ),
                    );
                }
                "TYPE_STRING" => {
                    data_field.set_singular_field(
                        &mut *msg_desc_inst,
                        ReflectValueBox::String(
                            cf_record
                                .get_string_for_key(
                                    output_req
                                        .get(&data_field.to_string())
                                        .unwrap_or(&"".to_string()),
                                )
                                .unwrap_or(&"".to_string())
                                .to_string(),
                        ),
                    );
                }
                "TYPE_MESSAGE" => {
                    data_field.set_singular_field(
                        &mut *msg_desc_inst,
                        ReflectValueBox::Message(val.to_owned()),
                    );
                }
                _ => panic!("Invalid Datatype: {} Encountered", data_type),
            }
        }
        output_file
            .write_all(
                msg_desc_inst
                    .write_length_delimited_to_bytes_dyn()
                    .expect("Error in writing MessageDyn to output")
                    .as_slice(),
            )
            .expect("Error in writing Output File");
        }
}
