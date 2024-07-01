use chrono::{Duration, NaiveDate};
use rbdate::{incr_dt_by_mon_presrv_eom_checked, num_days_start_to_end};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_bkt_slabs(path: &str, as_on_date: &NaiveDate) -> (i64, i64) {
    let bkt_file = File::open(path).expect("Failed to open bkt schema file");
    let reader = BufReader::new(bkt_file);

    let mut residual_u6m = 0;
    let mut residual_u1y = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            let fields: Vec<&str> = line.split('|').collect();

            if fields.len() >= 10 {
                let mat_id: i64 = fields[1].trim().to_string().parse().unwrap();
                let to_date = calculate_date(as_on_date, fields[4], fields[6]);

                if mat_id == 2 {
                    residual_u6m = num_days_start_to_end(*as_on_date, to_date);
                }
                if mat_id == 3 {
                    residual_u1y = num_days_start_to_end(*as_on_date, to_date);
                }
            }
        }
    }
    (residual_u6m, residual_u1y)
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
