use crate::configuration_parameters::ConfigurationParameters;
use crate::statics::{DEFAULT_FLOAT, DEFAULT_FLOAT_32, DEFAULT_INT, DEFAULT_INT_64};
use protobuf::reflect::ReflectValueBox;

pub fn get_output(
    input_vec: &mut [&str],
    message_descriptor: &mut protobuf::reflect::MessageDescriptor,
    config_params: &mut ConfigurationParameters,
    op_map: &mut std::collections::HashMap<std::string::String, i32>,
    date_fields_vec: &mut [String],
) -> std::boxed::Box<dyn protobuf::MessageDyn> {
    let mut msg_desc_inst = message_descriptor.new_instance();
    let field_desc = message_descriptor.fields();

    for field in field_desc {
        let data_field = message_descriptor
            .field_by_name(field.name())
            .expect("error getting field from MessageDescriptor");
        let data_type = format!(
            "{:?}",
            field
                .proto()
                .type_
                .expect("error getting type from FileDescriptor")
        );
        if date_fields_vec.contains(&data_field.to_string()) {
            data_field.set_singular_field(
                &mut *msg_desc_inst,
                ReflectValueBox::I64(
                    rbdate::timestamp(
                        rbdate::NaiveDate::parse_from_str(
                            input_vec[*op_map.get(&data_field.to_string()).unwrap_or(&DEFAULT_INT)
                                as usize
                                - 1],
                            config_params.input_date_format(),
                        )
                        .unwrap_or(*config_params.as_on_date()),
                    )
                    .to_string()
                    .parse::<i64>()
                    .unwrap_or(DEFAULT_INT_64),
                ),
            );
        } else {
            match data_type.as_str() {
                "TYPE_INT64" => {
                    data_field.set_singular_field(
                        &mut *msg_desc_inst,
                        ReflectValueBox::I64(
                            input_vec[*op_map.get(&data_field.to_string()).unwrap_or(&DEFAULT_INT)
                                as usize
                                - 1]
                            .parse::<i64>()
                            .unwrap_or(DEFAULT_INT_64),
                        ),
                    );
                }
                "TYPE_INT32" => {
                    data_field.set_singular_field(
                        &mut *msg_desc_inst,
                        ReflectValueBox::I32(
                            input_vec[*op_map.get(&data_field.to_string()).unwrap_or(&DEFAULT_INT)
                                as usize
                                - 1]
                            .parse::<i32>()
                            .unwrap_or(DEFAULT_INT),
                        ),
                    );
                }
                "TYPE_DOUBLE" => {
                    data_field.set_singular_field(
                        &mut *msg_desc_inst,
                        ReflectValueBox::F64(
                            input_vec[*op_map.get(&data_field.to_string()).unwrap_or(&DEFAULT_INT)
                                as usize
                                - 1]
                            .parse::<f64>()
                            .unwrap_or(DEFAULT_FLOAT),
                        ),
                    );
                }
                "TYPE_FLOAT" => {
                    data_field.set_singular_field(
                        &mut *msg_desc_inst,
                        ReflectValueBox::F32(
                            input_vec[*op_map.get(&data_field.to_string()).unwrap_or(&DEFAULT_INT)
                                as usize
                                - 1]
                            .parse::<f32>()
                            .unwrap_or(DEFAULT_FLOAT_32),
                        ),
                    );
                }
                "TYPE_STRING" => {
                    data_field.set_singular_field(
                        &mut *msg_desc_inst,
                        ReflectValueBox::String(
                            if !input_vec[*op_map
                                .get(&data_field.to_string())
                                .unwrap_or(&DEFAULT_INT)
                                as usize
                                - 1]
                            .to_string()
                            .is_empty()
                            {
                                input_vec[*op_map
                                    .get(&data_field.to_string())
                                    .unwrap_or(&DEFAULT_INT)
                                    as usize
                                    - 1]
                                .to_string()
                            } else {
                                " ".to_string()
                            },
                        ),
                    );
                }
                _ => panic!("Invalid Datatype: {} Encountered", data_type),
            }
        }
    }
    msg_desc_inst
}
