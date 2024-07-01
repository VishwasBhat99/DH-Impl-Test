use rbdate::incr_dt_by_mon_presrv_eom;
use rbdate::NaiveDate;

pub fn add_days(date: &NaiveDate, days: &u8) -> NaiveDate {
    let mut new_date = date.clone();
    let mut days_to_add = days.clone();
    loop {
        if days_to_add <= 0 {
            break;
        } else {
            new_date = new_date.succ();
            days_to_add -= 1;
        }
    }
    return new_date;
}

pub fn get_maturity_date(acc_open_date: &NaiveDate, tenor: &str) -> NaiveDate {
    if tenor.contains("D") {
        let days: u8 = tenor
            .trim_matches('D')
            .parse::<u8>()
            .expect("Invalid tenor 'D' format");
        add_days(acc_open_date, &days)
    } else if tenor.contains("M") {
        let months: usize = tenor
            .trim_matches('M')
            .parse::<usize>()
            .expect("Invalid tenor 'M' format");
        incr_dt_by_mon_presrv_eom(*acc_open_date, months)
            .expect("Cannot add month to as on date as per prd slab config")
    } else if tenor.contains("Y") {
        let years: usize = tenor
            .trim_matches('Y')
            .parse::<usize>()
            .expect("Invalid from year format");
        incr_dt_by_mon_presrv_eom(*acc_open_date, years * 12)
            .expect("Cannot add month to as on date as per prd slab config")
    } else {
        panic!("Invalid period type in prd config file.");
    }
}
