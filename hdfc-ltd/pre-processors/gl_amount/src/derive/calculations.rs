pub fn get_distribution_amount(amt: f64, limit: f64, tot_gl_amt: f64) -> f64 {
    (limit / tot_gl_amt) * amt
}
