use super::*;

pub fn get_op_line(input_fields: InputFields, tot_amt: &mut f64) -> String {
    let mut op_line = String::new();
    op_line.push_str(&input_fields.print("01-01-2099"));

    *tot_amt += input_fields.tot_mk_val_of_col;

    op_line
}
