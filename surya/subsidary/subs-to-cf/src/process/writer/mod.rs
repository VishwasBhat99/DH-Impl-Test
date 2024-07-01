use super::cashflow_output::*;
use protobuf::Message;
use std::fs::File;
use std::io::Write;

pub fn get_write_date<'a>(llg_id: &String, cashflows: Vec<Cashflow>) -> OuputAccount {
    let mut output_data = OuputAccount::new();
    output_data.set_llg_id(llg_id.to_string());
    output_data.set_currency("INR".to_string());
    output_data.set_int_rate(0.0);
    let repeated_cashflow = protobuf::RepeatedField::from_vec(cashflows);
    output_data.set_cashflows(repeated_cashflow);
    output_data
}
pub fn write_to_file(mut output_file: &File, output_data: OuputAccount) {
    let account_byte_info = output_data
        .write_length_delimited_to_bytes()
        .expect("unable convert into bytes");
    output_file
        .write_all(&account_byte_info)
        .expect("unable to write to output file");
}
