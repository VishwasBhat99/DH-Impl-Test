extern crate rbdate;
extern crate sdb_dyn_proto_rdr;
extern crate sdb_io;

use self::sdb_dyn_proto_rdr::reader;
use self::sdb_dyn_proto_rdr::reader::types::Type;
use self::sdb_io::buf_file_wrtr;
use self::sdb_io::open_file_read;
use std::io::Read;
use std::io::Write;

#[derive(Deserialize)]
struct RequiredFields {
    fields: Vec<String>,
}

#[allow(clippy::unused_io_amount)]
pub fn generate_result(
    cf_file_path: &str,
    cf_fields_file_path: &str,
    required_fields_file_path: &str,
    output_file_path: &str,
    delimiter: &str,
) {
    let mut reader_for_records = reader::Reader::new_at_path(cf_fields_file_path, cf_file_path);
    let reader_for_calling_method = reader::Reader::new_at_path(cf_fields_file_path, cf_file_path);

    let mut buffer_writer =
        buf_file_wrtr(&output_file_path, None).expect("Unable to create writer for output file.");

    let mut required_fields_file =
        open_file_read(required_fields_file_path).expect("Cannot open the required fields file.");

    let mut required_fields_buffer = String::new();
    required_fields_file
        .read_to_string(&mut required_fields_buffer)
        .expect("Cannot read the required fields file.");

    let rf: RequiredFields = serde_json::from_str(&required_fields_buffer[..])
        .expect("Unable to parse the required fields file.");
    let mut check_cashflows: bool = false;
    let record_reader = reader_for_records.iter();
    let mut str_final = String::new();
    let mut string_for_record = String::new();
    let mut records_in_input: i32 = 0;
    let mut records_in_output: i32 = 0;
    let mut number_of_cfs: i32 = 0;
    for mut record in record_reader {
        let mut acc_cf_count = 0;
        records_in_input += 1;
        string_for_record.clear();
        let mut string_for_record_prefix = String::new();
        let mut string_for_cashflows = String::new();
        for field_name in &rf.fields {
            match reader_for_calling_method
                .get_field_type(&field_name)
                .expect("Key type not known.")
            {
                Type::I32 => {
                    let val = match record.get_i32_for_key(&field_name) {
                        Ok(value) => value,
                        Err(_error) => continue,
                    };
                    string_for_record_prefix.push_str(&val.to_string());
                    string_for_record_prefix.push_str(&delimiter);
                }

                Type::I64 => {
                    let val = match record.get_i64_for_key(&field_name) {
                        Ok(value) => value,
                        Err(_error) => continue,
                    };
                    string_for_record_prefix.push_str(&val.to_string());
                    string_for_record_prefix.push_str(&delimiter);
                }

                Type::F32 => {
                    let val = match record.get_f32_for_key(&field_name) {
                        Ok(value) => value,
                        Err(_error) => continue,
                    };
                    string_for_record_prefix.push_str(&val.to_string());
                    string_for_record_prefix.push_str(&delimiter);
                }

                Type::F64 => {
                    let val = match record.get_f64_for_key(&field_name) {
                        Ok(value) => value,
                        Err(_error) => continue,
                    };
                    string_for_record_prefix.push_str(&val.to_string());
                    string_for_record_prefix.push_str(&delimiter);
                }

                Type::String => {
                    let val = match record.get_string_for_key(&field_name) {
                        Ok(value) => value,
                        Err(_error) => continue,
                    };
                    string_for_record_prefix.push_str(val);
                    string_for_record_prefix.push_str(&delimiter);
                }

                Type::Cashflows => {
                    let cashflows = match record.remove_cfs_for_key(&field_name) {
                        Ok(value) => value,
                        Err(_error) => continue,
                    };
                    check_cashflows = true;
                    string_for_cashflows.clear();
                    for cashflow in cashflows {
                        number_of_cfs += 1;
                        acc_cf_count += 1;
                        string_for_cashflows.push('#');
                        string_for_cashflows.push_str(&cashflow.interest_amount.to_string());
                        string_for_cashflows.push_str("~");
                        string_for_cashflows.push_str(&cashflow.principal_amount.to_string());
                        string_for_cashflows.push_str("~");
                        string_for_cashflows.push_str(
                            &naivedate_from_timestamp(cashflow.date)
                                .format("%d-%m-%Y")
                                .to_string(),
                        );
                    }
                }
            }
        }
        if check_cashflows {
            records_in_output += acc_cf_count;//todo better counter way
            str_final = string_for_cashflows.replace(
                "#",
                &(("\n".to_string()) + &string_for_record_prefix.clone()),
            );
            str_final = str_final[1..str_final.len()].to_string();
            string_for_record.push_str(&str_final);
        } else {
            records_in_output += 1;
            string_for_record.push_str(&string_for_record_prefix);
            string_for_record.pop();
        }
        string_for_record.push('\n');
        let record_bytes: &[u8] = string_for_record.as_bytes();

        // std::io::Write::write() returns the number of bytes that have been processed.
        // Since that result value is not being used here, so the lint (clippy::unused_io_amount) is allowed.
        // Now cargo clippy won't flash error message.
        buffer_writer
            .write(record_bytes)
            .expect("Unable to write byte array.");
    }
    buffer_writer.flush().expect("Unable to flush the writer.");
    println!("Records in input: {}", records_in_input);//todo it should be logged
    println!("Records in output: {}", records_in_output);
    println!("Number of cashflow processed: {}", number_of_cfs);
}

pub fn naivedate_from_timestamp(t: i64) -> rbdate::NaiveDate {
    let naive_date_time = rbdate::NaiveDateTime::from_timestamp(t, 0);
    naive_date_time.date()
}
