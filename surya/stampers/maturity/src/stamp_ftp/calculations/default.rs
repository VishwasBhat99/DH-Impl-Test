use super::*;

pub fn stamp_default(one_acc_out: &mut OneAccountView) {
    for _ in 0..6 {
        one_acc_out.adj_codes.push(0);
        one_acc_out.adj_rates.push(0.0);
    }
}
