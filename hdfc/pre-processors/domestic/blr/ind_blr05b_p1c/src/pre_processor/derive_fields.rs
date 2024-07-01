use super::{BLRInput, DEFAULT_FLOAT};

pub fn get_op_line(
    blr_data: BLRInput,
    as_on_date: String,
    order_no: usize,
    denomination: f64,
    tot_amt: &mut f64,
) -> String {
    let mut op_line = String::new();
    let face_value = blr_data.face_value.parse().unwrap_or(DEFAULT_FLOAT) * denomination;
    let amount_outstanding =
        blr_data.amount_outstanding.parse().unwrap_or(DEFAULT_FLOAT) * denomination;
    op_line.push_str(&blr_data.print(face_value, amount_outstanding, as_on_date));

    *tot_amt += amount_outstanding;
    op_line.push_str(&order_no.to_string());
    op_line.push('|');
    op_line.push('\n');

    op_line
}
