use chrono::{Duration, NaiveDate};
use rbdate::{incr_dt_by_mon_presrv_eom_checked, num_days_start_to_end};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
pub struct DateRange {
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
    pub num_days: i64,
}

pub fn get_bkt_slabs(path: &str, as_on_date: &NaiveDate) -> HashMap<i64, DateRange> {
    let bkt_file = File::open(path).expect("Failed to open bkt schema file");
    let reader = BufReader::new(bkt_file);
    let mut mat_id_to_dates = HashMap::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            let fields: Vec<&str> = line.split('|').collect();

            if fields.len() >= 10 {
                let mat_id = fields[1].trim().to_string().parse().unwrap();
                let from_date = calculate_date(as_on_date, fields[3], fields[5]);
                let to_date = calculate_date(as_on_date, fields[4], fields[6]);
                let num_days = num_days_start_to_end(from_date, to_date);
                let date_range = DateRange {
                    from_date: from_date.clone(),
                    to_date: to_date.clone(),
                    num_days: num_days.clone(),
                };
                mat_id_to_dates.insert(mat_id, date_range);
            }
        }
    }
    mat_id_to_dates
}

fn calculate_date(asondate: &NaiveDate, offset: &str, literal: &str) -> NaiveDate {
    let offset = offset.parse::<usize>().unwrap();

    let new_date = match literal {
        "Days" => *asondate + Duration::days(offset as i64),
        "Month" => incr_dt_by_mon_presrv_eom_checked(*asondate, offset).unwrap(),
        "Years" => incr_dt_by_mon_presrv_eom_checked(*asondate, offset * 12).unwrap(),
        _ => panic!("Unknown literal"),
    };
    new_date
}
