use super::incr_dt_by_mon_presrv_eom;
use super::{accrued_days_with_convn, days_with_convn, get_int_amt};
use chrono::NaiveDate;
use conventions::Conventions;

#[cfg(test)]
#[test]
pub fn test_invalid_date_range() {
    let st_dt = NaiveDate::from_ymd(2018, 7, 31);
    let end_dt = NaiveDate::from_ymd(2018, 1, 31);
    let num_days = days_with_convn(st_dt, end_dt, &Conventions::ACTbyACT);
    assert!(num_days.is_err())
}

#[test]
pub fn test_actbyact() {
    let st_dt = NaiveDate::from_ymd(2018, 2, 28);
    let end_dt = NaiveDate::from_ymd(2018, 3, 31);
    let num_days = days_with_convn(st_dt, end_dt, &Conventions::ACTbyACT);
    assert_eq!(num_days.unwrap().days_btw_dts, 31)
}

#[test]
pub fn test_actby365() {
    let st_dt = NaiveDate::from_ymd(2018, 2, 28);
    let end_dt = NaiveDate::from_ymd(2018, 3, 31);
    let num_days = days_with_convn(st_dt, end_dt, &Conventions::ACTby365);
    assert_eq!(num_days.unwrap().day_in_yr, 365)
}

#[test]
pub fn test_actby360() {
    let st_dt = NaiveDate::from_ymd(2018, 2, 28);
    let end_dt = NaiveDate::from_ymd(2018, 3, 31);
    let num_days = days_with_convn(st_dt, end_dt, &Conventions::ACTby360);
    assert_eq!(num_days.unwrap().day_in_yr, 360)
}

#[test]
pub fn test_thirtyby360() {
    let st_dt = NaiveDate::from_ymd(2020, 2, 29);
    let end_dt = NaiveDate::from_ymd(2020, 3, 31);
    let num_days = days_with_convn(st_dt, end_dt, &Conventions::Thirtyby360);
    assert_eq!(num_days.unwrap().days_btw_dts, 30)
}

#[test]
pub fn test_increment_dt_by_months() {
    let st_dt = NaiveDate::from_ymd(2020, 1, 31);
    let nxt_dt = incr_dt_by_mon_presrv_eom(st_dt, 1).unwrap();
    let expected_dt = NaiveDate::from_ymd(2020, 2, 29);
    assert_eq!(nxt_dt, expected_dt)
}

#[test]
pub fn test_int_amt_actbyact() {
    let st_dt = NaiveDate::from_ymd(2018, 1, 28);
    let end_dt = NaiveDate::from_ymd(2018, 3, 31);
    let prin_amt = 100.0;
    let int_rate = 10.0;
    let int_amt = get_int_amt(st_dt, end_dt, &Conventions::ACTbyACT, prin_amt, int_rate)
        .expect("Could not calculate interest amount");
    assert_eq!(int_amt, 1.6986301369863013)
}

#[test]
pub fn test_int_amt_actby365() {
    let st_dt = NaiveDate::from_ymd(2020, 2, 28);
    let end_dt = NaiveDate::from_ymd(2020, 3, 31);
    let prin_amt = 100.0;
    let int_rate = 10.0;
    let int_amt = get_int_amt(st_dt, end_dt, &Conventions::ACTby365, prin_amt, int_rate)
        .expect("Could not calculate interest amount");
    assert_eq!(int_amt, 0.8767123287671232)
}

#[test]
pub fn test_int_amt_actby360() {
    let st_dt = NaiveDate::from_ymd(2018, 2, 28);
    let end_dt = NaiveDate::from_ymd(2020, 3, 31);
    let prin_amt = 100.0;
    let int_rate = 10.0;
    let int_amt = get_int_amt(st_dt, end_dt, &Conventions::ACTby360, prin_amt, int_rate)
        .expect("Could not calculate interest amount");
    assert_eq!(int_amt, 21.166666666666668)
}

#[test]
pub fn test_int_amt_thirtyby360() {
    let st_dt = NaiveDate::from_ymd(2020, 2, 29);
    let end_dt = NaiveDate::from_ymd(2020, 3, 31);
    let prin_amt = 100.0;
    let int_rate = 10.0;
    let int_amt = get_int_amt(st_dt, end_dt, &Conventions::Thirtyby360, prin_amt, int_rate)
        .expect("Could not calculate interest amount");
    assert_eq!(int_amt, 0.8333333333333333)
}

#[test]
pub fn accrued_days_test_case1() {
    let st_dt = NaiveDate::from_ymd(2023, 1, 15);
    let end_dt = NaiveDate::from_ymd(2023, 4, 20);
    let accrued_days = accrued_days_with_convn(st_dt, end_dt, 1, &Conventions::AccruedThirtyby360);
    assert_eq!(accrued_days.unwrap().days_btw_dts, 26)
}

#[test]
pub fn accrued_days_test_case2() {
    let st_dt = NaiveDate::from_ymd(2024, 1, 15);
    let end_dt = NaiveDate::from_ymd(2023, 4, 20);
    let accrued_days = accrued_days_with_convn(st_dt, end_dt, 1, &Conventions::AccruedThirtyby360);
    assert_eq!(accrued_days.unwrap().days_btw_dts, 0)
}
#[test]
pub fn accrued_days_test_case3() {
    let st_dt = NaiveDate::from_ymd(2023, 1, 1);
    let end_dt = NaiveDate::from_ymd(2023, 4, 20);
    let accrued_days = accrued_days_with_convn(st_dt, end_dt, 3, &Conventions::AccruedThirtyby360);
    assert_eq!(accrued_days.unwrap().days_btw_dts, 72)
}

#[test]
pub fn accrued_days_test_case4() {
    let st_dt = NaiveDate::from_ymd(2023, 1, 1);
    let end_dt = NaiveDate::from_ymd(2023, 4, 1);
    let accrued_days = accrued_days_with_convn(st_dt, end_dt, 6, &Conventions::AccruedThirtyby360);
    assert_eq!(accrued_days.unwrap().days_btw_dts, 91)
}

#[test]
pub fn accrued_days_test_case5() {
    let st_dt = NaiveDate::from_ymd(2023, 1, 1);
    let end_dt = NaiveDate::from_ymd(2023, 6, 23);
    let accrued_days = accrued_days_with_convn(st_dt, end_dt, 6, &Conventions::AccruedThirtyby360);
    assert_eq!(accrued_days.unwrap().days_btw_dts, 9)
}

#[test]
pub fn accrued_days_test_case6() {
    let st_dt = NaiveDate::from_ymd(2023, 1, 1);
    let end_dt = NaiveDate::from_ymd(2023, 4, 1);
    let accrued_days = accrued_days_with_convn(st_dt, end_dt, 12, &Conventions::AccruedThirtyby360);
    assert_eq!(accrued_days.unwrap().days_btw_dts, 271)
}

#[test]
pub fn accrued_days_test_case7() {
    let st_dt = NaiveDate::from_ymd(2023, 1, 1);
    let end_dt = NaiveDate::from_ymd(2023, 8, 1);
    let accrued_days = accrued_days_with_convn(st_dt, end_dt, 12, &Conventions::AccruedThirtyby360);
    assert_eq!(accrued_days.unwrap().days_btw_dts, 151)
}

#[test]
pub fn accrued_days_test_case8() {
    let st_dt = NaiveDate::from_ymd(2023, 5, 31);
    let end_dt = NaiveDate::from_ymd(2034, 5, 24);
    let accrued_days = accrued_days_with_convn(st_dt, end_dt, 6, &Conventions::AccruedThirtyby360);
    assert_eq!(accrued_days.unwrap().days_btw_dts, 7)
}

#[test]
pub fn accrued_days_test_case9() {
    let st_dt = NaiveDate::from_ymd(2023, 5, 31);
    let end_dt = NaiveDate::from_ymd(2033, 2, 6);
    let accrued_days = accrued_days_with_convn(st_dt, end_dt, 6, &Conventions::AccruedThirtyby360);
    assert_eq!(accrued_days.unwrap().days_btw_dts, 115)
}
