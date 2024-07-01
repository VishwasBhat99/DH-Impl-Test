use chrono::Datelike;
use chrono::NaiveDate;
use rbdate::{get_days_from_month, decr_dt_by_mon_presrv_eom};

pub struct GetDatesForTwoMonths {
    pub start_date_1: NaiveDate,
    pub end_date_1: NaiveDate,
    pub no_of_days_1: i64,
    pub start_date_2: NaiveDate,
    pub end_date_2: NaiveDate,
    pub no_of_days_2: i64,
}

impl GetDatesForTwoMonths {
    pub fn new(as_on_date: NaiveDate) -> GetDatesForTwoMonths {
        let date = decr_dt_by_mon_presrv_eom(as_on_date, 3).unwrap();
        let start_date_1: NaiveDate = NaiveDate::from_ymd(date.year(), date.month(), 1);
        let no_of_days_1: i64 = get_days_from_month(start_date_1);
        let end_date_1: NaiveDate = NaiveDate::from_ymd(date.year(), date.month(), no_of_days_1 as u32);
        let date = decr_dt_by_mon_presrv_eom(as_on_date, 2).unwrap();
        let start_date_2: NaiveDate = NaiveDate::from_ymd(date.year(), date.month(), 1);
        let no_of_days_2: i64 = get_days_from_month(start_date_2);
        let end_date_2: NaiveDate = NaiveDate::from_ymd(date.year(), date.month(), no_of_days_2 as u32);

        GetDatesForTwoMonths {
            start_date_1,
            end_date_1,
            no_of_days_1,
            start_date_2,
            end_date_2,
            no_of_days_2,
        }
    }
}
