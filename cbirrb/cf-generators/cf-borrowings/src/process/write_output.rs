use super::acc_field_names::ReqFieldNames;
use crate::configuration_parameters::ConfigurationParameters;
use crate::statics::{DEFAULT_FLOAT, DEFAULT_FLOAT_32, DEFAULT_INT, DEFAULT_INT_64};
use chrono::{Datelike, NaiveDate};
use protobuf::reflect::ReflectValueBox;
use protobuf::MessageDyn;
use rbdate::{get_month_end_date, increment_date_by_months, timestamp};
use sdb_dyn_proto_rdr::compound_types::Cashflow;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

pub fn get_output(
    input_vec: &mut [&str],
    message_descriptor: &mut protobuf::reflect::MessageDescriptor,
    cf_message_descriptor: &mut protobuf::reflect::MessageDescriptor,
    config_params: &mut ConfigurationParameters,
    op_map: &mut std::collections::HashMap<std::string::String, i32>,
    date_fields_vec: &mut [String],
    output_file: &mut BufWriter<File>,
    acc_field_names: &ReqFieldNames,
) {
    let mut msg_desc_inst = message_descriptor.new_instance();
    let mut cf_msg_desc_inst = cf_message_descriptor.new_instance();
    let mut vec_of_cfs: Vec<Cashflow> = Vec::new();

    let instalment_start_date_field = message_descriptor
        .field_by_name(&acc_field_names.instalment_start_date)
        .expect("error getting `instalment_start_date` field from MessageDescriptor");
    let no_of_instalments_field = message_descriptor
        .field_by_name(&acc_field_names.no_of_instalments)
        .expect("error getting `no_of_instalments` field from MessageDescriptor");
    let instalment_freq_field = message_descriptor
        .field_by_name(&acc_field_names.instalment_freq)
        .expect("error getting `instalment_freq` field from MessageDescriptor");
    let amount_field = message_descriptor
        .field_by_name(&acc_field_names.amount)
        .expect("error getting `amount` field from MessageDescriptor");

    let instalment_start_date_field = NaiveDate::parse_from_str(
        input_vec[*op_map
            .get(&instalment_start_date_field.to_string())
            .unwrap_or(&DEFAULT_INT) as usize
            - 1],
        config_params.input_date_format(),
    )
    .unwrap_or(*config_params.as_on_date());

    let no_of_instalments_field = input_vec[*op_map
        .get(&no_of_instalments_field.to_string())
        .unwrap_or(&DEFAULT_INT) as usize
        - 1]
    .parse::<i64>()
    .expect("Unable to get no_of_installments field");
    let instalment_freq = match op_map.get(&instalment_freq_field.to_string()) {
        Some(val) => input_vec[*val as usize - 1],
        None => "M",
    };
    let amount = input_vec[*op_map
        .get(&amount_field.to_string())
        .unwrap_or(&DEFAULT_INT) as usize
        - 1]
    .parse::<f64>()
    .unwrap_or(DEFAULT_FLOAT);
    let mut is_month_end = false;
    let mut last_installment_date = instalment_start_date_field;
    let month_end_date = get_month_end_date(last_installment_date);
    if last_installment_date.day() == month_end_date.day() {
        is_month_end = true;
    }
    let freq = match instalment_freq {
        "M" => 1,
        "Q" => 3,
        "H" => 6,
        "Y" => 12,
        _ => 0,
    };
    let mut cf = Cashflow::new();
    cf.date = timestamp(last_installment_date);
    cf.principal_amount = amount;
    cf.interest_amount = DEFAULT_FLOAT;
    vec_of_cfs.push(cf);
    for _ in 0..no_of_instalments_field - 1 {
        let mut cf = Cashflow::new();
        last_installment_date = increment_date_by_months(last_installment_date, freq);
        if is_month_end {
            last_installment_date = get_month_end_date(last_installment_date);
        }

        cf.date = timestamp(last_installment_date);
        cf.principal_amount = amount;
        cf.interest_amount = DEFAULT_FLOAT;
        vec_of_cfs.push(cf);
    }

    let mut cf_vec: Vec<Box<dyn MessageDyn>> = Vec::new();
    for val in vec_of_cfs.iter() {
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
                                input_vec[*op_map
                                    .get(&data_field.to_string())
                                    .unwrap_or(&DEFAULT_INT)
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
                                input_vec[*op_map
                                    .get(&data_field.to_string())
                                    .unwrap_or(&DEFAULT_INT)
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
                                input_vec[*op_map
                                    .get(&data_field.to_string())
                                    .unwrap_or(&DEFAULT_INT)
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
                                input_vec[*op_map
                                    .get(&data_field.to_string())
                                    .unwrap_or(&DEFAULT_INT)
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
                                input_vec[*op_map
                                    .get(&data_field.to_string())
                                    .unwrap_or(&DEFAULT_INT)
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
                                input_vec[*op_map
                                    .get(&data_field.to_string())
                                    .unwrap_or(&DEFAULT_INT)
                                    as usize
                                    - 1]
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
