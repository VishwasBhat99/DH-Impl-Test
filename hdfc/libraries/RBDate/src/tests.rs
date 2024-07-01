use super::{
    decr_dt_by_mon_presrv_eom, get_days_from_month, get_month_end_date, get_num_days_from_add,
    incr_dt_by_mon_presrv_eom, increment_date_by_months, increment_date_by_months_unchecked,
    naivedatetime_to_timestamp, num_days_start_to_end, timestamp_to_naivedate,
    timestamp_to_naivedatetime, DateParser,
};
use chrono::{NaiveDate, NaiveDateTime};

#[test]
pub fn test_month_addition() {
    for c in get_test_candidates() {
        assert_eq!(
            c.expected_date,
            increment_date_by_months(c.date, c.difference)
        );
    }
}

#[test]
pub fn test_unchecked_month_addition() {
    for c in get_test_candidates() {
        assert_eq!(
            c.expected_date,
            increment_date_by_months(c.date, c.difference)
        );
    }
}

// MARK: Test Candidates

struct TestCandidate {
    date: NaiveDate,
    difference: u16,
    expected_date: NaiveDate,
}

impl TestCandidate {
    fn new(d: NaiveDate, diff: u16, e: NaiveDate) -> TestCandidate {
        TestCandidate {
            date: d,
            difference: diff,
            expected_date: e,
        }
    }
}

fn get_test_candidates() -> Vec<TestCandidate> {
    let c1 = TestCandidate::new(
        NaiveDate::from_ymd(2007, 7, 15),
        1,
        NaiveDate::from_ymd(2007, 8, 15),
    );

    let c2 = TestCandidate::new(
        NaiveDate::from_ymd(2007, 7, 15),
        25,
        NaiveDate::from_ymd(2009, 8, 15),
    );

    let c3 = TestCandidate::new(
        NaiveDate::from_ymd(2008, 2, 29),
        12,
        NaiveDate::from_ymd(2009, 2, 28),
    );

    let c4 = TestCandidate::new(
        NaiveDate::from_ymd(2007, 3, 31),
        1,
        NaiveDate::from_ymd(2007, 4, 30),
    );

    vec![c1, c2, c3, c4]
}

#[test]
fn test_incr_dt_by_month_genral_case() {
    let date = NaiveDate::from_ymd(2018, 2, 27);
    let next_dt = incr_dt_by_mon_presrv_eom(date, 3);
    assert_eq!(next_dt, NaiveDate::from_ymd_opt(2018, 5, 27));
}

#[test]
fn test_incr_dt_by_month_check_feb_leap_year() {
    let date = NaiveDate::from_ymd(2019, 12, 31);
    let next_dt = incr_dt_by_mon_presrv_eom(date, 2);
    assert_eq!(next_dt, NaiveDate::from_ymd_opt(2020, 2, 29));
}

#[test]
fn test_incr_dt_by_month_check_feb_no_leap_year() {
    let date = NaiveDate::from_ymd(2018, 12, 31);
    let next_dt = incr_dt_by_mon_presrv_eom(date, 2);
    assert_eq!(next_dt, NaiveDate::from_ymd_opt(2019, 2, 28));
}

#[test]
fn test_incr_dt_by_month_30_day_month() {
    let date = NaiveDate::from_ymd(2019, 11, 30);
    let next_dt = incr_dt_by_mon_presrv_eom(date, 1);
    assert_eq!(next_dt, NaiveDate::from_ymd_opt(2019, 12, 31));
}

#[test]
fn test_incr_dt_by_month_31_day_month() {
    let date = NaiveDate::from_ymd(2019, 12, 31);
    let next_dt = incr_dt_by_mon_presrv_eom(date, 1);
    assert_eq!(next_dt, NaiveDate::from_ymd_opt(2020, 1, 31));
}

#[test]
fn test_incr_dt_by_month_29_day_leap_month() {
    let date = NaiveDate::from_ymd(2020, 02, 29);
    let next_dt = incr_dt_by_mon_presrv_eom(date, 1);
    assert_eq!(next_dt, NaiveDate::from_ymd_opt(2020, 3, 31));
}

#[test]
fn test_decr_dt_by_month_presrv_eom() {
    let date = NaiveDate::from_ymd(2019, 1, 1);
    let next_dt = decr_dt_by_mon_presrv_eom(date, 3);
    assert_eq!(next_dt, NaiveDate::from_ymd_opt(2018, 10, 1));
}

#[test]
fn test_num_days_start_greater_end() {
    assert_eq!(
        -1,
        num_days_start_to_end(
            NaiveDate::from_ymd(2049, 04, 25),
            NaiveDate::from_ymd(2029, 04, 25)
        )
    );
}

#[test]
fn test_num_days_start_end_10_years() {
    assert_eq!(
        3653,
        num_days_start_to_end(
            NaiveDate::from_ymd(2019, 04, 25),
            NaiveDate::from_ymd(2029, 04, 25)
        )
    );
}

#[test]
fn test_num_days_start_end_leap_year() {
    assert_eq!(
        310,
        num_days_start_to_end(
            NaiveDate::from_ymd(2019, 04, 25),
            NaiveDate::from_ymd(2020, 02, 29)
        )
    );
}

#[test]
fn test_num_days_start_equal_end() {
    assert_eq!(
        0,
        num_days_start_to_end(
            NaiveDate::from_ymd(2019, 04, 25),
            NaiveDate::from_ymd(2019, 04, 25)
        )
    );
}

#[test]
fn test_max_day_in_month() {
    assert_eq!(30, get_days_from_month(NaiveDate::from_ymd(2019, 04, 25)));
}

#[test]
fn test_max_day_in_month_with_leap_year() {
    assert_eq!(29, get_days_from_month(NaiveDate::from_ymd(2020, 02, 25)));
}

#[test]
fn test_max_day_in_month_without_leap_year() {
    assert_eq!(28, get_days_from_month(NaiveDate::from_ymd(2019, 02, 25)));
}

#[test]
fn test_max_day_in_month_with_31_days() {
    assert_eq!(31, get_days_from_month(NaiveDate::from_ymd(2020, 01, 25)));
}

#[test]
fn test_get_month_end_date_with_leap_year() {
    assert_eq!(
        NaiveDate::from_ymd(2020, 02, 29),
        get_month_end_date(NaiveDate::from_ymd(2020, 02, 01))
    );
}

#[test]
fn test_get_month_end_date_without_leap_year() {
    assert_eq!(
        NaiveDate::from_ymd(2019, 02, 28),
        get_month_end_date(NaiveDate::from_ymd(2019, 02, 01))
    );
}

#[test]
fn test_get_month_end_date_random_date() {
    assert_eq!(
        NaiveDate::from_ymd(2019, 12, 31),
        get_month_end_date(NaiveDate::from_ymd(2019, 12, 15))
    );
}

#[test]
fn test_get_days_from_duartion() {
    assert_eq!(
        1984,
        get_num_days_from_add(NaiveDate::from_ymd(2020, 02, 29), 5, 5, 7)
    );
}

#[test]
fn test_get_days_from_day() {
    assert_eq!(
        7,
        get_num_days_from_add(NaiveDate::from_ymd(2020, 05, 15), 0, 0, 7)
    );
}

#[test]
fn test_get_days_from_day_with_leap_yr() {
    assert_eq!(
        30,
        get_num_days_from_add(NaiveDate::from_ymd(2020, 01, 31), 0, 0, 30)
    );
}

#[test]
fn test_get_days_from_day_without_leap_yr() {
    assert_eq!(
        29,
        get_num_days_from_add(NaiveDate::from_ymd(2021, 01, 31), 0, 0, 29)
    );
}

#[test]
#[should_panic(expected = "Could not parse date string.")]
fn test_parse_datetime_invalid_date() {
    let parser = DateParser::new("%d-%m-%Y %H:%M:%S".to_string(), false);
    parser.parse_datetime("2022-01-01 12:00:00");
}

#[test]
fn test_timestamp_to_naivedatetime() {
    let timestamp = 1671677400;
    let expected_datetime =
        NaiveDateTime::parse_from_str("2023-05-15 23:30:00", "%Y-%m-%d %H:%M:%S").unwrap();
    assert_eq!(timestamp_to_naivedatetime(timestamp), expected_datetime);
}

#[test]
fn test_naivedatetime_to_timestamp() {
    let datetime =
        NaiveDateTime::parse_from_str("2023-05-15 23:30:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let expected_timestamp = 1671677400;
    assert_eq!(naivedatetime_to_timestamp(datetime), expected_timestamp);
}

#[test]
fn test_timestamp_to_naivedate() {
    let timestamp = 1671677400;
    let expected_date = NaiveDate::parse_from_str("2023-05-15", "%Y-%m-%d").unwrap();
    assert_eq!(timestamp_to_naivedate(timestamp), expected_date);
}
#[test]
fn test_is_month_end_date() {
    let date = NaiveDate::parse_from_str("2023-05-31", "%Y-%m-%d").unwrap();
    assert!(is_month_end_date(date));
}
