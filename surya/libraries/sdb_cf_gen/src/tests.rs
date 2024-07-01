use super::*;

#[test]
fn test_calc_emi_amt() {
    assert_eq!(calc_emi_amt(18949.0, 7.65, 240.0, 0), 155.0);
}

#[test]
fn test_calc_emi_amt2() {
    assert_eq!(calc_emi_amt(75155.0, 8.35, 180.0, 0), 734.0);
}

#[test]
fn test_calc_emi_amt_nan() {
    assert!(calc_emi_amt(75155.0, 0.0, 180.0, 0).is_nan());
}

#[test]
fn test_get_freq_monthly() {
    assert_eq!(get_freq("Monthly"), 1);
}

#[test]
fn test_get_freq_bi_monthly() {
    assert_eq!(get_freq("BI-Monthly"), 2);
}

#[test]
fn test_get_freq_half_yearly() {
    assert_eq!(get_freq("Half Yearly"), 6);
}
