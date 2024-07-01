#[allow(unused_imports)]
use super::*;

#[test]
fn test_get_tenor_days_norm() {
    assert_eq!(
        get_tenor_days(
            Ok(NaiveDate::from_ymd(2021, 01, 01)),
            Ok(NaiveDate::from_ymd(2021, 02, 28)),
        ),
        58
    );
}

#[test]
fn test_get_tenor_days_start_gt_end_dt() {
    assert_eq!(
        get_tenor_days(
            Ok(NaiveDate::from_ymd(2021, 01, 01)),
            Ok(NaiveDate::from_ymd(2020, 05, 15)),
        ),
        -1
    );
}

#[test]
fn test_get_tenor_desc_below_7_days() {
    assert_eq!(
        get_tenor_desc(-1, NaiveDate::from_ymd(2020, 05, 15),),
        "below 7 days",
    );
}

#[test]
fn test_get_tenor_desc_bet_7_14() {
    assert_eq!(
        get_tenor_desc(10, NaiveDate::from_ymd(2020, 05, 15),),
        "7 - 14 days",
    );
}

#[test]
fn test_get_tenor_months() {
    assert_eq!(
        get_tenor_months(
            Ok(NaiveDate::from_ymd(2020, 05, 15)),
            Ok(NaiveDate::from_ymd(2020, 09, 21)),
        ),
        4,
    );
}

#[test]
fn test_get_tenor_months_error() {
    assert_eq!(
        get_tenor_months(
            Ok(NaiveDate::from_ymd(2021, 05, 15)),
            Ok(NaiveDate::from_ymd(2020, 09, 21)),
        ),
        0,
    );
}
