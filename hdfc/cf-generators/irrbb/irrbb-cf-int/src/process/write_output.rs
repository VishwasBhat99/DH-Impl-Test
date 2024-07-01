use std::collections::HashMap;
use std::io::BufWriter;

use super::acc_field_names::ReqFieldNames;
use crate::configuration_parameters::ConfigurationParameters;
use protobuf::reflect::ReflectValueBox;
use protobuf::MessageDyn;
use rbdate::{timestamp, DateParser};
use sdb_dyn_proto_rdr::compound_types::Cashflow;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use std::fs::File;
use std::io::Write;

pub fn get_output(
    mut cf_record: AccountWithCFs,
    message_descriptor: &mut protobuf::reflect::MessageDescriptor,
    cf_message_descriptor: &mut protobuf::reflect::MessageDescriptor,
    config_params: &mut ConfigurationParameters,
    op_map: &mut std::collections::HashMap<std::string::String, std::string::String>,
    output_file: &mut BufWriter<File>,
    acc_field_names: &ReqFieldNames,
    date_parser: &DateParser,
    cfs_map: &mut HashMap<String, Vec<Cashflow>>,
    
) {
    let mut msg_desc_inst = message_descriptor.new_instance();
    let mut cf_msg_desc_inst = cf_message_descriptor.new_instance();
    let mut cf_val = match cf_record.remove_cfs_for_key(&"cashflows".to_string()) {
        Ok(value) => value,
        Err(_error) => return,
    };

    let mut vec_of_cfs: Vec<Cashflow> = Vec::new();
    let mut prin_amt = 0.0;
    let mut highest_date = timestamp(date_parser.parse("01-01-1099"));
    let mut broken_int = 0.0;
    let next_rep_date = cf_record
        .get_i64_for_key(&acc_field_names.next_repr_date.to_string())
        .unwrap_or(timestamp(*config_params.as_on_date()));

    let mut next_cf_date = timestamp(date_parser.parse("01-01-3099"));

    if config_params.is_cf_lvl_data() {
        let acc_id = cf_record
            .get_string_for_key(&acc_field_names.acc_id.to_string())
            .expect("Could not get Acc-ID")
            .clone();
        cf_val = cfs_map
            .get(&acc_id.to_string())
            .expect("Error in reading Cashflows")
            .to_owned();
        cfs_map.remove_entry(&acc_id);
    }

    for cf in cf_val {
        if cf.date >= next_rep_date {
            prin_amt += cf.principal_amount;
            if cf.interest_amount > 0.0 && cf.date <= next_cf_date {
                if next_cf_date == cf.date {
                    broken_int += cf.interest_amount;
                } else {
                    broken_int = cf.interest_amount;
                }
                next_cf_date = cf.date;
            }
            continue;
        }
        if cf.date > highest_date && cf.interest_amount > 0.0 {
            highest_date = cf.date;
        }
        vec_of_cfs.push(cf);
    }
    let mut final_cf = Cashflow::new();
    //cf int amt
    if highest_date == timestamp(date_parser.parse("01-01-1099")) {
        let last_cf_date = cf_record
            .get_i64_for_key(&acc_field_names.last_cf_date)
            .unwrap_or(
                cf_record
                    .get_i64_for_key(&acc_field_names.acc_open_date.to_string())
                    .unwrap_or(timestamp(*config_params.as_on_date())),
            );
        if last_cf_date > next_rep_date {
            final_cf.interest_amount = broken_int;
        } else {
            let num_of_days = rbdate::num_days_start_to_end(
                rbdate::date_from_timestamp(last_cf_date),
                rbdate::date_from_timestamp(next_rep_date),
            );
            let num_of_days_last_to_next = rbdate::num_days_start_to_end(
                rbdate::date_from_timestamp(last_cf_date),
                rbdate::date_from_timestamp(next_cf_date),
            );
            final_cf.interest_amount =
                (num_of_days as f64 * broken_int) / num_of_days_last_to_next as f64;
        }
    } else {
        if next_rep_date == highest_date {
            final_cf.interest_amount = broken_int;
        } else {
            let num_of_days = rbdate::num_days_start_to_end(
                rbdate::date_from_timestamp(highest_date),
                rbdate::date_from_timestamp(next_rep_date),
            );
            let num_of_days_last_to_next = rbdate::num_days_start_to_end(
                rbdate::date_from_timestamp(highest_date),
                rbdate::date_from_timestamp(next_cf_date),
            );
            final_cf.interest_amount =
                (num_of_days as f64 * broken_int) / num_of_days_last_to_next as f64;
        }
    }
    final_cf.principal_amount = prin_amt;
    final_cf.date = cf_record
        .get_i64_for_key(&acc_field_names.next_repr_date.to_string())
        .expect("Could not get next repricing date.");
    vec_of_cfs.push(final_cf);
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
            match data_type.as_str() {
                "TYPE_INT64" => {
                    data_field.set_singular_field(
                        &mut *msg_desc_inst,
                        ReflectValueBox::I64(
                            cf_record
                                .get_i64_for_key(
                                    op_map
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
                                    op_map
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
                                    op_map
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
                                    op_map
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
                                    op_map
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
