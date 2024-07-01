pub fn calc_weighted_int_rt(
    out_bal_sum: f64,
    int_rt_sum: f64,
    sum_prod_outbal: f64,
    days: f64,
) -> f64 {
    let mut weighted_int_rt = 0.0;
    if out_bal_sum == 0.0 {
        weighted_int_rt = int_rt_sum / days;
    } else {
        weighted_int_rt = sum_prod_outbal / out_bal_sum;
    }

    return weighted_int_rt;
}

pub fn default_avg_amt_calc(out_bal_sum: f64, days: f64) -> f64 {
    return out_bal_sum / days;
}

pub fn int_sub_avg_amt_calc(out_bal_sum: f64, int_posted: f64, days: f64) -> f64 {
    return (out_bal_sum - int_posted) / days;
}

pub fn int_add_avg_amt_calc(out_bal_sum: f64, int_posted: f64, days: f64) -> f64 {
    return (out_bal_sum + int_posted) / days;
}
