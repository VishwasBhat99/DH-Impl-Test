use super::acc_field_names::ReqFieldNames;
use protobuf::reflect::ReflectValueBox;
use protobuf::MessageDyn;
use rbdate::{timestamp, DateParser};
use sdb_dyn_proto_rdr::compound_types::Cashflow;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

pub fn get_output(
    mut cf_record: AccountWithCFs,
    message_descriptor: &mut protobuf::reflect::MessageDescriptor,
    cf_message_descriptor: &mut protobuf::reflect::MessageDescriptor,
    op_map: &mut std::collections::HashMap<std::string::String, std::string::String>,
    output_file: &mut BufWriter<File>,
    acc_field_names: &ReqFieldNames,
    date_parser: &DateParser,
    is_missing_acc: bool,
) {
    let mut msg_desc_inst = message_descriptor.new_instance();
    let mut cf_msg_desc_inst = cf_message_descriptor.new_instance();
    let default_cf:Cashflow = Cashflow::new();
    let mut cf_val = match cf_record.remove_cfs_for_key(&"cashflows".to_string()) {
        Ok(value) => value,
        Err(_error) => vec![default_cf],
    };

    let mut final_cf = Cashflow::new();
    if is_missing_acc {
        let default_date = timestamp(date_parser.parse("01-01-1990"));

        final_cf.principal_amount = 0.0;
        final_cf.interest_amount = 0.0;
        final_cf.date = default_date;

        cf_val.clear();
        cf_val.push(final_cf);
    }

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
        cf_vec.push(cf_msg_desc_inst.to_owned());
    }

    for val in cf_vec.iter() {
        let field_desc = message_descriptor.fields();
        for field in field_desc {
            let data_field = message_descriptor
                .field_by_name(field.name())
                .expect("error getting field from MessageDescriptor");

            if is_missing_acc && field.name() == acc_field_names.outstanding_amount {
                data_field.set_singular_field(&mut *msg_desc_inst, ReflectValueBox::F64(0.0));
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
                                    &*op_map
                                        .get(&data_field.to_string())
                                        .unwrap_or(&"NA".to_string()),
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
                                    &*op_map
                                        .get(&data_field.to_string())
                                        .unwrap_or(&"NA".to_string()),
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
                                    &*op_map
                                        .get(&data_field.to_string())
                                        .unwrap_or(&"NA".to_string()),
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
                                    &*op_map
                                        .get(&data_field.to_string())
                                        .unwrap_or(&"NA".to_string()),
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
                                    &*op_map
                                        .get(&data_field.to_string())
                                        .unwrap_or(&"NA".to_string()),
                                )
                                .unwrap_or(&"NA".to_string())
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
