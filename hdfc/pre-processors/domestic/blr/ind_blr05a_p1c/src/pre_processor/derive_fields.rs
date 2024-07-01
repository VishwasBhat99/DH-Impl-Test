use super::{BLRInput, DEFAULT_FLOAT};

pub fn get_op_line(
    blr_data: BLRInput,
    as_on_date: &str,
    order_no: usize,
    denomination: f64,
    tot_amt: &mut f64,
) -> String {
    let mut op_line = String::new();
    let opening_bal = blr_data.opening_bal.parse().unwrap_or(DEFAULT_FLOAT) * denomination;
    let highst_price_amt =
        blr_data.highst_price_amt.parse().unwrap_or(DEFAULT_FLOAT) * denomination;
    let lowst_price_amt = blr_data.lowst_price_amt.parse().unwrap_or(DEFAULT_FLOAT) * denomination;
    let closng_amt = blr_data.closng_amt.parse().unwrap_or(DEFAULT_FLOAT) * denomination;
    *tot_amt += closng_amt;

    op_line.push_str(&blr_data.print(
        opening_bal,
        highst_price_amt,
        lowst_price_amt,
        closng_amt,
        as_on_date.to_string(),
    ));

    op_line.push_str(&order_no.to_string());
    op_line.push('|');
    op_line.push('\n');

    op_line
}
