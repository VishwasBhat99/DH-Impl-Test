use super::crm_data::CRMData;
use super::output_data::OutputData;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

pub fn write_data(op_data: OutputData, output_file: &mut BufWriter<File>) {
    write!(output_file, "{}\n", op_data);
}

pub fn write_col_data(op_data: &CRMData, output_file: &mut BufWriter<File>) {
    write!(output_file, "{}\n", op_data.print());
}
