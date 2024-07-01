use super::{NPAInput, DEFAULT_FLOAT};

pub fn get_op_line(npa_data: NPAInput, base_ccy: &str, tot_amt: &mut f64) -> String {
    let mut op_line = String::new();
    op_line.push_str(&npa_data.print());

    let npa_amt = npa_data.account_balance.parse().unwrap_or(DEFAULT_FLOAT)
        + npa_data.ho_provision.parse().unwrap_or(DEFAULT_FLOAT)
        + npa_data.claim.parse().unwrap_or(DEFAULT_FLOAT)
        - npa_data.ho_balance.parse().unwrap_or(DEFAULT_FLOAT);
    op_line.push_str(&npa_amt.to_string());
    *tot_amt += npa_amt;

    op_line.push('|');
    op_line.push_str(base_ccy);
    op_line.push('\n');

    op_line
}
